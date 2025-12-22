# v3_priority - 优先级调度 + 抢占

**目标：** 实现 vLLM 的 Preemption 和 Priority Scheduling

## 新增特性

- 🆕 **Priority Queue** - 基于优先级调度请求（使用 Heap）
- 🆕 **Preemption** - 高优先级请求可以抢占低优先级请求
- 🆕 **Circular Queue** - 更高效的请求队列管理（基于 LC 622）

## 依赖数据结构

- ✅ **Heap** (从 `00_mountain/leetcode/llm-infer-ds/heap/`)
- ✅ **Circular Queue** (从 `00_mountain/leetcode/llm-infer-ds/circular_queue/`)
- ✅ **LC 622** (从 `00_mountain/leetcode/llm-leetcode-20/lc622_circular_queue/`)
- ✅ **LC 23 Merge K Lists** (Beam Search 的基础)

## 核心文件

- `block_manager.py` - 继承自 v2_lru
- `scheduler.py` - 基于 Heap 的优先级调度器
- `engine.py` - 处理 preemption 的主循环
- `priority_queue.py` - 封装 Heap 实现

## 骨架代码

```python
# scheduler.py (你来实现！)

import heapq
from dataclasses import dataclass
from typing import List

@dataclass
class PriorityRequest:
    request_id: str
    priority: int  # 数字越小优先级越高
    prompt: str
    max_tokens: int
    
    def __lt__(self, other):
        return self.priority < other.priority

class PriorityScheduler:
    """基于 Heap 的优先级调度器"""
    
    def __init__(self, max_batch_size: int = 4):
        self.max_batch_size = max_batch_size
        self.waiting_heap = []  # Min-heap
        self.running_queue = []
    
    def add_request(self, request: PriorityRequest):
        """添加请求到优先级队列"""
        # TODO: 使用 heapq.heappush
        pass
    
    def schedule(self) -> List[PriorityRequest]:
        """调度，支持抢占"""
        # TODO: 实现抢占逻辑
        pass
    
    def preempt(self, request_id: str):
        """抢占指定请求"""
        # TODO: 实现
        pass
```

## 验证实验

完成后在 `02_surgery_room/experiments/exp002_scheduler_policy.py` 中对比：
- FCFS vs Priority 的吞吐量差异
- 抢占策略的效果

**下一步：** v4_prefix，引入 Prefix Caching

