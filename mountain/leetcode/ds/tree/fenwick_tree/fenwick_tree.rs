/**
 * 树状数组 (Fenwick Tree / BIT) - Rust 实现
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
 * 1. `new(size)` - 创建新树状数组
 * 2. `update(index, delta)` - 更新index位置的值 O(log n)
 * 3. `query(index)` - 查询前缀和[0, index] O(log n)
 * 4. `range_query(left, right)` - 查询区间和 O(log n)
 * 
 * ### 时间复杂度要求
 * - 更新/查询：O(log n)
 */
