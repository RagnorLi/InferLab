# ==============================================================================
# Hash Map / Table - 对应 LLM Inference 中的 Page Table (Block Manager)
# ==============================================================================
#
# 【对应引擎模块】
#   - Page Table: PagedAttention 的核心映射表
#   - Block Manager: logical block id -> physical block id 的映射
#
# 【学习重点】
#   1. PagedAttention 核心映射：逻辑块到物理显存块的O(1)查找
#   2. 碰撞处理：链地址法（Separate Chaining）处理哈希冲突
#   3. 哈希函数设计：如何将 block id 均匀分布到桶中
#
# 【实现要求】
#   - 实现 put, get, remove 等操作，要求平均 O(1) 时间复杂度
#   - 使用链地址法处理碰撞（list of lists）
#   - 使用内置 hash() 函数计算哈希值
#   - 可选：实现动态扩容（load factor > 0.75 时 rehash）
#
# 【练习目标】
#   - 深入理解 vLLM PagedAttention 的显存管理机制
#   - 掌握哈希表在高频查询场景下的性能优化
#   - 模拟 logical KV block 到 physical GPU memory block 的映射
#
# 【提示】
#   - Python dict 底层使用开放寻址法，这里练习链地址法
#   - 建议实现 __getitem__ 和 __setitem__ 支持 [] 语法
#   - 可以对比 dict 和自实现的性能差异
# ==============================================================================

