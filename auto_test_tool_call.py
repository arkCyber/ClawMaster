#!/usr/bin/env python3
"""
自动测试工具调用功能
测试新闻工具是否被正确调用
"""
import asyncio
import websockets
import json
import sys
import ssl
import time

async def test_tool_call():
    uri = "wss://localhost:59233/ws"
    
    # 禁用代理
    import os
    os.environ['NO_PROXY'] = 'localhost,127.0.0.1'
    os.environ['no_proxy'] = 'localhost,127.0.0.1'
    
    # 创建SSL上下文
    ssl_context = ssl.create_default_context()
    ssl_context.check_hostname = False
    ssl_context.verify_mode = ssl.CERT_NONE
    
    print("=" * 60)
    print("🧪 自动测试工具调用功能")
    print("=" * 60)
    
    try:
        async with websockets.connect(uri, ssl=ssl_context) as websocket:
            print("✅ WebSocket 连接成功")
            
            # 1. 握手 (使用正确的协议格式)
            handshake = {
                "type": "req",
                "id": "1",
                "method": "connect",
                "params": {
                    "minProtocol": 3,
                    "maxProtocol": 4,
                    "client": {
                        "id": "auto-test-tool-call",
                        "version": "1.0.0",
                        "platform": "python",
                        "mode": "operator"
                    },
                    "locale": "zh"
                }
            }
            await websocket.send(json.dumps(handshake))
            
            # 等待握手响应
            try:
                response = await asyncio.wait_for(websocket.recv(), timeout=5.0)
                resp_data = json.loads(response)
                if "result" in resp_data:
                    print(f"✅ 握手成功: {resp_data.get('result', {}).get('status', 'ok')}")
                elif "error" in resp_data:
                    print(f"❌ 握手失败: {resp_data['error']}")
                    return False
            except asyncio.TimeoutError:
                print("❌ 握手超时")
                return False
            
            # 2. 发送新闻查询 (使用Llama 3.2 1B - 更好的工具调用支持)
            chat_request = {
                "type": "req",
                "id": "2",
                "method": "chat.send",
                "params": {
                    "text": "美国新闻",
                    "session": "test-tool-call-llama32",
                    "model": "local-llm::llama-3.2-1b-q4_k_m"
                }
            }
            await websocket.send(json.dumps(chat_request))
            print("✅ 发送测试消息: '美国新闻'")
            print("\n等待响应...\n")
            
            # 3. 接收响应
            tool_called = False
            tool_name = None
            tool_args = None
            response_text = ""
            message_count = 0
            start_time = time.time()
            
            for _ in range(200):  # 最多等待200条消息
                try:
                    msg = await asyncio.wait_for(websocket.recv(), timeout=5.0)
                    data = json.loads(msg)
                    message_count += 1
                    
                    method = data.get("method", "")
                    params = data.get("params", {})
                    
                    # 检测工具调用开始
                    if method == "chat.tool_call_start":
                        tool_called = True
                        tool_name = params.get("name", "unknown")
                        tool_args = params.get("arguments", {})
                        print(f"🎉 检测到工具调用！")
                        print(f"   工具名称: {tool_name}")
                        print(f"   参数: {json.dumps(tool_args, ensure_ascii=False, indent=2)}")
                    
                    # 检测工具调用结束
                    elif method == "chat.tool_call_end":
                        result = params.get("result", "")
                        print(f"\n✅ 工具执行完成")
                        print(f"   结果长度: {len(str(result))} 字符")
                    
                    # 收集响应文本
                    elif method == "chat.stream":
                        chunk = params.get("chunk", "")
                        response_text += chunk
                    
                    # 对话完成
                    elif method == "chat.done":
                        elapsed = time.time() - start_time
                        print(f"\n✅ 对话完成 (耗时: {elapsed:.1f}秒)")
                        break
                    
                    # 错误
                    elif method == "chat.error":
                        error = params.get("error", "")
                        print(f"\n❌ 错误: {error}")
                        break
                        
                except asyncio.TimeoutError:
                    if message_count > 0:
                        print(f"\n⏱️  超时，已收到 {message_count} 条消息")
                        break
                    else:
                        print("\n❌ 超时，未收到任何响应")
                        return False
            
            # 输出测试结果
            print("\n" + "=" * 60)
            print("📊 测试结果:")
            print("=" * 60)
            print(f"  消息数量: {message_count}")
            print(f"  工具调用: {'✅ 成功' if tool_called else '❌ 失败'}")
            if tool_called:
                print(f"  工具名称: {tool_name}")
                print(f"  工具参数: {json.dumps(tool_args, ensure_ascii=False)}")
            print(f"  响应长度: {len(response_text)} 字符")
            
            if response_text:
                print(f"\n📝 响应预览:")
                preview = response_text[:500]
                print(f"  {preview}...")
            
            print("=" * 60)
            
            if tool_called and tool_name == "news_search":
                print("\n🎉🎉🎉 测试成功！新闻工具被正确调用！")
                return True
            elif tool_called:
                print(f"\n⚠️  工具被调用，但不是预期的 news_search (实际: {tool_name})")
                return False
            else:
                print("\n❌ 测试失败：工具没有被调用")
                print("\n💡 建议:")
                print("  1. 切换到 Llama 3.2 1B 模型")
                print("  2. 或使用 API 模型（OpenAI/Claude）")
                return False
            
    except Exception as e:
        print(f"\n❌ 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    result = asyncio.run(test_tool_call())
    sys.exit(0 if result else 1)
