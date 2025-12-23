# ==============================================================================
# Vector / Dynamic Array - 对应 LLM Inference 中的 KV Cache, Tensor
# ==============================================================================
#
# 【对应引擎模块】
#   - KV Cache: 存储每个 token 的 key/value 向量
#   - Tensor: 多维数组的底层存储
#
# 【学习重点】
#   1. 连续内存布局：所有元素在内存中连续存储，cache-friendly
#   2. 扩容机制：当容量不足时如何进行 2x 扩容
#   3. Stride (步长) 访问：模拟 Tensor 的多维访问模式
#
# 【实现要求】
#   - 实现 push, pop, get, resize 等核心操作
#   - 使用 Python list 或 ctypes.Array 模拟底层数组
#   - 支持 stride 访问：给定起始位置和步长，返回指定元素
#   - 测试扩容时的性能和内存拷贝开销
#
# 【练习目标】
#   - 理解 Python list 的底层实现（PyListObject）
#   - 对比 C++ vector 的性能差异
#   - 模拟 vLLM 中 KV Cache 的内存管理
#
# 【提示】
#   - 可以使用 array.array 获得更接近 C 的性能
#   - 可以用 __getitem__ 和 __setitem__ 实现下标访问
#   - 建议实现 __len__ 和 __repr__ 方法便于调试
# ==============================================================================

# ==============================================================================
# 【逻辑蓝图 - 用中文描述每一步】
# ==============================================================================
#
# 核心操作 1: push_back (添加元素到末尾)
#   步骤 1: 检查当前大小是否等于容量
#   步骤 2: 如果满了，调用扩容函数（容量翻倍）
#   步骤 3: 在 data[size] 位置放入新元素
#   步骤 4: size 加 1
#
# 核心操作 2: pop_back (删除末尾元素)
#   步骤 1: 检查是否为空（size == 0）
#   步骤 2: 如果为空，抛出错误或返回 None
#   步骤 3: size 减 1
#   步骤 4: 返回 data[size] 的值（注意：size 已经减 1 了）
#
# 核心操作 3: get (通过索引获取元素)
#   步骤 1: 检查索引是否有效（0 <= index < size）
#   步骤 2: 返回 data[index]
#
# 核心操作 4: resize (扩容)
#   步骤 1: 分配新的数组，大小为 new_capacity
#   步骤 2: 把旧数组的所有元素复制到新数组
#   步骤 3: 释放旧数组（Python 中自动，C++ 需要手动）
#   步骤 4: 更新 capacity 为 new_capacity
#
# 核心操作 5: stride (步长访问 - 模拟 Tensor)
#   步骤 1: 从 start_index 开始
#   步骤 2: 每次跳 stride 步
#   步骤 3: 收集所有访问到的元素
#   例子: [0, 1, 2, 3, 4, 5], start=1, stride=2 → [1, 3, 5]
#
# 核心操作 6: __getitem__ (魔法方法 - 支持 v[index] 读取)
#   步骤 1: 检查索引是否有效（0 <= index < size）
#   步骤 2: 返回 data[index]
#   作用: 让 Vector 可以像 list 一样使用 v[0] 语法
#
# 核心操作 7: __setitem__ (魔法方法 - 支持 v[index] = value 修改)
#   步骤 1: 检查索引是否有效（0 <= index < size）
#   步骤 2: 把 value 赋值给 data[index]
#   作用: 让 Vector 可以像 list 一样使用 v[0] = 5 语法
# ==============================================================================

# ==============================================================================
# 【Phase 3: Python 语法教学 - 骨架代码】
# ==============================================================================

