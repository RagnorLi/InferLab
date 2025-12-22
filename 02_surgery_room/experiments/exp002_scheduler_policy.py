"""
EXP-002: 调度策略对比实验

目标：
1. 对比 FCFS vs Priority Scheduling 的性能差异
2. 测试 Preemption 的效果
3. 找到不同场景下的最优策略

场景：
- 混合负载（短任务 + 长任务）
- 高优先级任务突发
- 资源受限场景
"""

import time
from typing import List
from dataclasses import dataclass


@dataclass
class WorkloadScenario:
    """工作负载场景"""
    name: str
    requests: List[tuple]  # (arrival_time, priority, max_tokens)


# =============================================================================
# Part 1: 定义测试场景
# =============================================================================

SCENARIOS = {
    "mixed_load": WorkloadScenario(
        name="混合负载（短+长任务）",
        requests=[
            (0, 1, 10),   # 低优先级短任务
            (1, 1, 100),  # 低优先级长任务
            (2, 0, 20),   # 高优先级短任务
            (3, 1, 50),   # 低优先级中任务
        ]
    ),
    
    "burst_high_priority": WorkloadScenario(
        name="高优先级突发",
        requests=[
            (0, 1, 50),   # 低优先级
            (1, 1, 50),   # 低优先级
            (5, 0, 10),   # 突发高优先级
            (5, 0, 10),   # 突发高优先级
        ]
    ),
    
    "resource_constrained": WorkloadScenario(
        name="资源受限",
        requests=[(i, i % 2, 30) for i in range(10)]  # 10个请求争抢资源
    ),
}


# =============================================================================
# Part 2: 测试 FCFS (v1_naive)
# =============================================================================

def test_fcfs_scheduler(scenario: WorkloadScenario):
    """测试 FCFS 调度器"""
    print(f"\n{'='*60}")
    print(f"测试 FCFS - {scenario.name}")
    print('='*60)
    
    # TODO: 使用 v1_naive 运行场景
    # from mini_vllm.v1_naive import InferenceEngine
    # 
    # engine = InferenceEngine(num_blocks=10, block_size=16, max_batch_size=2)
    # 
    # start_time = time.time()
    # # 模拟请求到达
    # for arrival, priority, tokens in scenario.requests:
    #     time.sleep(arrival - (time.time() - start_time))
    #     engine.add_request(f"req_{arrival}", f"prompt", max_tokens=tokens)
    # 
    # engine.run()
    # total_time = time.time() - start_time
    
    print("TODO: 实现 FCFS 测试")
    return {
        'total_time': 0,
        'avg_latency': 0,
        'throughput': 0,
    }


# =============================================================================
# Part 3: 测试 Priority Scheduler (v3_priority)
# =============================================================================

def test_priority_scheduler(scenario: WorkloadScenario):
    """测试优先级调度器"""
    print(f"\n{'='*60}")
    print(f"测试 Priority Scheduler - {scenario.name}")
    print('='*60)
    
    # TODO: 使用 v3_priority 运行场景
    print("TODO: 实现 Priority Scheduler 测试")
    return {
        'total_time': 0,
        'avg_latency': 0,
        'throughput': 0,
        'preemptions': 0,
    }


# =============================================================================
# Part 4: 对比分析
# =============================================================================

def compare_strategies(scenario_name: str):
    """对比两种策略"""
    print(f"\n{'🔬 '*20}")
    print(f"对比分析: {scenario_name}")
    print('🔬 '*20)
    
    scenario = SCENARIOS[scenario_name]
    
    fcfs_results = test_fcfs_scheduler(scenario)
    priority_results = test_priority_scheduler(scenario)
    
    print("\n结果对比：")
    print(f"{'指标':<20} {'FCFS':>15} {'Priority':>15} {'提升':>15}")
    print("-" * 70)
    
    # TODO: 打印对比结果
    print(f"{'总时间 (s)':<20} {fcfs_results['total_time']:>15.3f} {priority_results['total_time']:>15.3f}")
    print(f"{'平均延迟 (s)':<20} {fcfs_results['avg_latency']:>15.3f} {priority_results['avg_latency']:>15.3f}")
    print(f"{'吞吐量 (req/s)':<20} {fcfs_results['throughput']:>15.3f} {priority_results['throughput']:>15.3f}")
    
    print("\n💡 结论:")
    print("_" * 70)
    print("(记录你的观察)")
    print()


# =============================================================================
# Part 5: 与真实 vLLM 对比（可选）
# =============================================================================

def compare_with_vllm():
    """与真实 vLLM 的调度器对比"""
    print(f"\n{'='*60}")
    print("Part 5: 与真实 vLLM 对比")
    print('='*60)
    
    # TODO: 研究 vLLM 的调度策略
    # 1. vLLM 默认使用什么调度策略？
    # 2. 如何配置不同的策略？
    # 3. 我们的实现与它有什么差异？
    
    print("TODO: 研究 vLLM 的调度实现")
    print("路径: ../vllm_source/vllm/core/scheduler.py")


# =============================================================================
# 主函数
# =============================================================================

def main():
    print("\n" + "🔬 "*15)
    print("EXP-002: 调度策略对比实验")
    print("🔬 "*15 + "\n")
    
    print("可用场景:")
    for i, (key, scenario) in enumerate(SCENARIOS.items(), 1):
        print(f"  {i}. {key}: {scenario.name}")
    
    # 运行所有场景
    for scenario_name in SCENARIOS.keys():
        compare_strategies(scenario_name)
    
    # 可选：与真实 vLLM 对比
    # compare_with_vllm()
    
    print("\n" + "="*60)
    print("实验完成！")
    print("="*60)
    print("\n💡 思考：")
    print("1. 哪种场景下 Priority Scheduling 优势最明显？")
    print("2. Preemption 的代价是什么？")
    print("3. 如何在延迟和吞吐量之间权衡？")


if __name__ == "__main__":
    main()

