#!/usr/bin/env python3
"""
新闻工具自动化测试脚本
使用真实案例测试 news_search 工具是否被 LLM 正确调用
"""

import json
import time
import subprocess
import sys
from datetime import datetime

# 测试案例
TEST_CASES = [
    {
        "name": "美国新闻查询",
        "query": "美国新闻",
        "expected_tool": "news_search",
        "expected_params": {"location": "USA"},
        "description": "测试中文地理位置识别"
    },
    {
        "name": "上海新闻查询",
        "query": "上海新闻",
        "expected_tool": "news_search",
        "expected_params": {"location": "Shanghai, China"},
        "description": "测试中文城市识别"
    },
    {
        "name": "科技新闻查询",
        "query": "latest tech news",
        "expected_tool": "news_search",
        "expected_params": {"category": "technology"},
        "description": "测试英文类别识别"
    },
    {
        "name": "世界新闻查询",
        "query": "world news",
        "expected_tool": "news_search",
        "expected_params": {"query": "world news"},
        "description": "测试通用新闻查询"
    },
    {
        "name": "明确工具调用",
        "query": "请使用 news_search 工具查询美国新闻",
        "expected_tool": "news_search",
        "expected_params": {"location": "USA"},
        "description": "测试明确指定工具"
    }
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

def check_clawmaster_running():
    """检查 ClawMaster 是否运行"""
    try:
        result = subprocess.run(
            ["pgrep", "-f", "clawmaster"],
            capture_output=True,
            text=True
        )
        return result.returncode == 0
    except Exception as e:
        print(f"❌ 检查进程失败: {e}")
        return False

def tail_log(lines=50):
    """获取日志最后几行"""
    try:
        result = subprocess.run(
            ["tail", f"-{lines}", "/tmp/clawmaster.log"],
            capture_output=True,
            text=True,
            timeout=5
        )
        return result.stdout
    except Exception as e:
        return f"无法读取日志: {e}"

def search_log_for_tool_call(query_text, timeout=10):
    """在日志中搜索工具调用"""
    print(f"  等待 {timeout} 秒以捕获日志...")
    time.sleep(timeout)
    
    log_content = tail_log(200)
    
    # 搜索关键词
    tool_calls = []
    news_mentions = []
    
    for line in log_content.split('\n'):
        if 'news_search' in line.lower():
            tool_calls.append(line)
        if 'tool_call' in line.lower() or 'tool_use' in line.lower():
            tool_calls.append(line)
        if '无法实时获取新闻' in line or 'cannot get real-time news' in line.lower():
            news_mentions.append(line)
    
    return {
        'tool_calls': tool_calls,
        'news_mentions': news_mentions,
        'full_log': log_content
    }

def test_via_curl(query):
    """通过 curl 发送测试查询"""
    session_key = f"auto-test-{int(time.time())}"
    
    # 构建 JSON payload
    payload = {
        "_session_key": session_key,
        "content": query,
        "stream": False
    }
    
    print(f"  发送查询: {query}")
    print(f"  会话 ID: {session_key}")
    
    # 使用 curl 发送请求
    try:
        result = subprocess.run([
            "curl", "-s", "-k", "-X", "POST",
            "https://localhost:59233/api/chat.send",
            "-H", "Content-Type: application/json",
            "-d", json.dumps(payload)
        ], capture_output=True, text=True, timeout=30)
        
        print(f"  HTTP 响应: {result.stdout[:200]}...")
        return result.stdout
    except subprocess.TimeoutExpired:
        print("  ⚠️  请求超时")
        return None
    except Exception as e:
        print(f"  ❌ 请求失败: {e}")
        return None

def analyze_response(response, log_data):
    """分析响应和日志"""
    results = {
        'tool_called': False,
        'news_search_called': False,
        'got_news': False,
        'error_message': False,
        'details': []
    }
    
    # 检查日志中的工具调用
    if log_data['tool_calls']:
        results['tool_called'] = True
        results['details'].append(f"发现 {len(log_data['tool_calls'])} 条工具调用日志")
        
        for call in log_data['tool_calls']:
            if 'news_search' in call:
                results['news_search_called'] = True
                results['details'].append("✅ news_search 工具被调用")
    
    # 检查是否有错误消息
    if log_data['news_mentions']:
        results['error_message'] = True
        results['details'].append("❌ 发现 '无法获取新闻' 消息")
    
    # 检查响应内容
    if response:
        if 'news' in response.lower() or '新闻' in response:
            results['got_news'] = True
            results['details'].append("响应包含新闻相关内容")
        if 'cannot' in response.lower() or '无法' in response:
            results['error_message'] = True
            results['details'].append("响应包含错误消息")
    
    return results

def run_test_case(test_case):
    """运行单个测试案例"""
    print_section(f"测试案例: {test_case['name']}")
    print(f"  描述: {test_case['description']}")
    print(f"  查询: {test_case['query']}")
    print(f"  预期工具: {test_case['expected_tool']}")
    
    # 发送查询
    response = test_via_curl(test_case['query'])
    
    # 搜索日志
    log_data = search_log_for_tool_call(test_case['query'])
    
    # 分析结果
    results = analyze_response(response, log_data)
    
    # 打印结果
    print("\n  📊 测试结果:")
    print(f"    工具被调用: {'✅' if results['tool_called'] else '❌'}")
    print(f"    news_search 被调用: {'✅' if results['news_search_called'] else '❌'}")
    print(f"    获得新闻: {'✅' if results['got_news'] else '❌'}")
    print(f"    错误消息: {'❌' if results['error_message'] else '✅ 无'}")
    
    if results['details']:
        print("\n  详细信息:")
        for detail in results['details']:
            print(f"    - {detail}")
    
    # 显示相关日志
    if log_data['tool_calls']:
        print("\n  🔍 工具调用日志:")
        for call in log_data['tool_calls'][:5]:  # 只显示前5条
            print(f"    {call.strip()}")
    
    # 判断是否通过
    passed = results['news_search_called'] and not results['error_message']
    
    return {
        'test_case': test_case['name'],
        'passed': passed,
        'results': results,
        'log_data': log_data
    }

def main():
    """主函数"""
    print_header("新闻工具自动化测试")
    print(f"开始时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    
    # 检查 ClawMaster 是否运行
    print("\n🔍 检查系统状态...")
    if not check_clawmaster_running():
        print("❌ ClawMaster 未运行！")
        print("请先启动: ./target/debug/clawmaster > /tmp/clawmaster.log 2>&1 &")
        sys.exit(1)
    print("✅ ClawMaster 运行中")
    
    # 检查日志文件
    try:
        with open("/tmp/clawmaster.log", "r") as f:
            log_size = len(f.read())
        print(f"✅ 日志文件可访问 ({log_size} bytes)")
    except Exception as e:
        print(f"⚠️  日志文件问题: {e}")
    
    # 运行测试案例
    print_header("开始测试")
    
    all_results = []
    for i, test_case in enumerate(TEST_CASES, 1):
        print(f"\n[{i}/{len(TEST_CASES)}]")
        result = run_test_case(test_case)
        all_results.append(result)
        
        # 测试之间等待一下
        if i < len(TEST_CASES):
            print("\n  ⏳ 等待 3 秒后继续下一个测试...")
            time.sleep(3)
    
    # 生成总结报告
    print_header("测试总结")
    
    passed_count = sum(1 for r in all_results if r['passed'])
    total_count = len(all_results)
    
    print(f"\n总测试数: {total_count}")
    print(f"通过: {passed_count} ✅")
    print(f"失败: {total_count - passed_count} ❌")
    print(f"通过率: {passed_count/total_count*100:.1f}%")
    
    print("\n详细结果:")
    for result in all_results:
        status = "✅ 通过" if result['passed'] else "❌ 失败"
        print(f"  {status} - {result['test_case']}")
    
    # 诊断建议
    if passed_count == 0:
        print_header("🔍 诊断建议")
        print("""
所有测试都失败了。可能的原因：

1. TOOLS.md 指令未生效
   → 检查 system prompt 是否真的包含 TOOLS.md 内容
   → 验证 TOOLS.md 在 prompt 中的位置

2. LLM 模型能力限制
   → 本地 GGUF 模型可能不擅长工具调用
   → 尝试使用 API 模型（Claude, GPT-4）测试

3. 工具调用格式问题
   → 检查 native_tools 配置
   → 查看日志中的工具调用格式

4. 指令强度不够
   → 需要更强的强制性语句
   → 考虑将指令移到工具描述中

建议下一步：
- 手动通过 WebUI 测试，观察实际行为
- 获取完整 system prompt 进行人工检查
- 测试其他工具（如 calc）验证工具调用能力
        """)
    elif passed_count < total_count:
        print_header("⚠️  部分测试失败")
        print("\n失败的测试案例需要进一步分析。")
    else:
        print_header("🎉 所有测试通过！")
        print("\nnews_search 工具工作正常！")
    
    print(f"\n完成时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"{'='*80}\n")
    
    # 保存详细日志
    report_file = f"/tmp/news_test_report_{int(time.time())}.json"
    with open(report_file, "w") as f:
        json.dump(all_results, f, indent=2, ensure_ascii=False)
    print(f"📄 详细报告已保存: {report_file}")
    
    return 0 if passed_count == total_count else 1

if __name__ == "__main__":
    sys.exit(main())
