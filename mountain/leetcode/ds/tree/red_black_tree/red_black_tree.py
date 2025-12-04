"""
红黑树 (Red-Black Tree) - Python 实现

## 需求说明

实现一个红黑树数据结构，要求：

### 核心特性
- 自平衡二叉搜索树
- 通过颜色标记和旋转保持平衡
- 相比AVL树更宽松的平衡条件，插入删除更快

### 红黑树性质
1. 每个节点是红色或黑色
2. 根节点是黑色
3. 每个叶子节点（NIL）是黑色
4. 红色节点的子节点必须是黑色（不能有连续红色节点）
5. 从任一节点到其每个叶子节点的路径包含相同数目的黑色节点

### 必须实现的操作
1. `insert(value)` - 插入节点，自动平衡 O(log n)
2. `delete(value)` - 删除节点，自动平衡 O(log n)
3. `search(value)` - 查找节点 O(log n)
4. `left_rotate(node)` - 左旋转
5. `right_rotate(node)` - 右旋转
6. `fix_insert(node)` - 插入后修复
7. `fix_delete(node)` - 删除后修复

### 时间复杂度要求
- 所有操作：O(log n)

"""
