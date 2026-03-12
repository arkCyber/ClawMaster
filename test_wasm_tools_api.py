#!/usr/bin/env python3
"""
ClawMaster WASM 工具 API 测试脚本
模拟远程用户通过 HTTP/WebSocket 调用 WASM 工具
"""

import json
import asyncio
import websockets
import aiohttp
from typing import Dict, Any, List
from datetime import datetime

# 配置
GATEWAY_URL = "http://localhost:3000"
WS_URL = "ws://localhost:3000/ws"
API_BASE = f"{GATEWAY_URL}/api"

class WasmToolTester:
    def __init__(self):
        self.results = []
        self.session = None
        
    async def setup(self):
        """初始化 HTTP 会话"""
        self.session = aiohttp.ClientSession()
        
    async def cleanup(self):
        """清理资源"""
        if self.session:
            await self.session.close()
    
    async def call_tool(self, tool_name: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """调用 WASM 工具"""
        payload = {
            "tool": tool_name,
            "params": params
        }
        
        try:
            async with self.session.post(
                f"{API_BASE}/tools/wasm/execute",
                json=payload,
                timeout=aiohttp.ClientTimeout(total=30)
            ) as response:
                result = await response.json()
                return {
                    "tool": tool_name,
                    "success": response.status == 200,
                    "result": result,
                    "status_code": response.status
                }
        except Exception as e:
            return {
                "tool": tool_name,
                "success": False,
                "error": str(e)
            }
    
    async def test_text_processing(self):
        """测试文本处理工具"""
        print("\n📝 测试文本处理工具...")
        
        tests = [
            ("string-length", {"text": "Hello, ClawMaster!"}),
            ("string-trim", {"text": "  Hello World  ", "mode": "both"}),
            ("string-replace", {
                "text": "Hello World",
                "pattern": "World",
                "replacement": "ClawMaster"
            }),
            ("text-case", {"text": "hello world", "case": "upper"}),
            ("text-split", {"text": "apple,banana,orange", "delimiter": ","}),
            ("text-join", {"items": ["apple", "banana", "orange"], "separator": ", "}),
            ("text-truncate", {
                "text": "This is a very long text",
                "max_length": 20,
                "suffix": "..."
            }),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}")
    
    async def test_encoding(self):
        """测试编码/解码工具"""
        print("\n🔐 测试编码/解码工具...")
        
        tests = [
            ("base64-encode", {"data": "Hello, ClawMaster!"}),
            ("base64-decode", {"encoded": "SGVsbG8sIENsYXdNYXN0ZXIh"}),
            ("hex-encode", {"data": "Hello"}),
            ("hex-decode", {"hex": "48656c6c6f"}),
            ("url-encode", {"text": "Hello World & Special!"}),
            ("url-decode", {"encoded": "Hello%20World%20%26%20Special%21"}),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}")
    
    async def test_hashing(self):
        """测试哈希工具"""
        print("\n#️⃣ 测试哈希工具...")
        
        tests = [
            ("hash-md5", {"data": "Hello, ClawMaster!"}),
            ("hash-sha256", {"data": "Hello, ClawMaster!"}),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}")
    
    async def test_math(self):
        """测试数学工具"""
        print("\n🔢 测试数学工具...")
        
        tests = [
            ("math-ops", {"operation": "add", "a": 10, "b": 20}),
            ("math-ops", {"operation": "multiply", "a": 5, "b": 6}),
            ("math-stats", {
                "numbers": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                "operation": "mean"
            }),
            ("math-stats", {
                "numbers": [1, 2, 3, 4, 5],
                "operation": "median"
            }),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}: {params.get('operation', '')}")
    
    async def test_datetime(self):
        """测试日期时间工具"""
        print("\n📅 测试日期时间工具...")
        
        tests = [
            ("datetime-now", {"format": "iso8601"}),
            ("datetime-format", {
                "timestamp": "2026-03-12T10:00:00Z",
                "format": "%Y-%m-%d %H:%M:%S"
            }),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}")
    
    async def test_data_parsing(self):
        """测试数据解析工具"""
        print("\n📊 测试数据解析工具...")
        
        tests = [
            ("json-parse", {
                "json": '{"name": "ClawMaster", "version": "0.10.18"}'
            }),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}")
    
    async def test_regex(self):
        """测试正则表达式工具"""
        print("\n🔍 测试正则表达式工具...")
        
        tests = [
            ("regex-ops", {
                "text": "Email: test@example.com",
                "pattern": r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
                "operation": "match"
            }),
            ("regex-ops", {
                "text": "Phone: 123-456-7890",
                "pattern": r"\d{3}-\d{3}-\d{4}",
                "operation": "find_all"
            }),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}: {params.get('operation', '')}")
    
    async def test_random(self):
        """测试随机生成工具"""
        print("\n🎲 测试随机生成工具...")
        
        tests = [
            ("random-gen", {"type": "number", "min": 1, "max": 100}),
            ("random-gen", {"type": "string", "length": 16}),
            ("uuid-generate", {"version": "v4"}),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}")
    
    async def test_validation(self):
        """测试数据验证工具"""
        print("\n✅ 测试数据验证工具...")
        
        tests = [
            ("validate-data", {"data": "test@example.com", "type": "email"}),
            ("validate-data", {"data": "https://example.com", "type": "url"}),
            ("validate-data", {"data": "192.168.1.1", "type": "ipv4"}),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}: {params.get('type', '')}")
    
    async def test_array_ops(self):
        """测试数组操作工具"""
        print("\n📋 测试数组操作工具...")
        
        tests = [
            ("array-ops", {
                "operation": "filter",
                "array": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                "condition": "greater_than",
                "value": 5
            }),
            ("array-ops", {
                "operation": "map",
                "array": [1, 2, 3, 4, 5],
                "function": "square"
            }),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}: {params.get('operation', '')}")
    
    async def test_path_ops(self):
        """测试路径操作工具"""
        print("\n📁 测试路径操作工具...")
        
        tests = [
            ("path-ops", {
                "operation": "join",
                "paths": ["/home/user", "documents", "file.txt"]
            }),
            ("path-ops", {
                "operation": "basename",
                "path": "/home/user/documents/file.txt"
            }),
        ]
        
        for tool, params in tests:
            result = await self.call_tool(tool, params)
            self.results.append(result)
            status = "✓" if result["success"] else "✗"
            print(f"  {status} {tool}: {params.get('operation', '')}")
    
    def print_summary(self):
        """打印测试总结"""
        print("\n" + "="*60)
        print("📊 测试总结")
        print("="*60)
        
        total = len(self.results)
        passed = sum(1 for r in self.results if r.get("success", False))
        failed = total - passed
        
        print(f"总测试数: {total}")
        print(f"通过: {passed} ✓")
        print(f"失败: {failed} ✗")
        print(f"成功率: {(passed/total*100):.1f}%")
        
        if failed > 0:
            print("\n失败的测试:")
            for r in self.results:
                if not r.get("success", False):
                    print(f"  ✗ {r['tool']}: {r.get('error', 'Unknown error')}")
        
        # 保存详细结果
        with open("/tmp/wasm_test_results.json", "w") as f:
            json.dump(self.results, f, indent=2)
        print(f"\n详细结果已保存到: /tmp/wasm_test_results.json")

async def main():
    """主测试流程"""
    print("🚀 ClawMaster WASM 工具全面测试")
    print("="*60)
    print(f"开始时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"Gateway URL: {GATEWAY_URL}")
    print("="*60)
    
    tester = WasmToolTester()
    
    try:
        await tester.setup()
        
        # 运行所有测试
        await tester.test_text_processing()
        await tester.test_encoding()
        await tester.test_hashing()
        await tester.test_math()
        await tester.test_datetime()
        await tester.test_data_parsing()
        await tester.test_regex()
        await tester.test_random()
        await tester.test_validation()
        await tester.test_array_ops()
        await tester.test_path_ops()
        
        # 打印总结
        tester.print_summary()
        
    except Exception as e:
        print(f"\n❌ 测试过程中出错: {e}")
        import traceback
        traceback.print_exc()
    finally:
        await tester.cleanup()
    
    print(f"\n结束时间: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("="*60)

if __name__ == "__main__":
    asyncio.run(main())
