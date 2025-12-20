// ==============================================================================
// Hash Map / Table - 对应 LLM Inference 中的 Page Table (Block Manager)
// ==============================================================================
//
// 【对应引擎模块】
//   - Page Table: PagedAttention 的核心映射表
//   - Block Manager: logical block id -> physical block id 的映射
//
// 【学习重点】
//   1. PagedAttention 核心映射：逻辑块到物理显存块的O(1)查找
//   2. 碰撞处理：链地址法（Separate Chaining）处理哈希冲突
//   3. 哈希函数设计：如何将 block id 均匀分布到桶中
//
// 【实现要求】
//   - 实现 put, get, remove 等操作，要求平均 O(1) 时间复杂度
//   - 使用 Vec<Vec<(K,V)>> 存储数据（链地址法）
//   - 使用 std::collections::hash_map::DefaultHasher 计算哈希值
//   - 正确处理泛型约束：K 需要实现 Hash + Eq
//
// 【练习目标】
//   - 理解 Rust 的 trait 系统在泛型数据结构中的应用
//   - 掌握哈希表在高频查询场景下的性能优化
//   - 模拟 logical KV block 到 physical GPU memory block 的映射
// ==============================================================================
