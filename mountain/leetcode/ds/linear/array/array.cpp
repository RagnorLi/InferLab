/**
 * 数组 (Array) - C++ 实现
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
 * 1. `Array(capacity)` - 构造函数，初始化数组，指定容量
 * 2. `~Array()` - 析构函数，释放内存
 * 3. `operator[](index)` - 通过索引访问元素 O(1)
 * 4. `append(value)` - 在数组末尾追加元素 O(1) 平均
 * 5. `insert(index, value)` - 在指定位置插入元素 O(n)
 * 6. `remove(index)` - 删除指定位置的元素 O(n)
 * 7. `length()` - 返回数组长度 O(1)
 * 8. `empty()` - 判断数组是否为空 O(1)
 * 
 * ### 可选优化
 * - 使用模板支持泛型
 * - 动态扩容策略（2倍扩容或1.5倍扩容）
 * - 缩容机制（当元素数量远小于容量时）
 * - 拷贝构造函数和赋值运算符
 * 
 * ### 时间复杂度要求
 * - 随机访问：O(1)
 * - 插入/删除：O(n)
 * - 追加：O(1) 平均（考虑扩容摊销）
 */

 
/**
C++ 版本的关键点：
模板泛型：支持任意类型
内存管理：手动管理（new[]/delete[]），遵循 Rule of Five
移动语义：使用移动构造/赋值优化性能
异常安全：保证基本异常安全
边界检查：使用断言或异常
*/

#include <cstddef>
#include <iostream>
#include <ostream>
#include <stdio.h>
#include <algorithm>
#include <utility>
#include <cassert>


template<typename T>
class Array{
private:
    // c++ 命令规范习惯用 *_ 表示成员变量，成员变量即对象属性
    T* data_;       // 指向动态分配的内存
    size_t size_;   // 当前元素数量
    size_t capacity_;   // 当前容量

    /**
    内部方法： 调整数组容量

    时间复杂度：O(n), 但摊销到多次操作后，append平均为 O(1)

    @param new_capacity 新的容量
    */ 
    void resize(size_t new_capacity){
        if (new_capacity < size_){
            throw std::invalid_argument("New capavity is too small for current size");
        }

        // 这不是RAII(C++资源管理范式) 我理解错了单纯地在函数里new、然后delete，这不是RAII，只有把资源管理包装成对象、依赖析构自动释放，才符合RAII风格。

        // 分配新内存
        T* new_data = new T[new_capacity];

        // 移动构造现有元素（C++11 移动语义优化）： 所谓移动构造不是资源赋值而是资源所有权转移
        for (size_t i = 0; i < size_; ++i){
            new_data[i] = std::move(data_[i]);
        }

        // 释放旧内存
        delete [] data_;

        data_ = new_data;
        capacity_ = new_capacity;
    }

public:
    /** 
    构造函数： 初始化数组

    @param capacity 初始容量，默认为10
    */

    // explicit 关键字用于修饰构造函数，防止发生“隐式类型转换”。
    // 具体来说，如果没有 explicit，则允许像这样写：
    //   Array<int> arr = 5;        // 没有 explicit 时，相当于自动调用 Array(5) 构造
    // 这样会把整数 5 隐式转换成一个 Array<int> 对象，可能导致代码歧义或潜在 bug。
    // 加上 explicit 后，只有像 Array<int> arr(5); 这样“显式”调用构造函数才允许，不能自动由整数变成数组对象。
    explicit Array(size_t capacity = 10) : size_(0), capacity_(capacity){  // c++ 没有None类型 指针类型有nullptr 数值类型无 
        if (capacity == 0){
            throw std::invalid_argument("Capacity myust be positive");
        }
        // new T[capacity_] 用于分配一个包含 capacity_ 个元素的数组，而 new T() 只会分配一个单独的对象。
        data_ = new T[capacity_];  // ← 这里！用 new[] 在堆上分配内存
    }

    /**
    拷贝构造函数 （深拷贝）

    时间复杂度：O(n)
    
    * 为什么要用 `const Array& other`？
    * - `const`：保证不会修改被拷贝的源对象，提升安全性。
    * - `&`（引用:= 别名）：避免把整个 Array 对象拷贝一份，提高效率。
    * 这种写法（`const T&`）是 C++ 拷贝构造函数和大多数拷贝操作的标准写法，既安全又高效。
    */


