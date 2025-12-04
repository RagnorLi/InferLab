import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [message, setMessage] = useState("");

  useEffect(() => {
    // 调用 Rust 命令获取欢迎信息，体现 Tauri 的前后端通信精髓
    invoke<string>("get_welcome_message").then(setMessage);
  }, []);

  return (
    <main className="mountain-container">
      <div className="mountain">⛰️</div>
      <p className="welcome-text">{message}</p>
    </main>
  );
}

export default App;
