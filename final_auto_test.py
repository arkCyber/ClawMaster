#!/usr/bin/env python3
"""
新闻工具最终自动化测试
验证所有修复是否生效
"""

import json
import time
import subprocess
import sys
import re
from datetime import datetime

# 测试配置
WEBUI_URL = "https://localhost:59233"
LOG_FILE = "/tmp/clawmaster_final.log"

# 测试案例
TEST_CASES = [
    {
        "name": "美国新闻（中文）",
        "query": "美国新闻",
        "expected_tool": "news_search",
        "expected_in_response": ["tool_call", "news_search", "USA"],
        "forbidden_in_response": ["抱歉", "无法", "cannot", "I cannot"],
    },
    {
        "name": "上海新闻（中文城市）",
        "query": "上海新闻",
        "expected_tool": "news_search",
        "expected_in_response": ["tool_call", "news_search", "Shanghai"],
        "forbidden_in_response": ["抱歉", "无法"],
    },
    {
        "name": "科技新闻（英文）",
        "query": "latest tech news",
        "expected_tool": "news_search",
        "expected_in_response": ["tool_call", "news_search"],
        "forbidden_in_response": ["cannot", "unable"],
    },
    {
        "name": "计算测试（对照组）",
        "query": "请使用 calc 工具计算 2+2",
        "expected_tool": "calc",
        "expected_in_response": ["tool_call", "calc", "2+2"],
        "forbidden_in_response": ["抱歉", "无法"],
    },
]

def print_header(text):
    """打印标题"""
    print(f"\n{'='*80}")
    print(f"  {text}")
    print(f"{'='*80}\n")

def print_section(text):
    """打印章节"""
    print(f"\n{'─'*80}")
    print(f"  {text}")
    print(f"{'─'*80}")

