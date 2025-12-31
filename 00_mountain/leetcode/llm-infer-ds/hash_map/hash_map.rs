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
//   - 使用 Vec<Vec<(K,V)>> 存储数据（链地址法）
//   - 使用 std::collections::hash_map::DefaultHasher 计算哈希值
//   - 正确处理泛型约束：K 需要实现 Hash + Eq
//
// 【练习目标】
//   - 理解 Rust 的 trait 系统在泛型数据结构中的应用
//   - 掌握哈希表在高频查询场景下的性能优化
//   - 模拟 logical KV block 到 physical GPU memory block 的映射
// ==============================================================================

use std::mem;
use std::hash::{Hash, Hasher}; // 引入traits
use std::collections::hash_map::DefaultHasher;

#[derive(Debug)]
pub struct HashMap<K, V>
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    buckets: Vec<Vec<(K, V)>>, // 桶数组，每个桶是vector
    size: usize, // 桶的数量
    count: usize, // 元素总数
}


// 为了与c++保持设计一致，这里直接实现就行但是我们定义trait
pub trait HashMapTrait<K, V> {
    
    // 构造
    fn new(initial_size: usize) -> Self;
    
    // 核心
    fn put(& mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> bool;

    // 辅助
    fn hash_function(&self, key: &K) -> usize;
    fn resize(&mut self, new_size: usize);
}

impl<K, V> HashMapTrait<K, V> for HashMap<K, V>
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn new(initial_size: usize) -> Self {
        Self { 
            buckets: vec![Vec::new(); initial_size], 
            size: initial_size, 
            count: 0,
         }
    }

    fn hash_function(&self, key: &K) -> usize {
        // 1. 创建DefaultHasher实例
        let mut hasher = DefaultHasher::new();

        // 2. 将key哈希到hasher中
        key.hash(&mut hasher);

        // 3. 获取最终哈希值并取模
        (hasher.finish() as usize) % self.size
    }


    fn resize(&mut self, new_size: usize) {
        // 1. 确保新大小合理
        let new_size = new_size.max(4); 
        if new_size == self.size {
            return;
        }
    
        // 2. 准备新的空桶数组
        let new_buckets = vec![Vec::new(); new_size];
    
        // 3. 【关键一步】偷梁换柱！
        // 使用 mem::replace 将 self.buckets 替换成新的空数组。
        // 此时：
        //   - self.buckets 变成了全新的空桶
        //   - old_buckets 拿到了所有旧数据的所有权（是 Move，不是 Clone）
        let old_buckets = mem::replace(&mut self.buckets, new_buckets);
    
        // 4. 更新 size (这样后续计算 hash 若依赖 self.size 会是正确的，虽然后面我们手动算了)
        self.size = new_size;
    
        // 5. 遍历旧桶，把数据搬家
        for bucket in old_buckets {
            for (key, value) in bucket {
                // 注意：因为我们拥有 old_buckets，这里拿到的 key/value 是真身（Move）
                
                // 重新计算 Hash
                // 必须每次创建新的 Hasher，保证状态纯净
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                let hash_value = hasher.finish();
                
                // 计算新位置
                let new_index = (hash_value as usize) % new_size;
    
                // 移动插入到新家 (self.buckets 现在已经是新的了)
                self.buckets[new_index].push((key, value));
            }
        }
    }

    fn put(&mut self, key: K, value: V) {
        /*
         * 关键的Rust特性：
         *   1.所有权获取：key: K, value: V - 获取所有权
         *   2.引用借用：&key - 借用key计算哈希
         *   3.可变借用：&mut self.buckets[bucket_index] - 可变访问桶
         *   4.元组访问：pair.0 (key), pair.1 (value)
         *   5.类型转换：(self.count as f64) - 计算负载因子
        */


        // 1. 计算哈希值
        let bucket_index = self.hash_function(&key);

        // 2. 获取桶的可变引用
        let bucket = &mut self.buckets[bucket_index];

        // 3. 遍历桶内的链表元素，查找是否已存在key
        for pair in bucket.iter_mut(){
            // 显式加 &，表明我们在比较引用，逻辑更严谨
            if &pair.0 == &key{ 
                // 更新值
                pair.1 = value;
                return;
            }
        }

        // 4. 添加新元素
        bucket.push((key, value));
        self.count += 1;

        // 5.扩容检查
        if (self.count as f64) / (self.size as f64) > 0.75 {
            self.resize(self.size * 2);
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        /*
        get方法接收 key: &K（借用），返回 Option<&V>（可选引用）。
        */

        // 1. 计算哈希值
        let bucket_index = self.hash_function(key);

        // 2.获取桶的不可变引用
        let bucket = &self.buckets[bucket_index];

        // 3. 遍历查找
        for pair in bucket.iter(){
            if &pair.0 == key { // 注意这里用 &pair.0 == key
                return Some(&pair.1);
            }
        }

        // 4.没找到
        None
    }

    fn remove(&mut self, key: &K) -> bool {
        // 1. 计算哈希值
        let bucket_index = self.hash_function(key);

        // 2.获取桶的可变引用
        let bucket = &mut self.buckets[bucket_index]; // 这里不加mut 下面没办法remove

        // 3. 遍历查找要删除的元素
        if let Some(pos) = bucket.iter().position(|pair| &pair.0 == key) {
            // 删除元素
            bucket.remove(pos);
            self.count -= 1;

            // ==========================================
            // 新增：缩容逻辑 (Shrinking Logic)
            // ==========================================
            // 触发条件：
            // 1. 当前元素密度低于 25% (0.25)
            // 2. 当前桶大小大于最小限制 (比如 4)，防止缩没了
            if self.size > 4 && (self.count as f64 / self.size as f64) < 0.25 {
                // 缩容为当前的一半
                let new_size = self.size / 2;
                self.resize(new_size);
            }

            return true;
        }

        // 4. 没找到
        false
    }



    
}    

fn main() {
    println!("这是一个库文件，请使用 `cargo test` 或 `rustc --test` 运行测试。");
}

#[cfg(test)]
mod tests {
    use super::*; // 引入外面的 HashMap 和 HashMapTrait

