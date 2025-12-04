"""
哈希集合 (HashSet) - Python 实现

## 需求说明

实现一个哈希集合数据结构，要求：

### 核心特性
- 无序元素集合，基于哈希表实现
- 元素唯一，不允许重复
- 快速判断元素是否存在

### 必须实现的操作
1. `add(value)` - 添加元素 O(1) 平均
2. `remove(value)` - 删除元素 O(1) 平均
3. `contains(value)` - 判断元素是否存在 O(1) 平均
4. `union(other)` - 并集操作
5. `intersection(other)` - 交集操作
6. `difference(other)` - 差集操作
7. `size()` - 返回元素数量 O(1)
8. `is_empty()` - 判断是否为空 O(1)
9. `clear()` - 清空所有元素 O(n)

### 应用场景
- 去重
- 快速查找
- 集合运算

"""
