# ==============================================================================
# Trie (Prefix Tree) - 对应 LLM Inference 中的 Tokenizer, Prefix Caching
# ==============================================================================
#
# 【对应引擎模块】
#   - Tokenizer: 词表存储和高效查找（BPE 分词）
#   - Prefix Caching: 缓存共享前缀的 KV Cache（RadixAttention）
#
# 【学习重点】
#   1. 前缀匹配：多个序列共享相同前缀时的高效存储
#   2. 树形递归与字典操作：每个节点维护子节点的映射
#   3. 空间优化：相比哈希表，Trie 可以共享公共前缀
#
# 【实现要求】
#   - 实现 TrieNode 类：包含 children (dict) 和 is_end (bool)
#   - 实现 Trie 类：insert, search, starts_with 方法
#   - 支持前缀查询：判断某个前缀是否存在
#   - 可选：实现自动完成（autocomplete）功能
#
# 【练习目标】
#   - 理解 vLLM RadixAttention 中 Prefix Caching 的原理
#   - 掌握树形结构在前缀匹配场景的优势
#   - 模拟 Tokenizer 中词表的高效查找机制
#
# 【提示】
#   - Python dict 天然适合实现 Trie 的子节点映射
#   - 建议使用 defaultdict 简化节点创建
#   - 可以实现 __contains__ 支持 in 运算符
# ==============================================================================

