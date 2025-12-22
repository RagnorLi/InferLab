"""
EXP-001: 验证 mini-vLLM v2_lru 的 LRU 逻辑与真实 vLLM 的一致性

目标：
1. 理解 vLLM 的 LRU eviction 策略
2. 验证我们的实现是否正确
3. 找出差异并改进

工具：
- mini-vLLM v2_lru
- 真实 vLLM (需要先安装)
"""

import time
from typing import List, Tuple


# =============================================================================
# Part 1: Mini-vLLM 的 LRU 测试
# =============================================================================

def test_mini_vllm_lru():
    """测试我们自己实现的 LRU"""
    print("="*60)
    print("Part 1: 测试 mini-vLLM v2_lru")
    print("="*60)
    
    # TODO: 实现 v2_lru 后，取消注释并测试
    # from mini_vllm.v2_lru import LRUBlockManager
    # 
    # manager = LRUBlockManager(num_blocks=4, block_size=16)
    # 
    # # 测试场景：分配4个块，然后第5个请求会触发 eviction
    # manager.allocate("req1", 16)  # 分配 1 个块
    # manager.allocate("req2", 16)  # 分配 1 个块
    # manager.allocate("req3", 16)  # 分配 1 个块
    # manager.allocate("req4", 16)  # 分配 1 个块
    # 
    # # 访问 req1，使其变成最近使用
    # manager.access("req1")
    # 
    # # 现在分配 req5，应该驱逐 req2（最久未使用）
    # evicted = manager.allocate("req5", 16)
    # 
    # print(f"✓ 驱逐的请求: {evicted}")
    # assert evicted == "req2", f"预期驱逐 req2，实际驱逐 {evicted}"
    
    print("\n⚠️  请先完成 v2_lru 的实现")
    print("路径: 01_mini_vllm/core/v2_lru/block_manager.py")


# =============================================================================
# Part 2: 真实 vLLM 的行为观察
# =============================================================================

def observe_vllm_behavior():
    """观察真实 vLLM 的 LRU 行为"""
    print("\n" + "="*60)
    print("Part 2: 观察真实 vLLM 的行为")
    print("="*60)
    
    # TODO: 安装 vLLM 后，取消注释
    # try:
    #     from vllm.core.block_manager import BlockSpaceManager
    #     
    #     # 创建 BlockSpaceManager
    #     # 注意：需要研究 vLLM 的初始化参数
    #     
    #     print("✓ vLLM 已安装")
    #     # TODO: 运行相同的测试场景，观察输出
    # 
    # except ImportError:
    #     print("⚠️  vLLM 未安装")
    #     print("安装方式：")
    #     print("  cd ../vllm_source")
    #     print("  pip install -e .")
    
    print("\n⚠️  请先引入 vLLM 源码")
    print("步骤：")
    print("  cd ../vllm_source")
    print("  git submodule add https://github.com/vllm-project/vllm.git .")
    print("  pip install -e .")


# =============================================================================
# Part 3: 对比分析
# =============================================================================

def compare_results():
    """对比 mini-vLLM 和真实 vLLM 的差异"""
    print("\n" + "="*60)
    print("Part 3: 对比分析")
    print("="*60)
    
    print("\n分析维度：")
    print("1. Eviction 顺序是否一致？")
    print("2. Access tracking 的时机是否相同？")
    print("3. 边界情况的处理（如空队列）")
    
    print("\n记录你的发现：")
    print("_" * 60)
    print("(在这里记录你的观察和结论)")
    print()


# =============================================================================
# Part 4: 性能对比（可选）
# =============================================================================

def benchmark_performance():
    """对比性能（可选）"""
    print("\n" + "="*60)
    print("Part 4: 性能对比（可选）")
    print("="*60)
    
    print("\n测试场景：1000 次 allocate + access 操作")
    
    # TODO: 分别测试两个实现的性能
    # mini_vllm_time = benchmark_mini_vllm(iterations=1000)
    # real_vllm_time = benchmark_real_vllm(iterations=1000)
    # 
    # print(f"mini-vLLM: {mini_vllm_time:.3f}s")
    # print(f"真实 vLLM: {real_vllm_time:.3f}s")
    # print(f"差距: {mini_vllm_time / real_vllm_time:.2f}x")
    
    print("TODO: 实现性能基准测试")


# =============================================================================
# 主函数
# =============================================================================

def main():
    print("\n" + "🔬 "*15)
    print("EXP-001: LRU Eviction 验证实验")
    print("🔬 "*15 + "\n")
    
    # Step 1: 测试我们的实现
    test_mini_vllm_lru()
    
    # Step 2: 观察真实 vLLM
    observe_vllm_behavior()
    
    # Step 3: 对比分析
    compare_results()
    
    # Step 4: 性能对比（可选）
    # benchmark_performance()
    
    print("\n" + "="*60)
    print("实验完成！")
    print("="*60)
    print("\n💡 下一步：")
    print("1. 在 ../../PROGRESS.md 中更新实验状态")
    print("2. 将发现记录到学习笔记中")
    print("3. 如果发现问题，修复 v2_lru 的实现")


if __name__ == "__main__":
    main()

