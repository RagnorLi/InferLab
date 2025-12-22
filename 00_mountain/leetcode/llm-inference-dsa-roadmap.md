# LLM Inference DSA Roadmap

> **目标：** 通过实现这些数据结构和题目，深入理解 vLLM 的核心机制  
> **进度追踪：** 见根目录 [PROGRESS.md](../../PROGRESS.md)

---

## 🎯 核心数据结构 (llm-infer-ds)

| # | 数据结构 | vLLM 模块 | 学习重点 | 应用到 mini-vLLM |
|---|---------|-----------|---------|-----------------|
| 1 | **Vector** | KV Cache, Tensor | 连续内存、扩容、Stride | v2_lru (替换 list) |
| 2 | **Hash Map** | Page Table (BlockManager) | O(1) 查找、碰撞处理 | v2_lru (Block 映射) |
| 3 | **Circular Queue** | Input Queue | 环形读写、指针维护 | v3_priority (请求队列) |
| 4 | **Doubly Linked List** | LRU Cache | 指针操作、节点插删 | v2_lru (LRU eviction) |
| 5 | **Heap** | Scheduler | 优先级排序 (Sift) | v3_priority (调度器) |
| 6 | **Trie** | Tokenizer, Prefix Cache | 前缀匹配、树形递归 | v4_prefix (Prefix Cache) |

**实现要求：** Python (理解逻辑) → C++ (练指针/性能) → Rust (练所有权)

---

## 💪 LeetCode Top-20 (llm-leetcode-20)

### 🔴 P0 - 必做（直接应用到 mini-vLLM）

| # | 题目 | vLLM 模块 | 应用到版本 | 验证实验 |
|---|------|-----------|-----------|---------|
| 1 | [**LC 146 LRU Cache**](https://leetcode.cn/problems/lru-cache/) | 显存置换 | v2_lru | EXP-001 |
| 4 | [**LC 622 Circular Queue**](https://leetcode.cn/problems/design-circular-queue/) | 请求队列 | v3_priority | EXP-002 |
| 7 | [**LC 208 Trie**](https://leetcode.cn/problems/implement-trie-prefix-tree/) | Prefix Cache | v4_prefix | EXP-003 |
| 11 | [**LC 23 Merge K Lists**](https://leetcode.cn/problems/merge-k-sorted-lists/) | Beam Search | v3_priority | - |
| 12 | [**LC 239 Sliding Window Max**](https://leetcode.cn/problems/sliding-window-maximum/) | FlashAttention | v4_prefix | - |
| 14 | [**LC 54 Spiral Matrix**](https://leetcode.cn/problems/spiral-matrix/) | Kernel Tiling | - | - |

### 🟡 P1 - 重要（理解 vLLM 机制）

| # | 题目 | vLLM 模块 | 核心考点 |
|---|------|-----------|---------|
| 2 | [LC 56 Merge Intervals](https://leetcode.cn/problems/merge-intervals/) | 显存碎片整理 | 区间合并 |
| 3 | [LC 380 Insert/Delete/GetRandom](https://leetcode.cn/problems/insert-delete-getrandom-o1/) | Block Allocator | O(1) 操作 |
| 5 | [LC 215 Kth Largest](https://leetcode.cn/problems/kth-largest-element-in-an-array/) | Top-K Sampling | QuickSelect |
| 6 | [LC 253 Meeting Rooms II](https://leetcode.cn/problems/meeting-rooms-ii/) | 资源并发 | 扫描线 |
| 10 | [LC 238 Product Except Self](https://leetcode.cn/problems/product-of-array-except-self/) | Softmax/Norm | 前后缀积 |

### 🟢 P2 - 扩展（高级优化）

<details>
<summary>点击展开 9 道扩展题目</summary>

| # | 题目 | vLLM 模块 |
|---|------|-----------|
| 8 | [LC 211 Add & Search Words](https://leetcode.cn/problems/design-add-and-search-words-data-structure/) | Token 匹配 |
| 9 | [LC 42 Trapping Rain Water](https://leetcode.cn/problems/trapping-rain-water/) | Attention Mask |
| 13 | [LC 207 Course Schedule](https://leetcode.cn/problems/course-schedule/) | 计算图 |
| 15 | [LC 3 Longest Substring](https://leetcode.cn/problems/longest-substring-without-repeating-characters/) | KV Cache Window |
| 16 | [LC 435 Non-overlapping Intervals](https://leetcode.cn/problems/non-overlapping-intervals/) | 调度策略 |
| 17 | [LC 33 Search Rotated Array](https://leetcode.cn/problems/search-in-rotated-sorted-array/) | Sharded Tensor |
| 18 | [LC 15 3Sum](https://leetcode.cn/problems/3sum/) | Kernel 指针优化 |
| 19 | [LC 79 Word Search](https://leetcode.cn/problems/word-search/) | Constrained Decoding |
| 20 | [LC 295 Find Median](https://leetcode.cn/problems/find-median-from-data-stream/) | Online Quantization |

</details>

---

## 🚀 学习路径

```
Week 1-2: 数据结构 (Python)
  ├─ Vector, Hash Map, Linked List
  └─ 立即应用到 mini-vLLM v2_lru

Week 3-4: P0 题目 (Python)
  ├─ LC 146, 622, 208
  └─ 完成 v3_priority

Week 5-8: C++/Rust 重写
  └─ 性能对比 & 优化

Week 9+: P1/P2 题目 & 高级特性
```

**记住：边练边用，立即验证！** 详细进度见 [PROGRESS.md](../../PROGRESS.md)