    // “哪来的两次”：其实并不是每次都会有“两次”，要区分成员变量的构造方式。
    // 情况1：如果你不用初始化列表，而是在构造函数体内用赋值的方式（如 this->member = val），
    //       那么成员变量会先用它的默认构造函数初始化一次，然后你再赋值（调用赋值操作），等于它的生命里确实做了“默认构造 + 赋值”两步，属于“重复初始化”。
    // 情况2：如果成员变量是内置类型（如 int、指针），它的“默认构造”就是不初始化，只有赋值。
    // 情况3：如果成员变量是 const/引用或没有默认构造器的类，根本不能采用“先默认再赋值”，只能用初始化列表。
    // C++ 这么设计的原因，是让你能精确控制每个成员的构造或初始化方式（性能、语义安全），这跟别的语言不同，C++语法设计允许你掌控每颗螺丝，而不是单纯易用优先。
    // 并不是别的语言都“不好”，只是C++把初始化/赋值这件事设计得非常细致——这样利于做高性能、可控的系统，但门槛也高。
    Array(const Array& other) : size_(other.size_), capacity_(other.capacity_) {
        data_ = new T[capacity_];

        // 拷贝构造每个元素
        for (size_t i = 0; i < size_; ++i){
            data_[i] = other.data_[i];
        }
    }

    /** 
    移动构造函数 （c++11）

    时间复杂度： O(1)
    */
    
    // 移动语义的核心思想：如果源对象即将被销毁，就把资源“偷”过来，而不是复制。
    // 核心就两点：
    //     1. 把指针“偷”过来
    //     2. 把原对象置空（防止重复释放）
    // && 和 noexcept 是语法标记，让编译器知道何时调用移动版本。
    Array(Array&& other) noexcept
        : data_(other.data_), size_(other.size_), capacity_(other.capacity_) {
        // 置空原对象，防止析构时重复释放资源
        other.data_ = nullptr;
        other.size_ = 0;
        other.capacity_ = 0;
    }

    /**
    拷贝赋值运算符（深拷贝）

    使用copy-and-swap idiom 保证异常安全
    时间复杂度： 0(n)

    *和&都是“装饰”类型的符号，* 和 & 是类型的一部分，变量名只是接收这个类型，不影响语义。
    */
    Array& operator=(const Array& other){
        // Check for self-assignment. If `this` and `other` are the same object, do nothing.
        if (this != &other){
            // 创建临时副本
            Array temp(other);
            // 交换（异常安全）
            swap(temp);
        }
        return *this;
    }

    /**
    移动赋值运算符（c++11）

    时间复杂度：O(1)
    */
    Array& operator=(Array&& other) noexcept {
        // 'this' is a pointer to the current object, and &other is the address of the object 'other'
        if (this != &other){ // avoid self assignment
            // 释放当前资源
            // - 'delete' frees a single object allocated with 'new'.
            // - 'delete[]' frees an array allocated with 'new[]'.
            delete [] data_;

            // 移动资源
            data_ = other.data_;
            size_ = other.size_;
            capacity_ = other.capacity_;

            // 将源对象置为空状态
            other.data_ = nullptr;
            other.size_ = 0;
            other.capacity_ = 0;
        } 

        // 更准确的说法是：* 作用于指针表达式，& 作用于左值表达式。
        // "*expr 解引用" —— *expr 对求值为指针的表达式 expr 解引用，获取指向的对象。
        // "&expr 取地址" —— &expr 取左值表达式 expr 的地址，得到指向它的指针。
        return *this;
    }

    /**
    析构函数为什么需要显式 delete data_，但不用把 size_ 和 capacity_ 置为 0？
    TODO: C++ 内存管理模型虽迟但到 
    */
    ~Array() {
        delete[] data_;
    }

