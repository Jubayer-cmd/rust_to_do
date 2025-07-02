use crate::Task;
use rusqlite::{params, Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                done BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn add_task(&self, title: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (title, done) VALUES (?1, ?2)",
            params![title, false],
        )?;
        println!("✅ Task '{}' added successfully!", title);
        Ok(())
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, done FROM tasks ORDER BY id")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        task_iter.collect()
    }

    pub fn mark_task_done(&self, task_id: i32) -> Result<()> {
        let updated = self
            .conn
            .execute("UPDATE tasks SET done = 1 WHERE id = ?1", params![task_id])?;
        if updated == 0 {
            println!("❌ Task with ID {} not found.", task_id);
        } else {
            println!("✅ Task marked as done!");
        }
        Ok(())
    }

    pub fn delete_task(&self, task_id: i32) -> Result<()> {
        let deleted = self
            .conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![task_id])?;
        if deleted == 0 {
            println!("❌ Task with ID {} not found.", task_id);
        } else {
            println!("🗑️ Task deleted successfully!");
            // Reset auto-increment counter to reuse IDs
            self.reset_auto_increment()?;
        }
        Ok(())
    }

    fn reset_auto_increment(&self) -> Result<()> {
        // Get the maximum ID from existing tasks
        let max_id: Option<i32> = self.conn.query_row(
            "SELECT MAX(id) FROM tasks",
            [],
            |row| row.get(0)
        ).unwrap_or(None);

        // Reset the auto-increment counter
        if let Some(max_id) = max_id {
            self.conn.execute(
                "UPDATE sqlite_sequence SET seq = ?1 WHERE name = 'tasks'",
                params![max_id]
            )?;
        } else {
            // If no tasks exist, reset to 0
            self.conn.execute(
                "UPDATE sqlite_sequence SET seq = 0 WHERE name = 'tasks'",
                []
            )?;
        }
        Ok(())
    }
}
