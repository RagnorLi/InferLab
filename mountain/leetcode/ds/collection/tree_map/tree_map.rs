/**
 * 树映射 (TreeMap) - Rust 实现
 * 
 * ## 需求说明
 * 
 * 实现一个树映射数据结构，要求：
 * 
 * ### 核心特性
 * - 基于平衡二叉搜索树的键值映射
 * - 键有序存储
 * 
 * ### 必须实现的操作
 * 1. `insert(key, value)` - 添加或更新键值对 O(log n)
 * 2. `get(key)` - 获取值，返回 Option<&V>
 * 3. `remove(key)` - 删除键值对 O(log n)
 * 4. `first_key_value()` - 获取最小键值对
 * 5. `last_key_value()` - 获取最大键值对
 * 6. `range(start, end)` - 范围查询
 * 
 * ### 时间复杂度要求
 * - 所有操作：O(log n)
 */
