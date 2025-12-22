"""
EXP-004: Nsight Systems 性能分析实验

目标：
1. 学习使用 Nsight Systems 分析 GPU 推理性能
2. 找到 vLLM 的瓶颈（Kernel、内存拷贝、CPU 开销）
3. 实践优化并验证效果

工具：
- Nsight Systems (nsys)
- Nsight Compute (ncu) - 可选，用于深入分析单个 kernel

前置条件：
- 安装 NVIDIA Nsight Systems
- 有 GPU 可用
- vLLM 已正确安装

参考资料：
- Nsight Systems 文档: https://docs.nvidia.com/nsight-systems/
"""

import subprocess
import os
from pathlib import Path


# =============================================================================
# Part 1: 准备测试脚本
# =============================================================================

TEST_SCRIPT = """
# test_vllm_inference.py
# 这是一个简单的 vLLM 推理脚本，用于 profiling

from vllm import LLM, SamplingParams

# 创建模型（使用小模型以便快速测试）
llm = LLM(model="facebook/opt-125m", max_num_batched_tokens=2048)

# 准备一些测试 prompts
prompts = [
    "Once upon a time",
    "The capital of France is",
    "Machine learning is",
    "In the beginning",
] * 10  # 40 个请求

sampling_params = SamplingParams(temperature=0.8, top_p=0.95, max_tokens=100)

print("开始推理...")
outputs = llm.generate(prompts, sampling_params)
print(f"完成 {len(outputs)} 个请求")
"""


def prepare_test_script():
    """准备测试脚本"""
    script_path = Path("../debug_logs/test_vllm_inference.py")
    script_path.parent.mkdir(exist_ok=True)
    
    with open(script_path, 'w') as f:
        f.write(TEST_SCRIPT)
    
    print(f"✓ 测试脚本已创建: {script_path}")
    return script_path


# =============================================================================
# Part 2: 运行 Nsight Systems Profiling
# =============================================================================

def run_nsight_profiling():
    """使用 Nsight Systems 进行 profiling"""
    print("\n" + "="*60)
    print("Part 2: 运行 Nsight Systems Profiling")
    print("="*60)
    
    script_path = prepare_test_script()
    output_path = Path("../debug_logs/vllm_profile.nsys-rep")
    
    # Nsight Systems 命令
    cmd = [
        "nsys", "profile",
        "--trace=cuda,nvtx,osrt",  # 追踪 CUDA、NVTX 标记、系统调用
        "--output", str(output_path),
        "--force-overwrite", "true",
        "python", str(script_path)
    ]
    
    print(f"\n运行命令：")
    print(" ".join(cmd))
    print()
    
    try:
        # 检查 nsys 是否可用
        subprocess.run(["nsys", "--version"], check=True, capture_output=True)
        
        print("⚠️  即将开始 profiling，这可能需要几分钟...")
        print("(如果你还没安装 Nsight Systems，请访问 https://developer.nvidia.com/nsight-systems)")
        
        # TODO: 取消注释以实际运行
        # result = subprocess.run(cmd, check=True)
        # print(f"\n✓ Profiling 完成！")
        # print(f"报告保存在: {output_path}")
        
        print("\nTODO: 取消注释以运行实际 profiling")
        
    except FileNotFoundError:
        print("⚠️  Nsight Systems (nsys) 未找到")
        print("\n安装方式：")
        print("  Linux: sudo apt install nvidia-nsight-systems")
        print("  或下载: https://developer.nvidia.com/nsight-systems")
    
    except subprocess.CalledProcessError as e:
        print(f"❌ Profiling 失败: {e}")


# =============================================================================
# Part 3: 分析 Profiling 结果
# =============================================================================

def analyze_profile():
    """分析 profiling 结果"""
    print("\n" + "="*60)
    print("Part 3: 分析 Profiling 结果")
    print("="*60)
    
    print("\n使用 Nsight Systems GUI 查看：")
    print("  nsys-ui ../debug_logs/vllm_profile.nsys-rep")
    
    print("\n或使用命令行导出统计：")
    print("  nsys stats ../debug_logs/vllm_profile.nsys-rep")
    
    print("\n🔍 关键分析维度：")
    print()
    print("1. **Timeline 时间线**")
    print("   - GPU Kernel 执行占比")
    print("   - CPU 和 GPU 的空闲时间")
    print("   - 内存拷贝（H2D/D2H）开销")
    print()
    print("2. **CUDA Kernels**")
    print("   - 哪些 kernel 耗时最长？")
    print("   - Kernel 的并发度如何？")
    print("   - 是否有优化空间？")
    print()
    print("3. **NVTX 标记**")
    print("   - vLLM 的主要阶段（Prefill/Decode）")
    print("   - Scheduler 的开销")
    print("   - BlockManager 的开销")
    print()
    print("4. **瓶颈识别**")
    print("   - CPU bound 还是 GPU bound？")
    print("   - Memory bound 吗？")
    print("   - 有串行瓶颈吗？")


