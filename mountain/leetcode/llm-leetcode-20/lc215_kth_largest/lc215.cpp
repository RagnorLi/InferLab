// ==============================================================================
// LC 215. Kth Largest Element - Top-K Sampling
// ==============================================================================
//
// 【题目链接】https://leetcode.cn/problems/kth-largest-element-in-an-array/
//
// 【对应引擎模块】
//   - Top-K Sampling: 采样阶段选择概率最高的 K 个 token
//   - Beam Search: 保留分数最高的 K 个候选序列
//
// 【核心考点】
//   1. QuickSelect 算法：平均 O(n) 找到第 K 大元素
//   2. Min Heap 方法：维护大小为 K 的最小堆，O(n log k)
//   3. Partition 操作：快排的核心思想
//
// 【实现要求】
//   - 方法1: 使用 QuickSelect（平均 O(n)，最坏 O(n²)）
//   - 方法2: 使用 Min Heap（O(n log k)，稳定性能）
//   - 推荐实现两种方法并对比性能
//
// 【Inference 映射】
//   - 数组元素: token 的 logit 分数
//   - 第 K 大元素: Top-K Sampling 的阈值
//   - 应用场景: 在 vocab_size=50k 的数组中快速找到 Top-50
//
// 【推荐语言】Python (heapq), C++, Rust
// ==============================================================================

