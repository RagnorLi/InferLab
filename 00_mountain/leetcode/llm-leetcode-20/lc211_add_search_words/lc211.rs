// ==============================================================================
// LC 211. Design Add and Search Words - 复杂 Token 匹配
// ==============================================================================
//
// 【题目链接】https://leetcode.cn/problems/design-add-and-search-words-data-structure/
//
// 【对应引擎模块】复杂 Token 匹配
// 【核心考点】Trie + 回溯处理通配符
// 【Inference 映射】Constrained Decoding 中的模式匹配
//
// 【实现要求】
//   - 实现 WordDictionary 结构体：add_word(word), search(word)
//   - search 支持通配符 '.'（匹配任意字符）
//   - 使用 DFS 递归搜索，遇到 '.' 时遍历所有子节点
//   - 时间复杂度：最坏 O(26^m)
//
// 【推荐语言】Rust, Python, C++
// ==============================================================================