# =============================================================================
# Part 4: 实践优化（示例）
# =============================================================================

def optimization_example():
    """优化示例"""
    print("\n" + "="*60)
    print("Part 4: 实践优化示例")
    print("="*60)
    
    print("\n假设你发现的瓶颈：")
    print()
    print("📌 **Case 1: Scheduler 开销过大**")
    print("   症状: CPU 上 Scheduler._schedule() 占用 20% 时间")
    print("   优化: 使用更高效的数据结构（Heap 替换 List）")
    print("   验证: 重新 profile，对比优化前后")
    print()
    print("📌 **Case 2: 内存拷贝频繁**")
    print("   症状: H2D/D2H 拷贝占用 15% 时间")
    print("   优化: 合并小拷贝，使用 pinned memory")
    print("   验证: 查看 NVTX 中 memcpy 的次数和大小")
    print()
    print("📌 **Case 3: Kernel Launch 开销**")
    print("   症状: 大量小 kernel，launch overhead 明显")
    print("   优化: Kernel fusion，减少 launch 次数")
    print("   验证: 统计 kernel launch 数量")
    
    print("\n💡 优化流程：")
    print("  1. Profile → 找到瓶颈")
    print("  2. 提出假设 → 设计优化方案")
    print("  3. 实施优化")
    print("  4. 重新 Profile → 验证效果")
    print("  5. 迭代")


# =============================================================================
# Part 5: 使用 Nsight Compute 深入分析（可选）
# =============================================================================

def nsight_compute_analysis():
    """使用 Nsight Compute 分析单个 kernel"""
    print("\n" + "="*60)
    print("Part 5: Nsight Compute 深入分析（可选）")
    print("="*60)
    
    print("\nNsight Compute 用于分析单个 CUDA kernel 的性能瓶颈")
    print()
    print("使用方式：")
    print("  # 分析所有 kernel")
    print("  ncu --set full -o kernel_analysis python test_vllm_inference.py")
    print()
    print("  # 只分析特定 kernel（如 attention kernel）")
    print("  ncu --kernel-name=attention -o attention_analysis python test_vllm_inference.py")
    print()
    print("  # 查看报告")
    print("  ncu-ui kernel_analysis.ncu-rep")
    
    print("\n🔍 NCU 分析维度：")
    print("  - Compute utilization (计算利用率)")
    print("  - Memory throughput (内存吞吐)")
    print("  - Warp occupancy (warp 占用率)")
    print("  - Bank conflicts (bank 冲突)")
    print("  - Register/Shared memory 使用")


# =============================================================================
# 主函数
# =============================================================================

def main():
    print("\n" + "🔬 "*15)
    print("EXP-004: Nsight Systems 性能分析实验")
    print("🔬 "*15 + "\n")
    
    print("本实验将教你：")
    print("1. 如何使用 Nsight Systems 进行 profiling")
    print("2. 如何识别性能瓶颈")
    print("3. 如何验证优化效果")
    print()
    
    # Step 1: 准备并运行 profiling
    run_nsight_profiling()
    
    # Step 2: 分析结果
    analyze_profile()
    
    # Step 3: 优化示例
    optimization_example()
    
    # Step 4: 可选 - Nsight Compute
    # nsight_compute_analysis()
    
    print("\n" + "="*60)
    print("实验指南完成！")
    print("="*60)
    print("\n📝 作业：")
    print("1. 实际运行一次 profiling")
    print("2. 在 Nsight Systems GUI 中找到 3 个耗时最长的 kernel")
    print("3. 分析 CPU 和 GPU 的利用率")
    print("4. 记录你的发现到学习笔记")
    print()
    print("💡 提示：第一次 profiling 可能需要较长时间，请耐心等待")


if __name__ == "__main__":
    main()

