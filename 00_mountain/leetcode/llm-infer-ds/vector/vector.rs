// ==============================================================================
// Vector / Dynamic Array - 对应 LLM Inference 中的 KV Cache, Tensor
// ==============================================================================
//
// 【对应引擎模块】
//   - KV Cache: 存储每个 token 的 key/value 向量
//   - Tensor: 多维数组的底层存储
//
// 【学习重点】
//   1. 连续内存布局：所有元素在内存中连续存储，cache-friendly
//   2. 扩容机制：当容量不足时如何进行 2x 扩容（alloc -> realloc -> dealloc）
//   3. Stride (步长) 访问：模拟 Tensor 的多维访问模式
//
// 【实现要求】
//   - 实现 push, pop, get, resize 等核心操作
//   - 使用 std::alloc 手动管理内存（练习底层操作）
//   - 支持 stride 访问：给定起始位置和步长，返回指定元素
//   - 正确处理 Drop trait，避免内存泄漏
//
// 【练习目标】
//   - 理解 Rust 的所有权和生命周期在底层内存管理中的应用
//   - 掌握 unsafe 代码的正确使用方式
//   - 模拟 vLLM 中 KV Cache 的内存管理


// 【Phase 1: 思维模型建立】
// Rust Vector ≠ C++ Vector 简单翻译
//
// 核心区别：
//   1. 所有权系统：内存有明确的"主人"，编译器强制检查
//   2. 生命周期：变量的生存期被严格追踪
//   3. unsafe 哲学：默认安全，只有明确标记 unsafe 才允许危险操作
//   4. RAII：资源获取即初始化，离开作用域自动清理
//
// 为什么我们要手动管理内存？
//   - 学习底层原理：理解指针、分配、释放
//   - 性能控制：避免 Vec<T> 的额外开销
//   - 教育目的：掌握 unsafe 的正确使用
// ==============================================================================


// 任务1：定义 Vector 结构体
// 语法桥接：
// - struct Vector<T> 类似 C++ 的 template<typename T> class Vector
// - *mut T 是"可变裸指针"，类似 C++ 的 T*
// - usize 是"size type"，类似 C++ 的 size_t
struct Vector<T> {
    data: *mut T,       // 裸指针：直接指向内存地址，不受Rust所有权系统管理
    size: usize,        // 当前有多少元素
    capacity: usize,    // 总共能放多少元素
}

// ==============================================================================
// 【思维模型：Rust 的内存管理哲学】
// ==============================================================================
                                                                                                                                                                                                                                                                                                                       
// ==============================================================================

// 任务2：实现 Drop trait - 自动内存清理
// 语法桥接：
// - impl<T> Drop for Vector<T> 意思是"为 Vector<T> 实现 Drop trait"
// - drop() 在对象销毁时自动调用
// - unsafe 因为我们要手动释放内存
impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        // 【你来实现】在这里释放内存
        // 提示：如果 data 不为空，需要调用 dealloc
        // unsafe { ... } 包围危险操作
        // 1. 检查 self.data 是否为空
        // 2. 如果不为空，创建 layout
        // 3. 调用 dealloc 释放内存

        if !self.data.is_null(){
            unsafe {
                let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.data as *mut u8, layout);
            }
        }
    }
}

