"""
加权图 (Weighted Graph) - Python 实现

## 需求说明

实现一个加权图数据结构，要求：

### 核心特性
- 每条边有权重（权重可以是距离、成本、时间等）
- 可以是有向或无向
- 支持最短路径算法

### 必须实现的操作
1. `add_vertex(vertex)` - 添加顶点
2. `add_edge(u, v, weight)` - 添加带权重的边
3. `get_weight(u, v)` - 获取边的权重
4. `update_weight(u, v, weight)` - 更新边的权重
5. `remove_edge(u, v)` - 删除边
6. `dijkstra(start)` - 单源最短路径（Dijkstra算法）
7. `floyd_warshall()` - 全源最短路径（可选）
8. `mst()` - 最小生成树（Kruskal/Prim算法，可选）

### 边表示
- 使用三元组 (u, v, weight) 表示边
- 或使用 Edge 类包含起点、终点和权重

### 应用场景
- 路由算法
- 网络流
- 资源分配
- GPS导航

"""
