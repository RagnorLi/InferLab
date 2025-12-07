import { useState, useMemo } from "react";
import { getTodayStr, HABITS, HabitId, DailyRecord, getTimeSlotLabel, REQUIRED_HABITS } from "../models/schema";
import { openUrl } from "@tauri-apps/plugin-opener";
import "./TodayPanel.css";

interface Props {
  history: DailyRecord[];
  getRecordForDate: (date: string) => DailyRecord | undefined;
  toggleHabit: (date: string, habitId: HabitId) => void;
  logHabit: (date: string, habitId: HabitId, text: string) => void;
}

type TabId = "today" | "history" | "stats";

const HABIT_ICONS: Record<HabitId, string> = {
  leetcode: "🧩",
  hpc: "⚡",
  sysdesign: "🏗️",
  anki: "🧠",
  band: "📝",
};

// 计算单日获得的海拔
const getDayGain = (record: DailyRecord) => {
  return HABITS.reduce((sum, h) => sum + (record.completed[h.id] ? h.gain : 0), 0);
};

// 计算连续打卡天数
const calculateStreak = (history: DailyRecord[]): number => {
  const today = new Date();
  let streak = 0;
  
  for (let i = 0; i <= 365; i++) {
    const d = new Date(today);
    d.setDate(d.getDate() - i);
    const dateStr = d.toISOString().slice(0, 10);
    
    const record = history.find(r => r.date === dateStr);
    const hasRequiredDone = record && REQUIRED_HABITS.some(h => record.completed[h.id]);
    
    if (hasRequiredDone) {
      streak++;
    } else if (i > 0) {
      // 今天可以还没打卡，但之前的天必须连续
      break;
    }
  }
  
  return streak;
};

// 生成最近N天的日期
const getRecentDates = (days: number): string[] => {
  const dates: string[] = [];
  const today = new Date();
  
  for (let i = 0; i < days; i++) {
    const d = new Date(today);
    d.setDate(d.getDate() - i);
    dates.push(d.toISOString().slice(0, 10));
  }
  
  return dates;
};

