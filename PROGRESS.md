# 🎯 InferLab 学习进度总览

> **更新日期：** 2025-12-22  
> **目标：** Python/C++/Rust 三语言实现数据结构 + 深度掌握 vLLM

## 📊 整体进度

- **数据结构：** 0/6 ✅
- **LeetCode 题目：** 0/20 ✅
- **mini-vLLM 版本：** v1 (naive) → v4 (目标)
- **surgery_room 实验：** 0 个

---

## 🏔️ 阶段 1：核心数据结构 (00_mountain/leetcode/llm-infer-ds)

| # | 数据结构 | Python | C++ | Rust | 应用于 mini-vLLM | vLLM 验证 | 笔记 |
|---|---------|--------|-----|------|-----------------|-----------|------|
| 1 | **Vector** | ⬜ | ⬜ | ⬜ | ⬜ v2_lru | ⬜ | [→](#vector-notes) |
| 2 | **Hash Map** | ⬜ | ⬜ | ⬜ | ⬜ v2_lru | ⬜ | [→](#hashmap-notes) |
| 3 | **Circular Queue** | ⬜ | ⬜ | ⬜ | ⬜ v3_priority | ⬜ | [→](#queue-notes) |
| 4 | **Doubly Linked List** | ⬜ | ⬜ | ⬜ | ⬜ v2_lru | ⬜ | [→](#linkedlist-notes) |
| 5 | **Heap** | ⬜ | ⬜ | ⬜ | ⬜ v3_priority | ⬜ | [→](#heap-notes) |
| 6 | **Trie** | ⬜ | ⬜ | ⬜ | ⬜ v4_prefix | ⬜ | [→](#trie-notes) |

**图例：** ⬜ 未开始 | 🟨 进行中 | ✅ 完成 | 🔥 已应用

---

## 💪 阶段 2：LeetCode Top-20 (00_mountain/leetcode/llm-leetcode-20)

### 🔴 P0 - 必做（直接应用到 mini-vLLM）

| # | 题目 | Python | C++ | Rust | 应用版本 | 实验验证 | 笔记 |
|---|------|--------|-----|------|---------|---------|------|
| 1 | **LC 146 LRU Cache** | ⬜ | ⬜ | ⬜ | v2_lru | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc146_lru_cache/) |
| 4 | **LC 622 Circular Queue** | ⬜ | ⬜ | ⬜ | v3_priority | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc622_circular_queue/) |
| 7 | **LC 208 Trie** | ⬜ | ⬜ | ⬜ | v4_prefix | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc208_trie/) |
| 11 | **LC 23 Merge K Lists** | ⬜ | ⬜ | ⬜ | v3_priority | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc23_merge_k_lists/) |
| 12 | **LC 239 Sliding Window** | ⬜ | ⬜ | ⬜ | v4_prefix | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc239_sliding_window_max/) |
| 14 | **LC 54 Spiral Matrix** | ⬜ | ⬜ | ⬜ | - | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc54_spiral_matrix/) |

### 🟡 P1 - 重要（理解 vLLM 机制）

<details>
<summary>点击展开 P1 题目列表</summary>

| # | 题目 | Python | C++ | Rust | 应用 | 笔记 |
|---|------|--------|-----|------|------|------|
| 2 | LC 56 Merge Intervals | ⬜ | ⬜ | ⬜ | - | [→](00_mountain/leetcode/llm-leetcode-20/lc56_merge_intervals/) |
| 3 | LC 380 Insert/Delete/GetRandom | ⬜ | ⬜ | ⬜ | - | [→](00_mountain/leetcode/llm-leetcode-20/lc380_insert_delete_getrandom/) |
| 5 | LC 215 Kth Largest | ⬜ | ⬜ | ⬜ | - | [→](00_mountain/leetcode/llm-leetcode-20/lc215_kth_largest/) |
| 6 | LC 253 Meeting Rooms II | ⬜ | ⬜ | ⬜ | - | [→](00_mountain/leetcode/llm-leetcode-20/lc253_meeting_rooms_ii/) |
| 10 | LC 238 Product Except Self | ⬜ | ⬜ | ⬜ | - | [→](00_mountain/leetcode/llm-leetcode-20/lc238_product_except_self/) |

</details>

### 🟢 P2 - 扩展（高级优化）

<details>
<summary>点击展开 P2 题目列表</summary>

| # | 题目 | Python | C++ | Rust | 笔记 |
|---|------|--------|-----|------|------|
| 8 | LC 211 Add & Search Words | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc211_add_search_words/) |
| 9 | LC 42 Trapping Rain Water | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc42_trapping_rain_water/) |
| 13 | LC 207 Course Schedule | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc207_course_schedule/) |
| 15 | LC 3 Longest Substring | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc3_longest_substring/) |
| 16 | LC 435 Non-overlapping Intervals | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc435_non_overlapping_intervals/) |
| 17 | LC 33 Search Rotated Array | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc33_search_rotated_array/) |
| 18 | LC 15 3Sum | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc15_3sum/) |
| 19 | LC 79 Word Search | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc79_word_search/) |
| 20 | LC 295 Find Median | ⬜ | ⬜ | ⬜ | [→](00_mountain/leetcode/llm-leetcode-20/lc295_find_median/) |

