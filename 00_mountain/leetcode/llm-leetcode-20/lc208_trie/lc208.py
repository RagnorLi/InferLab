# ==============================================================================
# LC 208. Implement Trie - Tokenizer / Prefix Cache 【必做】
# ==============================================================================
#
# 【题目链接】https://leetcode.cn/problems/implement-trie-prefix-tree/
#
# 【对应引擎模块】Tokenizer, Prefix Caching
# 【核心考点】前缀树结构，高效前缀匹配
# 【Inference 映射】BPE 词表存储，RadixAttention 共享前缀
#
# 【实现要求】
#   - 实现 Trie 类：insert(word), search(word), startsWith(prefix)
#   - 使用 dict 存储子节点，is_end 标记完整单词
#   - 时间复杂度：O(m)，m 为字符串长度
#
# 【推荐语言】Python, C++, Rust
# ==============================================================================

