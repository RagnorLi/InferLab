"""
数组 (Array) - Python 实现

## 需求说明

实现一个数组数据结构，要求：

### 核心特性
- 连续内存存储，支持随机访问
- 通过索引访问元素，时间复杂度 O(1)
- 支持动态扩容

### 必须实现的操作
1. `__init__(capacity)` - 初始化数组，指定容量
2. `__getitem__(index)` - 通过索引获取元素 O(1)
3. `__setitem__(index, value)` - 通过索引设置元素 O(1)
4. `append(value)` - 在数组末尾追加元素 O(1) 平均
5. `insert(index, value)` - 在指定位置插入元素 O(n)
6. `delete(index)` - 删除指定位置的元素 O(n)
7. `__len__()` - 返回数组长度 O(1)
8. `__repr__()` - 字符串表示

### 可选优化
- 动态扩容策略（2倍扩容或1.5倍扩容）
- 缩容机制（当元素数量远小于容量时）

### 时间复杂度要求
- 随机访问：O(1)
- 插入/删除：O(n)
- 追加：O(1) 平均（考虑扩容摊销）

"""

from ast import Raise, main
from re import S
import re


class Array:
    """
    动态数组实现

    核心思想：
    1. 使用python list 作为底层存储 (连续内存)
    2. 维护容量 (capacity) 和实际长度 (size)
    3. 当容量不足时，按 2 倍扩容。虽然单次扩容操作最坏情况下需要 O(n) 时间，但平均到每次 append 操作上的时间复杂度是 O(1)，这就是摊销（amortized）O(1)。即，多次操作的总耗时均摊到每次操作单独计算，单次开销高但长期平均低。
    4. 当元素过少时，按 1/2 缩容 (可选优化)
    """

    def __init__(self, capacity: int = 10) -> None:
        """
        初始化数组

        Args:
            capacity: 初始容量, 默认为10
        """
        if capacity <= 0:
            raise ValueError("Capacity must be positive")
        
        # _对象的私有属性 __类的私有属性
        self._capacity = capacity
        self._size = 0
        # 使用 None 作为占位符，表示未使用的槽位 等价于 self._data = [None for _ in range(capacity)]
        self._data = [None] * capacity

    # 注意区分：前后各有两个下划线（如 __getitem__）的是“魔法方法”，用于 Python 内部协议（如支持下标访问、运算符重载等），不是私有方法。
    # 只有前面有两个下划线、但后面没有（如 __foo），才会触发“名称重整”（name mangling），相当于类的“伪私有方法”，但本质上依然可以通过特殊名字访问。
    def __getitem__(self, index: int):
        """
        通过索引获取元素 O(1)

        Args:
            index: 索引位置

        Returns:
            对应位置的元素

        Raises:
            IndexError: 索引越界
        """
        if not 0 <= index < self._size:
            raise IndexError(f"Index {index} out of range [0, {self._size}]")
        return self._data[index]

    # https://docs.python.org/3/reference/datamodel.html#special-method-names
    def __setitem__(self, index: int, value):
        """
        通过索引设置元素 O(1)

        Args:
            index: 索引位置
            value: 要设置的值
            
        Raises:
            IndexError: 索引越界
        """
        if not 0 <= index < self._size:
            raise IndexError(f"Index {index} out of range [0, {self._size})")
        self._data[index] = value


    def append(self, value):
        """
        在数组末尾追加元素 O(1) 摊销

        当容量不足时，先扩容再插入，摊销时间复杂度为 O（1）

        Args:
            Value: 要追加的值

        """
        # 如果容量不足，先扩容
        if self._size >= self._capacity:
            self._resize(self._capacity * 2)
        
        self._data[self._size] = value # self._size means the storaged value length
        self._size += 1

    def insert(self, index: int, value):
        """
        在指定位置插入元素 O(n)

        需要将 index 之后的元素都向后移动一位

        Args:
            index: 插入位置
            value: 要插入的值

        Raises:
            IndexError: 索引越界
        """
        if not 0 <= index <= self._size:
            raise IndexError(f"Index {index} out of range [0, {self._size}]")

        # 如果容量不足，先扩容
        if self._size >= self._capacity:
            self._resize(self._capacity * 2)

        # 将 index 之后的元素向后移动: 从最后一个开始一个一个向后退
        for i in range(self._size, index, -1):
            self._data[i] = self._data[i - 1]

        self._data[index] = value
        self._size += 1

    def delete(self, index: int):
        """
        删除指定位置的元素 O(n)

        需要将 index 之后的元素都向前移动一位

        Args:
            index: 要删除的位置
        
        Returns:
            Object: 被删除的元素
        
        Raises:
            IndexError: 索引越界
        """
        if not 0 <= index < self._size:
            raise IndexError(f"Index {index} out of range [0, {self._size}]")

        removed = self._data[index]

        # 将index 之后的元素向前移动
        for i in range(index, self._size - 1):
            self._data[i] = self._data[i + 1]

        self._size -= 1
        self._data[self._size] = None # 清理最后一个位置

        # 可选： 如果元素数量远小于容量，进行缩容
        # 避免频繁扩容缩容，使用1/4 作为阈值
        if self._size > 0 and self._size < self._capacity // 4:
            self._resize(self._capacity // 2)

        return removed

    def __len__(self) -> int:
        """
        返回数组长度 O(1)
        """
        return self._size

    def __repr__(self) -> str:
        """
        字符串表示，只显示实际元素
        """
        elements = [self._data[i] for i in range(self._size)]
        return f"Array({elements}), size={self._size}. capacity={self._capacity}"

    def _resize(self, new_capacity: int):
        """
        内部方法： 调整数组容量

        时间复杂度： 单次_resize复杂度是O(n)因为复制所有元素, 但摊销到多次操作后， 在整体 append/use 场景下平均为 O(1)

        Args: 
            new_capacity: 新的容量
        
        Raise:
            ValueError: 值错误
        """
        if new_capacity < self._size:
            raise ValueError(f"New capacity {new_capacity} is too small for {self.size} elements")

        old_data = self._data
        self._data = [None] * new_capacity
        self._capacity = new_capacity

        # 复制所有元素
        for i in range(self._size):
            self._data[i] = old_data[i]

if __name__ == "__main__":
    # 基础功能测试
    arr = Array(capacity=3)
    print(f"初始状态: {arr}")
    
    # 测试 append
    arr.append(1)
    arr.append(2)
    arr.append(3)
    print(f"追加 3 个元素: {arr}")
    
    # 测试扩容
    arr.append(4)
    print(f"触发扩容后: {arr}")
    
    # 测试随机访问
    print(f"arr[0] = {arr[0]}")
    arr[0] = 10
    print(f"修改后 arr[0] = {arr[0]}")
    
    # 测试 insert
    arr.insert(1, 99)
    print(f"在索引 1 插入 99: {arr}")
    
    # 测试 delete
    removed = arr.delete(2)
    print(f"删除索引 2 的元素 {removed}: {arr}")
    
    # 测试缩容（需要删除足够多的元素）
    arr.delete(0)
    arr.delete(0)
    print(f"删除后可能触发缩容: {arr}")