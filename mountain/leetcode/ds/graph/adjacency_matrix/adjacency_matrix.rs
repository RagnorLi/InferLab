/**
 * 图（邻接矩阵）(Graph - Adjacency Matrix) - Rust 实现
 * 
 * ## 需求说明
 * 
 * 实现一个基于邻接矩阵的图数据结构，要求：
 * 
 * ### 核心特性
 * - 用二维数组表示图
 * - 适合稠密图
 * 
 * ### 必须实现的操作
 * 1. `new(num_vertices)` - 创建图，指定顶点数
 * 2. `add_edge(u, v)` - 添加边 O(1)
 * 3. `has_edge(u, v)` - 判断是否有边 O(1)
 * 4. `get_neighbors(vertex)` - 获取邻接顶点列表
 * 
 * ### 存储方式
 * - 使用 Vec<Vec<bool>> 或 Vec<Vec<i32>>
 * 
 * ### 空间复杂度
 * - O(V²)
 */
