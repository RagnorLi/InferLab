"""
有向图 (Directed Graph) - Python 实现

## 需求说明

实现一个有向图数据结构，要求：

### 核心特性
- 边有方向，u->v 和 v->u 是不同的边
- 顶点有出度和入度
- 可以存在自环

### 必须实现的操作
1. `add_vertex(vertex)` - 添加顶点
2. `add_edge(u, v)` - 添加有向边 u->v
3. `remove_edge(u, v)` - 删除有向边
4. `has_edge(u, v)` - 判断是否有边 u->v
5. `get_in_degree(vertex)` - 获取入度
6. `get_out_degree(vertex)` - 获取出度
7. `get_neighbors(vertex)` - 获取出边邻接顶点
8. `get_reverse_neighbors(vertex)` - 获取入边邻接顶点（可选）
9. `topological_sort()` - 拓扑排序（无环图）
10. `is_acyclic()` - 判断是否有环

### 应用场景
- 任务依赖关系
- 网页链接
- 状态机
- 编译器依赖图

"""