export function TodayPanel({ history, getRecordForDate, toggleHabit, logHabit }: Props) {
  const today = getTodayStr();
  const [activeTab, setActiveTab] = useState<TabId>("today");
  const [selectedDate, setSelectedDate] = useState(today);
  const [expandedId, setExpandedId] = useState<HabitId | null>(null);
  const [noteText, setNoteText] = useState("");

  const record = getRecordForDate(selectedDate);
  const streak = useMemo(() => calculateStreak(history), [history]);
  const recentDates = useMemo(() => getRecentDates(14), []);

  // 当前选中日期的统计
  const completedCount = REQUIRED_HABITS.filter(h => record?.completed?.[h.id]).length;
  const totalGain = HABITS.reduce((acc, h) => acc + (record?.completed?.[h.id] ? h.gain : 0), 0);

  // 周统计
  const weekStats = useMemo(() => {
    const last7 = getRecentDates(7);
    let totalDays = 0;
    let totalGain = 0;
    let perfectDays = 0;
    
    last7.forEach(date => {
      const r = history.find(h => h.date === date);
      if (r) {
        const gain = getDayGain(r);
        if (gain > 0) totalDays++;
        totalGain += gain;
        
        const allRequired = REQUIRED_HABITS.every(h => r.completed[h.id]);
        if (allRequired) perfectDays++;
      }
    });
    
    return { totalDays, totalGain, perfectDays };
  }, [history]);

  // 月统计
  const monthStats = useMemo(() => {
    const last30 = getRecentDates(30);
    let totalDays = 0;
    let totalGain = 0;
    
    last30.forEach(date => {
      const r = history.find(h => h.date === date);
      if (r) {
        const gain = getDayGain(r);
        if (gain > 0) totalDays++;
        totalGain += gain;
      }
    });
    
    return { totalDays, totalGain };
  }, [history]);

  const handleExpand = (id: HabitId) => {
    if (expandedId === id) {
      setExpandedId(null);
    } else {
      setExpandedId(id);
      setNoteText(record?.logs?.[id] || "");
    }
  };

  const handleAction = async (link?: string) => {
    if (link) {
      try {
        await openUrl(link);
      } catch {
        window.open(link, "_blank");
      }
    }
  };

  const handleSaveNote = (id: HabitId) => {
    logHabit(selectedDate, id, noteText);
    if (!record?.completed?.[id]) {
      toggleHabit(selectedDate, id);
    }
    setExpandedId(null);
  };

  const handleExport = () => {
    const data = {
      exportDate: new Date().toISOString(),
      history: history,
    };
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `inferlab-export-${today}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const requiredHabits = HABITS.filter(h => !h.optional);
  const optionalHabits = HABITS.filter(h => h.optional);

  return (
    <div className="today-panel">
      {/* Header with Streak */}
      <div className="panel-header">
        <div className="header-left">
          <h2 className="panel-title">
            {activeTab === "today" ? "今日攀登" : activeTab === "history" ? "攀登日志" : "数据统计"}
          </h2>
          <span className="panel-date">{selectedDate}</span>
        </div>
        <div className="header-right">
          {activeTab === "today" && (
            <>
              <span className="gain-value">+{totalGain}</span>
              <span className="gain-unit">m</span>
            </>
          )}
          {activeTab !== "stats" && (
            <div className="streak-badge" title="连续打卡天数">
              <span className="streak-icon">🔥</span>
              <span className="streak-count">{streak}</span>
            </div>
          )}
        </div>
      </div>

      {/* Tabs */}
      <div className="panel-tabs">
        <button 
          className={`tab-btn ${activeTab === "today" ? "active" : ""}`}
          onClick={() => { setActiveTab("today"); setSelectedDate(today); }}
        >
          今日
        </button>
        <button 
          className={`tab-btn ${activeTab === "history" ? "active" : ""}`}
          onClick={() => setActiveTab("history")}
        >
          历史
        </button>
        <button 
          className={`tab-btn ${activeTab === "stats" ? "active" : ""}`}
          onClick={() => setActiveTab("stats")}
        >
          统计
        </button>
      </div>

      {/* Today Tab */}
      {activeTab === "today" && (
        <>
          {/* Progress Ring */}
          <div className="progress-section">
            <svg className="progress-ring" viewBox="0 0 100 100">
              <circle className="ring-bg" cx="50" cy="50" r="42" />
              <circle 
                className="ring-fill" 
                cx="50" cy="50" r="42"
                style={{ 
                  strokeDasharray: `${(completedCount / REQUIRED_HABITS.length) * 264} 264`,
                }} 
              />
            </svg>
            <div className="progress-text">
              <span className="progress-count">{completedCount}</span>
              <span className="progress-total">/ {REQUIRED_HABITS.length}</span>
            </div>
          </div>

          {/* Habit List */}
          <div className="habit-list">
            {requiredHabits.map((habit) => (
              <HabitCard
                key={habit.id}
                habit={habit}
                record={record}
                expandedId={expandedId}
                noteText={noteText}
                onExpand={handleExpand}
                onToggle={() => toggleHabit(selectedDate, habit.id)}
                onAction={handleAction}
                onNoteChange={setNoteText}
                onSaveNote={() => handleSaveNote(habit.id)}
              />
            ))}
          </div>

          {/* Optional Section */}
          {optionalHabits.length > 0 && (
            <>
              <div className="optional-divider">
                <span>可选</span>
              </div>
              <div className="habit-list optional">
                {optionalHabits.map((habit) => (
                  <HabitCard
                    key={habit.id}
                    habit={habit}
                    record={record}
                    expandedId={expandedId}
                    noteText={noteText}
                    onExpand={handleExpand}
                    onToggle={() => toggleHabit(selectedDate, habit.id)}
                    onAction={handleAction}
                    onNoteChange={setNoteText}
                    onSaveNote={() => handleSaveNote(habit.id)}
                    isOptional
                  />
                ))}
              </div>
            </>
          )}
        </>
      )}

      {/* History Tab */}
      {activeTab === "history" && (
        <div className="history-section">
          <div className="history-list">
            {recentDates.map(date => {
              const r = history.find(h => h.date === date);
              const gain = r ? getDayGain(r) : 0;
              const isSelected = date === selectedDate;
              const isToday = date === today;
              const completedHabits = r ? HABITS.filter(h => r.completed[h.id]) : [];
              
              return (
                <div 
                  key={date}
                  className={`history-item ${isSelected ? "selected" : ""} ${gain === 0 ? "empty" : ""}`}
                  onClick={() => setSelectedDate(date)}
                >
                  <div className="history-date">
                    <span className="date-day">{date.slice(8)}</span>
                    <span className="date-month">{date.slice(5, 7)}月</span>
                    {isToday && <span className="today-tag">今</span>}
                  </div>
                  <div className="history-content">
                    {gain > 0 ? (
                      <>
                        <div className="history-habits">
                          {completedHabits.map(h => (
                            <span key={h.id} className="habit-dot" title={h.label}>
                              {HABIT_ICONS[h.id]}
                            </span>
                          ))}
                        </div>
                        <span className="history-gain">+{gain}m</span>
                      </>
                    ) : (
                      <span className="history-empty">无记录</span>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
          
          {/* Edit selected date */}
          {selectedDate !== today && (
            <div className="history-edit">
              <div className="edit-header">
                <span>编辑 {selectedDate}</span>
              </div>
              <div className="edit-habits">
                {HABITS.map(habit => {
                  const isDone = !!record?.completed?.[habit.id];
                  return (
                    <button
                      key={habit.id}
                      className={`edit-habit-btn ${isDone ? "done" : ""}`}
                      onClick={() => toggleHabit(selectedDate, habit.id)}
                    >
                      <span className="edit-icon">{HABIT_ICONS[habit.id]}</span>
                      <span className="edit-label">{habit.label}</span>
                      <span className="edit-check">{isDone ? "✓" : ""}</span>
                    </button>
                  );
                })}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Stats Tab */}
      {activeTab === "stats" && (
        <div className="stats-section">
          {/* Week Stats */}
          <div className="stats-card">
            <div className="stats-card-header">
              <span className="stats-card-icon">📅</span>
              <span className="stats-card-title">本周统计</span>
            </div>
            <div className="stats-grid">
              <div className="stat-item">
                <span className="stat-value">{weekStats.totalDays}</span>
                <span className="stat-label">活跃天数</span>
              </div>
              <div className="stat-item">
                <span className="stat-value">{weekStats.perfectDays}</span>
                <span className="stat-label">全勤天数</span>
              </div>
              <div className="stat-item highlight">
                <span className="stat-value">+{weekStats.totalGain}</span>
                <span className="stat-label">获得海拔</span>
              </div>
            </div>
          </div>

          {/* Month Stats */}
          <div className="stats-card">
            <div className="stats-card-header">
              <span className="stats-card-icon">📊</span>
              <span className="stats-card-title">本月统计</span>
            </div>
            <div className="stats-grid">
              <div className="stat-item">
                <span className="stat-value">{monthStats.totalDays}</span>
                <span className="stat-label">活跃天数</span>
              </div>
              <div className="stat-item highlight">
                <span className="stat-value">+{monthStats.totalGain}</span>
                <span className="stat-label">获得海拔</span>
              </div>
            </div>
          </div>

          {/* Streak Card */}
          <div className="stats-card streak-card">
            <div className="stats-card-header">
              <span className="stats-card-icon">🔥</span>
              <span className="stats-card-title">连续打卡</span>
            </div>
            <div className="streak-display">
              <span className="streak-number">{streak}</span>
              <span className="streak-unit">天</span>
            </div>
            <p className="streak-tip">
              {streak === 0 ? "今天开始你的第一天！" :
               streak < 7 ? "继续保持，马上就一周了！" :
               streak < 30 ? "太棒了！向30天进发！" :
               "传奇！你是真正的攀登者！"}
            </p>
          </div>

          {/* Export */}
          <button className="export-btn" onClick={handleExport}>
            <span>📤</span>
            <span>导出数据</span>
          </button>
        </div>
      )}
    </div>
  );
}

// 习惯卡片组件
function HabitCard({
  habit,
  record,
  expandedId,
  noteText,
  onExpand,
  onToggle,
  onAction,
  onNoteChange,
  onSaveNote,
  isOptional = false,
}: {
  habit: typeof HABITS[0];
  record: DailyRecord | undefined;
  expandedId: HabitId | null;
  noteText: string;
  onExpand: (id: HabitId) => void;
  onToggle: () => void;
  onAction: (link?: string) => void;
  onNoteChange: (text: string) => void;
  onSaveNote: () => void;
  isOptional?: boolean;
}) {
  const isDone = !!record?.completed?.[habit.id];
  const isExpanded = expandedId === habit.id;
  const hasLog = !!record?.logs?.[habit.id];

  return (
    <div className={`habit-card ${isDone ? "done" : ""} ${isExpanded ? "expanded" : ""} ${isOptional ? "optional" : ""}`}>
      <div className="habit-row" onClick={() => onExpand(habit.id)}>
        <button 
          className={`check-btn ${isDone ? "checked" : ""}`}
          onClick={(e) => {
            e.stopPropagation();
            onToggle();
          }}
        >
          {isDone ? "✓" : ""}
        </button>
        
        <div className="habit-icon">{HABIT_ICONS[habit.id]}</div>
        
        <div className="habit-content">
          <span className={`habit-name ${isDone ? "done" : ""}`}>
            {habit.label}
            <span className="time-slot">
              ({getTimeSlotLabel(habit.timeSlot)})
            </span>
          </span>
          <span className="habit-gain">+{habit.gain}m</span>
        </div>
        
        {hasLog && <span className="log-dot" />}
        
        <span className="expand-icon">{isExpanded ? "−" : "+"}</span>
      </div>

      {isExpanded && (
        <div className="habit-detail">
          <p className="habit-desc">{habit.description}</p>
          
          {habit.link && (
            <button className="action-btn" onClick={() => onAction(habit.link)}>
              {habit.actionLabel} →
            </button>
          )}
          
          <div className="log-section">
            <textarea
              className="log-input"
              placeholder={isOptional ? "记录发布内容/平台..." : "记录今日产出..."}
              value={noteText}
              onChange={(e) => onNoteChange(e.target.value)}
              rows={2}
            />
            <button className="save-btn" onClick={onSaveNote}>
              {isDone ? "更新记录" : "完成打卡"}
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
