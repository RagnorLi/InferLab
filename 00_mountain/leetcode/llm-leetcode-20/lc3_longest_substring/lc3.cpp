// ==============================================================================
// LC 3. Longest Substring Without Repeating Characters - KV Cache Window
// ==============================================================================
//
// 【题目链接】https://leetcode.cn/problems/longest-substring-without-repeating-characters/
//
// 【对应引擎模块】
//   - KV Cache Window: 动态滑动窗口管理
//   - Sliding Window Attention: 限制 Attention 的上下文长度
//
// 【核心考点】
//   1. 滑动窗口: 动态调整窗口大小
//   2. 哈希表记录位置: 快速检测重复字符
//   3. 双指针: left 和 right 维护当前窗口
//
// 【实现要求】
//   - 使用哈希表记录每个字符的最新位置
//   - 遇到重复字符时，移动 left 指针到重复位置+1
//   - 动态更新最大长度
//   - 时间复杂度：O(n)，空间复杂度：O(min(n, charset))
//
// 【Inference 映射】
//   - 无重复子串: KV Cache 窗口内的有效 token 范围
//   - 滑动窗口: Streaming LLM 中的动态窗口管理
//   - 应用场景: 限制 Attention 窗口大小，降低显存占用
//
// 【推荐语言】C++, Python, Rust
// ==============================================================================

