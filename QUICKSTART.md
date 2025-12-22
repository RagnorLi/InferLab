# 🚀 Quick Start - 立即开始！

> **第一周任务：完成第一个"练习→应用→验证"闭环**

## Day 1-2: 实现 Vector (Python only)

### 1. 打开模板文件

```bash
cd 00_mountain/leetcode/llm-infer-ds/vector
# 编辑 vector.py
```

### 2. 实现核心功能

```python
# 你需要实现：
class Vector:
    def __init__(self, capacity=10):
        # TODO: 初始化
        pass
    
    def push(self, value):
        # TODO: 添加元素，必要时扩容
        pass
    
    def pop(self):
        # TODO: 删除末尾元素
        pass
    
    def get(self, index):
        # TODO: 获取元素
        pass
```

### 3. 测试你的实现

```python
# 在 vector.py 末尾添加测试
if __name__ == "__main__":
    v = Vector()
    for i in range(20):
        v.push(i)
    print(f"Length: {len(v)}")
    print(f"Capacity: {v.capacity}")
```

### 4. 更新进度

在根目录 `PROGRESS.md` 中，将 Vector 的 Python 列改为 ✅

---

## Day 3-4: 实现 Hash Map (Python only)

### 1. 打开模板

```bash
cd 00_mountain/leetcode/llm-infer-ds/hash_map
# 编辑 hash_map.py
```

### 2. 实现核心功能

关键：碰撞处理（链表法或开放寻址）

### 3. 测试

运行你的测试，确保 get/put 都是 O(1)

---

## Day 5: 实现 LC 146 LRU Cache (Python only)

### 1. 打开模板

```bash
cd 00_mountain/leetcode/llm-leetcode-20/lc146_lru_cache
# 编辑 lc146.py
```

### 2. 实现

使用双向链表 + 哈希表（或直接用 OrderedDict）

### 3. 提交 LeetCode 验证

确保通过所有测试用例！

---

## Day 6: 创建 mini-vLLM v2_lru

### 1. 打开骨架文件

```bash
cd 01_mini_vllm/core/v2_lru
# 编辑 block_manager.py
```

### 2. 集成你的 LRU Cache

```python
from path_to_your_lc146 import LRUCache

class LRUBlockManager:
    def __init__(self, num_blocks, block_size):
        self.lru_cache = LRUCache(capacity=num_blocks)
        # ... 实现逻辑
```

### 3. 测试

```bash
cd ../../tests
python test_v2_lru.py
```

---

## Day 7: 验证实验

### 1. 运行实验

```bash
cd 02_surgery_room/experiments
python exp001_verify_lru.py
```

### 2. 记录结果

在 `PROGRESS.md` 的"学习笔记区"记录：
- 你的 LRU 实现与 vLLM 的差异
- 遇到的问题
- Aha Moment

---

## 🎉 完成第一周！

你现在有了：
- ✅ 2 个数据结构的实现（Vector, Hash Map）
- ✅ 1 个 LeetCode 题的解答（LC 146）
- ✅ mini-vLLM 的第一次升级（v1 → v2）
- ✅ 第一个实验验证

**这是完整的一个闭环！**

---

## 下一步

查看 `PROGRESS.md` 的"本月目标"，继续前进：
- Doubly Linked List
- Heap
- Circular Queue
- 更多 LeetCode 题目
- v3_priority 版本

---

## 💡 Tips

1. **每天只专注一件事** - 不要想着一次完成所有
2. **代码自己写** - 不要复制粘贴，一行一行敲
3. **立即测试** - 写一点就测一点，不要等到最后
4. **记录思考** - 每天在 PROGRESS.md 写一段笔记
5. **遇到困难很正常** - 看文档、查资料、慢慢来

---

**现在就开始！打开 `00_mountain/leetcode/llm-infer-ds/vector/vector.py`**

