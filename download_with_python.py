#!/usr/bin/env python3
"""
使用 Python 下载 Llama 3.1 8B 模型
支持断点续传和进度显示
"""

import os
import sys
import requests
from pathlib import Path
from tqdm import tqdm

# 模型信息
MODEL_URL = "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"
MODEL_DIR = Path.home() / ".clawmaster" / "models"
MODEL_PATH = MODEL_DIR / "llama-3.1-8b-instruct-q4_k_m.gguf"

def download_file(url, dest_path, chunk_size=8192):
    """下载文件，支持断点续传"""
    
    # 创建目录
    dest_path.parent.mkdir(parents=True, exist_ok=True)
    
    # 检查已下载的大小
    if dest_path.exists():
        downloaded_size = dest_path.stat().st_size
        print(f"📦 发现已下载 {downloaded_size / (1024**3):.2f} GB")
    else:
        downloaded_size = 0
    
    # 设置请求头（断点续传）
    headers = {}
    if downloaded_size > 0:
        headers['Range'] = f'bytes={downloaded_size}-'
    
    # 发送请求
    print(f"🔗 连接到: {url}")
    response = requests.get(url, headers=headers, stream=True, timeout=30)
    
    # 检查是否支持断点续传
    if response.status_code == 206:
        print("✅ 支持断点续传")
        mode = 'ab'
    elif response.status_code == 200:
        print("⚠️  不支持断点续传，重新下载")
        downloaded_size = 0
        mode = 'wb'
    else:
        print(f"❌ 错误: HTTP {response.status_code}")
        return False
    
    # 获取总大小
    total_size = int(response.headers.get('content-length', 0)) + downloaded_size
    print(f"📊 总大小: {total_size / (1024**3):.2f} GB")
    
    # 下载
    try:
        with open(dest_path, mode) as f:
            with tqdm(
                total=total_size,
                initial=downloaded_size,
                unit='B',
                unit_scale=True,
                unit_divisor=1024,
                desc="下载进度"
            ) as pbar:
                for chunk in response.iter_content(chunk_size=chunk_size):
                    if chunk:
                        f.write(chunk)
                        pbar.update(len(chunk))
        
        print(f"\n✅ 下载完成: {dest_path}")
        print(f"📦 文件大小: {dest_path.stat().st_size / (1024**3):.2f} GB")
        return True
        
    except KeyboardInterrupt:
        print("\n⚠️  下载被中断")
        print(f"💾 已下载: {dest_path.stat().st_size / (1024**3):.2f} GB")
        print("📝 可以重新运行此脚本继续下载")
        return False
    except Exception as e:
        print(f"\n❌ 下载失败: {e}")
        return False

def main():
    print("🚀 Llama 3.1 8B Instruct 下载器")
    print("=" * 60)
    print()
    
    # 检查是否已完成
    if MODEL_PATH.exists():
        size = MODEL_PATH.stat().st_size
        expected_size = 5_268_000_000  # 约 4.9GB
        
        if size >= expected_size * 0.99:  # 允许 1% 误差
            print(f"✅ 模型已存在: {MODEL_PATH}")
            print(f"📦 大小: {size / (1024**3):.2f} GB")
            print()
            print("如果需要重新下载，请先删除文件:")
            print(f"  rm {MODEL_PATH}")
            return
        else:
            print(f"⚠️  文件不完整 ({size / (1024**3):.2f} GB)")
            print("继续下载...")
            print()
    
    # 下载
    success = download_file(MODEL_URL, MODEL_PATH)
    
    if success:
        print()
        print("=" * 60)
        print("🎉 下载成功！")
        print()
        print("📝 下一步:")
        print("1. 查找配置文件:")
        print("   find ~ -name 'clawmaster.toml' 2>/dev/null")
        print()
        print("2. 修改配置:")
        print("   [providers.local-llm]")
        print("   enabled = true")
        print('   model_id = "llama-3.1-8b-instruct-q4_k_m"')
        print("   gpu_layers = 33")
        print()
        print("3. 重启 ClawMaster:")
        print("   pkill -9 -f clawmaster")
        print("   cd /Users/arksong/ClawMaster")
        print("   ./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &")
        print()
    else:
        print()
        print("💡 提示: 可以重新运行此脚本继续下载")
        sys.exit(1)

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"\n❌ 错误: {e}")
        sys.exit(1)
