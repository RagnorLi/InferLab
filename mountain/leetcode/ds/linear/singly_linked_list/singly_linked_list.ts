/**
 * 单链表 (Singly Linked List) - TypeScript 实现
 * 
 * ## 需求说明
 * 
 * 实现一个单链表数据结构，要求：
 * 
 * ### 核心特性
 * - 链式存储，节点包含数据和指向下一个节点的指针
 * - 支持动态添加和删除节点
 * - 顺序访问，不支持随机访问
 * 
 * ### 必须实现的操作
 * 1. `constructor()` - 初始化空链表
 * 2. `append(value)` - 在链表末尾添加节点 O(n)
 * 3. `prepend(value)` - 在链表头部添加节点 O(1)
 * 4. `insertAfter(node, value)` - 在指定节点后插入 O(1)
 * 5. `delete(value)` - 删除第一个匹配值的节点 O(n)
 * 6. `deleteAt(index)` - 删除指定位置的节点 O(n)
 * 7. `find(value)` - 查找第一个匹配值的节点 O(n)
 * 8. `get(index)` - 获取指定位置的节点值 O(n)
 * 9. `length()` - 返回链表长度
 * 10. `isEmpty()` - 判断链表是否为空
 * 11. `reverse()` - 反转链表（可选）
 * 
 * ### 节点结构
 * - `Node`: 包含 `value` 和 `next` 指针
 * 
 * ### 时间复杂度要求
 * - 头部插入：O(1)
 * - 尾部插入：O(n)
 * - 查找/删除：O(n)
 */
