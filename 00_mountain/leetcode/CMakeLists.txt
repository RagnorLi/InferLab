cmake_minimum_required(VERSION 3.15)
project(LLMInferenceDSA LANGUAGES CXX)

# 设置 C++ 标准
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# 编译选项
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -g")

# ==============================================================================
# 【数据结构实现 - llm-infer-ds】
# ==============================================================================

# Vector
add_executable(vector_ds llm-infer-ds/vector/vector.cpp)
set_target_properties(vector_ds PROPERTIES OUTPUT_NAME "vector")

# Circular Queue
add_executable(circular_queue_ds llm-infer-ds/circular_queue/circular_queue.cpp)
set_target_properties(circular_queue_ds PROPERTIES OUTPUT_NAME "circular_queue")

# Doubly Linked List
add_executable(doubly_linked_list_ds llm-infer-ds/doubly_linked_list/doubly_linked_list.cpp)
set_target_properties(doubly_linked_list_ds PROPERTIES OUTPUT_NAME "doubly_linked_list")

# Hash Map
add_executable(hash_map_ds llm-infer-ds/hash_map/hash_map.cpp)
set_target_properties(hash_map_ds PROPERTIES OUTPUT_NAME "hash_map")

# Heap
add_executable(heap_ds llm-infer-ds/heap/heap.cpp)
set_target_properties(heap_ds PROPERTIES OUTPUT_NAME "heap")

# Trie
add_executable(trie_ds llm-infer-ds/trie/trie.cpp)
set_target_properties(trie_ds PROPERTIES OUTPUT_NAME "trie")

# ==============================================================================
# 【LeetCode 题目 - llm-leetcode-20】
# ==============================================================================

# LC 146 LRU Cache
add_executable(lc146 llm-leetcode-20/lc146_lru_cache/lc146.cpp)

# LC 622 Circular Queue
add_executable(lc622 llm-leetcode-20/lc622_circular_queue/lc622.cpp)

# LC 208 Trie
add_executable(lc208 llm-leetcode-20/lc208_trie/lc208.cpp)

# LC 23 Merge K Lists
add_executable(lc23 llm-leetcode-20/lc23_merge_k_lists/lc23.cpp)

# LC 239 Sliding Window Max
add_executable(lc239 llm-leetcode-20/lc239_sliding_window_max/lc239.cpp)

# LC 54 Spiral Matrix
add_executable(lc54 llm-leetcode-20/lc54_spiral_matrix/lc54.cpp)

# LC 56 Merge Intervals
add_executable(lc56 llm-leetcode-20/lc56_merge_intervals/lc56.cpp)

# LC 380 Insert/Delete/GetRandom
add_executable(lc380 llm-leetcode-20/lc380_insert_delete_getrandom/lc380.cpp)

# LC 215 Kth Largest
add_executable(lc215 llm-leetcode-20/lc215_kth_largest/lc215.cpp)

# LC 253 Meeting Rooms II
add_executable(lc253 llm-leetcode-20/lc253_meeting_rooms_ii/lc253.cpp)

# LC 238 Product Except Self
add_executable(lc238 llm-leetcode-20/lc238_product_except_self/lc238.cpp)

# LC 211 Add & Search Words
add_executable(lc211 llm-leetcode-20/lc211_add_search_words/lc211.cpp)

# LC 42 Trapping Rain Water
add_executable(lc42 llm-leetcode-20/lc42_trapping_rain_water/lc42.cpp)

# LC 207 Course Schedule
add_executable(lc207 llm-leetcode-20/lc207_course_schedule/lc207.cpp)

# LC 3 Longest Substring
add_executable(lc3 llm-leetcode-20/lc3_longest_substring/lc3.cpp)

# LC 435 Non-overlapping Intervals
add_executable(lc435 llm-leetcode-20/lc435_non_overlapping_intervals/lc435.cpp)

# LC 33 Search Rotated Array
add_executable(lc33 llm-leetcode-20/lc33_search_rotated_array/lc33.cpp)

# LC 15 3Sum
add_executable(lc15 llm-leetcode-20/lc15_3sum/lc15.cpp)

# LC 79 Word Search
add_executable(lc79 llm-leetcode-20/lc79_word_search/lc79.cpp)

# LC 295 Find Median
add_executable(lc295 llm-leetcode-20/lc295_find_median/lc295.cpp)

# ==============================================================================
# 【使用说明】
# ==============================================================================
# 编译所有项目：
#   mkdir build && cd build
#   cmake ..
#   make
#
# 编译单个项目：
#   make vector_ds
#   make lc146
#
# 运行：
#   ./vector_ds
#   ./lc146
# ==============================================================================

