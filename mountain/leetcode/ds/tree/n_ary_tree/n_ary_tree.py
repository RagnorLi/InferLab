"""
N 叉树 (N-ary Tree) - Python 实现

## 需求说明

实现一个 N 叉树数据结构，要求：

### 核心特性
- 每个节点可以有 N 个子节点（N >= 2）
- 是二叉树的一般化形式
- 适合表示层次关系

### 必须实现的操作
1. `__init__(value)` - 创建节点
2. `add_child(value)` - 添加子节点
3. `remove_child(value)` - 删除子节点
4. `get_children()` - 获取所有子节点
5. `preorder()` - 前序遍历 O(n)
6. `postorder()` - 后序遍历 O(n)
7. `level_order()` - 层序遍历 O(n)
8. `height()` - 计算树的高度 O(n)
9. `size()` - 计算节点数量 O(n)

### 节点结构
- `Node`: 包含 `value` 和 `children` 列表

### 应用场景
- 文件系统
- XML/HTML 解析树
- 组织架构
- 决策树

### 时间复杂度要求
- 遍历：O(n)
- 插入/删除子节点：O(1) 或 O(k)，k为子节点数

"""
