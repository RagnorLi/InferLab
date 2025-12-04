"""
树映射 (TreeMap) - Python 实现

## 需求说明

实现一个树映射数据结构，要求：

### 核心特性
- 基于平衡二叉搜索树（如红黑树）的键值映射
- 键有序存储，支持范围查询
- 键唯一，值可以重复

### 必须实现的操作
1. `put(key, value)` - 添加或更新键值对 O(log n)
2. `get(key)` - 获取值 O(log n)
3. `remove(key)` - 删除键值对 O(log n)
4. `contains_key(key)` - 判断键是否存在 O(log n)
5. `first_key()` - 获取最小键 O(log n)
6. `last_key()` - 获取最大键 O(log n)
7. `floor_key(key)` - 获取小于等于key的最大键
8. `ceiling_key(key)` - 获取大于等于key的最小键
9. `range_query(start, end)` - 范围查询
10. `keys()` - 有序获取所有键 O(n)

### 与HashMap的区别
- TreeMap：有序，O(log n) 操作
- HashMap：无序，O(1) 平均操作

"""
