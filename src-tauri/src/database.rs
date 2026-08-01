use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

use rusqlite::{Connection, Result, Row, params};

use crate::projects::{Project, ScanRoot};

const DATABASE_VERSION: i64 = 2;

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
        let mut projects = {
            let mut statement = connection
                .prepare(
                    "SELECT id, path, name, favorite, last_seen_at
                     FROM projects
                     ORDER BY favorite DESC, name COLLATE NOCASE",
                )
                .map_err(|error| error.to_string())?;
            statement
                .query_map([], project_from_row)
                .map_err(|error| error.to_string())?
                .collect::<Result<Vec<_>>>()
                .map_err(|error| error.to_string())?
        };
        for project in &mut projects {
            project.tags = project_tags(&connection, project.id)?;
        }

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
        let mut project = connection
            .query_row(
                "SELECT id, path, name, favorite, last_seen_at
                 FROM projects WHERE path = ?1",
                [&path],
                project_from_row,
            )
            .map_err(|error| error.to_string())?;
        project.tags = project_tags(&connection, project.id)?;
        Ok(project)
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

    pub fn set_project_tags(
        &self,
        id: i64,
        tags: Vec<String>,
    ) -> std::result::Result<Project, String> {
        let mut tags = tags
            .into_iter()
            .map(|tag| tag.trim().to_owned())
            .collect::<Vec<_>>();
        if tags
            .iter()
            .any(|tag| tag.is_empty() || tag.chars().count() > 20)
        {
            return Err("每个标签须为 1 至 20 个字符".to_owned());
        }
        let mut seen = HashSet::new();
        tags.retain(|tag| seen.insert(tag.to_lowercase()));
        if tags.len() > 8 {
            return Err("每个项目最多 8 个标签".to_owned());
        }

        let mut connection = self.connection()?;
        let transaction = connection
            .transaction()
            .map_err(|error| error.to_string())?;
        transaction
            .execute("DELETE FROM project_tags WHERE project_id = ?1", [id])
            .map_err(|error| error.to_string())?;
        for tag in &tags {
            transaction
                .execute(
                    "INSERT INTO project_tags (project_id, name) VALUES (?1, ?2)",
                    params![id, tag],
                )
                .map_err(|error| error.to_string())?;
        }
        transaction.commit().map_err(|error| error.to_string())?;
        drop(connection);

        self.list_projects()?
            .into_iter()
            .find(|project| project.id == id)
            .ok_or_else(|| "项目不存在".to_owned())
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

fn project_from_row(row: &Row<'_>) -> Result<Project> {
    Ok(Project {
        id: row.get(0)?,
        path: row.get(1)?,
        name: row.get(2)?,
        favorite: row.get(3)?,
        tags: Vec::new(),
        last_seen_at: row.get(4)?,
    })
}

fn project_tags(
    connection: &Connection,
    project_id: i64,
) -> std::result::Result<Vec<String>, String> {
    let mut statement = connection
        .prepare("SELECT name FROM project_tags WHERE project_id = ?1 ORDER BY rowid")
        .map_err(|error| error.to_string())?;
    statement
        .query_map([project_id], |row| row.get(0))
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>>>()
        .map_err(|error| error.to_string())
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

        CREATE TABLE IF NOT EXISTS project_tags (
            project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
            name TEXT NOT NULL COLLATE NOCASE CHECK (length(name) BETWEEN 1 AND 20),
            PRIMARY KEY (project_id, name)
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

    fn test_database(name: &str) -> (PathBuf, Database) {
        let directory = std::env::temp_dir().join(format!(
            "repo-radar-{name}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        (directory.clone(), Database::open(&directory).unwrap())
    }

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
                 WHERE type = 'table'
                 AND name IN ('projects', 'project_tags', 'scan_roots', 'settings')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(version, 2);
        assert_eq!(table_count, 4);
    }

    #[test]
    fn migration_is_repeatable() {
        let connection = Connection::open_in_memory().unwrap();

        migrate(&connection).unwrap();
        migrate(&connection).unwrap();
    }

    #[test]
    fn project_tags_are_normalized_and_persisted() {
        let (directory, database) = test_database("project-tags");
        let project = database.upsert_project(&directory, "RepoRadar").unwrap();

        let updated = database
            .set_project_tags(
                project.id,
                vec![" frontend ".into(), "FRONTEND".into(), "rust".into()],
            )
            .unwrap();

        assert_eq!(updated.tags, vec!["frontend", "rust"]);
        assert_eq!(database.list_projects().unwrap()[0].tags, updated.tags);

        let cleared = database.set_project_tags(project.id, Vec::new()).unwrap();
        assert!(cleared.tags.is_empty());
        drop(database);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn project_tags_reject_invalid_values() {
        let (directory, database) = test_database("invalid-project-tags");
        let project = database.upsert_project(&directory, "RepoRadar").unwrap();

        assert!(
            database
                .set_project_tags(project.id, vec![" ".into()])
                .is_err()
        );
        assert!(
            database
                .set_project_tags(project.id, vec!["x".repeat(21)])
                .is_err()
        );
        assert!(
            database
                .set_project_tags(project.id, (0..9).map(|index| index.to_string()).collect(),)
                .is_err()
        );
        drop(database);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn project_upsert_preserves_tags() {
        let (directory, database) = test_database("project-tag-upsert");
        let project = database.upsert_project(&directory, "RepoRadar").unwrap();
        database
            .set_project_tags(project.id, vec!["desktop".into()])
            .unwrap();

        let updated = database
            .upsert_project(&directory, "RepoRadar Next")
            .unwrap();

        assert_eq!(updated.tags, vec!["desktop"]);
        drop(database);
        fs::remove_dir_all(directory).unwrap();
    }
}
