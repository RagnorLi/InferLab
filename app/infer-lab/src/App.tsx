import { useState, useMemo } from "react";
import "./App.css";
import { Mountain3D } from "./components/Mountain3D";
import { TodayPanel } from "./components/TodayPanel";
import { useAppData } from "./state/useAppData";
import { DailyRecord, calculateAltitude } from "./models/schema";

// 种子随机数生成器（保证一致性）
const seededRandom = (seed: number) => {
  const x = Math.sin(seed * 9999) * 10000;
  return x - Math.floor(x);
};

// Demo data generator - 使用种子随机确保一致
const generateDemoHistory = (): DailyRecord[] => {
  const history: DailyRecord[] = [];
  const today = new Date();
  
  for (let i = 0; i < 60; i++) {
    const d = new Date(today);
    d.setDate(d.getDate() - (60 - i));
    const dateStr = d.toISOString().slice(0, 10);
    
    const completed: Record<string, boolean> = {};
    if (seededRandom(i * 4 + 1) > 0.2) completed["leetcode"] = true;
    if (seededRandom(i * 4 + 2) > 0.4) completed["hpc"] = true;
    if (seededRandom(i * 4 + 3) > 0.5) completed["sysdesign"] = true;
    if (seededRandom(i * 4 + 4) > 0.6) completed["anki"] = true;

    history.push({ date: dateStr, completed });
  }
  return history;
};

function App() {
  const { state, currentAltitude, getRecordForDate, actions, dbConnected, loading } = useAppData();
  const [isDemoMode, setIsDemoMode] = useState(false);
  const [panelOpen, setPanelOpen] = useState(true);

  // Demo Data
  const demoHistory = useMemo(() => isDemoMode ? generateDemoHistory() : [], [isDemoMode]);
  const activeHistory = isDemoMode ? demoHistory : state.history;
  const activeAltitude = isDemoMode ? calculateAltitude(demoHistory) : currentAltitude;

  // Milestone info
  const nextMilestone = state.milestones.find(m => m.altitude > activeAltitude);
  const prevMilestone = state.milestones.filter(m => m.altitude <= activeAltitude).pop();
  
  const progressPercent = nextMilestone 
    ? ((activeAltitude - (prevMilestone?.altitude || 0)) / (nextMilestone.altitude - (prevMilestone?.altitude || 0))) * 100
    : 100;

  return (
    <div className="app">
      {/* 3D Mountain Background */}
      <div className="mountain-layer">
        <Mountain3D
          milestones={state.milestones}
          currentAltitude={activeAltitude}
        />
      </div>

      {/* UI Overlay */}
      <div className="ui-overlay">
        {/* Top Bar */}
        <header className="top-bar">
          <div className="brand">
            <span className="brand-icon">⛰️</span>
            <span className="brand-text">InferLab</span>
            <span className={`db-status ${dbConnected ? 'connected' : 'local'}`} title={dbConnected ? 'PostgreSQL已连接' : '本地存储模式'}>
              {dbConnected ? '🟢' : '🟡'}
            </span>
          </div>

          <div className="altitude-badge">
            <div className="altitude-number">{activeAltitude.toLocaleString()}</div>
            <div className="altitude-unit">米</div>
          </div>

          <button 
            className={`demo-toggle ${isDemoMode ? 'active' : ''}`}
            onClick={() => setIsDemoMode(!isDemoMode)}
          >
            {isDemoMode ? "退出预览" : "预览进度"}
          </button>
        </header>

        {/* Current Target Card - Fixed at bottom right */}
        <div className="target-card">
          <div className="target-header">
            <span className="target-label">下一营地</span>
            <span className="target-distance">
              {nextMilestone ? `还需 ${nextMilestone.altitude - activeAltitude}m` : "已登顶!"}
            </span>
          </div>
          {nextMilestone ? (
            <>
              <div className="target-name">{nextMilestone.title}</div>
              <div className="target-progress">
                <div className="progress-fill" style={{ width: `${progressPercent}%` }} />
              </div>
            </>
          ) : (
            <div className="target-name">🏆 巅峰已至</div>
          )}
        </div>

        {/* Floating Panel Toggle */}
        <button 
          className={`panel-toggle ${panelOpen ? 'open' : ''}`}
          onClick={() => setPanelOpen(!panelOpen)}
        >
          {panelOpen ? '✕' : '📋'}
        </button>

        {/* Today Panel - Floating Card */}
        <div className={`floating-panel ${panelOpen ? 'open' : ''}`}>
          <TodayPanel
            history={activeHistory}
            getRecordForDate={getRecordForDate}
            toggleHabit={actions.toggleHabit}
            logHabit={actions.logHabit}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
