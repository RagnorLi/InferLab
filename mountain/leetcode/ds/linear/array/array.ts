/**
 * npx tsx mountain/leetcode/ds/linear/array/array.ts
 * 
 * 数组 (Array) - TypeScript 实现
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
 * 1. `constructor(capacity)` - 初始化数组，指定容量
 * 2. `get(index)` - 通过索引获取元素 O(1)
 * 3. `set(index, value)` - 通过索引设置元素 O(1)
 * 4. `append(value)` - 在数组末尾追加元素 O(1) 平均
 * 5. `insert(index, value)` - 在指定位置插入元素 O(n)
 * 6. `delete(index)` - 删除指定位置的元素 O(n)
 * 7. `length()` - 返回数组长度 O(1)
 * 8. `isEmpty()` - 判断数组是否为空 O(1)
 * 
 * ### 可选优化
 * - 使用泛型支持任意类型
 * - 动态扩容策略（2倍扩容或1.5倍扩容）
 * - 缩容机制（当元素数量远小于容量时）
 * - 实现迭代器接口
 * 
 * ### 时间复杂度要求
 * - 随机访问：O(1)
 * - 插入/删除：O(n)
 * - 追加：O(1) 平均（考虑扩容摊销）
 */

/**
 * 动态数组实现
 * 
 * 核心思想：
 * 1. 使用 JavaScript Array 作为底层存储（连续内存， V8引擎优化）
 * 2. 维护容量（capacity） 和实际长度（size）
 * 3. 当容量不足时，按 2 倍扩容。虽然单次扩容操作最坏情况下需要 O(n) 时间，但平均到每次 append 操作上的时间复杂度是 O(1)，这就是摊销（amortized）O(1)。即，多次操作的总耗时均摊到每次操作单独计算，单次开销高但长期平均低。
 * 4. 当元素过少时，按 1/2 缩容 (可选优化)
*/

export class Array<T>{
    private data: (T | undefined)[];
    private size: number;
    private capacity: number;

    /**
     * 构造函数：初始化数组
     * 
     * @param capacity 初始容量，默认为10
     * @throw {Error} 如果capacity <= 0
    */
   	constructor(capacity: number = 10){
    	if (capacity <= 0){
        	throw new Error("Capacity must be positive");
    	}

		this.capacity = capacity;
		this.size = 0;
		this.data = new globalThis.Array<T | undefined>(capacity).fill(undefined); // globalThis指向内置的Array不然默认是你自定义的这个是没有fill方法的
   	}

   	/**
    * 容错版本：通过索引获取元素
    * 可选类型（Optional Type）的隐式处理：JavaScript/TypeScript 中越界访问返回 undefined，而非抛出异常；这是语言的设计选择，不是缺陷。
    * 
    * @param index 索引位置
    * @returns 对应位置的元素，如果索引越界返回 undefined
    * 
    * 时间复杂度：O(1)
   	*/
   	get(index: number): (T | undefined) {
        if (index < 0 || index >= this.size){
            return undefined;
        }
        return this.data[index];
   	}

   	/**
    * 严格版本：
    * 错误处理策略：fail-fast（抛异常）vs 容错（返回标记值）
    * 
    * @param inde 索引位置
    * @returns 对应位置的元素，如果索引越界程序中断抛出错误
   	*/
   	getOrThrow(index: number): T {
    	if (index < 0 || index >= this.size) {
        	throw new Error(`Index ${index} out of bounds [0, ${this.size})`);  // 严格：立即失败
        	}
    	return this.data[index]!;  // ! 断言非 undefined
    }

    /**
     * 通过索引设置元素
     * 
     * @param index 索引位置
     * @param value 要设置的值
     * @returns 如果设置成功返回 true, 否则返回 false
     * 
     * 时间复杂度：O(1)
    */
   	set(index: number, value: T): boolean{
    	if (index < 0 || index >= this.size){
        	return false;
    	}
    	this.data[index] = value;
    	return true;
   	}

   /**
    * 在数组末尾追加元素
    * 
    * 当容量不足时，先扩容再插入，摊销时间复杂度为 O(1)
    * 
    * @param value要追加的值
    * 
    * 时间复杂度：O(1) 平均（摊销分析）
   */
  	append(value: T): void{
    	// 如果容量不足先扩容
    	if (this.size >= this.capacity){
        	this.resize(this.capacity * 2);
    	}

    	this.data[this.size] = value;
    	this.size++;
  	}

  	/**
	 * 在指定的位置插入元素
	 * 
	 * 需要将index之后的元素都向后移动一位
	 * 
	 * @param index插入位置，必须在 [0, size]范围内
	 * @param value 要插入的值
	 * @returns 如果插入成功返回 true,否则返回false
	 * 
	 * 时间复杂度：O(n)
	 * 1. `insert` 的核心成本是**把插入点后的所有元素右移一格**，移动次数为 (n - k)，最坏为 (n)。
	 * 2. 因为底层是**连续内存**，不能只改指针，必须真实搬数据，所以成本和元素个数线性相关。
	 * 3. 所以，动态数组的 `insert` 在最坏和平均意义上，都属于 (O(n)) 级别操作。
	 * 
  	*/
	//  [ANCHOR: INSERT_LOGIC]
  	insert(index: number, value: T): boolean{
    	if (index <0 || index > this.size){
        	return false;
    	}

		// 如果容量不足，先扩容
		if (this.size >= this.capacity){
			this.resize(this.capacity * 2);
		}

		// 将index 之后的元素向后移动
		for (let i = this.size; i > index; i--){
			this.data[i] = this.data[i - 1];
		}

		this.data[index] = value;
		this.size++;
		return true;
	}

