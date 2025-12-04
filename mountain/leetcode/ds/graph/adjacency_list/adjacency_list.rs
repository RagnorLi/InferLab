/**
 * 图（邻接表）(Graph - Adjacency List) - Rust 实现
 * 
 * ## 需求说明
 * 
 * 实现一个基于邻接表的图数据结构，要求：
 * 
 * ### 核心特性
 * - 用链表数组表示图
 * - 适合稀疏图
 * 
 * ### 必须实现的操作
 * 1. `add_vertex(vertex)` - 添加顶点
 * 2. `add_edge(u, v)` - 添加边
 * 3. `get_neighbors(vertex)` - 获取邻接顶点列表
 * 4. `has_edge(u, v)` - 判断是否有边
 * 
 * ### 存储方式
 * - 使用 Vec<Vec<usize>> 或 HashMap
 * 
 * ### 空间复杂度
 * - O(V + E)
 */