def check_system_ready():
    """检查系统是否就绪"""
    print("🔍 检查系统状态...")
    
    # 检查进程
    try:
        result = subprocess.run(
            ["pgrep", "-f", "clawmaster"],
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            print("❌ ClawMaster 未运行")
            return False
        print("✅ ClawMaster 进程运行中")
    except Exception as e:
        print(f"❌ 检查进程失败: {e}")
        return False
    
    # 检查日志文件
    try:
        with open(LOG_FILE, "r") as f:
            log_content = f.read()
        if "listening" in log_content:
            print("✅ 服务已启动")
        else:
            print("⚠️  服务可能未完全启动")
    except Exception as e:
        print(f"⚠️  无法读取日志: {e}")
    
    # 检查 API
    try:
        result = subprocess.run(
            ["curl", "-s", "-k", f"{WEBUI_URL}/api/gon"],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0 and len(result.stdout) > 0:
            print("✅ API 可访问")
            return True
        else:
            print("❌ API 不可访问")
            return False
    except Exception as e:
        print(f"❌ API 检查失败: {e}")
        return False

def get_log_tail(lines=200):
    """获取日志尾部"""
    try:
        result = subprocess.run(
            ["tail", f"-{lines}", LOG_FILE],
            capture_output=True,
            text=True,
            timeout=5
        )
        return result.stdout
    except Exception as e:
        return f"无法读取日志: {e}"

def analyze_log_for_tool_call(query, log_content):
    """分析日志中的工具调用"""
    results = {
        "tool_call_found": False,
        "tool_name": None,
        "tool_calls_count": 0,
        "response_text": None,
        "has_forbidden_text": False,
        "log_excerpts": []
    }
    
    # 搜索 tool_calls_count
    tool_count_matches = re.findall(r'tool_calls_count=(\d+)', log_content)
    if tool_count_matches:
        results["tool_calls_count"] = int(tool_count_matches[-1])
        results["tool_call_found"] = results["tool_calls_count"] > 0
    
    # 搜索工具名称
    tool_name_matches = re.findall(r'tool["\s:]+([a-z_]+)', log_content, re.IGNORECASE)
    if tool_name_matches:
        results["tool_name"] = tool_name_matches[-1]
    
    # 搜索响应文本
    response_matches = re.findall(r'response=([^\n]+)', log_content)
    if response_matches:
        results["response_text"] = response_matches[-1][:200]
        
        # 检查禁止的文本
        forbidden_patterns = ["抱歉", "无法", "cannot", "unable", "I cannot"]
        for pattern in forbidden_patterns:
            if pattern in results["response_text"]:
                results["has_forbidden_text"] = True
                break
    
    # 提取相关日志行
    lines = log_content.split('\n')
    for i, line in enumerate(lines):
        if 'tool_call' in line.lower() or 'news_search' in line or query in line:
            results["log_excerpts"].append(line.strip())
    
    return results

def run_test_case(test_case, test_num, total_tests):
    """运行单个测试案例"""
    print_section(f"测试 {test_num}/{total_tests}: {test_case['name']}")
    print(f"  查询: {test_case['query']}")
    print(f"  预期工具: {test_case['expected_tool']}")
    
    # 清空旧日志（标记测试开始）
    marker = f"=== TEST {test_num} START: {test_case['name']} ==="
    try:
        with open(LOG_FILE, "a") as f:
            f.write(f"\n{marker}\n")
    except:
        pass
    
    # 等待一下让系统准备好
    time.sleep(2)
    
    # 记录测试开始时间
    start_time = time.time()
    
    # 注意：这里我们通过观察日志来判断，因为直接 API 调用比较复杂
    # 实际测试需要用户在 WebUI 中输入查询
    print(f"\n  ⏳ 请在 WebUI 中输入: {test_case['query']}")
    print(f"  等待 15 秒以捕获响应...")
    
    # 等待响应
    time.sleep(15)
    
    # 获取日志
    log_content = get_log_tail(300)
    
    # 分析日志
    analysis = analyze_log_for_tool_call(test_case['query'], log_content)
    
    # 判断结果
    passed = True
    reasons = []
    
    # 检查工具调用
    if analysis["tool_calls_count"] == 0:
        passed = False
        reasons.append("❌ 未检测到工具调用 (tool_calls_count=0)")
    else:
        print(f"  ✅ 检测到 {analysis['tool_calls_count']} 次工具调用")
    
    # 检查工具名称
    if analysis["tool_name"] and test_case["expected_tool"] in analysis["tool_name"]:
        print(f"  ✅ 调用了正确的工具: {analysis['tool_name']}")
    elif analysis["tool_name"]:
        passed = False
        reasons.append(f"❌ 调用了错误的工具: {analysis['tool_name']} (预期: {test_case['expected_tool']})")
    
    # 检查禁止的文本
    if analysis["has_forbidden_text"]:
        passed = False
        reasons.append("❌ 响应包含禁止的文本（如'抱歉'、'无法'）")
    else:
        print(f"  ✅ 响应不包含禁止的文本")
    
    # 检查响应内容
    if analysis["response_text"]:
        print(f"  📝 响应摘要: {analysis['response_text'][:100]}...")
    
    # 显示日志摘录
    if analysis["log_excerpts"]:
        print(f"\n  🔍 相关日志 (最近 5 条):")
        for excerpt in analysis["log_excerpts"][-5:]:
            print(f"    {excerpt[:120]}")
    
    # 总结
    elapsed = time.time() - start_time
    print(f"\n  ⏱️  耗时: {elapsed:.1f}秒")
    
    if passed:
        print(f"  ✅ 测试通过")
    else:
        print(f"  ❌ 测试失败")
        for reason in reasons:
            print(f"    {reason}")
    
    return {
        "test_case": test_case["name"],
        "query": test_case["query"],
        "passed": passed,
        "reasons": reasons,
        "analysis": analysis,
        "elapsed": elapsed
    }

def run_automated_mode():
    """自动模式：通过脚本直接发送查询"""
    print("\n⚠️  注意：自动模式需要实现 WebSocket 或 RPC 客户端")
    print("当前版本需要手动在 WebUI 中输入查询")
    return False

def main():
    """主函数"""
    print_header("新闻工具最终自动化测试")
    print(f"开始时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"WebUI: {WEBUI_URL}")
    print(f"日志: {LOG_FILE}")
    
    # 检查系统
    if not check_system_ready():
        print("\n❌ 系统未就绪，请先启动 ClawMaster")
        return 1
    
    print("\n" + "="*80)
    print("  测试模式说明")
    print("="*80)
    print("""
此测试需要您的配合：
1. 保持 WebUI 打开 (https://localhost:59233)
2. 当提示时，在 WebUI 中输入指定的查询
3. 等待 15 秒让系统捕获响应
4. 脚本会自动分析日志并判断结果

或者，您可以先在 WebUI 中完成所有测试，然后运行此脚本分析日志。
    """)
    
    input("\n按 Enter 开始测试...")
    
    # 运行测试
    print_header("开始测试")
    
    all_results = []
    for i, test_case in enumerate(TEST_CASES, 1):
        result = run_test_case(test_case, i, len(TEST_CASES))
        all_results.append(result)
        
        # 测试之间等待
        if i < len(TEST_CASES):
            print("\n  ⏳ 等待 3 秒后继续...")
            time.sleep(3)
    
    # 生成报告
    print_header("测试总结")
    
    passed_count = sum(1 for r in all_results if r["passed"])
    total_count = len(all_results)
    pass_rate = (passed_count / total_count * 100) if total_count > 0 else 0
    
    print(f"\n总测试数: {total_count}")
    print(f"通过: {passed_count} ✅")
    print(f"失败: {total_count - passed_count} ❌")
    print(f"通过率: {pass_rate:.1f}%")
    
    print("\n详细结果:")
    for result in all_results:
        status = "✅ 通过" if result["passed"] else "❌ 失败"
        print(f"  {status} - {result['test_case']}")
        if not result["passed"] and result["reasons"]:
            for reason in result["reasons"]:
                print(f"      {reason}")
    
    # 关键发现
    print_header("关键发现")
    
    tool_call_count = sum(1 for r in all_results if r["analysis"]["tool_calls_count"] > 0)
    news_tool_count = sum(1 for r in all_results if r["analysis"]["tool_name"] and "news" in r["analysis"]["tool_name"])
    forbidden_text_count = sum(1 for r in all_results if r["analysis"]["has_forbidden_text"])
    
    print(f"\n工具调用统计:")
    print(f"  - 检测到工具调用: {tool_call_count}/{total_count}")
    print(f"  - 调用 news_search: {news_tool_count}/{total_count - 1}")  # 减去 calc 测试
    print(f"  - 包含禁止文本: {forbidden_text_count}/{total_count}")
    
    if passed_count == total_count:
        print_header("🎉 所有测试通过！")
        print("\n新闻工具修复成功！")
        print("LLM 现在能够正确调用 news_search 工具。")
    elif passed_count > 0:
        print_header("⚠️  部分测试通过")
        print(f"\n{passed_count}/{total_count} 测试通过")
        print("需要进一步调试失败的测试。")
    else:
        print_header("❌ 所有测试失败")
        print("\n修复未生效，需要进一步调试。")
        print("\n可能的原因:")
        print("1. 系统未正确重启")
        print("2. 修改未正确编译")
        print("3. LLM 模型能力限制")
        print("4. 需要更强的提示")
    
    # 保存报告
    report_file = f"/tmp/news_test_report_{int(time.time())}.json"
    with open(report_file, "w") as f:
        json.dump(all_results, f, indent=2, ensure_ascii=False)
    print(f"\n📄 详细报告已保存: {report_file}")
    
    print(f"\n完成时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"{'='*80}\n")
    
    return 0 if passed_count == total_count else 1

if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("\n\n⚠️  测试被用户中断")
        sys.exit(1)
    except Exception as e:
        print(f"\n\n❌ 测试出错: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
