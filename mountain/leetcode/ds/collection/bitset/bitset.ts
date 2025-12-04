/**
 * 位集合 (BitSet) - TypeScript 实现
 * 
 * ## 需求说明
 * 
 * 实现一个位集合数据结构，要求：
 * 
 * ### 核心特性
 * - 用位向量表示集合
 * - 每个位表示一个元素是否存在
 * 
 * ### 必须实现的操作
 * 1. `constructor(size)` - 初始化，指定位集合大小
 * 2. `set(index)` - 设置第index位为1 O(1)
 * 3. `clear(index)` - 设置第index位为0 O(1)
 * 4. `get(index)` - 获取第index位的值 O(1)
 * 5. `count()` - 统计1的个数
 * 6. `all()` - 判断是否所有位都是1
 * 7. `any()` - 判断是否有任何位是1
 * 8. `union(other)` - 位或运算
 * 9. `intersection(other)` - 位与运算
 * 
 * ### 时间复杂度要求
 * - 单个位操作：O(1)
 */
