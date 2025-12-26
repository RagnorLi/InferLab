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

    
    }

    // 任务9：实现 get() 方法
    // 返回 Option<&T>：安全的不可变引用
    pub fn get(&self, index: usize) -> Option<&T> {
        // 【你来实现】get 逻辑
        // 步骤 1: if index >= size { return None }
        // 步骤 2: unsafe { Some(&*data.add(index)) }

        // 你的代码在这里：
        // if ____________________ {
        //     ____________________;
        // }
        // unsafe {
        //     ____________________
        // }
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

        // 你的代码在这里：
        // ____________________!(____________________, ____________________);
        // unsafe {
        //     ____________________
        // }
    }
}

// 实现 IndexMut trait 让 Vector 支持 v[0] = value 语法
impl<T> std::ops::IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // 【你来实现】IndexMut trait
        // 类似 index() 但返回可变引用 &mut
        // assert!(index < size);
        // unsafe { &mut *data.add(index) }

        // 你的代码在这里：
        // ____________________!(____________________, ____________________);
        // unsafe {
        //     ____________________
        // }
    }
}

impl<T> Vector<T> {
    // 任务11：实现 resize() 方法 - 核心扩容逻辑
    fn resize(&mut self) {
        // 【你来实现】resize 扩容逻辑 - 最复杂的部分
        // 步骤 1: 计算新容量 (capacity == 0 ? 1 : capacity * 2)
        // 步骤 2: 分配新内存 new_data
        // 步骤 3: 复制旧数据 ptr::copy_nonoverlapping
        // 步骤 4: 释放旧内存 dealloc
        // 步骤 5: 更新 data 和 capacity

        // 你的代码在这里：
        // let new_capacity = ____________________;
        //
        // let new_data = ____________________;
        //
        // // 复制数据
        // if ____________________ {
        //     unsafe {
        //         ____________________;
        //     }
        // }
        //
        // // 释放旧内存
        // if ____________________ {
        //     unsafe {
        //         ____________________;
        //         ____________________;
        //     }
        // }
        //
        // self.data = ____________________;
        // self.capacity = ____________________;
    }

    // 任务12：实现 stride() 方法 - Tensor 风格访问
    pub fn stride(&self, start_index: usize, stride: isize) -> Vec<T>
    where
        T: Clone, // 需要 Clone trait 来复制元素
    {
        // 【你来实现】stride 步长访问
        // 类似 C++ 版本，但返回 Vec<T>
        // 步骤 1: 检查 start_index 和 stride 参数
        // 步骤 2: 创建 result 向量
        // 步骤 3: 根据 stride 正负实现不同的遍历逻辑
        // 步骤 4: 使用 get() 获取元素并 push 到 result

        // 你的代码在这里：
        // if ____________________ {
        //     return ____________________;
        // }
        // if ____________________ {
        //     return ____________________;
        // }
        //
        // let mut result = ____________________;
        // let mut current_index = ____________________;
        //
        // if ____________________ > 0 {
        //     while ____________________ {
        //         if let Some(value) = ____________________ {
        //             ____________________;
        //         }
        //         ____________________ += ____________________;
        //     }
        // } else {
        //     while ____________________ >= 0 {
        //         if let Some(value) = ____________________ {
        //             ____________________;
        //         }
        //         if ____________________ {
        //             break;
        //         }
        //         ____________________ += ____________________;
        //     }
        // }
        //
        // result
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

    // 【第二步：测试 push 和扩容】
    // 任务：添加元素并观察扩容
    // 提示：for i in 1..=8 { v.push(i); }
    // println!("[Step 2] 扩容后 size={}, capacity={}", v.size(), v.capacity());

    // 【第三步：测试索引访问】
    // 任务：使用 [] 语法访问元素
    // 提示：println!("[Step 3] 第一个元素: {}", v[0]);

    // 【第四步：测试 stride】
    // 任务：实现步长访问
    // 提示：let result = v.stride(1, 2);
    // println!("[Step 4] stride 结果: {:?}", result);

    // 【第五步：测试 pop】
    // 任务：弹出元素并观察缩容
    // 提示：while let Some(val) = v.pop() { print!("{} ", val); }

    println!("\n恭喜！你完成了 Rust Vector 的实现！");
    println!("现在可以运行: rustc vector.rs && ./vector");
}