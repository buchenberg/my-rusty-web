use crate::errors::AppError;
use crate::models::route::Route;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::sync::Mutex;

pub struct Database {
    pub connection: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &str) -> SqliteResult<Self> {
        let connection = Connection::open(db_path)?;
        Ok(Database {
            connection: Mutex::new(connection),
        })
    }

    pub fn init(&self) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS routes (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                is_enabled BOOLEAN NOT NULL,
                method TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_route(&self, route: &Route) -> Result<i64, AppError> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "INSERT INTO routes (name, path, is_enabled, method) VALUES (?1, ?2, ?3, ?4)",
            params![route.name, route.path, route.is_enabled, route.method],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_route(&self, id: i64) -> Result<Route, AppError> {
        let conn = self.connection.lock().unwrap();
        conn.query_row(
            "SELECT id, name, path, is_enabled, method FROM routes WHERE id = ?1",
            params![id],
            |row| {
                Ok(Route {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    path: row.get(2)?,
                    is_enabled: row.get(3)?,
                    method: row.get(4)?,
                })
            },
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound,
            e => AppError::DatabaseError(e),
        })
    }

    pub fn get_all_routes(&self) -> Result<Vec<Route>, AppError> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, path, is_enabled, method FROM routes")?;
        
        let routes = stmt
            .query_map([], |row| {
                Ok(Route {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    path: row.get(2)?,
                    is_enabled: row.get(3)?,
                    method: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(routes)
    }

    pub fn update_route(&self, id: i64, route: &Route) -> Result<bool, AppError> {
        let conn = self.connection.lock().unwrap();
        let updated = conn.execute(
            "UPDATE routes SET name = ?1, path = ?2, is_enabled = ?3, method = ?4 WHERE id = ?5",
            params![route.name, route.path, route.is_enabled, route.method, id],
        )?;
        Ok(updated > 0)
    }

    pub fn delete_route(&self, id: i64) -> Result<bool, AppError> {
        let conn = self.connection.lock().unwrap();
        let deleted = conn.execute("DELETE FROM routes WHERE id = ?1", params![id])?;
        Ok(deleted > 0)
    }
}