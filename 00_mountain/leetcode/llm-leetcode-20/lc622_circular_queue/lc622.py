# ==============================================================================
# LC 622. Design Circular Queue - 请求队列 (Input Q) 【必做】
# ==============================================================================
#
# 【题目链接】https://leetcode.cn/problems/design-circular-queue/
#
# 【对应引擎模块】请求队列 (Input Queue)
# 【核心考点】环形缓冲区：Head/Tail 指针维护
# 【Inference 映射】流式请求的接收和缓冲，Continuous Batching
#
# 【实现要求】
#   - 实现 MyCircularQueue 类：enQueue, deQueue, Front, Rear, isEmpty, isFull
#   - 使用固定大小的 list + 头尾指针
#   - 正确处理边界条件（满/空）
#   - 时间复杂度：所有操作 O(1)
#
# 【推荐语言】Python, C++, Rust
# ==============================================================================

