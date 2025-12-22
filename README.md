# InferLab 🚀

> **What I cannot create, I do not understand!**  
> — Richard Feynman

一个用于深入理解 LLM 推理系统（特别是 vLLM）的实验室。

## 🎯 项目哲学

- **没有巨型配置对象**
- **没有模型工厂**
- **没有 if-then-else 怪兽**
- **单一、连贯、最小、可读、可 hack、最大可 fork 的"强基线"代码库**

## 📁 项目结构

```
inferlab/
├── 00_mountain/          # [基础区] 你的思维健身房
│   ├── leetcode/         # 数据结构与算法基础
│   ├── hpc/              # 高性能计算练习
│   └── cuda_drills/      # CUDA kernel 手写练习
│
├── 01_mini_vllm/         # [核心造物区] 你的 "Llama2.c"
│   ├── core/             # 手写 vLLM 核心逻辑
│   │   ├── block_manager.py  # 用 Python list 模拟显存
│   │   ├── scheduler.py      # 最笨的调度逻辑
│   │   └── engine.py         # 串联一切的主循环
│   └── tests/            # 验证你对 vLLM 逻辑的猜想
│
├── 02_surgery_room/      # [解剖室] vLLM 源码的病理分析
│   ├── vllm_source/      # git submodule 引入官方 vLLM
│   ├── debug_logs/       # Timeline 和 Log 分析报告
│   └── experiments/      # 问题复现脚本
│
├── 03_oscilloscope/           # [驾驶舱] 可视化大屏
│   └── ...               # Tauri App - mini_vllm 的可视化界面
│
└── docs/                 # 航海日志，记录每天的 Aha Moment
```

## 🚦 快速开始

**⚡ 想立即开始？** 查看 [QUICKSTART.md](QUICKSTART.md) - 第一周完整行动指南！

**📊 追踪进度：** 查看 [PROGRESS.md](PROGRESS.md) - 你的学习仪表盘！

---

### 1️⃣ 从基础开始（00_mountain）

```bash
cd 00_mountain/leetcode
# 练习 LLM 推理相关的数据结构
```

推荐路线：
- 先搞定 **llm-infer-ds** 中的核心数据结构
- 再挑战 **llm-leetcode-20** 中的经典问题
- 最后进入 **cuda_drills** 手写简单 kernel

### 2️⃣ 手写 mini-vLLM（01_mini_vllm）

```bash
cd 01_mini_vllm

# 运行基础测试
python tests/test_basic.py
```

这里你将用最简单的 Python 实现：
- PagedAttention 的内存管理
- 连续批处理调度
- 推理主循环

**没有花里胡哨，只有核心逻辑！**

### 3️⃣ 解剖真实 vLLM（02_surgery_room）

```bash
cd 02_surgery_room

# 引入 vLLM 源码（首次）
git submodule add https://github.com/vllm-project/vllm.git vllm_source
cd vllm_source
pip install -e .

# 运行实验
cd ../experiments
python reproduce_oom.py          # 复现 OOM
python capture_timeline.py       # 捕获 Timeline
```

### 4️⃣ 可视化（03_oscilloscope）

```bash
cd 03_oscilloscope

# 安装依赖
pnpm install

# 启动开发服务器
pnpm dev
```

未来这里将成为 mini-vLLM 的可视化界面。

## 🗺️ 学习路径

```
阶段1: 基础健身 (00_mountain)
  ↓
  练习数据结构 → 做 LeetCode → 写简单 CUDA
  
阶段2: 手写理解 (01_mini_vllm)
  ↓
  实现 BlockManager → 实现 Scheduler → 串联 Engine
  
阶段3: 源码解剖 (02_surgery_room)
  ↓
  打断点调试 → 抓 Timeline → 复现问题 → 验证猜想
  
阶段4: 可视化 (03_oscilloscope)
  ↓
  构建实时监控大屏
```

## 🎓 核心概念映射

| 概念 | mini-vLLM | 真实 vLLM | 说明 |
|------|-----------|-----------|------|
| 显存管理 | `BlockManager.free_blocks` | `BlockSpaceManager` | PagedAttention 的核心 |
| 请求调度 | `Scheduler.schedule()` | `Scheduler._schedule()` | 连续批处理 |
| 推理循环 | `Engine.step()` | `LLMEngine.step()` | 主循环 |

## 📚 文档

- [我的推理之路](roadmaps/my-infer-road.md) - 学习路线图
- [LLM 推理 DSA Roadmap](00_mountain/leetcode/llm-inference-dsa-roadmap.md) - 数据结构学习指南

## 🤝 贡献

这是一个个人学习项目，但欢迎 fork 并创建你自己的版本！

核心原则：
- 保持简单
- 避免过度抽象
- 代码即文档

## 📜 License

MIT License - 详见 [LICENSE](LICENSE)
