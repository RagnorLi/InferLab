"""
图（邻接表）(Graph - Adjacency List) - Python 实现

## 需求说明

实现一个基于邻接表的图数据结构，要求：

### 核心特性
- 用链表数组表示图
- 每个顶点维护一个邻接顶点列表
- 适合稀疏图（边数远小于顶点数平方）

### 必须实现的操作
1. `add_vertex(vertex)` - 添加顶点 O(1)
2. `add_edge(u, v)` - 添加边 O(1)
3. `remove_edge(u, v)` - 删除边 O(degree(v))
4. `remove_vertex(vertex)` - 删除顶点 O(V + E)
5. `get_neighbors(vertex)` - 获取邻接顶点列表 O(1)
6. `has_edge(u, v)` - 判断是否有边 O(degree(v))
7. `get_vertices()` - 获取所有顶点 O(V)
8. `get_edges()` - 获取所有边 O(V + E)

### 存储方式
- 使用字典或数组存储顶点
- 每个顶点对应一个邻接列表（列表/集合）

### 应用场景
- 稀疏图
- 需要频繁遍历邻接顶点
- 社交网络

### 空间复杂度
- O(V + E)

"""
