"""测试 v1_naive 版本"""

import sys
sys.path.append('..')

from core.v1_naive import InferenceEngine


def test_single_request():
    """测试单个请求"""
    print("\n" + "="*50)
    print("测试1: 单个请求")
    print("="*50)
    
    engine = InferenceEngine(num_blocks=10, block_size=16, max_batch_size=1)
    engine.add_request("req1", "Hello", max_tokens=5)
    engine.run()


def test_batch_requests():
    """测试批处理"""
    print("\n" + "="*50)
    print("测试2: 批处理请求")
    print("="*50)
    
    engine = InferenceEngine(num_blocks=20, block_size=16, max_batch_size=4)
    engine.add_request("req1", "Hello", max_tokens=3)
    engine.add_request("req2", "World", max_tokens=5)
    engine.add_request("req3", "AI", max_tokens=4)
    engine.run()


def test_oom():
    """测试显存不足的情况"""
    print("\n" + "="*50)
    print("测试3: OOM 场景（v1 会崩溃，这是预期的）")
    print("="*50)
    
    # 只有2个块，但要处理多个请求
    engine = InferenceEngine(num_blocks=2, block_size=16, max_batch_size=4)
    engine.add_request("req1", "Hello", max_tokens=3)
    engine.add_request("req2", "World", max_tokens=3)
    engine.add_request("req3", "AI", max_tokens=3)  # 这个会 OOM
    engine.run()


if __name__ == "__main__":
    test_single_request()
    test_batch_requests()
    
    print("\n" + "="*50)
    print("⚠️  接下来会测试 OOM（预期会失败）")
    print("="*50)
    try:
        test_oom()
    except RuntimeError as e:
        print(f"\n✓ 预期的 OOM 错误: {e}")
    
    print("\n" + "="*50)
    print("✅ v1_naive 测试完成")
    print("💡 升级到 v2_lru 来优雅处理 OOM（使用 LRU eviction）")
    print("="*50)

