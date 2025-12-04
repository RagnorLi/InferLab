/**
 * 图（邻接表）(Graph - Adjacency List) - C++ 实现
 * 
 * ## 需求说明
 * 
 * 实现一个基于邻接表的图数据结构，要求：
 * 
 * ### 核心特性
 * - 用链表数组表示图
 * - 每个顶点维护一个邻接顶点列表
 * - 适合稀疏图
 * 
 * ### 必须实现的操作
 * 1. `addVertex(vertex)` - 添加顶点
 * 2. `addEdge(u, v)` - 添加边 O(1)
 * 3. `removeEdge(u, v)` - 删除边
 * 4. `getNeighbors(vertex)` - 获取邻接顶点列表
 * 5. `hasEdge(u, v)` - 判断是否有边
 * 6. `getVertices()` - 获取所有顶点
 * 
 * ### 存储方式
 * - 使用 vector<vector<int>> 或 map<int, vector<int>>
 * 
 * ### 空间复杂度
 * - O(V + E)
 */
