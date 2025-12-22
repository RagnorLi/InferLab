# Experiments - 实验室

这里是验证你对 vLLM 理解的地方。每个实验都对应 mini-vLLM 的一个版本。

## 🧪 实验列表

| ID | 实验名称 | 对应版本 | 工具 | 状态 |
|----|---------|---------|------|------|
| EXP-001 | LRU Eviction 验证 | v2_lru | Python | ⬜ |
| EXP-002 | 调度策略对比 | v3_priority | Python | ⬜ |
| EXP-003 | Prefix Cache 效果 | v4_prefix | Python | ⬜ |
| EXP-004 | Nsight 性能分析 | 真实 vLLM | Nsight Systems | ⬜ |

## 🎯 实验原则

1. **先猜测，再验证** - 在看真实代码前，先基于 mini-vLLM 的理解做猜测
2. **小步快跑** - 每个实验聚焦一个核心问题
3. **记录差异** - 发现 mini-vLLM 和真实 vLLM 的差异时，记录下来
4. **迭代改进** - 根据实验结果改进 mini-vLLM

## 🚀 快速开始

### 运行实验

```bash
# EXP-001: LRU 验证
python exp001_verify_lru.py

# EXP-002: 调度策略对比
python exp002_scheduler_policy.py

# EXP-004: Nsight Profiling
python exp004_nsight_profiling.py
```

### 查看结果

所有实验的输出和日志都在 `../debug_logs/` 目录。

## 📝 实验模板

每个实验脚本都遵循相同的结构：

```python
"""
EXP-XXX: 实验标题

目标：
1. ...
2. ...

工具：
- ...
"""

# Part 1: 测试 mini-vLLM
def test_mini_vllm():
    pass

# Part 2: 观察真实 vLLM
def observe_vllm():
    pass

# Part 3: 对比分析
def compare():
    pass

# Part 4: 可选扩展
def optional():
    pass

if __name__ == "__main__":
    main()
```

## 💡 实验技巧

### 1. 使用 NVTX 标记

在 mini-vLLM 中添加 NVTX 标记，方便在 Nsight 中查看：

```python
import nvtx

@nvtx.annotate("scheduler.schedule", color="blue")
def schedule(self):
    # ...
```

### 2. 对比日志

同时运行 mini-vLLM 和真实 vLLM，对比日志输出：

```bash
python test_mini_vllm.py > mini.log
python test_real_vllm.py > real.log
diff mini.log real.log
```

### 3. 可视化对比

使用 matplotlib 绘制性能对比图：

```python
import matplotlib.pyplot as plt

# 对比延迟分布
plt.hist([mini_latencies, vllm_latencies], label=['mini', 'vLLM'])
plt.legend()
plt.savefig('../debug_logs/latency_comparison.png')
```

## 🔬 EXP-004 特别说明（Nsight Profiling）

这个实验是最高级的，需要：
1. GPU 环境
2. 安装 Nsight Systems
3. 真实 vLLM 可运行

**学习路径：**
1. 先看 [Nsight Systems 入门教程](https://www.youtube.com/watch?v=Xz71JJlZwAE)
2. 跑一个简单的 CUDA 程序练手
3. 再来 profile vLLM

**关键指标：**
- GPU 利用率
- Kernel 占比
- 内存带宽
- CPU overhead

## 📊 实验结果记录

每次实验后，在 `../../PROGRESS.md` 中更新结果，并记录 Aha Moment。

---

**记住：实验的目的不是证明你对了，而是发现你哪里理解错了！**

