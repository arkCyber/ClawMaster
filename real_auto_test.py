#!/usr/bin/env python3
"""
真正的自动化测试 - 通过 WebSocket RPC 发送查询
"""

import json
import time
import asyncio
import websockets
import ssl
from datetime import datetime

# 配置
WEBSOCKET_URL = "wss://localhost:59233/ws"
LOG_FILE = "/tmp/clawmaster_final.log"

# 测试案例
TEST_CASES = [
    {
        "name": "美国新闻",
        "query": "美国新闻",
        "expected_tool": "news_search",
        "timeout": 30
    },
    {
        "name": "计算测试",
        "query": "请使用 calc 工具计算 2+2",
        "expected_tool": "calc",
        "timeout": 20
    },
]

def print_header(text):
    print(f"\n{'='*80}")
    print(f"  {text}")
    print(f"{'='*80}\n")

def print_section(text):
    print(f"\n{'─'*80}")
    print(f"  {text}")
    print(f"{'─'*80}")

async def send_chat_message(query, session_key="auto-test"):
    """通过 WebSocket 发送聊天消息"""
    
    # 创建 SSL 上下文（忽略证书验证）
    ssl_context = ssl.create_default_context()
    ssl_context.check_hostname = False
    ssl_context.verify_mode = ssl.CERT_NONE
    
    try:
        async with websockets.connect(WEBSOCKET_URL, ssl=ssl_context) as websocket:
            print(f"  ✅ WebSocket 已连接")
            
            # 构建 RPC 请求
            request = {
                "jsonrpc": "2.0",
                "id": int(time.time() * 1000),
                "method": "chat.send",
                "params": {
                    "_session_key": session_key,
                    "content": query,
                    "stream": True
                }
            }
            
            print(f"  📤 发送查询: {query}")
            await websocket.send(json.dumps(request))
            
            # 接收响应
            responses = []
            tool_calls = []
            final_text = ""
            
            start_time = time.time()
            timeout = 30
            
            while time.time() - start_time < timeout:
                try:
                    message = await asyncio.wait_for(websocket.recv(), timeout=2.0)
                    data = json.loads(message)
                    responses.append(data)
                    
                    # 检查是否有工具调用
                    if "result" in data:
                        result = data["result"]
                        if isinstance(result, dict):
                            if "tool_calls" in result:
                                tool_calls.extend(result["tool_calls"])
                            if "text" in result:
                                final_text += result["text"]
                            if "done" in result and result["done"]:
                                break
                    
                    # 检查通知
                    if "method" in data:
                        if data["method"] == "chat.chunk":
                            params = data.get("params", {})
                            if "text" in params:
                                final_text += params["text"]
                            if "tool_calls" in params:
                                tool_calls.extend(params["tool_calls"])
                        elif data["method"] == "chat.done":
                            break
                
                except asyncio.TimeoutError:
                    continue
                except websockets.exceptions.ConnectionClosed:
                    break
            
            return {
                "responses": responses,
                "tool_calls": tool_calls,
                "text": final_text,
                "success": True
            }
    
    except Exception as e:
        print(f"  ❌ WebSocket 错误: {e}")
        return {
            "responses": [],
            "tool_calls": [],
            "text": "",
            "success": False,
            "error": str(e)
        }

def get_log_excerpt():
    """获取日志摘录"""
    try:
        with open(LOG_FILE, "r") as f:
            lines = f.readlines()
        
        # 获取最后 50 行
        recent_lines = lines[-50:]
        
        # 查找工具调用相关的行
        tool_lines = []
        for line in recent_lines:
            if any(keyword in line for keyword in ["tool_call", "news_search", "calc", "tool_calls_count"]):
                tool_lines.append(line.strip())
        
        return tool_lines
    except Exception as e:
        return [f"无法读取日志: {e}"]

