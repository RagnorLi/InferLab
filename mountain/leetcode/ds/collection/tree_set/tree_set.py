"""
树集合 (TreeSet) - Python 实现

## 需求说明

实现一个树集合数据结构，要求：

### 核心特性
- 基于平衡二叉搜索树的有序元素集合
- 元素唯一，不允许重复
- 元素有序存储

### 必须实现的操作
1. `add(value)` - 添加元素 O(log n)
2. `remove(value)` - 删除元素 O(log n)
3. `contains(value)` - 判断元素是否存在 O(log n)
4. `first()` - 获取最小元素 O(log n)
5. `last()` - 获取最大元素 O(log n)
6. `floor(value)` - 获取小于等于value的最大元素
7. `ceiling(value)` - 获取大于等于value的最小元素
8. `range_query(start, end)` - 范围查询
9. `size()` - 返回元素数量 O(1)

### 与HashSet的区别
- TreeSet：有序，O(log n) 操作
- HashSet：无序，O(1) 平均操作

"""
