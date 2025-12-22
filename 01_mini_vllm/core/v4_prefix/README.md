# v4_prefix - Prefix Caching

**目标：** 实现 vLLM 的 Automatic Prefix Caching

## 新增特性

- 🆕 **Prefix Cache** - 使用 Trie 存储和复用 KV Cache（基于 LC 208）
- 🆕 **Sliding Window** - 高效管理 Cache 窗口（基于 LC 239）
- 🆕 **Cache 命中率统计** - 监控复用效果

## 依赖数据结构

- ✅ **Trie** (从 `00_mountain/leetcode/llm-infer-ds/trie/`)
- ✅ **LC 208 Implement Trie** (从 `00_mountain/leetcode/llm-leetcode-20/lc208_trie/`)
- ✅ **LC 239 Sliding Window Maximum** (Cache 管理)

## 核心文件

- `block_manager.py` - 继承自 v2_lru，增加 Prefix 管理
- `scheduler.py` - 继承自 v3_priority
- `engine.py` - 集成 Prefix Caching
- `prefix_cache.py` - Trie 实现的 Prefix Cache

## 骨架代码

```python
# prefix_cache.py (你来实现！)

from typing import List, Optional, Tuple

class TrieNode:
    def __init__(self):
        self.children = {}  # token_id -> TrieNode
        self.block_ids = []  # 存储的 KV Cache block IDs
        self.is_end = False
        self.hit_count = 0

class PrefixCache:
    """使用 Trie 实现的 Prefix Cache"""
    
    def __init__(self):
        self.root = TrieNode()
        self.total_hits = 0
        self.total_misses = 0
    
    def search(self, token_ids: List[int]) -> Tuple[List[int], int]:
        """
        搜索最长匹配前缀
        
        Returns:
            (cached_block_ids, matched_length)
        """
        # TODO: 遍历 Trie，找到最长匹配
        pass
    
    def insert(self, token_ids: List[int], block_ids: List[int]):
        """插入新的 Prefix 及其对应的 blocks"""
        # TODO: 在 Trie 中插入
        pass
    
    def get_stats(self) -> dict:
        """获取缓存统计信息"""
        return {
            'hit_rate': self.total_hits / (self.total_hits + self.total_misses),
            'total_hits': self.total_hits,
            'total_misses': self.total_misses
        }
```

## 验证实验

完成后在 `02_surgery_room/experiments/exp003_prefix_cache.py` 中测试：
- 不同场景下的 Cache 命中率
- 对吞吐量的提升效果
- 与真实 vLLM 的对比

## 性能目标

- Prefix 查找：O(L)，L 为序列长度
- Cache 命中时节省大量计算

**这是最终版本！** 恭喜你完成了 mini-vLLM 的完整演进 🎉

