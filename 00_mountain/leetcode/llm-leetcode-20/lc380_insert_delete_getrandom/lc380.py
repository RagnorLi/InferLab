# ==============================================================================
# LC 380. Insert Delete GetRandom O(1) - Block Allocator
# ==============================================================================
#
# 【题目链接】https://leetcode.cn/problems/insert-delete-getrandom-o1/
#
# 【对应引擎模块】Block Allocator
# 【核心考点】O(1) 插入/删除/随机获取：结合数组和哈希表
# 【Inference 映射】高效的显存块分配器，随机淘汰策略
#
# 【实现要求】
#   - 实现 RandomizedSet 类：insert(val), remove(val), getRandom()
#   - 使用 list + dict (val -> index)
#   - 删除时将待删元素与末尾交换，再 pop
#   - 所有操作平均 O(1) 时间复杂度
#
# 【推荐语言】Python, C++, Rust
# ==============================================================================

