use std::collections::hash_map::VacantEntry;
/**
 * 数组 (Array) - Rust 实现
 * 
 * ## 需求说明
 * 
 * 实现一个数组数据结构，要求：
 * 
 * ### 核心特性
 * - 连续内存存储，支持随机访问
 * - 通过索引访问元素，时间复杂度 O(1)
 * - 支持动态扩容
 * 
 * ### 必须实现的操作
 * 1. `new(capacity)` - 创建新数组，指定容量
 * 2. `get(index)` - 通过索引获取元素 O(1)，返回 Option<&T>
 * 3. `get_mut(index)` - 通过索引获取可变引用 O(1)，返回 Option<&mut T>
 * 4. `append(value)` - 在数组末尾追加元素 O(1) 平均
 * 5. `insert(index, value)` - 在指定位置插入元素 O(n)
 * 6. `remove(index)` - 删除指定位置的元素 O(n)，返回 Option<T>
 * 7. `len()` - 返回数组长度 O(1)
 * 8. `is_empty()` - 判断数组是否为空 O(1)
 * 
 * ### 可选优化
 * - 使用泛型支持任意类型
 * - 动态扩容策略（2倍扩容或1.5倍扩容）
 * - 缩容机制（当元素数量远小于容量时）
 * - 实现 Iterator trait
 * 
 * ### 时间复杂度要求
 * - 随机访问：O(1)
 * - 插入/删除：O(n)
 * - 追加：O(1) 平均（考虑扩容摊销）
 * 
 * 
 * 
 * 提供 Rust 版本的动态数组实现。Rust 版本的关键点：
*  1. 所有权系统：利用 Rust 的所有权，无需手动内存管理
*  2. 错误处理：使用 Result 和 Option，而非异常
*  3.Trait 实现：Index、IndexMut、Iterator 等
*  4.泛型支持：使用泛型
*  5.底层存储：使用 Vec<T>（惯用法）或原始指针（更接近 C++）
*/

#[allow(unused_imports)]
use std::fmt;
#[allow(unused_imports)]
use std::ops::{Index, IndexMut};

/* 
 动态数组实现

 核心思想：
 1. 使用 Vec<T> 作为底层存储（连续内存，自动管理）
 2. 维护容量（capacity）和实际长度（size）
 3. 当容量不足时，按 2 倍扩容 （摊销O（1））
 4. 当元素过少时，按 1/2 缩容 (可选优化)

 Rust 的所有权系统自动处理内存管理，无需手动释放
 所有权的本质是：编译器通过类型系统强制执行的线性类型规则，确保每个值有且仅有一条访问路径，必须被消费一次且仅一次。
*/
pub struct Array<T>{
    data: Vec<T>, // 底层存储，Vec自动管理内存
    size: usize, // 当前元素数量
    capacity: usize, // 当前容量
}

// In Rust, `impl<T> Array<T>` means we are implementing methods for Array of any type T.
// T 作为泛型参数，必须在 impl 块引入。否则编译器会报错，提示 T 未知类型参数。
impl<T> Array<T>{
    /* 
     创建新数组，指定初始容量

     # Arguments

     * `capcacity` - 初始容量，必须大于0

     # Panics

     如果 capacity 为 0，就会 panic（即程序遇到严重错误立即中止，并打印错误信息到标准错误输出。通常用于不可恢复的错误，由 Rust 的 panic! 宏触发）。
    */
    pub fn new(capacity: usize) -> Self{
        if capacity == 0 {
            panic!("Capacity must be positive");
        }

        Self { 
            data: Vec::with_capacity(capacity), 
            size: 0,
            capacity,
        }
    }
    

    /* 
     通过索引获取元素的不可变引用

     # Arguments
     * `index` -- 索引位置

     # Returns

     如果索引有效，返回`Some(&T)`, 否则返回`None`

     # 时间复杂度 O(1)
    */
    pub fn get(&self, index: usize) -> Option<&T>{
        if index < self.size{
            Some(&self.data[index])
        }else {
            None
        }
    }