</details>

---

## 🚀 阶段 3：mini-vLLM 版本演进 (01_mini_vllm)

| 版本 | 核心特性 | 依赖数据结构/题目 | 状态 | 实验验证 |
|------|---------|-----------------|------|---------|
| **v1_naive** | 最简实现：list + FCFS | - | ✅ 已完成 | - |
| **v2_lru** | LRU 显存管理 | LC 146, Linked List, Hash Map | ⬜ | [EXP-001](02_surgery_room/experiments/exp001_verify_lru.py) |
| **v3_priority** | 优先级调度 + 抢占 | Heap, LC 622, LC 23 | ⬜ | [EXP-002](02_surgery_room/experiments/exp002_scheduler_policy.py) |
| **v4_prefix** | Prefix Caching | Trie, LC 208, LC 239 | ⬜ | [EXP-003](02_surgery_room/experiments/exp003_prefix_cache.py) |

**当前版本：** v1_naive  
**下一个里程碑：** v2_lru（需要完成 LC 146 + Linked List + Hash Map）

---

## 🔬 阶段 4：surgery_room 实验记录 (02_surgery_room)

| ID | 实验名称 | 目标 | 状态 | 结论/笔记 |
|----|---------|------|------|----------|
| EXP-001 | LRU vs vLLM BlockManager | 验证 v2_lru 的 LRU 逻辑与真实 vLLM 一致性 | ⬜ | - |
| EXP-002 | 调度策略对比 | 对比 FCFS/Priority/Preemption 的性能差异 | ⬜ | - |
| EXP-003 | Prefix Cache 命中率 | 测试不同场景下的 Prefix 复用效果 | ⬜ | - |
| EXP-004 | Nsight 性能分析 | 用 Nsight Systems 找到 vLLM 的瓶颈 | ⬜ | - |

---

## 📅 当前冲刺计划

### 本周目标 (Week 1)

- [ ] **Day 1-2:** 实现 Vector (Python only)
- [ ] **Day 3-4:** 实现 Hash Map (Python only)  
- [ ] **Day 5:** 实现 LC 146 LRU Cache (Python only)
- [ ] **Day 6:** 创建 `01_mini_vllm/core/v2_lru/` 骨架，集成 LRU
- [ ] **Day 7:** 写 EXP-001：对比你的 LRU 和真实 vLLM

**完成标准：** 
- ✅ 3 个数据结构有可运行的 Python 代码
- ✅ v2_lru 能跑起来并展示 LRU eviction
- ✅ 第一个实验有输出结果（即使简单）

### 本月目标 (Month 1)

- [ ] 完成所有 6 个核心数据结构（Python + C++）
- [ ] 完成 P0 的 6 道必做题（Python + C++）
- [ ] mini-vLLM 迭代到 v3_priority
- [ ] 完成 3 个 surgery_room 实验

---

## 📝 学习笔记区

### <a id="vector-notes"></a>Vector 笔记

- **开始日期：** _待填写_
- **Python vs C++ 性能对比：** _待填写_
- **在 vLLM 中的实际应用：** _待填写_
- **Aha Moment：** _待填写_

### <a id="hashmap-notes"></a>Hash Map 笔记

- **开始日期：** _待填写_
- **碰撞处理策略：** _待填写_
- **PagedAttention 映射逻辑：** _待填写_

### <a id="queue-notes"></a>Circular Queue 笔记

_待填写_

### <a id="linkedlist-notes"></a>Doubly Linked List 笔记

_待填写_

### <a id="heap-notes"></a>Heap 笔记

_待填写_

### <a id="trie-notes"></a>Trie 笔记

_待填写_

---

## 🎓 里程碑成就

- [ ] 🏅 **第一滴血：** 完成第一个数据结构的三语言实现
- [ ] 🏅 **闭环完成：** 完成第一个"练习→应用→验证"完整流程
- [ ] 🏅 **版本升级：** mini-vLLM 从 v1 升级到 v2
- [ ] 🏅 **性能优化：** 通过 Nsight 找到并优化一个瓶颈
- [ ] 🏅 **数据结构大师：** 完成所有 6 个核心数据结构
- [ ] 🏅 **刷题王者：** 完成所有 20 道 LeetCode 题目
- [ ] 🏅 **vLLM 通关：** mini-vLLM 达到 v4_prefix 版本

---

## 📚 快速导航

- **数据结构实现：** [00_mountain/leetcode/llm-infer-ds/](00_mountain/leetcode/llm-infer-ds/)
- **LeetCode 题目：** [00_mountain/leetcode/llm-leetcode-20/](00_mountain/leetcode/llm-leetcode-20/)
- **mini-vLLM 源码：** [01_mini_vllm/core/](01_mini_vllm/core/)
- **实验室：** [02_surgery_room/experiments/](02_surgery_room/experiments/)
- **学习路线图：** [00_mountain/leetcode/llm-inference-dsa-roadmap.md](00_mountain/leetcode/llm-inference-dsa-roadmap.md)

---

**记住：每完成一个数据结构，立即应用到 mini-vLLM，然后在 surgery_room 验证！**

