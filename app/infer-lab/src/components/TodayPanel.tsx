import { useState } from "react";
import { getTodayStr, HABITS, HabitId, DailyRecord, getTimeSlotLabel, REQUIRED_HABITS } from "../models/schema";
import { openUrl } from "@tauri-apps/plugin-opener";
import "./TodayPanel.css";

interface Props {
  getRecordForDate: (date: string) => DailyRecord | undefined;
  toggleHabit: (date: string, habitId: HabitId) => void;
  logHabit: (date: string, habitId: HabitId, text: string) => void;
}

const HABIT_ICONS: Record<HabitId, string> = {
  leetcode: "🧩",
  hpc: "⚡",
  sysdesign: "🏗️",
  anki: "🧠",
  band: "📝",
};

export function TodayPanel({ getRecordForDate, toggleHabit, logHabit }: Props) {
  const today = getTodayStr();
  const record = getRecordForDate(today);
  const [expandedId, setExpandedId] = useState<HabitId | null>(null);
  const [noteText, setNoteText] = useState("");

  // 只计算必做习惯的完成度
  const completedCount = REQUIRED_HABITS.filter(h => record?.completed?.[h.id]).length;
  const totalGain = HABITS.reduce((acc, h) => acc + (record?.completed?.[h.id] ? h.gain : 0), 0);

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
    logHabit(today, id, noteText);
    if (!record?.completed?.[id]) {
      toggleHabit(today, id);
    }
    setExpandedId(null);
  };

  const requiredHabits = HABITS.filter(h => !h.optional);
  const optionalHabits = HABITS.filter(h => h.optional);

  return (
    <div className="today-panel">
      {/* Header */}
      <div className="panel-header">
        <div className="header-left">
          <h2 className="panel-title">今日攀登</h2>
          <span className="panel-date">{today}</span>
        </div>
        <div className="header-right">
          <span className="gain-value">+{totalGain}</span>
          <span className="gain-unit">m</span>
        </div>
      </div>

      {/* Progress Ring - 只显示必做习惯进度 */}
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

      {/* 必做习惯列表 */}
      <div className="habit-list">
        {requiredHabits.map((habit) => (
          <HabitCard
            key={habit.id}
            habit={habit}
            record={record}
            expandedId={expandedId}
            noteText={noteText}
            onExpand={handleExpand}
            onToggle={() => toggleHabit(today, habit.id)}
            onAction={handleAction}
            onNoteChange={setNoteText}
            onSaveNote={() => handleSaveNote(habit.id)}
          />
        ))}
      </div>

      {/* 可选习惯 */}
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
                onToggle={() => toggleHabit(today, habit.id)}
                onAction={handleAction}
                onNoteChange={setNoteText}
                onSaveNote={() => handleSaveNote(habit.id)}
                isOptional
              />
            ))}
          </div>
        </>
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
