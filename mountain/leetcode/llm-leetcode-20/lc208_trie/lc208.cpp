// ==============================================================================
// LC 208. Implement Trie - Tokenizer / Prefix Cache 【必做】
// ==============================================================================
//
// 【题目链接】https://leetcode.cn/problems/implement-trie-prefix-tree/
//
// 【对应引擎模块】
//   - Tokenizer: BPE 词表存储和高效查找
//   - Prefix Caching: 共享前缀的 KV Cache（RadixAttention）
//
// 【核心考点】
//   1. 前缀树结构：每个节点维护子节点映射（map 或数组）
//   2. 高效前缀匹配：共享公共前缀，节省空间
//   3. 递归与指针操作：树形结构的遍历
//
// 【实现要求】
//   - 实现 Trie 类：insert(word), search(word), startsWith(prefix)
//   - 每个节点包含 map<char, TrieNode*> 和 is_end 标志
//   - 时间复杂度：O(m)，m 为字符串长度
//
// 【Inference 映射】
//   - 单词: token 序列
//   - 前缀匹配: 多个请求共享相同的 prompt 前缀
//   - Prefix Caching: vLLM RadixAttention 共享前缀的 KV Cache
//
// 【推荐语言】C++ (练习指针), Python, Rust (Box + HashMap)
// ==============================================================================

