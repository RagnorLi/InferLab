// ============================================
// 每日习惯 - 4个必做 + 1个可选
// ============================================

export type HabitId = "leetcode" | "hpc" | "sysdesign" | "anki" | "band";

export interface HabitDef {
  id: HabitId;
  label: string;
  timeSlot: "morning" | "afternoon" | "evening" | "anytime";
  gain: number;
  description: string;
  actionLabel: string;
  link?: string;
  optional?: boolean;  // 可选习惯
}

export const HABITS: HabitDef[] = [
  {
    id: "leetcode",
    label: "LeetCode",
    timeSlot: "morning",
    gain: 20,
    description: "上午：每日一题或精选 2 题 (Medium+)",
    actionLabel: "Go to LeetCode",
    link: "https://leetcode.cn/problemset/all/",
  },
  {
    id: "hpc",
    label: "HPC / AI 推理",
    timeSlot: "afternoon",
    gain: 30,
    description: "下午：PyTorch → vLLM → SGLang → CUDA",
    actionLabel: "Open Docs",
    link: "https://pytorch.org/docs/stable/index.html",
  },
  {
    id: "sysdesign",
    label: "System Design",
    timeSlot: "evening",
    gain: 20,
    description: "晚上：系统设计学习",
    actionLabel: "Learn Design",
    link: "https://github.com/donnemartin/system-design-primer",
  },
  {
    id: "anki",
    label: "Flashcard",
    timeSlot: "anytime",
    gain: 10,
    description: "每日：清理待复习卡片",
    actionLabel: "Open Anki",
  },
  {
    id: "band",
    label: "Band 总结",
    timeSlot: "anytime",
    gain: 15,
    description: "可选：发布学习总结到博客/公众号",
    actionLabel: "记录发布",
    optional: true,
  },
];

// 必做习惯
export const REQUIRED_HABITS = HABITS.filter(h => !h.optional);
// 每日必做最大: 20 + 30 + 20 + 10 = 80m
export const MAX_DAILY_GAIN = REQUIRED_HABITS.reduce((sum, h) => sum + h.gain, 0);

// ============================================
// 每日记录
// ============================================

export interface DailyRecord {
  date: string;
  completed: Record<string, boolean>;
  logs?: Record<string, string>;
}

// ============================================
// 营地/里程碑
// ============================================

export interface Milestone {
  id: string;
  title: string;
  description: string;
  altitude: number;
  coords: [number, number, number];
}

export const DEFAULT_MILESTONES: Milestone[] = [
  { id: "basecamp", title: "Basecamp: 起点", description: "环境就绪，计划制定", altitude: 0, coords: [0,0,0] },
  { id: "algo", title: "Camp 1: 算法基石", description: "LeetCode 习惯养成", altitude: 500, coords: [0,0,0] },
  { id: "pytorch", title: "Camp 2: PyTorch", description: "框架掌握，模型训练", altitude: 1500, coords: [0,0,0] },
  { id: "inference", title: "Camp 3: 推理引擎", description: "vLLM/SGLang 部署", altitude: 3000, coords: [0,0,0] },
  { id: "cuda", title: "Summit: CUDA 巅峰", description: "Kernel优化，年薪百万", altitude: 5000, coords: [0,0,0] },
];

// ============================================
// App State
// ============================================

export interface AppState {
  milestones: Milestone[];
  history: DailyRecord[];
}

export const defaultState: AppState = {
  milestones: DEFAULT_MILESTONES,
  history: [],
};

// ============================================
// Helpers
// ============================================

export const getTodayStr = () => new Date().toISOString().slice(0, 10);

export const calculateAltitude = (history: DailyRecord[]) => {
  let total = 0;
  history.forEach((day) => {
    HABITS.forEach((habit) => {
      if (day.completed[habit.id]) {
        total += habit.gain;
      }
    });
  });
  return total;
};

export const getTimeSlotLabel = (slot: HabitDef["timeSlot"]) => {
  return { morning: "上午", afternoon: "下午", evening: "晚上", anytime: "每日" }[slot];
};
