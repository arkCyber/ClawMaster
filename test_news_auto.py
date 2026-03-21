#!/usr/bin/env python3
import asyncio
import websockets
import json
import sys
import ssl

async def test_news_tool():
    uri = "wss://localhost:59233/ws"
    
    # 创建SSL上下文，忽略证书验证
    ssl_context = ssl.create_default_context()
    ssl_context.check_hostname = False
    ssl_context.verify_mode = ssl.CERT_NONE
    
    try:
        async with websockets.connect(uri, ssl=ssl_context) as websocket:
            print("✅ WebSocket 连接成功")
            
            # 1. 发送握手
            handshake = {
                "jsonrpc": "2.0",
                "method": "handshake",
                "params": {
                    "client_id": "auto-test",
                    "client_version": "1.0.0",
                    "role": "operator"
                },
                "id": 1
            }
            await websocket.send(json.dumps(handshake))
            response = await websocket.recv()
            print(f"✅ 握手成功: {json.loads(response).get('result', {}).get('status')}")
            
            # 2. 发送新闻查询
            chat_request = {
                "jsonrpc": "2.0",
                "method": "chat.send",
                "params": {
                    "text": "美国新闻",
                    "session": "test-session",
                    "model": "local-llm::llama-3.2-1b-q4_k_m"
                },
                "id": 2
            }
            await websocket.send(json.dumps(chat_request))
            print("✅ 发送新闻查询: '美国新闻'")
            
            # 3. 接收响应
            tool_called = False
            response_text = ""
            message_count = 0
            
            for _ in range(100):
                try:
                    msg = await asyncio.wait_for(websocket.recv(), timeout=3.0)
                    data = json.loads(msg)
                    message_count += 1
                    
                    method = data.get("method", "")
                    
                    if method == "chat.stream":
                        chunk = data.get("params", {}).get("chunk", "")
                        response_text += chunk
                        
                        if "tool_call" in chunk.lower() or "news_search" in chunk.lower():
                            tool_called = True
                            print(f"✅ 检测到工具调用！")
                    
                    elif method == "chat.done":
                        print(f"✅ 对话完成")
                        break
                    
                    elif method == "chat.error":
                        error = data.get("params", {}).get("error", "")
                        print(f"❌ 错误: {error}")
                        break
                        
                except asyncio.TimeoutError:
                    if message_count > 0:
                        print(f"⏱️  超时，已收到 {message_count} 条消息")
                        break
                    else:
                        print("❌ 超时，未收到任何响应")
                        return False
            
            print(f"\n{'='*60}")
            print(f"📊 测试结果:")
            print(f"  消息数量: {message_count}")
            print(f"  工具调用: {'✅ 成功' if tool_called else '❌ 失败'}")
            print(f"  响应长度: {len(response_text)} 字符")
            if response_text:
                print(f"  响应预览: {response_text[:300]}...")
            print(f"{'='*60}\n")
            
            if tool_called:
                print("🎉 自动测试成功！新闻工具已被调用！")
                return True
            else:
                print("⚠️  未检测到工具调用")
                print(f"完整响应: {response_text}")
                return False
            
    except Exception as e:
        print(f"❌ 测试失败: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    result = asyncio.run(test_news_tool())
    sys.exit(0 if result else 1)
