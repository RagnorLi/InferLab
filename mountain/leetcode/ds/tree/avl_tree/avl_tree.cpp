/**
 * AVL 树 (AVL Tree) - C++ 实现
 * 
 * ## 需求说明
 * 
 * 实现一个 AVL 树数据结构，要求：
 * 
 * ### 核心特性
 * - 自平衡二叉搜索树
 * - 每个节点的左右子树高度差不超过1
 * - 通过旋转操作保持平衡
 * 
 * ### 必须实现的操作
 * 1. `insert(value)` - 插入节点，自动平衡 O(log n)
 * 2. `remove(value)` - 删除节点，自动平衡 O(log n)
 * 3. `search(value)` - 查找节点 O(log n)
 * 4. `leftRotate(node)` - 左旋转
 * 5. `rightRotate(node)` - 右旋转
 * 6. `getBalance(node)` - 获取节点平衡因子
 * 7. `height(node)` - 获取节点高度
 * 
 * ### 时间复杂度要求
 * - 所有操作：O(log n)
 */
