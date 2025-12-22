# v2_lru - LRU 显存管理

**目标：** 实现 vLLM 的 Eviction 机制

## 新增特性

- 🆕 **LRU Cache** - 当显存不足时，驱逐最久未使用的块（基于 LC 146）
- 🆕 **Block Access Tracking** - 追踪每个块的访问时间
- 🆕 **Graceful Degradation** - 显存不足时不崩溃，而是驱逐旧块

## 依赖数据结构

- ✅ **Doubly Linked List** (从 `00_mountain/leetcode/llm-infer-ds/doubly_linked_list/`)
- ✅ **Hash Map** (从 `00_mountain/leetcode/llm-infer-ds/hash_map/`)
- ✅ **LC 146 LRU Cache** (从 `00_mountain/leetcode/llm-leetcode-20/lc146_lru_cache/`)

## 核心文件

- `block_manager.py` - 集成 LRU 的显存管理器
- `scheduler.py` - 继承自 v1，略作调整
- `engine.py` - 处理 eviction 的主循环

## 骨架代码

```python
# block_manager.py (你来实现！)

from typing import List, Dict, Optional

class LRUBlockManager:
    """带 LRU eviction 的显存管理器"""
    
    def __init__(self, num_blocks: int, block_size: int):
        # TODO: 集成你在 LC 146 中实现的 LRU Cache
        pass
    
    def allocate(self, request_id: str, num_tokens: int) -> List[int]:
        """分配块，如果不足则驱逐 LRU 块"""
        # TODO: 实现
        pass
    
    def access(self, request_id: str):
        """标记块被访问（更新 LRU 顺序）"""
        # TODO: 实现
        pass
    
    def evict_lru(self) -> Optional[str]:
        """驱逐最久未使用的块"""
        # TODO: 实现
        pass
```

## 验证实验

完成后在 `02_surgery_room/experiments/exp001_verify_lru.py` 中验证：
- 你的 LRU 逻辑是否正确
- 与真实 vLLM 的行为差异

## 性能目标

- Eviction 决策：O(1)
- Block 访问：O(1)

**下一步：** v3_priority，引入优先级调度

