// ==============================================================================
// LC 380. Insert Delete GetRandom O(1) - Block Allocator
// ==============================================================================
//
// 【题目链接】https://leetcode.cn/problems/insert-delete-getrandom-o1/
//
// 【对应引擎模块】
//   - Block Allocator: 高效的显存块分配器
//   - Random Eviction: 随机淘汰策略（某些场景的替代方案）
//
// 【核心考点】
//   1. O(1) 插入/删除/随机获取：结合数组和哈希表
//   2. 数组末尾交换删除：避免移动元素
//   3. 哈希表维护索引：快速定位元素位置
//
// 【实现要求】
//   - 实现 RandomizedSet 类：insert(val), remove(val), getRandom()
//   - 使用 vector + unordered_map<val, index>
//   - 删除时将待删元素与末尾交换，再 pop_back
//   - 所有操作平均 O(1) 时间复杂度
//
// 【Inference 映射】
//   - val: physical block id
//   - insert/remove: 分配/释放显存块
//   - getRandom: 随机选择一个块进行淘汰（Random Eviction Policy）
//
// 【推荐语言】C++, Python, Rust
// ==============================================================================

