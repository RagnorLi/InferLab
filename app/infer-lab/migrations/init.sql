-- InferLab Database Schema v3
-- 爬山学习进度追踪应用
-- 
-- 结构说明：
-- 1. 每日习惯 (4个): leetcode, hpc, sysdesign, anki
-- 2. 营地/里程碑 (5个): 进度标记点
-- 3. Band总结 (可选): 用户自主创建，想发就发

-- ============================================
-- 习惯定义表 (Daily Habits) - 每日打卡
-- ============================================
CREATE TABLE IF NOT EXISTS habits (
    id SERIAL PRIMARY KEY,
    key VARCHAR(50) UNIQUE NOT NULL,
    label VARCHAR(100) NOT NULL,
    time_slot VARCHAR(50),                  -- morning/afternoon/evening/anytime
    gain_meters INTEGER NOT NULL DEFAULT 0,
    description TEXT,
    action_label VARCHAR(100),
    action_link TEXT,
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- 每日记录表 (Daily Records)
-- ============================================
CREATE TABLE IF NOT EXISTS daily_records (
    id SERIAL PRIMARY KEY,
    habit_id INTEGER NOT NULL REFERENCES habits(id) ON DELETE CASCADE,
    record_date DATE NOT NULL,
    completed BOOLEAN DEFAULT FALSE,
    log_text TEXT,                          -- 今日产出笔记
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    
    UNIQUE(habit_id, record_date)
);

-- ============================================
-- 营地/里程碑表 (Milestones)
-- ============================================
CREATE TABLE IF NOT EXISTS milestones (
    id SERIAL PRIMARY KEY,
    key VARCHAR(50) UNIQUE NOT NULL,
    title VARCHAR(100) NOT NULL,
    description TEXT,
    altitude INTEGER NOT NULL,              -- 所需海拔(米)
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- Band总结发布表 - 用户自主创建，可选功能
-- ============================================
CREATE TABLE IF NOT EXISTS band_posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    content TEXT,                           -- 文章内容/摘要
    tags TEXT[],                            -- 标签
    platforms TEXT[],                       -- 发布平台 ['wechat', 'zhihu', 'juejin']
    published_at TIMESTAMPTZ,
    status VARCHAR(20) DEFAULT 'draft',     -- draft/published
    external_links JSONB,                   -- {"wechat": "url", "zhihu": "url"}
    notes TEXT,                             -- 备注
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================
-- 索引
-- ============================================
CREATE INDEX IF NOT EXISTS idx_daily_records_date ON daily_records(record_date);
CREATE INDEX IF NOT EXISTS idx_daily_records_habit_date ON daily_records(habit_id, record_date);
CREATE INDEX IF NOT EXISTS idx_band_posts_status ON band_posts(status);

-- ============================================
-- 初始数据: 每日习惯 (4个必做 + 1个可选)
-- ============================================
INSERT INTO habits (key, label, time_slot, gain_meters, description, action_label, action_link, sort_order) VALUES
    ('leetcode', 'LeetCode', 'morning', 20, 
     '上午：每日一题或精选 2 题 (Medium+)', 
     'Go to LeetCode', 'https://leetcode.cn/problemset/all/', 1),
    
    ('hpc', 'HPC / AI 推理', 'afternoon', 30, 
     '下午：PyTorch → vLLM → SGLang → CUDA', 
     'Open Docs', 'https://pytorch.org/docs/stable/index.html', 2),
    
    ('sysdesign', 'System Design', 'evening', 20, 
     '晚上：系统设计学习', 
     'Learn Design', 'https://github.com/donnemartin/system-design-primer', 3),
    
    ('anki', 'Flashcard', 'anytime', 10, 
     '每日：清理待复习卡片', 
     'Open Anki', NULL, 4),
    
    ('band', 'Band 总结', 'anytime', 15, 
     '可选：发布学习总结到博客/公众号', 
     '记录发布', NULL, 5)
ON CONFLICT (key) DO UPDATE SET
    label = EXCLUDED.label,
    time_slot = EXCLUDED.time_slot,
    gain_meters = EXCLUDED.gain_meters,
    description = EXCLUDED.description,
    action_label = EXCLUDED.action_label,
    action_link = EXCLUDED.action_link,
    sort_order = EXCLUDED.sort_order;

-- ============================================
-- 初始数据: 营地/里程碑 (5个)
-- ============================================
INSERT INTO milestones (key, title, description, altitude, sort_order) VALUES
    ('basecamp', 'Basecamp: 起点', '环境就绪，计划制定', 0, 1),
    ('algo', 'Camp 1: 算法基石', 'LeetCode 习惯养成', 500, 2),
    ('pytorch', 'Camp 2: PyTorch', '框架掌握，模型训练', 1500, 3),
    ('inference', 'Camp 3: 推理引擎', 'vLLM/SGLang 部署', 3000, 4),
    ('cuda', 'Summit: CUDA 巅峰', 'Kernel优化，年薪百万', 5000, 5)
ON CONFLICT (key) DO UPDATE SET
    title = EXCLUDED.title,
    description = EXCLUDED.description,
    altitude = EXCLUDED.altitude,
    sort_order = EXCLUDED.sort_order;

-- ============================================
-- 触发器: 自动更新 updated_at
-- ============================================
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS habits_updated_at ON habits;
CREATE TRIGGER habits_updated_at
    BEFORE UPDATE ON habits
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

DROP TRIGGER IF EXISTS daily_records_updated_at ON daily_records;
CREATE TRIGGER daily_records_updated_at
    BEFORE UPDATE ON daily_records
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

DROP TRIGGER IF EXISTS band_posts_updated_at ON band_posts;
CREATE TRIGGER band_posts_updated_at
    BEFORE UPDATE ON band_posts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- ============================================
-- 视图: 当前海拔
-- ============================================
CREATE OR REPLACE VIEW v_current_altitude AS
SELECT 
    COALESCE(SUM(h.gain_meters), 0) AS total_altitude,
    COUNT(*) AS total_completions
FROM daily_records dr
JOIN habits h ON dr.habit_id = h.id
WHERE dr.completed = TRUE;

-- ============================================
-- 视图: 每日统计
-- ============================================
CREATE OR REPLACE VIEW v_daily_stats AS
SELECT 
    dr.record_date,
    COUNT(*) FILTER (WHERE dr.completed) AS completed_count,
    COALESCE(SUM(h.gain_meters) FILTER (WHERE dr.completed), 0) AS daily_gain
FROM daily_records dr
JOIN habits h ON dr.habit_id = h.id
GROUP BY dr.record_date
ORDER BY dr.record_date DESC;
