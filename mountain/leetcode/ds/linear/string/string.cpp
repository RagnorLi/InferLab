/**
 * 字符串 (String) - C++ 实现
 * 
 * ## 需求说明
 * 
 * 实现一个字符串数据结构，要求：
 * 
 * ### 核心特性
 * - 字符序列，基于字符数组
 * - 支持字符串的基本操作
 * 
 * ### 必须实现的操作
 * 1. `String(const char* str)` - 构造函数
 * 2. `~String()` - 析构函数
 * 3. `length()` - 返回字符串长度 O(1)
 * 4. `operator[](index)` - 通过索引访问字符 O(1)
 * 5. `operator+(const String& other)` - 字符串拼接 O(n)
 * 6. `substring(start, end)` - 获取子串 O(n)
 * 7. `find(pattern)` - 查找子串位置 O(n*m)
 * 8. `replace(old, new)` - 替换子串 O(n)
 * 9. `split(delimiter)` - 分割字符串 O(n)
 * 10. `c_str()` - 返回 C 风格字符串
 * 
 * ### 实现方式
 * - 基于字符数组
 * 
 * ### 时间复杂度要求
 * - 随机访问：O(1)
 * - 拼接/替换：O(n)
 */