// ==============================================================================
// 【Phase 2: 逻辑蓝图 - 用中文描述数据流】
// ==============================================================================
//
// 核心操作流程：
// 1. new() 构造函数：
//    步骤 1: 分配初始内存（alloc）
//    步骤 2: 初始化指针、size、capacity
//
// 2. push() 添加元素：
//    步骤 1: 检查是否需要扩容 (size >= capacity)
//    步骤 2: 如需要扩容，调用 resize()
//    步骤 3: 在 data[size] 写入值
//    步骤 4: size += 1
//
// 3. pop() 删除元素：
//    步骤 1: 检查是否为空 (size == 0)
//    步骤 2: 如为空，返回错误
//    步骤 3: size -= 1
//    步骤 4: 返回 data[size] 的值
//
// 4. resize() 扩容：
//    步骤 1: 计算新容量 (capacity * 2 或 1)
//    步骤 2: 分配新内存 (alloc)
//    步骤 3: 复制旧数据到新内存 (ptr::copy)
//    步骤 4: 释放旧内存 (dealloc)
//    步骤 5: 更新指针和容量
//
// 5. get() 获取元素：
//    步骤 1: 检查索引有效性
//    步骤 2: 返回 data[index] 的引用
//
// 6. stride() 步长访问：
//    步骤 1: 从 start_index 开始
//    步骤 2: 按 stride 步长收集元素
//    步骤 3: 返回收集到的元素向量
// ==============================================================================

impl<T> Vector<T> {
    // 任务3：实现构造函数 new()
    // 语法桥接：
    // - pub fn new() -> Self 意思是"公开函数，返回自己的类型"
    // - Self 是 Vector<T> 的别名
    // - -> Self 是返回类型注解
    pub fn new() -> Self{
        Self::with_capacity(4)
    }

    // 任务4：实现带初始容量的构造函数 with_capacity()
    pub fn with_capacity(initial_capacity: usize) -> Self {
        // 【你来实现】带初始容量的构造函数
        // 语法桥接：
        // - let capacity = 确定容量（0则保持0，否则用参数）
        // - let data = 如果容量>0，分配内存，否则 null_mut()
        // - unsafe { alloc(layout) } 分配内存
        // - Layout::array::<T>(capacity) 创建数组布局

        let capacity = if initial_capacity == 0 { 0 } else { initial_capacity };

        let data = if capacity > 0 {
            // 标准写法: 先构造 Layout，再 unsafe 分配内存，再转换为 T 指针
            let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
            unsafe {
                std::alloc::alloc(layout)
            }
        }else {
            std::ptr::null_mut()
        };

        // 可以不写成 capacity: capacity，只写 capacity 是因为结构体字段和变量同名时的 Rust 简写语法
        Self { data: data as *mut T, size: 0, capacity:capacity }

    }

    // 任务5：实现 size() 方法
    pub fn size(&self) -> usize {
        self.size
    }

    // 任务6：实现 capacity() 方法
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // 任务7：实现 push() 方法
    // 语法桥接：
    // - pub fn push(&mut self, value: T) 接受可变引用和值
    // - value: T 表示取得值的所有权（move语义）
    // - unsafe 因为要写内存
    pub fn push(&mut self, value: T) {
        // 【你来实现】push 逻辑
        // 按照 Python/C++ 蓝图：
        // 步骤 1: if size >= capacity { resize() }
        // 步骤 2: unsafe { ptr::write(data.add(size), value) }
       
        if self.size >= self.capacity {
            self.resize(self.capacity * 2); // 内部加分号：因为这是side effect操作
        } // 外部不加分号：又不是let result = if condition {};

        unsafe {
            std::ptr::write(self.data.add(self.size), value);
        }

        self.size += 1; 

    }

    // 任务8：实现 pop() 方法
    // 返回 Option<T>：有值时返回 Some(value)，空时返回 None
    pub fn pop(&mut self) -> Option<T> {
        // 【你来实现】pop 逻辑
        // 按照蓝图：
        // 步骤 1: if size == 0 { return None }
        // 步骤 2: size -= 1
        // 步骤 3: let value = unsafe { ptr::read(data.add(size)) }
        // 步骤 4: Some(value)
        // 步骤 5（opt）: Shrinking

        if self.size == 0 {
            return None;
        }

        self.size -= 1;

        let value = unsafe {
            std::ptr::read(self.data.add(self.size))
        };
            
        if self.size > 0 && self.size < self.capacity / 4 {
           self.resize(self.capacity / 2);
        }

        Some(value)
    
    }

