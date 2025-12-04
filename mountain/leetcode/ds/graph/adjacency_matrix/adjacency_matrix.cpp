/**
 * 图（邻接矩阵）(Graph - Adjacency Matrix) - C++ 实现
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
 * 1. `Graph(numVertices)` - 构造函数，指定顶点数
 * 2. `addEdge(u, v)` - 添加边 O(1)
 * 3. `removeEdge(u, v)` - 删除边 O(1)
 * 4. `hasEdge(u, v)` - 判断是否有边 O(1)
 * 5. `getNeighbors(vertex)` - 获取邻接顶点列表
 * 
 * ### 存储方式
 * - 使用 vector<vector<bool>> 或 vector<vector<int>>
 * 
 * ### 空间复杂度
 * - O(V²)
 */
