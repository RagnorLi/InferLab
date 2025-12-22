# 项目结构迁移指南

## 变更概览

项目已重构为更清晰的四层架构，每一层都有明确的职责。

## 目录映射

| 旧路径 | 新路径 | 说明 |
|--------|--------|------|
| `mountain/` | `00_mountain/` | 基础练习区，重命名以体现顺序 |
| `app/infer-lab/` | `03_cockpit/` | Tauri 应用，改名突出其"驾驶舱"职能 |
| - | `01_mini_vllm/` | **新增**：手写 vLLM 核心逻辑 |
| - | `02_surgery_room/` | **新增**：vLLM 源码解剖室 |

## 具体变更

### 1. 00_mountain（原 mountain）

```bash
# 新增目录
00_mountain/cuda_drills/    # 未来练习 CUDA kernel 的地方
```

**不变部分：**
- `leetcode/` - 保持原样
- `hpc/` - 保持原样  
- `flashcard/` - 保持原样
- `resource/` - 保持原样
- `sysdesign/` - 保持原样

### 2. 01_mini_vllm（全新）

手写的 mini-vLLM 实现：

```
01_mini_vllm/
├── core/
│   ├── block_manager.py   # 显存块管理
│   ├── scheduler.py       # 请求调度
│   └── engine.py          # 推理主循环
└── tests/
    └── test_basic.py      # 基础测试
```

**快速测试：**
```bash
cd 01_mini_vllm
python tests/test_basic.py
```

### 3. 02_surgery_room（全新）

vLLM 源码分析和实验：

```
02_surgery_room/
├── vllm_source/      # 通过 submodule 引入
├── debug_logs/       # 调试日志（已加入 .gitignore）
└── experiments/      # 实验脚本
```

**设置步骤：**
```bash
cd 02_surgery_room
# 引入 vLLM 源码
git submodule add https://github.com/vllm-project/vllm.git vllm_source
cd vllm_source
pip install -e .
```

### 4. 03_cockpit（原 app/infer-lab）

没有内部结构变化，只是移动和重命名：

```bash
# 旧路径
app/infer-lab/

# 新路径  
03_cockpit/
```

**启动方式不变：**
```bash
cd 03_cockpit
pnpm install
pnpm dev
```

## 如果你有本地修改

### Git 状态处理

如果你在旧结构上有本地修改：

```bash
# 1. 检查状态
git status

# 2. 如果有未提交的更改，先 stash
git stash

# 3. 拉取新结构
git pull

# 4. 恢复你的更改
git stash pop
```

### 路径更新

如果你的脚本或配置中硬编码了旧路径，需要更新：

```python
# 旧
from mountain.leetcode import something

# 新  
from 00_mountain.leetcode import something
```

```bash
# 旧
cd app/infer-lab

# 新
cd 03_cockpit
```

## 新工作流

### 学习路径

```
1. 00_mountain/leetcode     → 打基础
2. 01_mini_vllm            → 手写理解
3. 02_surgery_room         → 深入源码
4. 03_cockpit              → 可视化
```

### 日常开发

```bash
# 练习数据结构
cd 00_mountain/leetcode

# 测试 mini-vLLM
cd 01_mini_vllm && python tests/test_basic.py

# 调试真实 vLLM
cd 02_surgery_room && python experiments/reproduce_oom.py

# 开发 UI
cd 03_cockpit && pnpm dev
```

## FAQ

### Q: 为什么用数字前缀？

A: 体现学习顺序和层级关系：
- `00_` - 基础层
- `01_` - 核心实现层
- `02_` - 高级研究层
- `03_` - 应用层

### Q: 旧的 `mountain` 目录去哪了？

A: 重命名为 `00_mountain`，内容完全保留。

### Q: `app/infer-lab` 为什么叫 `03_cockpit`？

A: 
1. 突出其作为"驾驶舱"（可视化界面）的定位
2. 编号 `03` 表示它是应用层，基于前面的基础层

### Q: 我需要重新配置环境吗？

A: 
- `00_mountain` 和 `03_cockpit` 不需要
- `01_mini_vllm` 只需标准 Python 环境
- `02_surgery_room` 需要安装 vLLM（见上文）

## 回滚

如果需要回到旧结构：

```bash
# 查看历史
git log --oneline

# 回退到重构前的 commit
git checkout <commit-hash>
```

但建议先尝试新结构一段时间，它会让你的学习路径更清晰！

---

**有问题？** 查看根目录的 [README.md](../README.md) 了解新结构的详细说明。

