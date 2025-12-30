// ==============================================================================
// Hash Map / Table - 对应 LLM Inference 中的 Page Table (Block Manager)
// ==============================================================================
//
// 【对应引擎模块】
//   - Page Table: PagedAttention 的核心映射表
//   - Block Manager: logical block id -> physical block id 的映射
//
// 【学习重点】
//   1. PagedAttention 核心映射：逻辑块到物理显存块的O(1)查找
//   2. 碰撞处理：链地址法（Separate Chaining）处理哈希冲突
//   3. 哈希函数设计：如何将 block id 均匀分布到桶中
//
// 【实现要求】
//   - 实现 put, get, remove 等操作，要求平均 O(1) 时间复杂度
//   - 使用链地址法处理碰撞（vector<list<pair<K,V>>>）
//   - 实现简单的哈希函数（可使用 std::hash）
//   - 可选：实现动态扩容（load factor > 0.75 时 rehash）
//
// 【练习目标】
//   - 深入理解 vLLM PagedAttention 的显存管理机制
//   - 掌握哈希表在高频查询场景下的性能优化
//   - 模拟 logical KV block 到 physical GPU memory block 的映射
// ==============================================================================

#include <cstddef>
#include <functional>
#include <list>
#include <utility>
#include <vector>
#include <iostream>

template<typename K, typename V>
class HashMap {
private:
    std::vector<std::list<std::pair<K, V>>> buckets; // 桶数组，每个桶是一个链表
    size_t size_; // 桶的数量
    size_t count_; // 元素总数

public:
    // 构造函数只需要在这里声明，在类外实现
    explicit HashMap(size_t initial_size);

    // 核心方法
    void put(const K& key, const V& value);
    V* get(const K& key);
    void remove(const K& key);

    // 辅助方法
    size_t getSize() const { return size_; }
    size_t getCount() const { return count_; }

private:
    // 辅助方法
    size_t hash_function(const K& key) const;
    void resize();
};

// Task-01 在类外实现构造函数
template<typename K, typename V>
HashMap<K, V>::HashMap(size_t initial_size)
    : buckets(initial_size), size_(initial_size), count_(0) {}

// Task-02 实现hash函数
template<typename K, typename V>
size_t HashMap<K, V>::hash_function(const K& key) const {
    // 使用标准库的hash函数
    std::hash<K> hasher;
    size_t hash_value = hasher(key);

    return hash_value % size_;
}

// Task-03 实现 put 方法
template<typename K, typename V>
void HashMap<K, V>::put(const K& key, const V& value){
    // 1. 计算哈希值，确定桶位置
    size_t bucket_index = hash_function(key);

    // 2. 获取对应的桶（链表）
    // std::list<std::pair<K, V>>& bucket = buckets[bucket_index];  // 等价下面的，简洁带来的坏处是没ide提示了auto 是编译时编译器算出来的
    auto& bucket = buckets[bucket_index]; 

    // 3. 遍历链表查找是否已存在相同key
    for (auto& pair : bucket){
        if (pair.first == key){
            // 更新现有值
            pair.second = value;
            return;
        }
    }

    // 4. 如果没找到，添加到链表末尾
    bucket.emplace_back(key, value);
    count_ ++;

    // 5. 检查是否需要扩容
    if (static_cast<double>(count_) / size_ > 0.75){
        resize();
    }
}

// Task-04 实现 get 方法
/**
在 C++ 中，我们不能像python一样返回None 而是用指针：
- 找到值：返回指向值的指针
- 没找到：返回nullptr
*/
template<typename K, typename V>
V* HashMap<K, V>::get(const K& key){
    
    // 1. 计算哈希值，确定桶位置
    size_t bucket_index = hash_function(key);

    // 2. 遍历桶中的链表，查找匹配的key
    auto& bucket = buckets[bucket_index];
    for (auto& pair : bucket){
        if (pair.first == key){
            // 返回值的指针
            return &pair.second;
        }
    }

    // 3.如果没找到，返回nullptr
    return nullptr;
}

// Task-05 实现remove方法
template<typename K, typename V>
void HashMap<K, V>::remove(const K& key){
    // 1. 计算哈希值，确定桶位置
    size_t bucket_index = hash_function(key);

    // 2. 遍历桶中的链表，查找匹配的key
    auto& bucket = buckets[bucket_index];

    for (auto it = bucket.begin(); it != bucket.end(); ++it){
        if (it -> first == key ) {
            // 删除元素并更新计数
            bucket.erase(it);
            count_ --;
            return;
        }
    } 

    // 4.如果没找到，静默返回
}


// Task-06 实现resize方法
template<typename K, typename V>
void HashMap<K, V>:: resize(){
    // 1. 计算新大小
    size_t new_size = size_ * 2;

    // 2. 创建新的更大的桶数组
    std::vector<std::list<std::pair<K, V>>> new_buckets(new_size);

    // 3. 重新哈希所有现在元素到新桶中
    std::hash<K> hasher;

    for (auto& bucket : buckets){ // 遍历所有旧桶
        for (auto& pair : bucket){ // 遍历每个桶中所有pair
            
            size_t hash_value = hasher(pair.first);
            size_t new_index = hash_value % new_size;

            new_buckets[new_index].push_back(std::move(pair));

        }
    }

    // 4. 更新实例属性
    buckets = std::move(new_buckets);
    size_ = new_size;
}

int main() {
    // 创建HashMap实例，从小size开始便于测试扩容
    HashMap<std::string, int> hm(4);
    
    std::cout << "初始状态 - 大小:" << hm.getSize() << ", 元素数:" << hm.getCount() << std::endl;
    
    // 测试put和扩容
    for (int i = 0; i < 10; ++i) {
        std::string key = "key_" + std::to_string(i);
        hm.put(key, 1000 + i);
        
        // 每插入一个就打印状态
        if (i % 3 == 0) {
            std::cout << "插入" << i + 1 << "个元素后 - 大小:" << hm.getSize() 
                     << ", 元素数:" << hm.getCount() << std::endl;
        }
    }
    
    std::cout << "\n测试查找:" << std::endl;
    for (int i = 0; i < 5; ++i) {
        std::string key = "key_" + std::to_string(i);
        int* value = hm.get(key);
        if (value) {
            std::cout << key << " -> " << *value << std::endl;
        }
    }
    
    // 测试更新
    hm.put("key_0", 9999);
    int* updated = hm.get("key_0");
    if (updated) {
        std::cout << "\n更新后 key_0 -> " << *updated << std::endl;
    }
    
    // 测试删除
    hm.remove("key_1");
    int* removed = hm.get("key_1");
    if (!removed) {
        std::cout << "key_1 已被删除" << std::endl;
    }
    
    std::cout << "\n最终状态 - 大小:" << hm.getSize() 
             << ", 元素数:" << hm.getCount() << std::endl;
    
    return 0;
}