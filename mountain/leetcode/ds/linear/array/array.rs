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

use std::fmt;
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
     
     如果索引有效, 返回 `Some(&mut T)`, 否则返回 `None`； mut 就是mutable （可变的）
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

    /**
     * 在指定位置插入元素
     * 
     * 需要将 index 之后的元素都向后移动一位
     * 
     * # Arguments
     * 
     * * `index` - 插入位置，必须在 [0, size] 范围内
     * * `value` - 要插入的值
     * 
     * # Returns
     * 
     * 如果索引有效，返回 `Ok(())`，否则返回 `Err(String)`
     * 
     * # 时间复杂度
     * 
     * O(n)
     */
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), String> {
        if index > self.size {
            return Err(format!("Index {} out of range [0, {}]", index, self.size));
        }
    
        // 如果容量不足，先扩容
        if self.size >= self.capacity {
            self.resize(self.capacity * 2);
        }
    
        // 确保 Vec 有足够的空间
        if self.data.len() <= self.size {
            self.data.push(value);
        } else {
            // 如果 Vec 有足够容量，先 push 一个占位符
            self.data.push(value);
        }
        
        // 手动移动元素：将 index 到 size-1 的元素向后移动一位
        // 从后往前移动，避免覆盖
        for i in (index..self.size).rev() {
            self.data.swap(i, i + 1);
        }
        
        self.size += 1;
        Ok(())
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
        self.size == 0
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
 Rust 中的 trait（特征）在语法和机制上接近 Java 的 interface（接口）、C++ 的抽象基类（Abstract Base Class），
 但本质是编译期的行为约束接口系统。

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

/// 实现Index trait,支持 arr[index] 语法，这里Index index是固定的
impl<T> Index<usize> for Array<T>{

    // 这里声明了关联类型 Output（特征关联类型机制）
    // 【本质】type 关键字在 trait impl 块中用于“指定此类型实现该 trait 时，某个类型成员的具体类型”。
    // 【翻译】告诉编译器：对于实现了 Index 的 Array<T>，arr[index] 表达式的结果类型必须是 T。
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size{
            panic!("Index {} out of range [0, {}]", index, self.size);
        }

        // 设计为只读，是因为实现 Index trait 时参数类型为 &self，
        // 表示只允许不可变借用（不会修改自身），所以只能返回元素的不可变引用。
        // 这样 arr[index] 语法天然只做读取，不允许通过它修改数组内容，符合 Index trait 的安全抽象设计。
        &self.data[index]
    }    
}

/// 实现IndexMut trait, 支持arr[index] = value 语法，这里IndexMut index_mut是固定的
impl<T> IndexMut<usize> for Array<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size{
            panic!("Index {} out of range [0, {}]", index, self.size);
        }
        &mut self.data[index]
    }

}

/// 实现 Display trait, 支持 println!("{}", arr); fmt -> format
/// 总结：
/// - impl<T: fmt::Display> ... —— 这是限定“元素 T 必须可打印”
/// - ... fmt::Display for Array<T> —— 这是实际给 Array<T> 实现 Display trait
///
/// 真正“约束”T的，是冒号前面的 `<T: fmt::Display>`，而 for 前面那个 fmt::Display 是指定在为谁实现该 trait

/**
 * Trait/impl 是编译期的能力系统，通过静态单态化实现零成本抽象，而非运行时的接口继承系统。

 * Trait = 技能证书，Impl = 颁发证书
    - 技能证书（trait）：声明能力（如“会开车”“会说英语”）
    - 颁发证书（impl）：证明某人/某类型拥有该能力
    - 能力组合：一个人可以同时有多个证书
    - 招聘要求（泛型约束）：要求应聘者必须有某证书才能应聘
    - 能力验证（单态化）：在编译期就确认每个应聘者都有所需证书，直接匹配岗位，无需运行时再查证
   对应关系：
    - 证书 = trait（能力声明）
    - 颁发 = impl（提供实现）
    - 多个证书 = 多 trait 实现（能力组合）
    - 招聘要求 = 泛型约束（T: Display）
    - 编译期验证 = 单态化（零成本）

    特性	      传统 OOP（接口继承）	Rust Trait
    实现方式	    运行时虚表查找	    编译期静态分发
    性能	        虚函数调用开销	    零成本（内联）
    组合能力	    单继承限制	        多 trait 组合
    为外部类型扩展	 无法扩展	         可以扩展
 * 
*/
impl<T: fmt::Display> fmt::Display for Array<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array([")?;
        for (i, item) in self.data.iter().take(self.size).enumerate(){
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }    
        write!(f, "], size={}, capavity={})", self.size, self.capacity)
    }
}

/// 实现Debug trait,支持 println!("{:?}", arr); 语法
impl<T: fmt::Debug> fmt::Debug for Array<T>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array {{ data: {:?}, size:{}, capacity: {} }}",
            // self.data[..self.size] 表示从 self.data 的第 0 个元素（含）到第 self.size 个元素（不含）获得一个切片视图。
            &self.data[..self.size], self.size, self.capacity)
    }
}

