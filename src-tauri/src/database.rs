use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

use rusqlite::{Connection, Result};

use crate::projects::{Project, ScanRoot};

const DATABASE_VERSION: i64 = 1;

pub struct Database {
    connection: Mutex<Connection>,
}

impl Database {
    pub fn open(app_data_dir: &Path) -> std::result::Result<Self, Box<dyn Error>> {
        fs::create_dir_all(app_data_dir)?;
        let connection = Connection::open(database_path(app_data_dir))?;
        migrate(&connection)?;

        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn is_ready(&self) -> bool {
        self.connection()
            .is_ok_and(|connection| connection.is_autocommit())
    }

    pub fn list_projects(&self) -> std::result::Result<Vec<Project>, String> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, path, name, favorite, last_seen_at
                 FROM projects
                 ORDER BY favorite DESC, name COLLATE NOCASE",
            )
            .map_err(|error| error.to_string())?;
        let projects = statement
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    name: row.get(2)?,
                    favorite: row.get(3)?,
                    last_seen_at: row.get(4)?,
                })
            })
            .map_err(|error| error.to_string())?
            .collect::<Result<Vec<_>>>()
            .map_err(|error| error.to_string())?;

        Ok(projects)
    }

    pub fn upsert_project(&self, path: &Path, name: &str) -> std::result::Result<Project, String> {
        let path = path.to_string_lossy().into_owned();
        let connection = self.connection()?;
        connection
            .execute(
                "INSERT INTO projects (path, name)
                 VALUES (?1, ?2)
                 ON CONFLICT(path) DO UPDATE SET
                    name = excluded.name,
                    last_seen_at = CURRENT_TIMESTAMP",
                (&path, name),
            )
            .map_err(|error| error.to_string())?;
        connection
            .query_row(
                "SELECT id, path, name, favorite, last_seen_at
                 FROM projects WHERE path = ?1",
                [&path],
                |row| {
                    Ok(Project {
                        id: row.get(0)?,
                        path: row.get(1)?,
                        name: row.get(2)?,
                        favorite: row.get(3)?,
                        last_seen_at: row.get(4)?,
                    })
                },
            )
            .map_err(|error| error.to_string())
    }

    pub fn set_project_favorite(&self, id: i64, favorite: bool) -> std::result::Result<(), String> {
        let connection = self.connection()?;
        connection
            .execute(
                "UPDATE projects SET favorite = ?1 WHERE id = ?2",
                (favorite, id),
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub fn list_scan_roots(&self) -> std::result::Result<Vec<ScanRoot>, String> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare(
                "SELECT id, path, max_depth, enabled
                 FROM scan_roots
                 WHERE enabled = 1
                 ORDER BY path COLLATE NOCASE",
            )
            .map_err(|error| error.to_string())?;
        let roots = statement
            .query_map([], |row| {
                Ok(ScanRoot {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    max_depth: row.get(2)?,
                    enabled: row.get(3)?,
                })
            })
            .map_err(|error| error.to_string())?
            .collect::<Result<Vec<_>>>()
            .map_err(|error| error.to_string())?;

        Ok(roots)
    }

    pub fn upsert_scan_root(
        &self,
        path: &Path,
        max_depth: u32,
    ) -> std::result::Result<ScanRoot, String> {
        let path = path.to_string_lossy().into_owned();
        let connection = self.connection()?;
        connection
            .execute(
                "INSERT INTO scan_roots (path, max_depth)
                 VALUES (?1, ?2)
                 ON CONFLICT(path) DO UPDATE SET
                    max_depth = excluded.max_depth,
                    enabled = 1",
                (&path, max_depth),
            )
            .map_err(|error| error.to_string())?;
        connection
            .query_row(
                "SELECT id, path, max_depth, enabled
                 FROM scan_roots WHERE path = ?1",
                [&path],
                |row| {
                    Ok(ScanRoot {
                        id: row.get(0)?,
                        path: row.get(1)?,
                        max_depth: row.get(2)?,
                        enabled: row.get(3)?,
                    })
                },
            )
            .map_err(|error| error.to_string())
    }

    fn connection(&self) -> std::result::Result<MutexGuard<'_, Connection>, String> {
        self.connection
            .lock()
            .map_err(|_| "database lock poisoned".to_owned())
    }
}

fn database_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("repo-radar.db")
}

fn migrate(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            favorite INTEGER NOT NULL DEFAULT 0 CHECK (favorite IN (0, 1)),
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_seen_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS scan_roots (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            max_depth INTEGER NOT NULL DEFAULT 8 CHECK (max_depth > 0),
            enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1))
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        ",
    )?;
    connection.pragma_update(None, "user_version", DATABASE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_creates_versioned_schema() {
        let connection = Connection::open_in_memory().unwrap();

        migrate(&connection).unwrap();

        let version: i64 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        let table_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master
                 WHERE type = 'table' AND name IN ('projects', 'scan_roots', 'settings')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(version, DATABASE_VERSION);
        assert_eq!(table_count, 3);
    }

    #[test]
    fn migration_is_repeatable() {
        let connection = Connection::open_in_memory().unwrap();

        migrate(&connection).unwrap();
        migrate(&connection).unwrap();
    }
}
