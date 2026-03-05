-- Create the calculations table matching the Python SQLModel schema.
CREATE TABLE IF NOT EXISTS calculation (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    operation TEXT NOT NULL,
    a REAL NOT NULL,
    b REAL NOT NULL,
    result REAL NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
