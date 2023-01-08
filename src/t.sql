CREATE TABLE IF NOT EXISTS records (
            id INTEGER AUTOINCREMENT PRIMARY KEY,
            name CHAR(50) NOT NULL,
            cents INTEGER NOT NULL,
            date TEXT,
            category CHAR(50),
            description TEXT
)