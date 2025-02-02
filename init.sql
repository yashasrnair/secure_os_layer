-- Table for storing user data.
CREATE TABLE IF NOT EXISTS user_data (
    id TEXT PRIMARY KEY,
    key TEXT NOT NULL,
    value TEXT NOT NULL
);

-- Table for registered apps.
CREATE TABLE IF NOT EXISTS registered_apps (
    app_id TEXT PRIMARY KEY,
    app_name TEXT NOT NULL,
    allowed_permissions TEXT  -- Could be a JSON string, CSV, etc.
);
