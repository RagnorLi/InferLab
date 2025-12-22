```plaintext

数据流：
  后端 db.rs (默认数据) 
    ↓
  lib.db (持久化)
    ↓
  Tauri API
    ↓
  前端 (空数组 → 加载 → 显示数据)
```

```plaintext
启动应用
  ↓
lib.db 存在？
  ├─ NO → 使用 db.rs 中的 get_default_events() → 写入 lib.db → 返回数据
  └─ YES → 直接读取 lib.db → 返回数据（忽略 db.rs 中的默认值）
```