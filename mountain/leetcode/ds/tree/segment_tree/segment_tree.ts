/**
 * 线段树 (Segment Tree) - TypeScript 实现
 * 
 * ## 需求说明
 * 
 * 实现一个线段树数据结构，要求：
 * 
 * ### 核心特性
 * - 用于区间查询和区间更新的数据结构
 * - 支持区间求和、最大值、最小值等操作
 * 
 * ### 必须实现的操作
 * 1. `buildTree(arr)` - 从数组构建线段树 O(n)
 * 2. `query(left, right)` - 查询区间聚合值 O(log n)
 * 3. `update(index, value)` - 单点更新 O(log n)
 * 4. `updateRange(left, right, value)` - 区间更新 O(log n)
 * 
 * ### 时间复杂度要求
 * - 构建：O(n)
 * - 查询/更新：O(log n)
 */