async def run_test_case(test_case, test_num, total_tests):
    """运行单个测试案例"""
    print_section(f"测试 {test_num}/{total_tests}: {test_case['name']}")
    print(f"  查询: {test_case['query']}")
    print(f"  预期工具: {test_case['expected_tool']}")
    
    # 发送查询
    session_key = f"auto-test-{int(time.time())}"
    result = await send_chat_message(test_case['query'], session_key)
    
    # 等待一下让日志写入
    await asyncio.sleep(2)
    
    # 分析结果
    passed = True
    reasons = []
    
    if not result["success"]:
        passed = False
        reasons.append(f"❌ 连接失败: {result.get('error', 'Unknown')}")
    else:
        # 检查工具调用
        tool_calls = result["tool_calls"]
        if len(tool_calls) == 0:
            passed = False
            reasons.append("❌ 未检测到工具调用")
        else:
            print(f"  ✅ 检测到 {len(tool_calls)} 次工具调用")
            
            # 检查工具名称
            tool_names = [tc.get("name") or tc.get("tool") for tc in tool_calls]
            if test_case["expected_tool"] in tool_names:
                print(f"  ✅ 调用了正确的工具: {test_case['expected_tool']}")
            else:
                passed = False
                reasons.append(f"❌ 调用了错误的工具: {tool_names} (预期: {test_case['expected_tool']})")
        
        # 检查响应文本
        if result["text"]:
            print(f"  📝 响应摘要: {result['text'][:150]}...")
            
            # 检查禁止的文本
            forbidden = ["抱歉", "无法", "cannot", "unable"]
            if any(word in result["text"] for word in forbidden):
                passed = False
                reasons.append("❌ 响应包含禁止的文本")
    
    # 获取日志
    log_lines = get_log_excerpt()
    if log_lines:
        print(f"\n  🔍 相关日志 (最近 5 条):")
        for line in log_lines[-5:]:
            print(f"    {line[:120]}")
    
    # 总结
    if passed:
        print(f"\n  ✅ 测试通过")
    else:
        print(f"\n  ❌ 测试失败")
        for reason in reasons:
            print(f"    {reason}")
    
    return {
        "test_case": test_case["name"],
        "query": test_case["query"],
        "passed": passed,
        "reasons": reasons,
        "tool_calls": result.get("tool_calls", []),
        "response_text": result.get("text", "")[:500]
    }

async def main():
    """主函数"""
    print_header("新闻工具真正的自动化测试")
    print(f"开始时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    
    # 运行测试
    print_header("开始测试")
    
    all_results = []
    for i, test_case in enumerate(TEST_CASES, 1):
        result = await run_test_case(test_case, i, len(TEST_CASES))
        all_results.append(result)
        
        if i < len(TEST_CASES):
            print("\n  ⏳ 等待 3 秒后继续...")
            await asyncio.sleep(3)
    
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
        if not result["passed"]:
            for reason in result["reasons"]:
                print(f"      {reason}")
    
    # 保存报告
    report_file = f"/tmp/real_test_report_{int(time.time())}.json"
    with open(report_file, "w") as f:
        json.dump(all_results, f, indent=2, ensure_ascii=False)
    print(f"\n📄 详细报告已保存: {report_file}")
    
    if passed_count == total_count:
        print_header("🎉 所有测试通过！")
        print("\n新闻工具修复成功！")
    elif passed_count > 0:
        print_header("⚠️  部分测试通过")
    else:
        print_header("❌ 所有测试失败")
        print("\n需要进一步调试。")
    
    print(f"\n完成时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"{'='*80}\n")
    
    return 0 if passed_count == total_count else 1

if __name__ == "__main__":
    try:
        exit_code = asyncio.run(main())
        exit(exit_code)
    except KeyboardInterrupt:
        print("\n\n⚠️  测试被用户中断")
        exit(1)
    except Exception as e:
        print(f"\n\n❌ 测试出错: {e}")
        import traceback
        traceback.print_exc()
        exit(1)