    // 任务9：实现 get() 方法
    // 返回 Option<&T>：安全的不可变引用
    pub fn get(&self, index: usize) -> Option<&T> {
        // 【你来实现】get 逻辑
        // 步骤 1: if index >= size { return None }
        // 步骤 2: unsafe { Some(&*data.add(index)) }

        if index >= self.size {
            return None;
        }

        unsafe {

             /* 
                self.data              // 类型：*mut T（可变裸指针）
                   .add(index)         // 类型：*mut T（指针偏移，指向第 index 个元素）
                   *                   // 类型：T（解引用，得到值）
                   &                   // 类型：&T（取引用，得到不可变引用）
                Some(...)              // 类型：Option<&T>
            */
            Some(&*self.data.add(index))
        }
    }

    // 任务10：实现 Index trait - 支持 v[0] 语法
    // 语法桥接：
    // - impl<T> std::ops::Index<usize> for Vector<T>
    // - type Output = T; 指定返回类型
    // - fn index(&self, index: usize) -> &Self::Output
}

// 实现 Index trait 让 Vector 支持 v[0] 语法
impl<T> std::ops::Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // 【你来实现】Index trait
        // 类似 get() 但遇到无效索引时 panic        
        // assert!(index < size);
        // unsafe { &*data.add(index) }

        assert!(index < self.size);

        unsafe {
            &*self.data.add(index)
        }
    }
}

// 实现 IndexMut trait 让 Vector 支持 v[0] = value 语法
impl<T> std::ops::IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // 【你来实现】IndexMut trait
        // 类似 index() 但返回可变引用 &mut
        // assert!(index < size);
        // unsafe { &mut *data.add(index) }

        assert!(index < self.size);

        unsafe {
            &mut * self.data.add(index)
        }
    }
}

impl<T> Vector<T> {
    // 任务11：实现 resize() 方法 - 核心扩容逻辑
    fn resize(&mut self, new_capacity: usize) {
        // 0. 这里的核心决策：我们使用 alloc 还是 realloc?
        // 通常 std::Vec 会用realloc来优化，但为了教学，我们修正你的 alloc/copy 逻辑。

        // 1.计算新的 Layout
        let new_layout = if new_capacity > 0 {
            Some(std::alloc::Layout::array::<T>(new_capacity).unwrap())
        } else {
            None
        };

        // 2. 分配新内存
        let new_data = if let Some(layout) = new_layout {
            unsafe {
                let ptr = std::alloc::alloc(layout) as *mut T;

                // 检查内存分配是否成功
                if ptr.is_null(){
                    std::alloc::handle_alloc_error(layout);
                }

                ptr
            }
        } else {
            std::ptr::null_mut()
        };

        // 3. 决定要拷贝多少数据
        // 如果你在缩容，你不能拷贝 self.size 为了自身健壮性你只能拷贝 min(self.size, new_capacity)
        let copy_count = if self.size < new_capacity { self.size } else {
            new_capacity
        };

        // 4. 复制旧数据
        if !self.data.is_null() && copy_count > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(self.data, new_data, copy_count);
            }
        }

        // 5. 释放旧内存
        if !self.data.is_null() {
            unsafe {
                // 注： dealloc 必须使用当初 alloc 时完全一致的 capacity
                let old_layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
                std::alloc::dealloc(self.data as *mut u8, old_layout);
            }
        }

        // 6. 更新结构体成员
        self.data = new_data;
        self.capacity = new_capacity;

        // 如果缩容了，size 也必须被截断！
        if self.size > new_capacity {
            self.size = new_capacity;
        }

        // 潜在问题： 如果 T 拥有堆内存 （比如 String）, 被截断丢弃的那些元素 （从 new_capacity 到 old_size） 需要被手动drop()掉，否则会内存泄漏！但这取决于 resize 设计定义上是 ”仅仅调整容量“ 还是 ”强制截断“。

    }

    // 任务12：实现 stride() 方法 - Tensor 风格访问
    pub fn stride(&self, start_index: usize, stride: isize) -> Vec<T>
    where
        T: Clone, // 必须有 Clone, 因为我们要把数据复制一份带走
    {

        // 1. 检查参数 
        // start_index 是 usize , 正整数 本身不可能 小于 0， 所以无需检查 < 0
        if start_index >= self.size {
            panic!("start index out of bounds!")
        }

        if stride == 0 {
            panic!("stride can not be zero!")
        } 

        // 2. 创建result 向量
        // 直接用标准库 Vec::new() , 如果想优化性能，可以用Vec::with_capacity(预估大小)
        let mut result = Vec::new();

        // 3. 遍历逻辑
        // usize 无符号 mix isize 有符号，那安全的做法就是把所有坐标都暂时当成isize 有符号来处理
        let mut current_pos = start_index as isize;
        let limit = self.size as isize;

        loop {
            // 检查当前位置是否越界
            //  如果是正步长不能超过limit 
            //  如果是负步长不能小于0
            if current_pos < 0 || current_pos >= limit {
                break;
            }

            // 4. 获取元素并push
            // 需要把isize转回usize才能去访问内存
            let idx = current_pos as usize;

            if let Some(val) = self.get(idx) {
                result.push(val.clone());
            }

            // 移动一步
            current_pos += stride;
        }
        
        return result;
        
    }
}

