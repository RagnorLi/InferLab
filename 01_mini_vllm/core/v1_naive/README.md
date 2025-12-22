# v1_naive - 最简实现

**目标：** 建立对 vLLM 的基本直觉

## 特性

- ✅ 简单的 Python list 管理显存块
- ✅ FCFS (First Come First Serve) 调度
- ✅ 无 Eviction，无 Preemption，无 Prefix Caching

## 核心文件

- `block_manager.py` - 用 list 模拟显存块分配/释放
- `scheduler.py` - 最简单的 FCFS 调度器
- `engine.py` - 串联一切的主循环

## 运行

```bash
cd ../../tests
python test_v1_naive.py
```

## 限制

- 显存不足时直接抛异常（无优雅处理）
- 无法处理优先级
- 无法复用计算（Prefix Cache）

**下一步：** 升级到 v2_lru，引入 LRU eviction

