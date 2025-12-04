"""
字符串 (String) - Python 实现

## 需求说明

实现一个字符串数据结构，要求：

### 核心特性
- 字符序列，基于字符数组
- 支持字符串的基本操作

### 必须实现的操作
1. `__init__(value)` - 初始化字符串
2. `__len__()` - 返回字符串长度 O(1)
3. `__getitem__(index)` - 通过索引访问字符 O(1)
4. `__add__(other)` - 字符串拼接 O(n)
5. `substring(start, end)` - 获取子串 O(n)
6. `find(pattern)` - 查找子串位置 O(n*m)
7. `replace(old, new)` - 替换子串 O(n)
8. `split(delimiter)` - 分割字符串 O(n)
9. `lower()` - 转小写 O(n)
10. `upper()` - 转大写 O(n)
11. `strip()` - 去除首尾空白字符 O(n)

### 实现方式
- 基于字符数组（列表）

### 时间复杂度要求
- 随机访问：O(1)
- 拼接/替换：O(n)
- 查找：O(n*m)

"""
