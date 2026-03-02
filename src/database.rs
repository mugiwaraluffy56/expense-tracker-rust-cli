use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".ept").join("expenses.db")
}

pub fn open_or_fail() -> Connection {
    let path = db_path();
    if !path.exists() {
        eprintln!("Database not found. Run 'ept init' first.");
        std::process::exit(1);
    }
    Connection::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open database: {}", e);
        std::process::exit(1);
    })
}

pub fn init() -> Result<()> {
    let path = db_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }
    let conn = Connection::open(&path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS expenses (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            category   TEXT NOT NULL,
            amount     REAL NOT NULL,
            note       TEXT,
            created_at TEXT NOT NULL
        );",
    )?;
    println!("Database initialized at {}", path.display());
    Ok(())
}
