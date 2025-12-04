/**
 * 循环链表 (Circular Linked List) - C++ 实现
 * 
 * ## 需求说明
 * 
 * 实现一个循环链表数据结构，要求：
 * 
 * ### 核心特性
 * - 链式存储，最后一个节点的 next 指向头节点
 * - 可以从任意节点开始遍历整个链表
 * - 支持单链表或双链表的循环版本
 * 
 * ### 必须实现的操作
 * 1. `CircularLinkedList()` - 构造函数
 * 2. `~CircularLinkedList()` - 析构函数
 * 3. `append(value)` - 在链表末尾添加节点 O(1)
 * 4. `prepend(value)` - 在链表头部添加节点 O(1)
 * 5. `insert_after(node, value)` - 在指定节点后插入 O(1)
 * 6. `delete_value(value)` - 删除第一个匹配值的节点 O(n)
 * 7. `find(value)` - 查找第一个匹配值的节点 O(n)
 * 8. `length()` - 返回链表长度
 * 9. `empty()` - 判断链表是否为空
 * 
 * ### 节点结构
 * - `Node`: 包含 `value` 和 `next` 指针
 * 
 * ### 时间复杂度要求
 * - 头部/尾部插入：O(1)
 * - 查找/删除：O(n)
 */
