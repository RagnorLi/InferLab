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
//   2. 扩容机制：当容量不足时如何进行 2x 扩容（malloc -> memcpy -> free）
//   3. Stride (步长) 访问：模拟 Tensor 的多维访问模式
//
// 【实现要求】
//   - 实现 push_back, pop_back, operator[], resize 等核心操作
//   - 支持 stride 访问：给定起始位置和步长，返回指定元素
//   - 测试扩容时的性能和内存拷贝开销
//
// 【练习目标】
//   - 深入理解连续内存的优势（SIMD、预取）
//   - 掌握动态扩容的 amortized O(1) 复杂度分析
//   - 模拟 vLLM 中 KV Cache 的内存管理
// ==============================================================================

#include <cstddef>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

// ==============================================================================
// 【Phase 3: C++ 语法教学 - 模板泛型 Vector】
// 【填空练习：你来实现，我来指导】
// ==============================================================================

// 任务1：用 template 关键字定义泛型类
// 语法解释：
// - template <typename T> 告诉编译器这是一个模板，可以接受任何类型 T
// - 就像 Python 的 class Vector，但 T 可以是 int, float, 甚至自定义类型
// - 这就是 C++ 的"泛型"特性，让代码可以复用

template <typename T>
class Vector {
private:
    // 任务2：声明三个私有成员变量
    // 语法解释：
    // - T* data 指向连续内存的指针（相当于 Python 的 list）
    // - size_t size_ 当前元素数量（下划线避免和方法名冲突）
    // - size_t capacity_ 当前容量
    // 提示：size_t 是无符号整数，专门用于数组大小和索引
    T* data;
    size_t size_;  // 我是这个类的私有财产（成员变量），请不要把我和外面传进来的临时变量搞混了。也可以m_size(老派写法)编译器把__和_Size这种用了，防冲突
    size_t capacity_;
    

public:
    // ==============================================================================
    // 【构造函数 - 初始化内存】
    // ==============================================================================

    // 任务3：实现默认构造函数
    // 语法解释：
    // - Vector() : 这是构造函数，没有返回类型，名字和类名一样
    // - 初始化列表 : data(nullptr), size_(0), capacity_(0)
    //   * nullptr 相当于 Python 的 None，表示空指针
    //   * size_t 是无符号整数类型，专门用于大小/索引
    Vector() : data(nullptr), size_(0), capacity_(0) {}
    

    // 任务4：实现带初始容量的构造函数
    // 语法解释：
    // - explicit 防止隐式转换（比如 int -> Vector）
    // - new T[initial_capacity] 分配连续内存，就像 malloc 但有类型安全
    // 初始化列表：size_(0), capacity_(initial_capacity)
    explicit Vector(size_t initial_capacity) : data(nullptr), size_(0), capacity_(initial_capacity){
        
        if (initial_capacity > 0){
            data = new T[capacity_];
            // this->data = new T[this->capacity_];
        }

    }

    // ==============================================================================
    // 【析构函数 - 释放内存】
    // ==============================================================================

    // 任务5：实现析构函数
    // 语法解释：
    // - ~Vector() 是析构函数，对象销毁时自动调用
    // - delete[] data 释放数组内存（和 new[] 配对使用）
    // - 没有这步就会内存泄漏！

    // 在这里实现析构函数：
    ~Vector(){
        if (data){
            delete[] data; // 删除数组 data 跟上面的new T[] 配对的
        }
    }

    // ==============================================================================
    // 【核心方法 - 按照 Python 逻辑蓝图实现】
    // ==============================================================================

    // 任务6：实现 size() 方法
    // 语法解释：返回当前元素数量，const 表示不会修改对象
    size_t size() const { // 修饰this
        return this->size_;
    }


    // 任务7：实现 capacity() 方法
    // 语法解释：返回当前容量，const 表示不会修改对象
    size_t capacity() const {
        return this->capacity_;
    }

