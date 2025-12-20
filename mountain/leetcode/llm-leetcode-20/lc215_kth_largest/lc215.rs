// ==============================================================================
// LC 215. Kth Largest Element - Top-K Sampling
// ==============================================================================
//
// 【题目链接】https://leetcode.cn/problems/kth-largest-element-in-an-array/
//
// 【对应引擎模块】Top-K Sampling
// 【核心考点】QuickSelect 算法或 Min Heap
// 【Inference 映射】采样阶段选择概率最高的 K 个 token
//
// 【实现要求】
//   - 方法1: 使用 QuickSelect（平均 O(n)）
//   - 方法2: 使用 BinaryHeap（O(n log k)）
//   - 推荐实现两种方法并对比性能
//
// 【推荐语言】Rust, Python, C++
// ==============================================================================

