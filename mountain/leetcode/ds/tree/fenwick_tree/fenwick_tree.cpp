/**
 * 树状数组 (Fenwick Tree / BIT) - C++ 实现
 * 
 * ## 需求说明
 * 
 * 实现一个树状数组数据结构，要求：
 * 
 * ### 核心特性
 * - 用于前缀和查询和单点更新的高效数据结构
 * - 基于二进制索引
 * 
 * ### 必须实现的操作
 * 1. `FenwickTree(size)` - 构造函数
 * 2. `update(index, delta)` - 更新index位置的值 O(log n)
 * 3. `query(index)` - 查询前缀和[0, index] O(log n)
 * 4. `rangeQuery(left, right)` - 查询区间和 O(log n)
 * 
 * ### 核心思想
 * - 利用lowbit函数：x & (-x)
 * 
 * ### 时间复杂度要求
 * - 更新/查询：O(log n)
 */
