// ==============================================================================
// LC 3. Longest Substring Without Repeating Characters - KV Cache Window
// ==============================================================================
//
// 【题目链接】https://leetcode.cn/problems/longest-substring-without-repeating-characters/
//
// 【对应引擎模块】KV Cache Window
// 【核心考点】滑动窗口，哈希表记录位置
// 【Inference 映射】动态窗口管理，Sliding Window Attention
//
// 【实现要求】
//   - 使用 HashMap 记录每个字符的最新位置
//   - 遇到重复字符时，移动 left 指针到重复位置+1
//   - 动态更新最大长度
//   - 时间复杂度：O(n)
//
// 【推荐语言】Rust, Python, C++
// ==============================================================================

