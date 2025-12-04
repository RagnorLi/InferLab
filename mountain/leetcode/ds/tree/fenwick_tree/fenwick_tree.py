"""
树状数组 (Fenwick Tree / BIT) - Python 实现

## 需求说明

实现一个树状数组数据结构，要求：

### 核心特性
- 用于前缀和查询和单点更新的高效数据结构
- 空间复杂度 O(n)，比线段树更简单
- 基于二进制索引

### 必须实现的操作
1. `__init__(size)` - 初始化树状数组
2. `update(index, delta)` - 更新index位置的值，增加delta O(log n)
3. `query(index)` - 查询前缀和[0, index] O(log n)
4. `range_query(left, right)` - 查询区间和[left, right] O(log n)
5. `get(index)` - 获取单点值（通过前缀和计算）O(log n)

### 核心思想
- 利用二进制的lowbit函数
- lowbit(x) = x & (-x)
- 每个节点存储一定范围的前缀和

### 应用场景
- 前缀和查询
- 逆序对统计
- 单点更新、区间查询

### 时间复杂度要求
- 更新：O(log n)
- 查询：O(log n)

"""