// ==============================================================================
// 【Phase 4: vLLM 连接 - 为什么这对推理引擎重要】
// ==============================================================================
//
// 1. KV Cache 内存管理：
//    - 每个请求的 KV 向量需要连续存储
//    - 请求完成时需要高效清理（Drop trait）
//    - 动态扩容模拟请求长度变化
//
// 2. 性能特征：
//    - 连续内存：SIMD 和缓存友好
//    - 2x 扩容：平衡内存使用和拷贝开销
//    - 手动管理：精确控制分配/释放时机
//
// 3. Rust 优势：
//    - 编译时内存安全保证
//    - 零成本抽象：运行时无额外开销
//    - RAII：异常安全，资源不会泄漏
// ==============================================================================

// ==============================================================================
// 【Phase 3: 语法教学 - 分步骤实现和测试】
// ==============================================================================

fn main() {
    println!("=== Rust Vector 教学测试 ===\n");

    // 【第一步：测试基本创建】
    // 任务：创建 Vector 实例并检查初始状态
    // 提示：Vector<i32> v = Vector::new();
    // println!("[Step 1] Vector 创建完成, size={}, capacity={}", v.size(), v.capacity());
    let mut v = Vector::<i32>::new();
    println!("[Step 1] Vector 创建完成, size={}, capacity={}", v.size(), v.capacity());

    // 【第二步：测试 push 和扩容】
    // 任务：添加元素并观察扩容
    // 提示：for i in 1..=8 { v.push(i); }
    // println!("[Step 2] 扩容后 size={}, capacity={}", v.size(), v.capacity());
    for i in 1..=8{
        v.push(i);
    }
    println!("[Step 2] 扩容后 size={}, capacity={}", v.size(), v.capacity());

    // 【第三步：测试索引访问】
    // 任务：使用 [] 语法访问元素
    // 提示：println!("[Step 3] 第一个元素: {}", v[0]);
    println!("[Step 3] 第三个元素: {}", v[2]);

    // 【第四步：测试 stride】
    // 任务：实现步长访问
    // 提示：let result = v.stride(1, 2);
    // println!("[Step 4] stride 结果: {:?}", result);
    let result = v.stride(v.size - 1, -2);
    println!("[Step 4] stride 结果: {:?}", result);

    // 【第五步：测试 pop】
    // 任务：弹出元素并观察缩容
    // 提示：while let Some(val) = v.pop() { print!("{} ", val); }
    while let Some(val) = v.pop() {
        print!("{}", val);
    }
    println!("运行: rustc vector.rs && ./vector");
}