    /* 
     通过索引获取元素的可变引用
     
     # Arguments
     
     * `index` - 索引位置
     
     # Returns
     
     如果索引有效, 返回 `Some(&mut T)`, 否则返回 `None`
     在 Rust 中，`&mut self` 表示方法以*可变借用*方式访问对象，可以修改自身状态。
     这也是语法固定写法：
       - `&self`：不可变借用（可有多个，不可修改）
       - `&mut self`：可变借用（只能有一个，可修改）
     
     # 时间复杂度： O(1)
     
    */
    // 解释：在 Rust 中，“加分号”和“不加分号”不是随便选的。 哈哈哈有意思
    // 分号意味着“执行这个表达式，丢弃计算结果”，相当于把它作为语句；
    // 不加分号，则是“把这个表达式的值作为当前块的返回值”。
    // 比如在 if-else 结构或者函数块的最后，如果你希望返回 Some(...)，就不要加分号，否则就只是单纯执行，没有返回值，结果是 ()。
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.size {
            Some(&mut self.data[index]) // 这里不要加分号，返回 Some(~)
        } else {
            None // 这里同理，不加分号，返回 None
        }
    }


    /*
    在数组末尾追加元素，当容量不足时，先扩容再插入，摊销时间复杂度为O(1)

    # Arguments

    * `value` - 要追加的值（会被移动进数组）

    # 时间复杂度 O(1) 平均（摊销分析）

    # Rust 的参数类型注解语法——“参数名: 类型” 跟其他语言不同
    # &mut self 习惯写法，等价于self: &mut Self
    */ 
    pub fn append(&mut self, value:T){
        // 如果容量不足，先扩容
        if self.size >= self.capacity{
            self.resize(self.capacity * 2);
        }

        // 如果Vec的容量不足，需要手动push(因为我们要维护自己的size)
        if self.data.len() <= self.size{
            self.data.push(value);
        }else{
            // 如果 Vec有足够容量，直接赋值
            self.data[self.size] = value;
        }

        self.size += 1;
    }

    /*
    删除指定位置的元素1

    需要将 index 之后的元素都向前移动一位

    # Arguments

    * `index` - 要删除的位置

    # Returns

    如果索引有效，返回 `Some(T)` (被删除的元素)，否则返回 `None`

    # 时间复杂度： O(n)    
    */
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let removed = self.data.remove(index); // 不是你以为的掩耳盗铃而是底层用Vec实现Array 这是一种抽象
        self.size -= 1;

        // 可选：如果元素数量远小于容量，进行缩容
        // 避免频繁扩容缩容，使用 1/4 作为阈值
        if self.size > 0 && self.size < self.capacity / 4 {
            self.resize(self.capacity / 2);
        }

        Some(removed)
    }

    /**
     * 返回数组长度
     * 
     * 时间复杂度：O(1)
     * 
    */
    pub fn len(&self) -> usize{
        self.size
    }

    /**
     * 判断数组是否为空
     * 时间复杂度 O(1)
    */
    pub fn is_empty(&self) -> bool{
        self.size == 0;
    }

    /**
     * 判断数组是否为空
     * 
     * 时间复杂度 O(1)
    */
    pub fn capacity(&self) -> usize{
        self.capacity
    }

    /**
     * 内部方法：调整数组容量
     * 
     * # Arguments
     * 
     * * `new_capacity` - 新的容量
     * 
     * 时间复杂度：O(n) 但摊销到多次操作后，append 平均为O(1)
     * 
    */
    fn resize(&mut self, new_capacity: usize){
        if new_capacity < self.size{
            panic!("New capacity {} is too small for {} elements", new_capacity, self.size);
        }

        // 创建新的 Vec,指定容量
        let mut new_data = Vec::with_capacity(new_capacity);

        // 移动现有元素 （Rust 的移动是零成本的）
        // 【翻译】drain(..) 让你按指定范围（这里是全范围 ..）取出并移走每个元素，原 vec 清空。
        // 【比喻】就像一排信箱，drain(..) 相当于顺序把每个信拿出来，拿走后信箱变空；.. 表示“起点到终点全部”。
        new_data.extend(self.data.drain(..));

        // 确保新 Vec 的长度等于size （因为extend会设置长度）
        // 但我们需要保持 size 不变，值是改变容量
        // 实际上，Vec的容量和长度是分开的，我们只需要确保容量足够
        self.data = new_data;
        self.capacity = new_capacity;
    }
}


/*
【本质】Trait 是编译期约束类型行为的接口系统——规定类型必须实现哪些方法、操作或属性。

【翻译】
- Trait 就是“要求你会做这些事情”的接口清单，静态检查谁能干哪些操作。
- 实现 Trait = 承诺“我能完成这些行为”，编译器生成多态代码。

【比喻】
- 就像“厨师”资格证：要求你必须会炒菜、做饭。“拥有厨师证的对象”可以被当成“会做饭的人”用；trait object 允许多类型统一调用约定好的一组功能。
- 特征关键点：静态（编译时）检查，多个类型可实现同一 trait，并据此抽象/约束函数参数泛型。

【证明】
```rust
trait Cook {
    fn cook(&self);
}
struct ChineseChef;
impl Cook for ChineseChef {
    fn cook(&self) { println!("Chinese food!"); }
}
fn hire<C: Cook>(chef: C) {
    chef.cook();
}
```
- 这里 trait `Cook` 规定了“必须实现 cook 方法”，ChineseChef 的 hire/hire 动态分发/多态都由 trait 驱动。

*/

/// 实现Index trait,支持


fn main(){
    // 基础功能测试
    let mut arr = Array::new(3);
    // println! 是 Rust 的打印宏，用于输出到控制台；
    // "!" 表示它是宏（不是普通函数）；
    // "{}" 是占位符，用于插入变量的值；
    // ":?" 表示采用 Debug 格式输出，适用于打印像 Array 这种自定义结构体。
    println!("初始状态: {:?}", arr);
}




