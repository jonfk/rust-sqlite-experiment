
CREATE TABLE IF NOT EXISTS tasks (
       id INTEGER primary key NOT NULL,
       status TEXT NOT NULL DEFAULT 'WAITING',
       errors TEXT
);

PRAGMA journal_mode=WAL;
PRAGMA busy_timeout=6000;
