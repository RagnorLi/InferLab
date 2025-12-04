/**
 * 树映射 (TreeMap) - C++ 实现
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
 * 2. `find(key)` - 查找值 O(log n)
 * 3. `erase(key)` - 删除键值对 O(log n)
 * 4. `begin()` - 获取最小键
 * 5. `end()` - 获取最大键
 * 6. `lower_bound(key)` - 获取大于等于key的最小键
 * 7. `upper_bound(key)` - 获取大于key的最小键
 * 
 * ### 时间复杂度要求
 * - 所有操作：O(log n)
 */
