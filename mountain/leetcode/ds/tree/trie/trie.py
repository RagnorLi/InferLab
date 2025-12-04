"""
前缀树 (Trie) - Python 实现

## 需求说明

实现一个前缀树（字典树）数据结构，要求：

### 核心特性
- 用于字符串检索的树形结构
- 从根节点到某个节点的路径表示一个字符串前缀

### 必须实现的操作
1. `insert(word)` - 插入单词 O(m)，m为单词长度
2. `search(word)` - 查找完整单词是否存在 O(m)
3. `starts_with(prefix)` - 检查是否有以prefix为前缀的单词 O(m)
4. `delete(word)` - 删除单词 O(m)
5. `get_all_with_prefix(prefix)` - 获取所有以prefix为前缀的单词

### 节点结构
- `TrieNode`: 包含 `children` 字典和 `is_end` 标志
- `children`: 键为字符，值为子节点
- `is_end`: 标记是否是单词结尾

### 应用场景
- 字符串前缀匹配
- 单词自动补全
- IP路由表

### 时间复杂度要求
- 插入/查找/删除：O(m)，m为字符串长度
- 空间复杂度：O(ALPHABET_SIZE * N * M)

"""
