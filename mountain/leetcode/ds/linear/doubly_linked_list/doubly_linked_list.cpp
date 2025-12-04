/**
 * 双链表 (Doubly Linked List) - C++ 实现
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
 * 1. `DoublyLinkedList()` - 构造函数
 * 2. `~DoublyLinkedList()` - 析构函数，释放所有节点
 * 3. `append(value)` - 在链表末尾添加节点 O(1)
 * 4. `prepend(value)` - 在链表头部添加节点 O(1)
 * 5. `insert_after(node, value)` - 在指定节点后插入 O(1)
 * 6. `insert_before(node, value)` - 在指定节点前插入 O(1)
 * 7. `delete_value(value)` - 删除第一个匹配值的节点 O(n)
 * 8. `delete_node(node)` - 删除指定节点 O(1)
 * 9. `find(value)` - 查找第一个匹配值的节点 O(n)
 * 10. `get(index)` - 获取指定位置的节点值 O(n)
 * 11. `length()` - 返回链表长度
 * 12. `empty()` - 判断链表是否为空
 * 
 * ### 节点结构
 * - `Node`: 包含 `value`、`prev` 和 `next` 指针
 * 
 * ### 时间复杂度要求
 * - 头部/尾部插入：O(1)
 * - 指定节点前后插入：O(1)
 * - 删除指定节点：O(1)
 */