    // 任务8：实现 push_back
    // 按照 Python 逻辑蓝图：
    // 步骤 1: 检查是否需要扩容（size_ >= capacity_）
    // 步骤 2: 如果需要，计算新容量（capacity_ == 0 ? 1 : capacity_ * 2），调用 resize
    // 步骤 3: 把 value 放到 data[size_] 位置
    // 步骤 4: size_ 加 1
    void push_back(const T& value){ // push_back 用 const T&，因为你只是读取它，复制进你的 data 里。
        
        if (size_ == capacity_){
            capacity_ = (capacity_ == 0 ? 1 : capacity_ * 2);
            resize(capacity_);
        }

        data[size_] = value;

        size_ += 1;
    }


    // 任务9：实现 pop_back
    // 按照逻辑蓝图：
    // 步骤 1: 检查是否为空（size_ == 0）
    // 步骤 2: 如果为空，抛出异常 std::out_of_range("Vector is empty")
    // 步骤 3: size_ 减 1
    // 步骤 4: 返回 data[size_] 的值（注意：size_ 已经减 1 了）
    T pop_back(){
        if (size_ == 0){
            throw std::out_of_range("Vector is empty");
        }
        
        if (capacity_ > 0 && size_ < capacity_ / 4) {
            resize(capacity_ / 2);
        }

        size_ --;

        return data[size_];
    }



    // 任务10：实现 operator[] 重载（下标访问）
    // 语法解释：
    // - T& operator[](size_t index) 让 v[0] 可以工作
    // - 返回引用 & 意味着可以修改：v[0] = 5
    // - 需要检查索引是否有效

    // 在这里实现 operator[]：
    // T& operator[](size_t index) {
    //     if (_________ >= _________) {
    //         throw std::__________("Index out of range");
    //     }
    //     return _________[_________];
    // }
    T& operator[](size_t index){
        if  (!(0 <= index && index < size_)){
            throw std::out_of_range("Index out of range");
        }

        return data[index];
    }


    // 任务11：实现 const 版本的 operator[]
    // 语法解释：让 const 对象也能用下标访问
    // const T& operator[](size_t index) const { /* 类似上面的实现 */ }
    //(1) 返回值只读              (2) 函数内部只读
    //↓↓↓                              ↓↓↓
    const T& operator[](size_t index) const {
        if  (!(0 <= index && index < size_)){
            throw std::out_of_range("Index out of range");
        }

        return data[index];
    }


    // 任务12：实现 resize 方法
    // 按照逻辑蓝图：
    // 步骤 1: 分配新的数组 T* new_data = new T[new_capacity];
    // 步骤 2: 复制旧数据 for (size_t i = 0; i < size_; ++i) new_data[i] = data[i];
    // 步骤 3: 释放旧数组 delete[] data;
    // 步骤 4: 更新指针和容量 data = new_data; capacity_ = new_capacity;
    void resize(size_t new_capacity){
        
        T* new_data = new T[new_capacity];

        for (size_t i = 0; i < size_; ++i){
            new_data[i] = data[i];
        }

        delete [] data;

        data = new_data;
        capacity_ = new_capacity;
        
    }



    // 任务13：实现 stride 访问（模拟 Tensor 多维访问）
    // 语法解释：
    // - std::vector<T> 是 C++ 标准库的动态数组
    // - 从 start_index 开始，每次跳 stride 步
    // - 返回所有访问到的元素

