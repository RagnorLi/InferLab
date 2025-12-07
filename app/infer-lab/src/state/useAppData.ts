import { useEffect, useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  AppState,
  DailyRecord,
  defaultState,
  getTodayStr,
  HabitId,
  HABITS,
  Milestone,
  DEFAULT_MILESTONES,
} from "../models/schema";

// ============================================
// Types from Rust backend
// ============================================

interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}

interface HabitFromDb {
  id: number;
  key: string;
  label: string;
  gain_meters: number;
  description: string | null;
  action_label: string | null;
  action_link: string | null;
  sort_order: number;
  is_active: boolean;
}

interface MilestoneFromDb {
  id: number;
  key: string;
  title: string;
  description: string | null;
  altitude: number;
  sort_order: number;
}

interface DailyRecordFromDb {
  habit_key: string;
  record_date: string;
  completed: boolean;
  log_text: string | null;
}

interface AppDataFromDb {
  habits: HabitFromDb[];
  milestones: MilestoneFromDb[];
  history: DailyRecordFromDb[];
  current_altitude: number;
}

// ============================================
// Fallback to localStorage when DB unavailable
// ============================================

const STORAGE_KEY = "infer-lab.v2";

const loadLocalState = (): AppState => {
  if (typeof window === "undefined") return defaultState;
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) return defaultState;
    return { ...defaultState, ...JSON.parse(raw) };
  } catch {
    return defaultState;
  }
};

const saveLocalState = (state: AppState) => {
  if (typeof window !== "undefined") {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  }
};

// ============================================
// Convert DB data to frontend format
// ============================================

function convertDbToFrontend(dbData: AppDataFromDb): { 
  milestones: Milestone[], 
  history: DailyRecord[], 
  altitude: number 
} {
  // Convert milestones
  const milestones: Milestone[] = dbData.milestones.map(m => ({
    id: m.key,
    title: m.title,
    description: m.description || "",
    altitude: m.altitude,
    coords: [0, 0, 0] as [number, number, number], // Not used in 2D version
  }));

  // Group history by date
  const historyByDate = new Map<string, DailyRecord>();
  
  for (const record of dbData.history) {
    const date = record.record_date;
    
    if (!historyByDate.has(date)) {
      historyByDate.set(date, {
        date,
        completed: {},
        logs: {},
      });
    }
    
    const dayRecord = historyByDate.get(date)!;
    dayRecord.completed[record.habit_key] = record.completed;
    if (record.log_text) {
      dayRecord.logs = dayRecord.logs || {};
      dayRecord.logs[record.habit_key] = record.log_text;
    }
  }

  return {
    milestones,
    history: Array.from(historyByDate.values()),
    altitude: dbData.current_altitude,
  };
}

// ============================================
// Main Hook
// ============================================

export const useAppData = () => {
  const [state, setState] = useState<AppState>(() => loadLocalState());
  const [dbConnected, setDbConnected] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Initialize database and load data
  useEffect(() => {
    const init = async () => {
      try {
        setLoading(true);
        setError(null);

        // Try to connect to database
        const initResult = await invoke<ApiResponse<string>>("init_database");
        
        if (initResult.success) {
          setDbConnected(true);
          
          // Load data from database
          const dataResult = await invoke<ApiResponse<AppDataFromDb>>("load_app_data");
          
          if (dataResult.success && dataResult.data) {
            const { milestones, history, altitude } = convertDbToFrontend(dataResult.data);
            setState({
              milestones: milestones.length > 0 ? milestones : DEFAULT_MILESTONES,
              history,
            });
          }
        } else {
          console.warn("Database connection failed, using localStorage:", initResult.error);
          setDbConnected(false);
        }
      } catch (err) {
        console.warn("Database unavailable, using localStorage:", err);
        setDbConnected(false);
      } finally {
        setLoading(false);
      }
    };

    init();
  }, []);

  // Save to localStorage as backup
  useEffect(() => {
    if (!loading) {
      saveLocalState(state);
    }
  }, [state, loading]);

  // Toggle habit
  const toggleHabit = useCallback(async (date: string, habitId: HabitId) => {
    if (dbConnected) {
      try {
        const result = await invoke<ApiResponse<boolean>>("toggle_habit", {
          habitKey: habitId,
          date,
        });
        
        if (result.success) {
          // Reload data from database
          const dataResult = await invoke<ApiResponse<AppDataFromDb>>("load_app_data");
          if (dataResult.success && dataResult.data) {
            const { milestones, history } = convertDbToFrontend(dataResult.data);
            setState(prev => ({
              ...prev,
              milestones: milestones.length > 0 ? milestones : prev.milestones,
              history,
            }));
          }
          return;
        }
      } catch (err) {
        console.error("Database toggle failed:", err);
      }
    }
    
    // Fallback to local state
    setState((prev) => {
      const existingIndex = prev.history.findIndex((h) => h.date === date);
      let newHistory = [...prev.history];

      if (existingIndex >= 0) {
        const record = newHistory[existingIndex];
        newHistory[existingIndex] = {
          ...record,
          completed: {
            ...record.completed,
            [habitId]: !record.completed[habitId],
          },
        };
      } else {
        newHistory.push({
          date,
          completed: { [habitId]: true },
        });
      }

      return { ...prev, history: newHistory };
    });
  }, [dbConnected]);

  // Save habit log
  const logHabit = useCallback(async (date: string, habitId: HabitId, text: string) => {
    if (dbConnected) {
      try {
        const result = await invoke<ApiResponse<null>>("save_habit_log", {
          habitKey: habitId,
          date,
          logText: text,
        });
        
        if (result.success) {
          // Reload data from database
          const dataResult = await invoke<ApiResponse<AppDataFromDb>>("load_app_data");
          if (dataResult.success && dataResult.data) {
            const { history } = convertDbToFrontend(dataResult.data);
            setState(prev => ({ ...prev, history }));
          }
          return;
        }
      } catch (err) {
        console.error("Database log save failed:", err);
      }
    }
    
    // Fallback to local state
    setState((prev) => {
      const existingIndex = prev.history.findIndex((h) => h.date === date);
      let newHistory = [...prev.history];

      if (existingIndex >= 0) {
        const record = newHistory[existingIndex];
        newHistory[existingIndex] = {
          ...record,
          logs: { ...record.logs, [habitId]: text },
        };
      } else {
        newHistory.push({
          date,
          completed: {},
          logs: { [habitId]: text },
        });
      }

      return { ...prev, history: newHistory };
    });
  }, [dbConnected]);

  // Calculate current altitude
  const currentAltitude = state.history.reduce((total, day) => {
    return total + HABITS.reduce((dayTotal, habit) => {
      return dayTotal + (day.completed[habit.id] ? habit.gain : 0);
    }, 0);
  }, 0);

  const getRecordForDate = useCallback((date: string) => {
    return state.history.find((h) => h.date === date);
  }, [state.history]);

  return {
    state,
    currentAltitude,
    getRecordForDate,
    dbConnected,
    loading,
    error,
    actions: {
      toggleHabit,
      logHabit,
    },
  };
};
