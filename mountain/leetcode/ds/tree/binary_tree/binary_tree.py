"""
二叉树 (Binary Tree) - Python 实现

## 需求说明

实现一个二叉树数据结构，要求：

### 核心特性
- 每个节点最多有两个子节点：左子节点和右子节点
- 支持递归和迭代的遍历方式

### 必须实现的操作
1. `__init__(value)` - 创建节点，初始化值
2. `insert_left(value)` - 插入左子节点
3. `insert_right(value)` - 插入右子节点
4. `preorder()` - 前序遍历 O(n)
5. `inorder()` - 中序遍历 O(n)
6. `postorder()` - 后序遍历 O(n)
7. `level_order()` - 层序遍历 O(n)
8. `height()` - 计算树的高度 O(n)
9. `size()` - 计算节点数量 O(n)
10. `search(value)` - 查找节点 O(n)

### 节点结构
- `TreeNode`: 包含 `value`、`left` 和 `right` 指针

### 遍历方式
- 前序：根 -> 左 -> 右
- 中序：左 -> 根 -> 右
- 后序：左 -> 右 -> 根
- 层序：按层级从上到下，从左到右

### 时间复杂度要求
- 遍历：O(n)
- 插入：O(1)
- 查找：O(n)

"""
