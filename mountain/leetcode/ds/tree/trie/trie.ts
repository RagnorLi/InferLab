/**
 * 前缀树 (Trie) - TypeScript 实现
 * 
 * ## 需求说明
 * 
 * 实现一个前缀树数据结构，要求：
 * 
 * ### 核心特性
 * - 用于字符串检索的树形结构
 * - 从根节点到某个节点的路径表示一个字符串前缀
 * 
 * ### 必须实现的操作
 * 1. `insert(word)` - 插入单词 O(m)
 * 2. `search(word)` - 查找完整单词是否存在 O(m)
 * 3. `startsWith(prefix)` - 检查是否有以prefix为前缀的单词 O(m)
 * 4. `delete(word)` - 删除单词 O(m)
 * 
 * ### 节点结构
 * - `TrieNode`: 包含 `children` Map 和 `isEnd` 标志
 * - `isEnd`: 标记是否是单词结尾
 * 
 * ### 时间复杂度要求
 * - 插入/查找：O(m)，m为字符串长度
 */
