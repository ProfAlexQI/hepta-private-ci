//! Shared SQLite connection configuration.

use crate::LOGS_DB_FILENAME;
use crate::STATE_DB_FILENAME;
use crate::THREAD_HISTORY_DB_FILENAME;
use log::LevelFilter;
use sqlx::ConnectOptions;
use sqlx::Error;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteAutoVacuum;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqliteSynchronous;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Clone, Copy)]
struct RuntimeDbSpec {
    label: &'static str,
    filename: &'static str,
}

impl RuntimeDbSpec {
    fn path(self, sqlite_home: &Path) -> PathBuf {
        sqlite_home.join(self.filename)
    }
}

const STATE_DB: RuntimeDbSpec = RuntimeDbSpec {
    label: "state DB",
    filename: STATE_DB_FILENAME,
};

const LOGS_DB: RuntimeDbSpec = RuntimeDbSpec {
    label: "log DB",
    filename: LOGS_DB_FILENAME,
};

const THREAD_HISTORY_DB: RuntimeDbSpec = RuntimeDbSpec {
    label: "thread history DB",
    filename: THREAD_HISTORY_DB_FILENAME,
};

const RUNTIME_DBS: [RuntimeDbSpec; 3] = [STATE_DB, LOGS_DB, THREAD_HISTORY_DB];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeDbPath {
    pub label: &'static str,
    pub path: PathBuf,
}

/// Resolved configuration shared by all Hepta SQLite connections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SqliteConfig {
    sqlite_home: PathBuf,
}

impl SqliteConfig {
    pub fn from_sqlite_home(sqlite_home: PathBuf) -> Self {
        Self { sqlite_home }
    }

    pub fn new_for_testing(sqlite_home: PathBuf) -> Self {
        Self::from_sqlite_home(sqlite_home)
    }

    pub fn home(&self) -> &Path {
        self.sqlite_home.as_path()
    }

    pub fn state_db_path(&self) -> PathBuf {
        STATE_DB.path(self.home())
    }

    pub fn logs_db_path(&self) -> PathBuf {
        LOGS_DB.path(self.home())
    }

    pub fn thread_history_db_path(&self) -> PathBuf {
        THREAD_HISTORY_DB.path(self.home())
    }

    pub fn runtime_db_paths(&self) -> Vec<RuntimeDbPath> {
        RUNTIME_DBS
            .iter()
            .map(|spec| RuntimeDbPath {
                label: spec.label,
                path: spec.path(self.home()),
            })
            .collect()
    }

    /// Open a writable Hepta SQLite database, creating it if necessary.
    pub async fn open_read_write_pool(&self, path: &Path) -> Result<SqlitePool, Error> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal)
            .auto_vacuum(SqliteAutoVacuum::Incremental)
            .busy_timeout(Duration::from_secs(5))
            .log_statements(LevelFilter::Off);
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
    }

    /// Open an existing Hepta SQLite database without creating or modifying it.
    pub async fn open_read_only_pool(&self, path: &Path) -> Result<SqlitePool, Error> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(false)
            .read_only(true)
            .log_statements(LevelFilter::Off);
        SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
    }
}

impl From<PathBuf> for SqliteConfig {
    fn from(sqlite_home: PathBuf) -> Self {
        Self::from_sqlite_home(sqlite_home)
    }
}

impl From<&Path> for SqliteConfig {
    fn from(sqlite_home: &Path) -> Self {
        Self::from_sqlite_home(sqlite_home.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::RuntimeDbPath;
    use super::SqliteConfig;
    use crate::LOGS_DB_FILENAME;
    use crate::STATE_DB_FILENAME;
    use crate::THREAD_HISTORY_DB_FILENAME;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn sqlite_config_derives_every_runtime_path_from_one_home() {
        let sqlite_home = PathBuf::from("/tmp/hepta-sqlite-home");
        let config = SqliteConfig::new_for_testing(sqlite_home.clone());

        assert_eq!(config.home(), sqlite_home.as_path());
        assert_eq!(config.state_db_path(), sqlite_home.join(STATE_DB_FILENAME));
        assert_eq!(config.logs_db_path(), sqlite_home.join(LOGS_DB_FILENAME));
        assert_eq!(
            config.thread_history_db_path(),
            sqlite_home.join(THREAD_HISTORY_DB_FILENAME)
        );
        assert_eq!(
            config.runtime_db_paths(),
            vec![
                RuntimeDbPath {
                    label: "state DB",
                    path: sqlite_home.join(STATE_DB_FILENAME),
                },
                RuntimeDbPath {
                    label: "log DB",
                    path: sqlite_home.join(LOGS_DB_FILENAME),
                },
                RuntimeDbPath {
                    label: "thread history DB",
                    path: sqlite_home.join(THREAD_HISTORY_DB_FILENAME),
                },
            ]
        );
    }
}
