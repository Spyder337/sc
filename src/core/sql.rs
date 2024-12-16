use rusqlite::{params, Connection, Result};

pub struct Item {
    pub id: i32,
    pub var: String,
    pub val: Option<String>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Database { conn })
    }

    pub fn create_table(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE
  IF NOT EXISTS ShellCommanders (
    ID INTEGER PRIMARY KEY AUTOINCREMENT,
    VAR TEXT NOT NULL,
    VAL TEXT
  );

INSERT
OR IGNORE INTO ShellCommanders (VAR, VAL)
VALUES
  ('GIT_DIR', '~/Code'),
  ('GIT_AUTHOR', 'Author'),
  ('GIT_EMAIL', 'email.address@site.dom');",
            [],
        )?;
        Ok(())
    }

    pub fn insert_item(&self, var: &str, val: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO ShellCommanders (VAR, VAL) VALUES (?1, ?2)",
            params![var, val],
        )?;
        Ok(())
    }

    pub fn get_item_by_id(&self, id: i32) -> Result<Item> {
        self.conn.query_row(
            "SELECT ID, VAR, VAL FROM ShellCommanders WHERE ID = ?1",
            params![id],
            |row| {
                Ok(Item {
                    id: row.get(0)?,
                    var: row.get(1)?,
                    val: row.get(2)?,
                })
            },
        )
    }

    pub fn get_all_items(&self) -> Result<Vec<Item>> {
        let mut stmt = self
            .conn
            .prepare("SELECT ID, VAR, VAL FROM ShellCommanders")?;
        let items = stmt.query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                var: row.get(1)?,
                val: row.get(2)?,
            })
        })?;

        let mut results = Vec::new();
        for item in items {
            results.push(item?);
        }
        Ok(results)
    }

    pub fn update_item(&self, id: i32, var: &str, val: Option<&str>) -> Result<()> {
        self.conn.execute(
            "UPDATE ShellCommanders SET VAR = ?1, VAL = ?2 WHERE ID = ?3",
            params![var, val, id],
        )?;
        Ok(())
    }

    pub fn delete_item(&self, id: i32) -> Result<()> {
        self.conn
            .execute("DELETE FROM ShellCommanders WHERE ID = ?1", params![id])?;
        Ok(())
    }

    pub fn item_exists(&self, id: i32) -> Result<bool> {
        let exists: bool = self.conn.query_row(
            "SELECT EXISTS (SELECT 1 FROM ShellCommanders WHERE ID = ?1)",
            params![id],
            |row| row.get(0),
        )?;
        Ok(exists)
    }

    pub fn item_exists_by_var(&self, var: &str) -> Result<bool> {
        let exists: bool = self.conn.query_row(
            "SELECT EXISTS (SELECT 1 FROM ShellCommanders WHERE VAR = ?1)",
            params![var],
            |row| row.get(0),
        )?;
        Ok(exists)
    }
}
