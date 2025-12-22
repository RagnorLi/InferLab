# 01_mini_vllm

**你的 "Llama2.c" - 手写 vLLM 核心逻辑**

## 🎯 目标

用最简单的 Python 代码，实现 vLLM 的核心概念：
- PagedAttention 的内存管理
- 连续批处理调度
- LRU Eviction、优先级调度、Prefix Caching

**没有复杂配置，没有工厂模式，只有最直接的逻辑。**

---

## 📁 版本演进架构

```
core/
├── v1_naive/           ✅ 最简实现
│   ├── block_manager.py   # list 模拟显存
│   ├── scheduler.py       # FCFS 调度
│   └── engine.py          # 主循环
│
├── v2_lru/             ⬜ LRU 显存管理
│   ├── block_manager.py   # 集成 LC 146 LRU Cache
│   ├── scheduler.py       
│   └── engine.py          
│
├── v3_priority/        ⬜ 优先级调度 + 抢占
│   ├── block_manager.py   
│   ├── scheduler.py       # 基于 Heap
│   ├── engine.py          
│   └── priority_queue.py  
│
└── v4_prefix/          ⬜ Prefix Caching
    ├── block_manager.py   
    ├── scheduler.py       
    ├── engine.py          
    └── prefix_cache.py    # Trie 实现
```

**当前版本：** v1_naive  
**查看详细进度：** [../../PROGRESS.md](../../PROGRESS.md)

---

## 🚀 快速开始

### 测试 v1_naive（当前版本）

```bash
cd tests
python test_v1_naive.py
```

### 实现 v2_lru（下一步）

1. **先完成前置数据结构：**
   - `00_mountain/leetcode/llm-infer-ds/doubly_linked_list/`
   - `00_mountain/leetcode/llm-infer-ds/hash_map/`
   - `00_mountain/leetcode/llm-leetcode-20/lc146_lru_cache/`

2. **然后实现 v2_lru：**
   ```bash
   cd core/v2_lru
   # 参考 README.md 中的骨架代码，开始编写！
   ```

3. **最后验证：**
   ```bash
   cd tests
   python test_v2_lru.py
   
   # 在 surgery_room 中对比真实 vLLM
   cd ../../02_surgery_room/experiments
   python exp001_verify_lru.py
   ```

---

## 🗺️ 版本路线图

| 版本 | 核心特性 | 依赖 | 实验验证 |
|------|---------|------|---------|
| v1_naive | 最简实现 | - | - |
| v2_lru | LRU 显存管理 | LC 146, Linked List, Hash Map | EXP-001 |
| v3_priority | 优先级调度 | Heap, LC 622, LC 23 | EXP-002 |
| v4_prefix | Prefix Caching | Trie, LC 208, LC 239 | EXP-003 |

---

## 💡 核心概念映射

| mini-vLLM | 真实 vLLM | 说明 |
|-----------|-----------|------|
| `v1_naive/BlockManager` | `BlockSpaceManager` (简化版) | 显存块分配 |
| `v2_lru/LRUBlockManager` | `BlockSpaceManager` (with eviction) | LRU 驱逐 |
| `v3_priority/PriorityScheduler` | `Scheduler._schedule()` | 优先级调度 |
| `v4_prefix/PrefixCache` | `AutomaticPrefixCaching` | Prefix 复用 |

---

## 📚 学习路径

### 阶段 1：理解 v1_naive（已完成）

1. 看 `v1_naive/block_manager.py` - 理解块管理
2. 看 `v1_naive/scheduler.py` - 理解 FCFS 调度
3. 看 `v1_naive/engine.py` - 理解主循环
4. 运行 `tests/test_v1_naive.py` - 看到 OOM 崩溃

### 阶段 2：实现 v2_lru（下一步）

1. 去 `00_mountain` 实现 LC 146 LRU Cache
2. 回来填充 `v2_lru/block_manager.py`
3. 运行测试，验证 OOM 时不再崩溃
4. 去 `02_surgery_room` 对比真实 vLLM

### 阶段 3-4：类推

按照相同的模式完成 v3 和 v4。

---

## ⚠️ 重要提醒

**每个版本都要：**
1. ✅ 先在 `00_mountain` 完成依赖的数据结构
2. ✅ 然后实现对应版本的代码
3. ✅ 写测试验证功能
4. ✅ 在 `02_surgery_room` 中与真实 vLLM 对比

**不要跳步！** 一步一个脚印，从 v1 到 v4。

