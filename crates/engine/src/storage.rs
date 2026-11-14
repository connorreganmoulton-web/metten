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
        if version > 3 {
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
            CREATE TABLE IF NOT EXISTS strategy_versions (
                id INTEGER PRIMARY KEY,
                json TEXT NOT NULL,
                recorded_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
            ) STRICT;
            CREATE TRIGGER IF NOT EXISTS strategy_no_update BEFORE UPDATE ON strategy_versions BEGIN SELECT RAISE(ABORT,'Strategy versions are immutable'); END;
            CREATE TRIGGER IF NOT EXISTS strategy_no_delete BEFORE DELETE ON strategy_versions BEGIN SELECT RAISE(ABORT,'Strategy versions are immutable'); END;
            ",
            )
            .map_err(|e| e.to_string())?;
        if version < 3 {
            connection.execute_batch("ALTER TABLE strategy_versions ADD COLUMN format_revision INTEGER;
                ALTER TABLE strategy_versions ADD COLUMN meaning_id TEXT;
                ALTER TABLE strategy_versions ADD COLUMN explanation TEXT;
                ALTER TABLE strategy_versions ADD COLUMN display_name TEXT;
                CREATE TRIGGER strategy_complete_insert BEFORE INSERT ON strategy_versions
                WHEN NEW.format_revision IS NOT 2 OR NEW.meaning_id IS NOT 'metten_rules_2'
                  OR NEW.explanation IS NULL OR NEW.display_name IS NULL
                BEGIN SELECT RAISE(ABORT,'Complete format-2 metadata required'); END;
                PRAGMA user_version=3;").map_err(|_| "Workspace migration failed; original records are preserved. Reopen with the current Metten.".to_string())?;
        }
        connection
            .execute_batch("COMMIT")
            .map_err(|_| "Workspace migration not confirmed. Reopen Metten.".to_string())?;
        Ok((
            Self {
                connection,
                _lock: lock,
            },
            settings,
        ))
    }

    pub(crate) fn save_strategy(
        &mut self,
        rules: &crate::ValidatedStrategy,
    ) -> Result<crate::StrategySummary, crate::StrategyFailure> {
        let failed = |_| crate::StrategyFailure::new("storage");
        let tx = self.connection.transaction().map_err(failed)?;
        tx.execute("INSERT INTO strategy_versions(json,format_revision,meaning_id,explanation,display_name) VALUES (?1,2,'metten_rules_2',?2,?3)", params![rules.json(), rules.explanation(), rules.name()]).map_err(failed)?;
        let id = tx.last_insert_rowid();
        let recorded_at = tx
            .query_row(
                "SELECT recorded_at FROM strategy_versions WHERE id=?1",
                [id],
                |r| r.get(0),
            )
            .map_err(failed)?;
        let summary = crate::StrategySummary {
            id: id.to_string(),
            recorded_at,
            display_name: Some(rules.name().into()),
            format_revision: Some(2),
            status: crate::VersionStatus::Supported,
        };
        tx.commit().map_err(failed)?;
        Ok(summary)
    }

    pub(crate) fn strategy_versions(
        &self,
        before: Option<i64>,
    ) -> Result<crate::StrategyResponse, crate::StrategyFailure> {
        use crate::{StrategyFailure, StrategyResponse};
        let failed = |_| StrategyFailure::new("storage");
        let mut statement = self.connection.prepare("SELECT id, CASE WHEN length(CAST(recorded_at AS BLOB)) <= 40 THEN recorded_at ELSE 'Unknown recording time' END, format_revision, CASE WHEN length(CAST(meaning_id AS BLOB)) <= 128 THEN meaning_id END, CASE WHEN length(CAST(display_name AS BLOB)) <= 480 THEN display_name END, explanation IS NOT NULL FROM strategy_versions WHERE (?1 IS NULL OR id < ?1) ORDER BY id DESC LIMIT 51").map_err(failed)?;
        let mut rows = statement.query([before]).map_err(failed)?;
        let mut versions = Vec::new();
        let mut more = false;
        while let Some(row) = rows.next().map_err(failed)? {
            let summary = version_summary(row).map_err(failed)?;
            if versions.len() == 50 {
                more = true;
                break;
            }
            versions.push(summary);
            // Reserve the real cursor in the measured envelope even on a full final page.
            let page = StrategyResponse::Versions {
                next_cursor: versions.last().map(|v| v.id.clone()),
                versions: versions.clone(),
            };
            if serde_json::to_vec(&page)
                .map_err(|_| StrategyFailure::new("storage"))?
                .len()
                > crate::STRATEGY_PAGE_BYTES
            {
                versions.pop();
                more = true;
                break;
            }
        }
        let next_cursor = if more {
            versions.last().map(|v| v.id.clone())
        } else {
            None
        };
        Ok(StrategyResponse::Versions {
            versions,
            next_cursor,
        })
    }

    pub(crate) fn strategy_detail(
        &self,
        id: i64,
    ) -> Result<crate::StrategyResponse, crate::StrategyFailure> {
        use crate::{StrategyDetail, StrategyFailure, StrategyResponse, VersionStatus};
        let failed = |_| StrategyFailure::new("storage");
        let mut statement = self.connection.prepare("SELECT id, CASE WHEN length(CAST(recorded_at AS BLOB)) <= 40 THEN recorded_at ELSE 'Unknown recording time' END, format_revision, CASE WHEN length(CAST(meaning_id AS BLOB)) <= 128 THEN meaning_id END, CASE WHEN length(CAST(display_name AS BLOB)) <= 480 THEN display_name END, explanation IS NOT NULL, CASE WHEN length(CAST(json AS BLOB)) <= 1048576 THEN json END, CASE WHEN length(CAST(explanation AS BLOB)) <= 1048576 THEN explanation END FROM strategy_versions WHERE id=?1").map_err(failed)?;
        let mut rows = statement.query([id]).map_err(failed)?;
        let row = rows
            .next()
            .map_err(failed)?
            .ok_or_else(|| StrategyFailure::new("not_found"))?;
        let mut summary = version_summary(row).map_err(failed)?;
        let json: Option<String> = row.get(6).map_err(failed)?;
        let explanation: Option<String> = row.get(7).map_err(failed)?;
        let meaning_id: Option<String> = row.get(3).map_err(failed)?;
        let mut notice = match summary.status {
            VersionStatus::Supported => "Supported rules only; no trading is connected.",
            VersionStatus::LegacyReadOnly => {
                "Legacy read-only history. An absent explanation was never recorded; it is not reconstructed."
            }
            VersionStatus::Unsupported => {
                "This format or meaning is unsupported. Inspect or edit a copy; it cannot execute."
            }
            VersionStatus::InvalidRecord => {
                "Incomplete or inconsistent version record; it cannot execute."
            }
        };
        if summary.status == VersionStatus::Supported {
            let valid = json
                .as_deref()
                .and_then(|text| crate::ValidatedStrategy::parse(text).ok())
                .is_some_and(|rules| {
                    Some(rules.explanation()) == explanation.as_deref()
                        && Some(rules.name()) == summary.display_name.as_deref()
                });
            if !valid {
                summary.status = VersionStatus::InvalidRecord;
                notice = "Inconsistent version record; it cannot execute.";
            }
        }
        let mut detail = StrategyDetail {
            summary,
            json,
            explanation,
            meaning_id,
            notice,
        };
        let too_large = detail.json.is_none()
            || (row.get::<_, bool>(5).map_err(failed)? && detail.explanation.is_none())
            || serde_json::to_vec(&StrategyResponse::Detail {
                version: detail.clone(),
            })
            .map_err(|_| StrategyFailure::new("storage"))?
            .len()
                > crate::STRATEGY_DETAIL_BYTES;
        if too_large {
            detail.json = None;
            detail.explanation = None;
            detail.notice = "This version is too large to show completely. Original content is preserved in storage.";
        }
        Ok(StrategyResponse::Detail { version: detail })
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

fn version_summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<crate::StrategySummary> {
    use crate::VersionStatus;
    let format_revision: Option<i64> = row.get(2)?;
    let meaning: Option<String> = row.get(3)?;
    let name: Option<String> = row.get(4)?;
    let has_explanation: bool = row.get(5)?;
    let name_valid = name.as_ref().is_some_and(|s| {
        !s.trim().is_empty() && s.chars().count() <= 120 && !s.chars().any(char::is_control)
    });
    let status = match (format_revision, meaning.as_deref()) {
        (None, None) => VersionStatus::LegacyReadOnly,
        (Some(2), Some("metten_rules_2")) if name_valid && has_explanation => {
            VersionStatus::Supported
        }
        (Some(2), Some("metten_rules_2")) | (None, _) => VersionStatus::InvalidRecord,
        _ => VersionStatus::Unsupported,
    };
    let time: String = row.get(1)?;
    Ok(crate::StrategySummary {
        id: row.get::<_, i64>(0)?.to_string(),
        recorded_at: if time.len() <= 40 && !time.chars().any(char::is_control) {
            time
        } else {
            "Unknown recording time".into()
        },
        format_revision,
        display_name: if name_valid { name } else { None },
        status,
    })
}
