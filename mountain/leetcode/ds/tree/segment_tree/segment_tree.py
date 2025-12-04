"""
线段树 (Segment Tree) - Python 实现

## 需求说明

实现一个线段树数据结构，要求：

### 核心特性
- 用于区间查询和区间更新的数据结构
- 支持区间求和、最大值、最小值等操作
- 基于完全二叉树实现，用数组存储

### 必须实现的操作
1. `build_tree(arr)` - 从数组构建线段树 O(n)
2. `query(left, right)` - 查询区间[left, right]的聚合值 O(log n)
3. `update(index, value)` - 单点更新 O(log n)
4. `update_range(left, right, value)` - 区间更新 O(log n)
5. `lazy_propagation()` - 懒标记传播（可选，用于区间更新优化）

### 数组索引关系
- 根节点：1（索引0不使用或存储总和）
- 左子节点：2*i
- 右子节点：2*i + 1

### 应用场景
- 区间求和、最大值、最小值查询
- 区间更新
- RMQ（Range Minimum Query）问题

### 时间复杂度要求
- 构建：O(n)
- 查询：O(log n)
- 更新：O(log n)

"""