/// 实现迭代器（不可变引用）
/**
 * 
 * 【本质1】这是“生命周期标记 + 借用实现”的语法：`impl<'a, T> IntoIterator for &'a Array<T>` 意味着“对活在 `'a` 借用期内的 `&Array<T>` 实现可迭代”，确保迭代出的引用寿命不超过这次借用。

【翻译】
- `&'a Array<T>`：带名字 `'a` 的不可变借用，`'a` 表示“这次借用能活多久”。
- `IntoIterator for &'a Array<T>`：给这种带借用的引用实现迭代，返回的每个元素引用都绑定在同一个 `'a` 上。
- 编译器用 `'a` 静态检查：迭代器里的引用绝不会比原借用活得更久，防止悬垂。

【比喻】
借书证有效期 = `'a`。`&'a Array<T>` 是这张有有效期的借书证，迭代器借出的每一页复印件（元素引用）有效期都不能超出这张证的有效期。

【证明（简化版）】
```rust
impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;               // 元素引用活在 'a 内
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[..self.size].iter() // 迭代器借用同一个 'a
    }
}
// 若试图让迭代器或元素引用活得更久于原 &Array 借用，编译期直接报错。
```

【动机】显式生命周期让编译器在编译期证明“引用不悬垂”，零运行时开销。这是 Rust 的借用检查模型。
 * 
 * 
 * 【本质2】类型别名（type alias）：用一个新名字绑定已有类型，不造新类型，只是重命名。

【翻译】`type MyInt = i32;` 之后 `MyInt` 和 `i32` 在类型系统里完全等价，只是换个更有语义的名字，减少重复、提升可读性。

【比喻】给“身份证号”起一个标签名，比如“员工编号”，实质还是同一个号码，没有新证件，只是换个称呼。

【证明】
```rust
type MyInt = i32;

fn add(a: MyInt, b: i32) -> i32 {  // MyInt 和 i32 可互换
    a + b
}
```

【动机】减少冗长类型写法、提升语义清晰度（如为复杂泛型取短名），不引入运行时或类型系统新开销。
 * 
*/
impl<'a, T> IntoIterator for &'a Array<T>{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[..self.size].iter()
    }
}

/// 实现迭代器（可变引用）
impl<'a, T> IntoIterator for &'a mut Array<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[..self.size].iter_mut()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_basic_operations(){
        let mut arr = Array::new(3);
        assert_eq!(arr.len(), 0);
        assert!(arr.is_empty());

        // 测试append
        arr.append(1);
        arr.append(2);
        arr.append(3);

        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
        assert_eq!(arr[2], 3);

        // 测试扩容
        arr.append(4);
        assert_eq!(arr.len(), 4);
        assert!(arr.capacity() >= 4);

         // 测试随机访问
         arr[0] = 10;
         assert_eq!(arr[0], 10);
 
         // 测试 insert
         arr.insert(1, 99).unwrap();
         assert_eq!(arr[1], 99);
         assert_eq!(arr.len(), 5);
 
         // 测试 remove
         let removed = arr.remove(2).unwrap();
         assert_eq!(removed, 2);
         assert_eq!(arr.len(), 4);
    }    

    #[test]
    fn test_get_methods() {
        let mut arr = Array::new(5);
        arr.append(10);
        arr.append(20);

        assert_eq!(arr.get(0), Some(&10));
        assert_eq!(arr.get(1), Some(&20));
        assert_eq!(arr.get(2), None);

        if let Some(value) = arr.get_mut(0) {
            *value = 100;
        }
        assert_eq!(arr[0], 100);
    }

    #[test]
    fn test_iterator() {
        let mut arr = Array::new(5);
        arr.append(1);
        arr.append(2);
        arr.append(3);

        let mut sum = 0;
        for item in &arr {
            sum += *item;
        }
        assert_eq!(sum, 6);

        // 测试可变迭代器
        for item in &mut arr {
            *item *= 2;
        }
        assert_eq!(arr[0], 2);
        assert_eq!(arr[1], 4);
        assert_eq!(arr[2], 6);
    }

    #[test]
    fn test_string_array() {
        let mut arr = Array::new(2);
        arr.append("hello".to_string());
        arr.append("world".to_string());

        assert_eq!(arr[0], "hello");
        assert_eq!(arr[1], "world");
    }
}


fn main(){
    // 基础功能测试
    let mut arr = Array::new(3);
    println!("初始状态: {:?}", arr);

     // 测试 append
     arr.append(1);
     arr.append(2);
     arr.append(3);
     println!("追加 3 个元素: {}", arr);
 
     // 测试扩容
     arr.append(4);
     println!("触发扩容后: {}", arr);
 
     // 测试随机访问
     println!("arr[0] = {}", arr[0]);
     arr[0] = 10;
     println!("修改后 arr[0] = {}", arr[0]);
 
     // 测试 insert
     arr.insert(1, 99).unwrap();
     println!("在索引 1 插入 99: {}", arr);
 
     // 测试 remove
     if let Some(removed) = arr.remove(2) {
         println!("删除索引 2 的元素 {}: {}", removed, arr);
     }
 
     // 测试 get 方法
     match arr.get(0) {
         Some(value) => println!("arr.get(0) = {}", value),
         None => println!("索引越界"),
     }
 
     // 测试迭代器
     println!("迭代数组:");
     for item in &arr {
         print!("{} ", item);
     }
     println!();
 
     // 测试字符串数组
     let mut str_arr = Array::new(2);
     str_arr.append("hello".to_string());
     str_arr.append("world".to_string());
     println!("字符串数组: {}", str_arr);
}




