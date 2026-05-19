CREATE TABLE IF NOT EXISTS rate_plan_definitions (
    plan_code TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    is_active INTEGER NOT NULL
);