    /**
    通过索引访问元素（非const版本）

    时间复杂度：O(1)

    @param index 索引的位置
    @return 元素的引用
     */
    // operator[] 返回 T&（T 的引用），这样用户既可以读取，也可以直接原地修改元素。
    // 返回引用本质上就是“提供一个别名”，引用和原对象共享内存，所有修改会直接反映到原对象上。
    T& operator[](size_t index){
        if (index >= size_){
            throw std::out_of_range("Index out of range");
        }
        // data_ is a pointer; in C++, pointers support the [] operator.
        // data_[index] is equivalent to *(data_ + index).
        return data_[index];
    }

    /**
    通过索引访问元素 （const 版本）

    时间复杂度：O(1)

    @param index 索引位置
    @return 元素的const引用
    */
    // 下面这个函数的作用是：让 Array 在被声明为 const 时，也能通过下标访问元素（只读）。
    // 这里有两个 const，第一个 const 放在参数后面，表示“这个成员函数不会修改类的成员变量”，
    // 第二个 const 写在 T& 前面，表示“返回的引用不能修改数组里的元素”。
    // 两个 const 加起来，保证这个函数只能读，不能改。不管是类本身，还是返回的元素。
    const T& operator[](size_t index) const {
        if (index >= size_) {
            throw std::out_of_range("Index out of range");
        }
        return data_[index];
    }

    /**
    在数组末尾追加元素

    当容量不足时，先扩容再插入，摊销时间复杂度为 O(1)

    @param value 要追加的值
    */
    void append(const T& value){
        if (size_ >= capacity_){
            resize(capacity_ * 2);
        }
        data_[size_] = value;
        ++size_;
    }

    /**
    在数组末尾追加元素（移动版本，性能优化）

    @param value 要最佳的元素（右值引用）
    */
    void append(T&& value){
        if (size_ >= capacity_){
            resize(capacity_ * 2);
        }
        // 之所以称为“移动版本性能优化”，
        // 是因为 std::move(value) 会让 data_[size_] = std::move(value) 这个赋值操作调用 T 的移动赋值运算符（如果有），
        // 这样能避免内存内容的深度拷贝，只转移（“窃取”）源对象 value 拥有的资源指针或内部状态，
        // 典型如 string、vector 等拥有堆内存资源的类型，因此大幅减少资源分配与复制的开销。
        // 这就是比 const T& 更高效的原因，但要注意 move 后 value 变为“空壳”，不可再安全使用。
        data_[size_] = std::move(value);
        ++size_;
    }

    /**
    在指定位置插入元素

    需要将index之后的元素都向后移动一位

    时间复杂度O(n)
    
    @param index插入位置
    @param value 要插入的值
    */
    void insert(size_t index, const T& value){ // 参数 value 是 T 类型的常量引用（const reference）
        if (index > size_){
            throw std::out_of_range("Index out of range");
        }

        // 如果容量不足，先扩容
        if (size_ >= capacity_){
            resize(capacity_ * 2);
        }

        // 将index之后的元素向后移动
        for (size_t i = size_; i > index; --i){
            data_[i] = std::move(data_[i - 1]);
        }

        data_[index] = value;
        ++size_;
    }

    /**
    在指定位置插入元素（移动版本）

    @param index 插入位置
    @param value 要插入的值 （右值引用）
    */
    // 这里不用 const T&& value 是因为移动语义要求能够修改参数 value，
    // 即将 value 的内部资源“移动”（窃取）到容器内。如果加上 const，
    // 就无法执行 move 或修改操作，失去了使用右值引用的意义。
    void insert(size_t index, T&& value){
        if (index > size_){
            throw std::out_of_range("Index out of range");
        }

        if (size_ >= capacity_){
            resize(capacity_ * 2);
        }

        for (size_t i = size_; i > index; --i){
            data_[i] = std::move(data_[i - 1]); // 倒车请注意 倒车请注意哈哈
        }

        data_[index] = std::move(value);
        ++size_;
    }

    /**
    删除指定位置的元素

    需要将index之后的元素都都向前移动一位

    时间复杂度：O(n)
    
    @param index 要删除的位置
    @return 被删除的元素
    */
    T remove(size_t index){

        if (index >= size_){
            throw std::out_of_range("Index out of range");
        }

        T removed = std::move(data_[index]); // 这里索引取的是内存地址 还是 值呢

        // 将index之后的元素向前移动
        for (size_t i = index; i < size_ - 1; ++i){
            data_[i] = std::move(data_[i+1]);
        }

        --size_;

        // 可选: 如果元素数量远小于容量，进行缩容
        // 避免频繁扩容缩容，使用1/4作为阈值
        if (size_ > 0 && size_ < capacity_ / 4){
            resize(capacity_ / 2);
        }

        return removed;
    }


