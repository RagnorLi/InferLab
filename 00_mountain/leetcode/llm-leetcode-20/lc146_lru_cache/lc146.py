# ==============================================================================
# LC 146. LRU Cache - 显存置换 (Eviction) 【必做】
# ==============================================================================
#
# 【题目链接】https://leetcode.cn/problems/lru-cache/
#
# 【对应引擎模块】显存置换 (Eviction)
# 【核心考点】双向链表 + 哈希表：O(1) 的 get/put 操作
# 【Inference 映射】vLLM Block Manager 管理显存块生命周期
#
# 【实现要求】
#   - 实现 LRUCache 类：__init__(capacity), get(key), put(key, value)
#   - 使用 dict + 自定义双向链表（或 collections.OrderedDict）
#   - get 和 put 都要将访问的节点移到链表头部
#   - 时间复杂度：O(1)，空间复杂度：O(capacity)
#
# 【推荐语言】Python (OrderedDict), C++, Rust
# ==============================================================================

