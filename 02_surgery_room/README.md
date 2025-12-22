# 02_surgery_room

**解剖室 - vLLM 源码的病理分析**

## 目标

这里是你深入 vLLM 源码的地方：
- 打断点、单步调试
- 抓取 Timeline 和性能数据
- 复现具体问题（OOM、延迟尖峰等）
- 乱改代码，验证你的猜想

## 目录结构

```
02_surgery_room/
├── vllm_source/      # git submodule: 官方 vLLM 源码
├── debug_logs/       # 存放调试日志、Timeline、Profiling 结果
└── experiments/      # 问题复现脚本
```

## 使用方式

### 1. 引入 vLLM 源码（首次）

```bash
cd 02_surgery_room
git submodule add https://github.com/vllm-project/vllm.git vllm_source
cd vllm_source
pip install -e .  # 可编辑模式安装
```

### 2. 打断点调试

在 `vllm_source/vllm/` 中任意位置打断点，然后：

```python
# experiments/debug_scheduler.py
import pdb
from vllm import LLM

llm = LLM(model="facebook/opt-125m")
pdb.set_trace()  # 在这里打断点
outputs = llm.generate("Hello")
```

### 3. 抓取 Timeline

```bash
cd experiments
python capture_timeline.py > ../debug_logs/timeline_$(date +%Y%m%d_%H%M%S).log
```

### 4. 复现问题

```bash
cd experiments
python reproduce_oom.py  # 复现 OOM 问题
python reproduce_latency_spike.py  # 复现延迟尖峰
```

## 示例实验

- `reproduce_oom.py` - 复现显存不足场景
- `capture_timeline.py` - 抓取推理过程的 Timeline
- `benchmark_scheduler.py` - 对比不同调度策略

## 学习建议

1. **先在 01_mini_vllm 中建立直觉**
2. **然后来这里验证**：你的理解是否正确
3. **遇到困惑时**：写实验脚本，用日志和断点验证

## 注意事项

- `vllm_source/` 是 submodule，可以随意修改，不会影响 mini-vLLM
- `debug_logs/` 中的文件很大，建议加入 `.gitignore`
- 实验脚本要小而美，针对具体问题

