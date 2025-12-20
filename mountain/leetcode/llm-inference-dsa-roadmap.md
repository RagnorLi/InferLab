### 表 1：llm-infer-ds (核心数据结构)

| 数据结构 | 对应引擎模块 | 学习重点 | 推荐语言 |
| --- | --- | --- | --- |
| **Vector / Dynamic Array** | **KV Cache, Tensor** | 连续内存布局、扩容机制、Stride (步长) 访问 | C++ (必做), Rust |
| **Circular Queue (Ring Buffer)** | **Input Queue, NCCL 通信环** | 环形读写逻辑、Head/Tail 指针维护 | C++, Rust |
| **Hash Map / Table** | **Page Table (Block Manager)** | PagedAttention 核心映射、O(1) 查找、碰撞处理 | C++, Rust |
| **Linked List (Doubly)** | **LRU Cache (显存置换)** | 显存块热度维护、节点插入删除的指针操作 | C++ (练指针) |
| **Heap (Priority Queue)** | **Scheduler (调度器)** | 任务优先级排序 (Sift Down/Up) | Python, Rust |
| **Trie (Prefix Tree)** | **Tokenizer, Prefix Caching** | 前缀匹配、树形递归与指针操作 | C++, Python |

---

### 表 2：llm-leetcode-20 (Infer-Top-20 特种兵题单)

| ID | 题目 | 对应引擎模块 | 核心考点 (Inference 映射) |
| --- | --- | --- | --- |
| **1** | **LC 146. LRU Cache** | **显存置换 (Eviction)** | **必做**。双向链表+哈希表，管理显存块生命周期 |
| **2** | **LC 56. Merge Intervals** | **显存合并 (Defrag)** | 处理连续显存块的碎片整理与合并 |
| **3** | **LC 380. Insert Delete GetRandom O(1)** | **Block Allocator** | 设计高效的 Block 分配器 |
| **4** | **LC 622. Design Circular Queue** | **请求队列 (Input Q)** | **必做**。环形缓冲区，处理流式请求 |
| **5** | **LC 215. Kth Largest Element** | **Top-K Sampling** | 采样算法中的 Top-K 选择 (QuickSelect) |
| **6** | **LC 253. Meeting Rooms II** | **资源并发计算** | 计算同一时间有多少请求占用 GPU 资源 |
| **7** | **LC 208. Implement Trie** | **Tokenizer / Prefix Cache** | **必做**。词表存储与前缀缓存匹配 |
| **8** | **LC 211. Design Add & Search Words** | **复杂 Token 匹配** | 带有通配符的 Token 查找逻辑 |
| **9** | **LC 42. Trapping Rain Water** | **Attention Mask** | 数组双指针极致操作，类似 Mask 处理 |
| **10** | **LC 238. Product of Array Except Self** | **Softmax / Norm** | 前缀/后缀积，归一化算子的思维 |
| **11** | **LC 23. Merge k Sorted Lists** | **Beam Search** | **必做**。多路候选序列的高效合并 |
| **12** | **LC 239. Sliding Window Maximum** | **FlashAttention** | **必做**。滑动窗口最值，用于 Softmax 缩放优化 |
| **13** | **LC 207. Course Schedule** | **计算图 (Graph)** | 算子执行的拓扑排序 (Dependency) |
| **14** | **LC 54. Spiral Matrix** | **Triton Kernel Tiling** | **必做**。2D 矩阵复杂坐标变换，训练 Tiling 思维 |
| **15** | **LC 3. Longest Substring w/o Repeating** | **KV Cache Window** | 动态滑动窗口管理 |
| **16** | **LC 435. Non-overlapping Intervals** | **调度策略** | 贪心算法处理请求时间片 |
| **17** | **LC 33. Search in Rotated Sorted Array** | **Sharded Tensor** | 分布式切分数据的二分查找 |
| **18** | **LC 15. 3Sum** | **Kernel 指针优化** | 多指针扫描以减少显存读取 |
| **19** | **LC 79. Word Search** | **Constrained Decoding** | JSON Mode 等受限解码的回溯搜索 |
| **20** | **LC 295. Find Median from Data Stream** | **Online Quantization** | 动态统计激活值分布 (双堆) |