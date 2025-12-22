# ==============================================================================
# LC 295. Find Median from Data Stream - Online Quantization
# ==============================================================================
#
# 【题目链接】https://leetcode.cn/problems/find-median-from-data-stream/
#
# 【对应引擎模块】Online Quantization
# 【核心考点】双堆（对顶堆），动态维护中位数
# 【Inference 映射】动态统计激活值分布，INT8 量化校准
#
# 【实现要求】
#   - 使用两个堆：heapq（最大堆需要取负）
#   - max_heap（左半部分）和 min_heap（右半部分）
#   - 插入时保持 max_heap 的最大值 <= min_heap 的最小值
#   - addNum: O(log n)，findMedian: O(1)
#
# 【推荐语言】Python (heapq), C++, Rust
# ==============================================================================