    /**
    返回数组长度

    时间复杂度：O(1)

    @return 当前元素数量
    */
    // Yes, marking the method as 'const' tells the compiler that this method will not modify any member variables
    // of the class, not just 'size_'. That is, no member variables (except those marked as 'mutable')
    // can be changed in any way inside 'length'. This guarantees that calling 'length' does not alter the object's state.
    size_t length() const {
        return size_;
    }

    /**
    判断数组是否为空

    时间复杂度：O(1)

    @return 如果为空返回true,否则返回false
    */
    bool empty() const{
        return size_ == 0;
    }

    /**
    获取当前容量

    @return 当前容量
    */
    size_t capacity() const{
        return capacity_;
    }

    /**
    交换两个数组（用于copy-and-swap idiom）

    @param other 要交换的数组
    */
    void swap(Array& other) noexcept{

        // Directly exchange the internal state of the two arrays.
        // auto tmp_data = data_; data_ = other.data_; other.data_ = tmp_data;
        // auto tmp_size = size_; size_ = other.size_; other.size_ = tmp_size;
        // auto tmp_capacity = capacity_; capacity_ = other.capacity_; other.capacity_ = tmp_capacity;

        // clean且地道的C++写法（对所有成员使用std::swap）
        std::swap(data_, other.data_);
        std::swap(size_, other.size_);
        std::swap(capacity_, other.capacity_);
    }

    /**
    输出数组内容用于调试
    */
    friend std::ostream& operator<<(std::ostream& os, const Array& arr){
        os << "Array([";
        for (size_t i = 0; i < arr.size_; ++i){
            os << arr.data_[i];
            if (i < arr.size_ - 1) {
                os << ", ";
            }
        }
        os << "], size=" << arr.size_ << ", capacity=" << arr.capacity_ << ")";
        return os;
    }
};  

/**
测试代码 优化 vs 调试
   clang++ -std=c++11 -Wall -Wextra -O2 array.cpp -o array
   clang++ -std=c++11 -Wall -Wextra -g array.cpp -o array
TODO: GDB调试
*/ 
int main(){
    try{
        // 基础功能测试
        Array<int> arr(3);
        std::cout << "初始状态: " << arr << std::endl;

        // 测试 append
        arr.append(1);
        arr.append(2);
        arr.append(3);
        std::cout << "追加 3 个元素: " << arr << std::endl;

        // 测试扩容
        arr.append(4);
        std::cout << "触发扩容后: " << arr << std::endl;

        // 测试随机访问
        std::cout << "arr[0] = " << arr[0] << std::endl;
        arr[0] = 10;
        std::cout << "修改后 arr[0] = " << arr[0] << std::endl;

        // 测试 insert
        arr.insert(1, 99);
        std::cout << "在索引 1 插入 99: " << arr << std::endl;

        // 测试 remove
        int removed = arr.remove(2);
        std::cout << "删除索引 2 的元素 " << removed << ": " << arr << std::endl;

        // 测试缩容（需要删除足够多的元素）
        arr.remove(0);
        arr.remove(0);
        std::cout << "删除后可能触发缩容: " << arr << std::endl;

        // 测试拷贝构造
        Array<int> arr2 = arr;
        std::cout << "拷贝构造 arr2: " << arr2 << std::endl;

        // 测试移动构造
        Array<int> arr3 = std::move(arr2);
        std::cout << "移动构造 arr3: " << arr3 << std::endl;
        std::cout << "移动后的 arr2: " << arr2 << std::endl;

        // 测试泛型（字符串）
        Array<std::string> strArr(2);
        strArr.append("hello");
        strArr.append("world");
        std::cout << "字符串数组: " << strArr << std::endl;

    }catch (const std::exception& e) {
        std::cerr << "异常: " << e.what() << std::endl;
        return 1;
    }

    return 0;
}