class Vector:
    """
    Vector 类：模拟动态数组
    
    语法解释：
    - class Vector: 定义一个类（就像设计一个蓝图）
    - def __init__(self, initial_capacity=4): 
        * __init__ 是"构造函数"，创建对象时自动调用
        * self 是"自己"的意思，代表这个对象本身
        * initial_capacity=4 是默认参数，如果不传就用 4
    """
    
    def __init__(self, initial_capacity=4):
        """
        初始化 Vector
        
        任务：你需要在这里初始化三个变量
        1. self.data = ?  # 用什么来存储数据？（提示：Python 的 list）
        2. self.size = ?  # 当前有多少元素？（提示：开始时是 0）
        3. self.capacity = ?  # 容量是多少？（提示：用传入的 initial_capacity）
        """
        # TODO: 在这里填写你的代码
        # self.data = 
        # self.size = 
        # self.capacity = 
        pass
    
    def push_back(self, value):
        """
        在末尾添加元素
        
        语法解释：
        - def push_back(self, value): 
            * def 定义函数
            * self 是对象自己（必须写）
            * value 是要添加的值
        
        任务：按照逻辑蓝图的步骤实现
        步骤 1: 检查是否需要扩容（size >= capacity）
        步骤 2: 如果需要，调用 self._resize()（我们稍后实现）
        步骤 3: 把 value 放到 data[size] 位置
        步骤 4: size 加 1
        """
        # TODO: 步骤 1 - 检查是否需要扩容
        # if 
        
        # TODO: 步骤 2 - 如果需要扩容，调用 self._resize()
        #   提示：扩容时容量翻倍，即 self.capacity * 2
        
        # TODO: 步骤 3 - 把 value 放到 data[size] 位置
        # self.data[?] = ?
        
        # TODO: 步骤 4 - size 加 1
        # self.size = ?
        pass
    
    def pop_back(self):
        """
        删除并返回末尾元素
        
        任务：
        步骤 1: 检查是否为空（size == 0）
        步骤 2: 如果为空，抛出 IndexError("Vector is empty")
        步骤 3: size 减 1
        步骤 4: 返回 data[size]（注意：size 已经减 1 了）
        """
        # TODO: 步骤 1 & 2 - 检查是否为空
        # if 
        #     raise IndexError("Vector is empty")
        
        # TODO: 步骤 3 - size 减 1
        # self.size = ?
        
        # TODO: 步骤 4 - 返回最后一个元素
        # return ?
        pass
    
    def get(self, index):
        """
        通过索引获取元素
        
        语法解释：
        - 索引检查：0 <= index < self.size
        
        任务：
        步骤 1: 检查索引是否有效
        步骤 2: 如果无效，抛出 IndexError(f"Index {index} out of range")
        步骤 3: 返回 data[index]
        """
        # TODO: 步骤 1 & 2 - 检查索引
        # if 
        #     raise IndexError(f"Index {index} out of range")
        
        # TODO: 步骤 3 - 返回元素
        # return ?
        pass
    
    def _resize(self, new_capacity):
        """
        扩容函数（私有方法，前面有下划线）
        
        语法解释：
        - _resize 前面的下划线表示这是"内部方法"，外部不应该直接调用
        - new_capacity 是新的容量大小
        
        任务：
        步骤 1: 创建新数组（用 list，大小为 new_capacity）
        步骤 2: 把旧数组的所有元素复制到新数组（用 for 循环）
        步骤 3: 更新 self.data 和 self.capacity
        """
        # TODO: 步骤 1 - 创建新数组
        # new_data = [0] * new_capacity  # 或者用 None
        
        # TODO: 步骤 2 - 复制旧数据
        # for i in range(?):  # 循环多少次？
        #     new_data[i] = ?
        
        # TODO: 步骤 3 - 更新
        # self.data = ?
        # self.capacity = ?
        pass
    
    def stride(self, start_index, stride):
        """
        步长访问（模拟 Tensor 的多维访问）
        
        任务：
        从 start_index 开始，每次跳 stride 步，收集所有元素
        例子: [0, 1, 2, 3, 4, 5], start=1, stride=2 → [1, 3, 5]
        
        提示：用 while 循环，条件是 index < size
        """
        # TODO: 实现步长访问
        # result = []
        # index = start_index
        # while ?:
        #     result.append(?)
        #     index += ?
        # return result
        pass
    
    def __len__(self):
        """
        魔法方法：让 len(vector) 可以工作
        
        语法解释：
        - __len__ 是 Python 的特殊方法
        - 当你写 len(my_vector) 时，Python 会自动调用这个方法
        """
        # TODO: 返回当前大小
        # return ?
        pass
    
    def __repr__(self):
        """
        魔法方法：让 print(vector) 可以显示内容
        
        语法解释：
        - __repr__ 定义对象的字符串表示
        - 返回一个字符串，描述这个对象
        """
        # TODO: 返回一个字符串，比如 "Vector(size=3, capacity=4, data=[1, 2, 3])"
        # return f"Vector(size={?}, capacity={?}, data={?})"
        pass
    
    def __getitem__(self, index):
        """
        魔法方法：让 v[index] 可以工作（读取元素）
        
        语法解释：
        - __getitem__ 是 Python 的特殊方法
        - 当你写 v[0] 时，Python 会自动调用 v.__getitem__(0)
        - 这让你可以像使用 list 一样使用 Vector
        
        任务：
        步骤 1: 检查索引是否有效（0 <= index < size）
        步骤 2: 如果无效，抛出 IndexError(f"Index {index} out of range")
        步骤 3: 返回 data[index]
        
        提示：这个方法和 get() 方法逻辑一样，可以直接调用 self.get(index)
        """
        # TODO: 步骤 1 & 2 - 检查索引
        # if 
        #     raise IndexError(f"Index {index} out of range")
        
        # TODO: 步骤 3 - 返回元素
        # return ?
        # 或者直接调用：return self.get(index)
        pass
    
    def __setitem__(self, index, value):
        """
        魔法方法：让 v[index] = value 可以工作（修改元素）
        
        语法解释：
        - __setitem__ 是 Python 的特殊方法
        - 当你写 v[0] = 5 时，Python 会自动调用 v.__setitem__(0, 5)
        - 这让你可以像使用 list 一样修改 Vector 的元素
        
        任务：
        步骤 1: 检查索引是否有效（0 <= index < size）
        步骤 2: 如果无效，抛出 IndexError(f"Index {index} out of range")
        步骤 3: 把 value 赋值给 data[index]
        """
        # TODO: 步骤 1 & 2 - 检查索引
        # if 
        #     raise IndexError(f"Index {index} out of range")
        
        # TODO: 步骤 3 - 赋值
        # self.data[?] = ?
        pass


