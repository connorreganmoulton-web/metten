use crate::{Record, Status, WriterSettings};
use rusqlite::{Connection, params};
use std::{fs::File, path::Path, time::Duration};

pub(crate) struct Recorder {
    connection: Connection,
    _lock: File,
}

impl Recorder {
    pub(crate) fn open(path: &Path) -> Result<(Self, WriterSettings), String> {
        let lock = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path.with_extension("lock"))
            .map_err(|e| e.to_string())?;
        lock.try_lock().map_err(|_| {
            "This workspace is already open. Close the other Metten window and reopen.".to_string()
        })?;
        let connection = Connection::open(path).map_err(|e| e.to_string())?;
        connection
            .busy_timeout(Duration::from_millis(250))
            .map_err(|e| e.to_string())?;
        connection
            .execute_batch(
                "PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL;
            PRAGMA fullfsync=ON; PRAGMA checkpoint_fullfsync=ON;",
            )
            .map_err(|e| e.to_string())?;
        let settings = WriterSettings {
            journal_mode: connection
                .query_row("PRAGMA journal_mode", [], |r| r.get(0))
                .map_err(|e| e.to_string())?,
            synchronous: connection
                .query_row("PRAGMA synchronous", [], |r| r.get(0))
                .map_err(|e| e.to_string())?,
            fullfsync: connection
                .query_row("PRAGMA fullfsync", [], |r| r.get(0))
                .map_err(|e| e.to_string())?,
            checkpoint_fullfsync: connection
                .query_row("PRAGMA checkpoint_fullfsync", [], |r| r.get(0))
                .map_err(|e| e.to_string())?,
            sqlite_version: rusqlite::version().into(),
        };
        if settings.journal_mode != "wal"
            || settings.synchronous != 2
            || settings.fullfsync != 1
            || settings.checkpoint_fullfsync != 1
        {
            return Err("Required recording settings are unavailable on this workspace. Use a local Mac disk.".into());
        }
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .map_err(|e| e.to_string())?;
        if version > 1 {
            return Err(
                "This workspace was created by a newer Metten. Open it with that version.".into(),
            );
        }
        connection
            .execute_batch(
                "BEGIN IMMEDIATE;
            CREATE TABLE IF NOT EXISTS session_events (
                id INTEGER PRIMARY KEY,
                status TEXT NOT NULL CHECK(status IN ('stopped','running','paused')),
                tick INTEGER NOT NULL CHECK(tick >= 0),
                reason TEXT NOT NULL,
                recorded_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
            ) STRICT;
            PRAGMA user_version=1; COMMIT;",
            )
            .map_err(|e| e.to_string())?;
        Ok((
            Self {
                connection,
                _lock: lock,
            },
            settings,
        ))
    }

    pub(crate) fn latest(&self) -> Result<Vec<Record>, String> {
        let mut statement = self.connection.prepare("SELECT id,status,tick,reason,recorded_at FROM session_events ORDER BY id DESC LIMIT 100").map_err(|e| e.to_string())?;
        statement
            .query_map([], |row| {
                let status: String = row.get(1)?;
                let status = match status.as_str() {
                    "stopped" => Status::Stopped,
                    "running" => Status::Running,
                    "paused" => Status::Paused,
                    _ => return Err(rusqlite::Error::InvalidQuery),
                };
                Ok(Record {
                    id: row.get(0)?,
                    status,
                    tick: row.get(2)?,
                    reason: row.get(3)?,
                    recorded_at: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())
    }

    pub(crate) fn append(
        &mut self,
        status: Status,
        tick: i64,
        reason: &str,
    ) -> Result<Vec<Record>, String> {
        let transaction = self.connection.transaction().map_err(|e| e.to_string())?;
        transaction
            .execute(
                "INSERT INTO session_events(status,tick,reason) VALUES (?1,?2,?3)",
                params![status.as_str(), tick, reason],
            )
            .map_err(|e| e.to_string())?;
        transaction.commit().map_err(|e| e.to_string())?;
        self.latest()
    }
}