    #[test]
    fn test_basic_operations() {
        // 初始化一个容量为 4 的哈希表
        let mut map: HashMap<i32, String> = HashMap::new(4);

        // 1. 测试 Put 和 Get
        map.put(1, "Value1".to_string());
        map.put(2, "Value2".to_string());

        assert_eq!(map.get(&1), Some(&"Value1".to_string()));
        assert_eq!(map.get(&2), Some(&"Value2".to_string()));
        assert_eq!(map.get(&3), None); // 不存在的 key

        // 2. 测试覆盖 (Update)
        map.put(1, "Value1_Updated".to_string());
        assert_eq!(map.get(&1), Some(&"Value1_Updated".to_string()));

        // 3. 测试 Count
        assert_eq!(map.count, 2);
    }

    #[test]
    fn test_remove() {
        let mut map = HashMap::new(8);
        map.put(10, 100);
        map.put(20, 200);

        // 删除存在的元素
        let removed = map.remove(&10);
        assert!(removed); // 应该返回 true
        assert_eq!(map.get(&10), None); // 应该查不到了
        assert_eq!(map.count, 1); // 数量减少

        // 删除不存在的元素
        let removed_again = map.remove(&10);
        assert!(!removed_again); // 应该返回 false
        assert_eq!(map.count, 1); // 数量不变
    }

    #[test]
    fn test_resize() {
        // 初始大小为 4，阈值是 4 * 0.75 = 3
        let mut map = HashMap::new(4);
        assert_eq!(map.size, 4);

        // 插入 3 个元素，未触发扩容
        map.put(1, 1);
        map.put(2, 2);
        map.put(3, 3);
        assert_eq!(map.size, 4);

        // 插入第 4 个元素，(4/4 = 1.0 > 0.75)，触发扩容
        map.put(4, 4);
        
        // 验证是否扩容 (通常是翻倍，变成 8)
        assert!(map.size > 4); 
        assert_eq!(map.size, 8); 

        // 验证扩容后旧数据是否还在 (Rehash 是否正确)
        assert_eq!(map.get(&1), Some(&1));
        assert_eq!(map.get(&2), Some(&2));
        assert_eq!(map.get(&3), Some(&3));
        assert_eq!(map.get(&4), Some(&4));
    }

    #[test]
    fn test_string_keys() {
        // 测试非数字类型的 Key (验证泛型 K: Hash + Eq)
        let mut map = HashMap::new(4);
        
        map.put("apple".to_string(), 10);
        map.put("banana".to_string(), 20);

        assert_eq!(map.get(&"apple".to_string()), Some(&10));
        assert_eq!(map.remove(&"banana".to_string()), true);
        assert_eq!(map.get(&"banana".to_string()), None);
    }

    #[test]
    fn test_large_volume() {
        // 大量插入测试
        let mut map = HashMap::new(4);
        let count = 1000;

        for i in 0..count {
            map.put(i, i * 10);
        }

        assert_eq!(map.count, count);
        assert!(map.size >= count); // 桶的大小应该增长了

        // 验证所有数据都能找回
        for i in 0..count {
            assert_eq!(map.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    fn test_shrinking() {
        // 1. 初始小容量
        let mut map = HashMap::new(4);
        
        // 2. 疯狂插入，触发扩容
        // 插入 20 个元素。
        // 4 -> 8 (insert #4) -> 16 (insert #7) -> 32 (insert #13)
        for i in 0..20 {
            map.put(i, i);
        }
        
        println!("扩容后的 Size: {}", map.size);
        assert!(map.size >= 16); // 应该是 32
        assert_eq!(map.count, 20);

        // 3. 开始删除，触发缩容
        // 目前 size=32, count=20. 
        // 阈值是 0.25 * 32 = 8.
        // 我们删除直到剩 7 个元素 (7/32 < 0.25)，应该触发缩容变成 16
        for i in 0..13 {
            map.remove(&i);
        }
        
        println!("删除部分后的 Count: {}, Size: {}", map.count, map.size);
        assert_eq!(map.count, 7);
        // 此时应该触发了一次缩容 (32 -> 16)
        assert_eq!(map.size, 16);

        // 4. 继续删除
        // 目前 size=16, count=7.
        // 阈值是 0.25 * 16 = 4.
        // 删除直到剩 3 个 (3/16 < 0.25)，应该触发缩容变成 8
        for i in 13..17 {
            map.remove(&i);
        }

        println!("再次删除后的 Count: {}, Size: {}", map.count, map.size);
        assert_eq!(map.count, 3);
        assert_eq!(map.size, 8);
        
        // 5. 验证剩下的数据还能查到 (Rehash 没问题)
        assert_eq!(map.get(&19), Some(&19));
    }
}