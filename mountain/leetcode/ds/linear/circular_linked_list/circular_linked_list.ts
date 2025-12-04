/**
 * 循环链表 (Circular Linked List) - TypeScript 实现
 * 
 * ## 需求说明
 * 
 * 实现一个循环链表数据结构，要求：
 * 
 * ### 核心特性
 * - 链式存储，最后一个节点的 next 指向头节点
 * - 可以从任意节点开始遍历整个链表
 * 
 * ### 必须实现的操作
 * 1. `constructor()` - 初始化空链表
 * 2. `append(value)` - 在链表末尾添加节点 O(1)
 * 3. `prepend(value)` - 在链表头部添加节点 O(1)
 * 4. `insertAfter(node, value)` - 在指定节点后插入 O(1)
 * 5. `delete(value)` - 删除第一个匹配值的节点 O(n)
 * 6. `find(value)` - 查找第一个匹配值的节点 O(n)
 * 7. `length()` - 返回链表长度
 * 8. `isEmpty()` - 判断链表是否为空
 * 
 * ### 节点结构
 * - `Node`: 包含 `value` 和 `next` 指针
 * 
 * ### 时间复杂度要求
 * - 头部/尾部插入：O(1)
 * - 查找/删除：O(n)
 */