# ==============================================================================
# 【测试代码 - 填空后运行这个】
# ==============================================================================

if __name__ == "__main__":
    # 创建 Vector
    v = Vector(initial_capacity=4)
    print(f"初始状态: {v}")
    
    # 测试 push_back
    print("\n=== 测试 push_back ===")
    for i in range(6):
        v.push_back(i)
        print(f"添加 {i} 后: {v}")
    
    # 测试 get
    print("\n=== 测试 get ===")
    print(f"v.get(0) = {v.get(0)}")
    print(f"v.get(2) = {v.get(2)}")
    
    # 测试 __getitem__ 和 __setitem__（下标访问）
    print("\n=== 测试下标访问 v[index] ===")
    print(f"v[0] = {v[0]}")  # 使用 __getitem__
    print(f"v[1] = {v[1]}")
    v[0] = 99  # 使用 __setitem__
    print(f"修改 v[0] = 99 后: {v}")
    print(f"v[0] = {v[0]}")
    
    # 测试 stride
    print("\n=== 测试 stride ===")
    print(f"stride(1, 2) = {v.stride(1, 2)}")
    
    # 测试 pop_back
    print("\n=== 测试 pop_back ===")
    while len(v) > 0:
        val = v.pop_back()
        print(f"弹出 {val} 后: {v}")