    // 在这里实现 stride（这是高级功能，可以最后做）：
    // std::vector<T> stride(size_t start_index, int stride) const {
    //     // 检查参数有效性
    //     if (_________ >= _________) {
    //         throw std::__________("Start index out of range");
    //     }
    //     if (_________ == 0) {
    //         throw std::__________("Stride cannot be zero");
    //     }
    //
    //     std::vector<T> result;
    //     size_t current_index = _________;
    //
    //     if (_________ > 0) {
    //         // 正向步进
    //         while (_________ < _________) {
    //             result.push_back(_________[__________]);
    //             _________ += _________;
    //         }
    //     } else {
    //         // 负向步进 - 这个比较复杂，可以暂时跳过
    //         // TODO: 实现负stride逻辑
    //     }
    //
    //     return result;
    // }
    std::vector<T> stride(size_t start_index, int stride) const{

        if (!(0 <= start_index && start_index < size_)){
            throw std::out_of_range("Start index " + std::to_string(start_index) + " out of range");
        }

        if (stride == 0){
            throw std::invalid_argument("Stride cannot be zero");
        }

        std::vector<T> result;
        size_t current_index = start_index;

        if (stride > 0 ){

            while (current_index < size_) {
                result.push_back(data[current_index]);
                current_index += stride;
            }

        }else {

            while (current_index >= 0) {
                result.push_back(data[current_index]);

                // BUG fix: 在减法前检查：如果 stride 的绝对值大于 current_index，会下溢
                /** 
                根本问题：负向 stride 的边界检查有bug
                当 stride = -2, start_index = 5 时：
                current_index = 5
                while (5 >= 0): 访问 data[5] ✓
                current_index = 5 + (-2) = 3
                while (3 >= 0): 访问 data[3] ✓
                current_index = 3 + (-2) = 1
                while (1 >= 0): 访问 data[1] ✓
                current_index = 1 + (-2) = -1 ⚠️
                while (-1 >= 0): false，退出循环
                
                size_t 是无符号类型！当 current_index = 1 + (-2) = -1 时，由于 size_t 是无符号的，-1 会变成一个非常大的正数！
                这就是问题！size_t current_index 在减法时会下溢，变成 18446744073709551615 (size_t的最大值)。
                */
                if (static_cast<size_t>(-stride) > current_index) {
                    break;  // 退出循环，避免下溢
                }

                current_index += stride;
            }
        }

        return result;
    }


};

// ==============================================================================
// 【测试代码框架 - 你来填内容】
// ==============================================================================

int main() {
    std::cout << "=== C++ Vector Template Test ===\n";

    // 任务14：创建 Vector 实例并测试基本功能
    // 提示：
    // - Vector<int> v(4); 创建容量为4的int Vector
    // - 测试 push_back, pop_back, operator[]
    // - 测试扩容（添加超过4个元素）
    // - 测试不同类型：Vector<std::string>

    // TestCase 001 : 测试Vector创建
    Vector<int> matrix;
    Vector<int> matrix_4 (4);
    std::cout << "[TC001] Vector<int> matrix_4(4) 创建完成, size=" << matrix_4.size() << ", capacity=" << matrix_4.capacity() << std::endl;

    // TestCase 002 : 测试 push_back + 自动扩容
    matrix_4.push_back(1);
    matrix_4.push_back(2);
    matrix_4.push_back(3);
    matrix_4.push_back(4);
    matrix_4.push_back(5);
    matrix_4.push_back(6);

    std::cout << "[TC002] 自动扩容后, size=" << matrix_4.size() << ", capacity=" << matrix_4.capacity() << std::endl;
    std::cout << "[TC002] 当前 vector 内容: ";
    for (size_t i = 0; i < matrix_4.size(); ++i) std::cout << matrix_4[i] << " ";
    std::cout << std::endl;

    // TestCase 003 : 测试stride
    auto stride_res1 = matrix_4.stride(1, 2);
    std::cout << "[TC003] stride(1, 2) 结果: ";
    for (auto v : stride_res1) std::cout << v << " ";
    std::cout << std::endl;

    auto stride_res2 = matrix_4.stride(5, -2);
    std::cout << "[TC003] stride(5, -2) 结果: ";
    for (auto v : stride_res2) std::cout << v << " ";
    std::cout << std::endl;

    // TestCase 004 : 测试 pop_back + 自动缩容
    int n_pops = 6;
    std::cout << "[TC004] 依次 pop_back: ";
    for (int i = 0; i < n_pops; ++i) {
        try {
            int val = matrix_4.pop_back();
            std::cout << val << " ";
        } catch (std::out_of_range& e) {
            std::cout << "\n[TC004] pop_back 触发异常: " << e.what() << "\n";
            break;
        }
    }
    std::cout << std::endl;


    return 0;
}

