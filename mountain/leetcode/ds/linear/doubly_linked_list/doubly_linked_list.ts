/**
 * 双链表 (Doubly Linked List) - TypeScript 实现
 * 
 * ## 需求说明
 * 
 * 实现一个双链表数据结构，要求：
 * 
 * ### 核心特性
 * - 链式存储，节点包含数据、指向前一个节点和后一个节点的指针
 * - 支持双向遍历
 * - 支持动态添加和删除节点
 * 
 * ### 必须实现的操作
 * 1. `constructor()` - 初始化空链表
 * 2. `append(value)` - 在链表末尾添加节点 O(1)
 * 3. `prepend(value)` - 在链表头部添加节点 O(1)
 * 4. `insertAfter(node, value)` - 在指定节点后插入 O(1)
 * 5. `insertBefore(node, value)` - 在指定节点前插入 O(1)
 * 6. `delete(value)` - 删除第一个匹配值的节点 O(n)
 * 7. `deleteNode(node)` - 删除指定节点 O(1)
 * 8. `find(value)` - 查找第一个匹配值的节点 O(n)
 * 9. `get(index)` - 获取指定位置的节点值 O(n)
 * 10. `length()` - 返回链表长度
 * 11. `isEmpty()` - 判断链表是否为空
 * 
 * ### 节点结构
 * - `Node`: 包含 `value`、`prev` 和 `next` 指针
 * 
 * ### 时间复杂度要求
 * - 头部/尾部插入：O(1)
 * - 指定节点前后插入：O(1)
 * - 删除指定节点：O(1)
 */
