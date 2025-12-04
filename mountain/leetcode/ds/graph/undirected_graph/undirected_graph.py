"""
无向图 (Undirected Graph) - Python 实现

## 需求说明

实现一个无向图数据结构，要求：

### 核心特性
- 边没有方向，连接是双向的
- (u, v) 和 (v, u) 表示同一条边
- 可以基于邻接表或邻接矩阵实现

### 必须实现的操作
1. `add_vertex(vertex)` - 添加顶点
2. `add_edge(u, v)` - 添加无向边（同时添加 u->v 和 v->u）
3. `remove_edge(u, v)` - 删除无向边
4. `has_edge(u, v)` - 判断是否有边
5. `get_degree(vertex)` - 获取顶点的度
6. `get_neighbors(vertex)` - 获取邻接顶点
7. `is_connected()` - 判断图是否连通
8. `bfs(start)` - 广度优先搜索
9. `dfs(start)` - 深度优先搜索

### 应用场景
- 社交网络
- 道路网络（双向道路）
- 计算机网络

"""
