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
    allowed_permissions TEXT
);

-- New table for installed apps.
CREATE TABLE IF NOT EXISTS installed_apps (
    install_id INTEGER PRIMARY KEY AUTOINCREMENT,
    app_id TEXT NOT NULL,
    install_date TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(app_id) REFERENCES registered_apps(app_id)
);
