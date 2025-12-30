# ==============================================================================
# Hash Map / Table - 对应 LLM Inference 中的 Page Table (Block Manager)
# ==============================================================================
#
# 【对应引擎模块】
#   - Page Table: PagedAttention 的核心映射表
#   - Block Manager: logical block id -> physical block id 的映射
#
# 【学习重点】
#   1. PagedAttention 核心映射：逻辑块到物理显存块的O(1)查找
#   2. 碰撞处理：链地址法（Separate Chaining）处理哈希冲突
#   3. 哈希函数设计：如何将 block id 均匀分布到桶中
#
# 【实现要求】
#   - 实现 put, get, remove 等操作，要求平均 O(1) 时间复杂度
#   - 使用链地址法处理碰撞（list of lists）
#   - 使用内置 hash() 函数计算哈希值
#   - 可选：实现动态扩容（load factor > 0.75 时 rehash）
#
# 【练习目标】
#   - 深入理解 vLLM PagedAttention 的显存管理机制
#   - 掌握哈希表在高频查询场景下的性能优化
#   - 模拟 logical KV block 到 physical GPU memory block 的映射
#
# 【提示】
#   - Python dict 底层使用开放寻址法，这里练习链地址法
#   - 建议实现 __getitem__ 和 __setitem__ 支持 [] 语法
#   - 可以对比 dict 和自实现的性能差异
# ==============================================================================

from ast import main
import re


class HashMap:

    def __init__(self, size=16) -> None:
        """
        初始化Hash Map
        size: 桶的数量，默认16个桶
        """
        self.buckets = [ [] for _ in range(size)]  # 每个桶都是一个空列表
        self.size = size # 桶的数量
        self.count = 0  # 不是桶（buckets）的个数，而是当前所有桶里存的(key, value)对的总数量，比如插入三组键值对，这里就是3

    def put(self, key, value):
        # 1. 计算哈希值，确定桶位置
        bucket_index = hash(key) % self.size # 取余，因为余数永远不可能大于除数

        # 2. 检查桶中是否已有相同key
        bucket = self.buckets[bucket_index]

        for i, (k, v) in enumerate(bucket):
            if k == key:
                # 更新现有值 - 覆盖
                bucket[i] = (key, value)
                return
        
        # 3. 如果没有相同key, 添加到链表末尾
        bucket.append((key, value))
        self.count += 1

        # 4.可选：检查是否需要扩容
        if self.count / self.size > 0.75: # 泊松分布 + 生日悖论 决定的
            self._resize()
    
    def get(self, key):
        """
        
        查找操作：返回key对应的value, 如果不存在返回None

        key: 索引
        """

        # 1. 计算哈希值，确定桶位置
        bucket_index = hash(key) % self.size

        # 2.遍历桶中的链表，查找匹配的key
        bucket = self.buckets[bucket_index]
        for k, v in bucket:
            if k == key:
                return v
        
        # 3. 如果没找到，返回None
        return None

    def remove(self , key):
        """
        删除指定的key-value对
        """
        # 1.计算哈希值，确定桶位置
        bucket_index = hash(key) % self.size

        # 2. 遍历桶中的链表，查找匹配的key
        bucket = self.buckets[bucket_index]

        for i, (k, v) in enumerate(bucket):
            if k == key:
                # 删除找到的元素
                bucket.pop(i)
                self.count -= 1
                return
        
        # 3. 如果没找到，静默返回，什么都不做

    def _resize(self):
        # 1.计算新大小 （通常是当前大小的2倍）
        new_size = self.size * 2

        # 2. 创建新的更大的桶数组
        new_buckets = [ [] for _ in range(new_size)]

        # 3. 重新哈希所有现在元素到新桶中
        for bucket in self.buckets: # 遍历所有旧桶
            for key, value in bucket: # 遍历每个桶中的所有 (key, value)对
                new_index = hash(key) % new_size # 重新计算哈希值在新桶中的位置
                new_buckets[new_index].append((key, value)) # 从旧桶复制过来不存在key重复

        # 4. 更新实例属性
        self.buckets = new_buckets
        self.size = new_size


def test_resize():
    hm = HashMap(size=4)  # 从更小的size开始，更容易触发扩容
    
    print(f"初始大小: {hm.size}")
    print(f"初始元素数: {hm.count}")
    
    # 插入足够多的元素触发扩容 (4 * 0.75 = 3)
    for i in range(5):  # 插入5个元素，应该会触发扩容
        hm.put(f"key_{i}", f"value_{i}")
        print(f"插入key_{i}后 - 大小:{hm.size}, 元素数:{hm.count}")
    
    print("\n扩容后的验证:")
    for i in range(5):
        value = hm.get(f"key_{i}")
        print(f"key_{i} -> {value}")
    
    print(f"\n最终状态 - 大小:{hm.size}, 元素数:{hm.count}")

def main():
    # 创建Hash Map实例
    hm = HashMap()
    
    # 测试put和get
    hm.put("张三", "138-0001")
    hm.put("李四", "138-0002") 
    hm.put("王五", "138-0003")
    
    print("张三的电话:", hm.get("张三"))  # 应该输出: 138-0001
    print("李四的电话:", hm.get("李四"))  # 应该输出: 138-0002
    
    # 测试更新
    hm.put("张三", "138-9999")
    print("张三更新后的电话:", hm.get("张三"))  # 应该输出: 138-9999
    
    # 测试删除
    hm.remove("李四")
    print("删除李四后:", hm.get("李四"))  # 应该输出: None
    
    print("当前元素总数:", hm.count)  # 应该输出: 2

    # 测试resiz
    test_resize()


if __name__ == "__main__":
    main()