   /**
   * 删除指定位置的元素
   * 
   * 需要将 index 之后的元素都向前移动一位
   * 
   * @param index 要删除的位置
   * @returns 被删除的元素，如果索引越界返回 undefined
   * 
   * 时间复杂度：O(n)
   */
 	delete(index: number) : T | undefined{
    	if (index < 0 || index >= this.size){
        	return undefined;
    	} 

    	const removed = this.data[index];

    	// 将index之后的元素向前移动
    	for (let i = index; i < this.size - 1; i++){
        	this.data[i] = this.data[i + 1];
    	}

		this.size--;
		this.data[this.size] = undefined; // 清理最后一个位置

		// 可选：如果元素数量远小于容量，进行缩容
		// 避免频繁扩容缩容，使用 1/4 最为阈值
		if (this.size > 0 && this.size < this.capacity / 4){
			this.resize(Math.floor(this.capacity / 2));
		}

		return removed;
 	} 

	/**
	 * 返回数组长度
	 * 
	 * 时间复杂度：O(1)
	 * 
	 * @returns 当前元素数量
	 * 
	 */
	length(): number{
		return this.size;
	}

	/**
	 * 判断数组是否为空
	 * 
	 * 时间复杂度:O(1)
	 * 
	 * @returns 如果为空返回true,否则返回 false
	*/
	isEmpty(): boolean{
		return this.size === 0;
	}

	/**
	 * 获取当前容量
	 * 
	 * @returns 当前容量
	 * 
	*/
	getCapacity() : number{
		return this.capacity;
	}

	/**
	 * 内部方法： 调整数组容量
	 * 
	 * 时间复杂度:O(n), 但摊销到多次操作后，append 平均为O（1）
	 * 
	 * @params newCapacity新的容量
	 * @throws {Error} 如果newCapacity < size
	 * 
	*/
	private resize(newCapacity: number): void{
		if(newCapacity < this.size){
			throw new Error(
				`New capacity ${newCapacity} is too small for ${this.size} elements`
			);
		}

		const oldData = this.data;
		this.data = new globalThis.Array<T | undefined>(newCapacity).fill(undefined);
		this.capacity = newCapacity;

		// 复制所有元素
		for (let i = 0; i < this.size; i++){
			this.data[i] = oldData[i];
		}
	}

	/**
	 * 返回数组的字符串表示
	 * 
	 * @returns 格式化的字符串
	 * 
	*/
	toString(): string{
		const elements:T[] = [];
		for (let i = 0; i < this.size; i++){
			elements.push(this.data[i]!); // 这里的 '!' 是 TypeScript 的非空断言操作符，表示“我保证 this.data[i] 不是 undefined”
		}

		return `Array([${elements.join(", ")}], size=${this.size}, capacity=${this.capacity})`;
	}

	/**
	 * 实现迭代器接口，支持for ... of循环
	 * 
	 * @returns 迭代器对象
	*/
	[Symbol.iterator](): Iterator<T>{
		let index = 0;
		const data = this.data;
		const size = this.size;

		return {
			next(): IteratorResult<T>{
				if (index < size){
					const value = data[index]!;
					index++;
					return {value, done: false};
				}else{
					return {value: undefined, done: true};
				}
			}
		};
	}

	 /**
     * 返回数组的浅拷贝（只包含实际元素）
     * 
     * @returns 新的 JavaScript 数组
     */
	 toArray(): T[] {
        const result: T[] = [];
        for (let i = 0; i < this.size; i++) {
            result.push(this.data[i]!);
        }
        return result;
    }

    /**
     * 清空数组
     */
    clear(): void {
        this.size = 0;
        // 可选：重置容量到初始值
        // this.resize(10);
    }

}

// Test code - can be executed directly with tsx or ts-node
// Usage: npx tsx array.ts  or  npx ts-node array.ts
const arr = new Array<number>(3);
console.log(`初始状态: ${arr.toString()}`);

// 测试 append
arr.append(1);
arr.append(2);
arr.append(3);
console.log(`追加 3 个元素: ${arr.toString()}`);

// 测试扩容
arr.append(4);
console.log(`触发扩容后: ${arr.toString()}`);

// 测试随机访问
console.log(`arr.get(0) = ${arr.get(0)}`);
arr.set(0, 10);
console.log(`修改后 arr.get(0) = ${arr.get(0)}`);

// 测试 insert
arr.insert(1, 99);
console.log(`在索引 1 插入 99: ${arr.toString()}`);

// 测试 delete
const removed = arr.delete(2);
console.log(`删除索引 2 的元素 ${removed}: ${arr.toString()}`);

// 测试缩容（需要删除足够多的元素）
arr.delete(0);
arr.delete(0);
console.log(`删除后可能触发缩容: ${arr.toString()}`);

// 测试迭代器
console.log("迭代数组:");
for (const item of arr) {
    console.log(`  ${item}`);
}

// 测试字符串数组
const strArr = new Array<string>(2);
strArr.append("hello");
strArr.append("world");
console.log(`字符串数组: ${strArr.toString()}`);

// 测试边界情况
console.log(`空数组长度: ${arr.isEmpty() ? "是空的" : "非空"}`);
console.log(`数组容量: ${arr.getCapacity()}`);