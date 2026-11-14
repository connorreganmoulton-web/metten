use metten_engine::{
    Engine, StrategyRequest as Request, StrategyResponse as Response, ValidatedStrategy,
    VersionStatus, decode_strategy_request,
};
use rusqlite::{Connection, params};
const EXAMPLE: &str = include_str!("../../../examples/learning.json");
fn save(engine: &Engine, text: &str) -> metten_engine::StrategySummary {
    match engine
        .strategies(Request::Save {
            json: text.into(),
            reviewed_json: text.into(),
        })
        .unwrap()
    {
        Response::Saved { version } => version,
        _ => panic!("expected committed version"),
    }
}
fn list(
    engine: &Engine,
    cursor: Option<i64>,
) -> (Vec<metten_engine::StrategySummary>, Option<String>) {
    match engine
        .strategies(Request::List { before_id: cursor })
        .unwrap()
    {
        Response::Versions {
            versions,
            next_cursor,
        } => (versions, next_cursor),
        _ => panic!("expected versions"),
    }
}
fn detail(engine: &Engine, id: &str) -> metten_engine::StrategyDetail {
    match engine
        .strategies(Request::Detail {
            version_id: id.parse().unwrap(),
        })
        .unwrap()
    {
        Response::Detail { version } => version,
        _ => panic!("expected detail"),
    }
}
#[test]
fn preview_save_edit_reopen_preserves_exact_text_explanations_and_sessions() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.sqlite");
    let first;
    {
        let engine = Engine::open(&path).unwrap();
        assert!(matches!(
            engine
                .strategies(Request::Preview {
                    json: EXAMPLE.into()
                })
                .unwrap(),
            Response::Preview { .. }
        ));
        assert!(list(&engine, None).0.is_empty());
        assert_eq!(
            engine
                .strategies(Request::Save {
                    json: EXAMPLE.into(),
                    reviewed_json: "old text".into()
                })
                .unwrap_err()
                .code,
            "not_reviewed"
        );
        first = save(&engine, EXAMPLE);
        let edited = EXAMPLE.replacen("\"30\"", "\"29\"", 1);
        let second = save(&engine, &edited);
        assert_ne!(first.id, second.id);
        assert_ne!(
            detail(&engine, &first.id).explanation,
            detail(&engine, &second.id).explanation
        );
        assert!(matches!(
            engine
                .strategies(Request::Save {
                    json: "{}".into(),
                    reviewed_json: "{}".into()
                })
                .unwrap(),
            Response::RuleErrors { .. }
        ));
        let db = Connection::open(&path).unwrap();
        for sql in [
            "UPDATE strategy_versions SET json='{}'",
            "DELETE FROM strategy_versions",
            "INSERT INTO strategy_versions(json) VALUES ('{}')",
        ] {
            assert!(db.execute(sql, []).is_err());
        }
        engine.stop_and_save().unwrap();
    }
    let engine = Engine::open(&path).unwrap();
    assert!(!engine.snapshot().running);
    assert!(engine.snapshot().history.len() >= 3);
    let original = detail(&engine, &first.id);
    assert_eq!(original.json.as_deref(), Some(EXAMPLE));
    assert_eq!(
        original.explanation.as_deref(),
        Some(ValidatedStrategy::parse(EXAMPLE).unwrap().explanation())
    );
    assert_eq!(list(&engine, None).0.len(), 2);
}
fn legacy(path: &std::path::Path, revision: i64) {
    let db = Connection::open(path).unwrap();
    db.execute_batch("CREATE TABLE session_events(id INTEGER PRIMARY KEY,status TEXT NOT NULL,tick INTEGER NOT NULL,reason TEXT NOT NULL,recorded_at TEXT NOT NULL DEFAULT 'old-time') STRICT;
        INSERT INTO session_events VALUES(77,'paused',42,'preserved','old-time');
        CREATE TABLE strategy_versions(id INTEGER PRIMARY KEY,json TEXT NOT NULL,recorded_at TEXT NOT NULL DEFAULT 'old-time') STRICT;
        INSERT INTO strategy_versions VALUES(99,'{\"format\":1,\"private_original\":true}','old-time');").unwrap();
    db.pragma_update(None, "user_version", revision).unwrap();
}
#[test]
fn all_prior_revisions_migrate_without_reinterpreting_or_rewriting_history() {
    for revision in 0..=2 {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("db.sqlite");
        legacy(&path, revision);
        for _ in 0..2 {
            let engine = Engine::open(&path).unwrap();
            let old = detail(&engine, "99");
            assert_eq!(old.summary.status, VersionStatus::LegacyReadOnly);
            assert_eq!(old.summary.display_name, None);
            assert_eq!(old.meaning_id, None);
            assert_eq!(old.explanation, None);
            assert_eq!(
                old.json.as_deref(),
                Some("{\"format\":1,\"private_original\":true}")
            );
            assert!(
                engine
                    .snapshot()
                    .history
                    .iter()
                    .any(|r| r.id == 77 && r.tick == 42 && r.reason == "preserved")
            );
            let db = Connection::open(&path).unwrap();
            assert_eq!(
                db.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
                    .unwrap(),
                3
            );
            assert_eq!(
                db.query_row(
                    "SELECT json,recorded_at FROM strategy_versions WHERE id=99",
                    [],
                    |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
                )
                .unwrap(),
                (
                    "{\"format\":1,\"private_original\":true}".into(),
                    "old-time".into()
                )
            );
        }
    }
}
#[test]
fn failed_migration_rolls_back_every_schema_change_and_can_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("db.sqlite");
    legacy(&path, 2);
    let db = Connection::open(&path).unwrap();
    db.execute_batch("CREATE TRIGGER strategy_complete_insert BEFORE INSERT ON strategy_versions BEGIN SELECT RAISE(ABORT,'injected migration conflict'); END;").unwrap();
    assert!(Engine::open(&path).is_err());
    assert_eq!(
        db.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        2
    );
    assert_eq!(
        db.query_row(
            "SELECT count(*) FROM pragma_table_info('strategy_versions')",
            [],
            |r| r.get::<_, i64>(0)
        )
        .unwrap(),
        3
    );
    db.execute_batch("DROP TRIGGER strategy_complete_insert")
        .unwrap();
    drop(db);
    assert!(Engine::open(&path).is_ok());
}
#[test]
fn interrupted_migration_child() {
    let Some(path) = std::env::var_os("METTEN_MIGRATION_CHILD") else {
        return;
    };
    let db = Connection::open(path).unwrap();
    db.execute_batch("BEGIN IMMEDIATE; ALTER TABLE strategy_versions ADD COLUMN format_revision INTEGER; PRAGMA user_version=3;").unwrap();
    std::process::exit(0);
}
#[test]
fn termination_before_commit_rolls_back_and_newer_revisions_refuse() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("db.sqlite");
    legacy(&path, 2);
    assert!(
        std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "interrupted_migration_child"])
            .env("METTEN_MIGRATION_CHILD", &path)
            .status()
            .unwrap()
            .success()
    );
    let db = Connection::open(&path).unwrap();
    assert_eq!(
        db.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        2
    );
    assert_eq!(
        db.query_row(
            "SELECT count(*) FROM pragma_table_info('strategy_versions')",
            [],
            |r| r.get::<_, i64>(0)
        )
        .unwrap(),
        3
    );
    drop(db);
    drop(Engine::open(&path).unwrap());
    let db = Connection::open(&path).unwrap();
    // The revision-2 application's exact revision gate rejects the upgraded fixture.
    let revision: i64 = db
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert!(revision > 2);
    db.pragma_update(None, "user_version", 4).unwrap();
    drop(db);
    assert!(Engine::open(&path).err().unwrap().contains("newer Metten"));
}
#[test]
fn mixed_records_safe_detail_pagination_and_write_failure() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("db.sqlite");
    let engine = Engine::open(&path).unwrap();
    let db = Connection::open(&path).unwrap();
    // Construct synthetic future/corrupt history; production never removes its insert guard.
    db.execute_batch("DROP TRIGGER strategy_complete_insert;")
        .unwrap();
    db.execute("INSERT INTO strategy_versions(json,format_revision,meaning_id,explanation,display_name) VALUES (?1,9,'future','stored <literal>','future')", ["{\"format\":9}"]).unwrap();
    db.execute("INSERT INTO strategy_versions(json,format_revision,meaning_id,explanation,display_name) VALUES (?1,2,'metten_rules_2','wrong explanation','bad')", [EXAMPLE]).unwrap();
    db.execute(
        "INSERT INTO strategy_versions(json) VALUES (?1)",
        ["x".repeat(1024 * 1024 + 1)],
    )
    .unwrap();
    for i in 0..90 {
        // Maximum multibyte names exercise actual serialized bytes on each page.
        let name = format!("{}{}", "🦀".repeat(118), i);
        let text = EXAMPLE.replace("Learning: remember a side, consider its opposite", &name);
        save(&engine, &text);
    }
    assert_eq!(
        detail(&engine, "1").summary.status,
        VersionStatus::Unsupported
    );
    assert_eq!(
        detail(&engine, "1").explanation.as_deref(),
        Some("stored <literal>")
    );
    assert_eq!(
        detail(&engine, "2").summary.status,
        VersionStatus::InvalidRecord
    );
    assert!(detail(&engine, "3").json.is_none());
    let mut ids = Vec::new();
    let mut cursor = None;
    loop {
        let response = engine
            .strategies(Request::List { before_id: cursor })
            .unwrap();
        assert!(serde_json::to_vec(&response).unwrap().len() <= 32768);
        let Response::Versions {
            versions,
            next_cursor,
        } = response
        else {
            panic!()
        };
        assert!(versions.len() <= 50);
        ids.extend(versions.iter().map(|s| s.id.parse::<i64>().unwrap()));
        match next_cursor {
            Some(c) => cursor = Some(c.parse().unwrap()),
            None => break,
        }
    }
    assert_eq!(ids, (1..=93).rev().collect::<Vec<_>>());
    db.execute_batch("CREATE TRIGGER injected_failure BEFORE INSERT ON strategy_versions BEGIN SELECT RAISE(ABORT,'private-marker'); END;").unwrap();
    let failure = engine
        .strategies(Request::Save {
            json: EXAMPLE.into(),
            reviewed_json: EXAMPLE.into(),
        })
        .unwrap_err();
    assert!(!failure.message.contains("private-marker"));
    assert_eq!(
        db.query_row("SELECT count(*) FROM strategy_versions", [], |r| r
            .get::<_, i64>(0))
            .unwrap(),
        93
    );
    db.execute(
        "INSERT INTO strategy_versions(json,format_revision) VALUES (?1,9)",
        params!["{}"],
    )
    .unwrap_err();
}
#[test]
fn direct_request_decode_is_strict_bounded_and_does_not_echo_input() {
    use serde_json::json;
    for body in [
        json!("secret-marker"),
        json!({"operation":"secret-marker"}),
        json!({"operation":"list", "path":"secret-marker"}),
        json!({"operation":"detail","version_id":"9223372036854775808"}),
        json!({"operation":"detail","version_id":"0"}),
        json!({"operation":"detail","version_id":99}),
        json!({"operation":"list","before_id":null}),
        json!({"operation":"save","json":"{}"}),
        json!({"operation":"preview","json":"x".repeat(262145)}),
    ] {
        let failure = decode_strategy_request(&body).unwrap_err();
        assert!(
            !serde_json::to_string(&failure)
                .unwrap()
                .contains("secret-marker")
        );
    }
    assert!(
        decode_strategy_request(&json!({"operation":"detail","version_id":"9223372036854775807"}))
            .is_ok()
    );
}

#[test]
fn legacy_stored_explanations_and_large_string_ids_remain_literal() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("db.sqlite");
    let engine = Engine::open(&path).unwrap();
    let db = Connection::open(&path).unwrap();
    db.execute_batch("DROP TRIGGER strategy_complete_insert;")
        .unwrap();
    db.execute(
        "INSERT INTO strategy_versions(id,json,explanation) VALUES (9007199254740993,?1,?2)",
        params!["{\"format\":1}", "Historical <literal> explanation"],
    )
    .unwrap();
    let original = detail(&engine, "9007199254740993");
    assert_eq!(original.summary.id, "9007199254740993");
    assert_eq!(original.summary.status, VersionStatus::LegacyReadOnly);
    assert_eq!(
        original.explanation.as_deref(),
        Some("Historical <literal> explanation")
    );
    assert_eq!(list(&engine, None).0[0].id, "9007199254740993");
    db.execute("INSERT INTO strategy_versions(json,format_revision,meaning_id,explanation,display_name) VALUES (?1,2,'metten_rules_2','invalid',?2)", params![EXAMPLE, "a\0malformed"]).unwrap();
    let versions = list(&engine, None).0;
    assert_eq!(versions[0].status, VersionStatus::InvalidRecord);
    assert_eq!(versions[0].display_name, None);
    assert!(matches!(
        engine
            .strategies(Request::Preview {
                json: "{\"format\":1}".into()
            })
            .unwrap(),
        Response::RuleErrors { .. }
    ));
}

#[test]
fn hostile_future_metadata_hits_serialized_page_limit_without_skipping_rows() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("db.sqlite");
    let engine = Engine::open(&path).unwrap();
    let db = Connection::open(&path).unwrap();
    db.execute_batch("DROP TRIGGER strategy_complete_insert")
        .unwrap();
    let high = i64::MAX - 100;
    for id in high..high + 90 {
        db.execute("INSERT INTO strategy_versions(id,json,recorded_at,format_revision,meaning_id,display_name) VALUES (?1,'{}',?2,9223372036854775807,'future',?3)", params![id, "\\".repeat(40), "🦀".repeat(120)]).unwrap();
    }
    let mut cursor = None;
    let mut ids = Vec::new();
    let mut shortened = false;
    loop {
        let page = engine
            .strategies(Request::List { before_id: cursor })
            .unwrap();
        assert!(serde_json::to_vec(&page).unwrap().len() <= 32768);
        let Response::Versions {
            versions,
            next_cursor,
        } = page
        else {
            panic!()
        };
        if next_cursor.is_some() && versions.len() < 50 {
            shortened = true;
        }
        assert!(
            versions
                .iter()
                .all(|v| v.status == VersionStatus::Unsupported)
        );
        ids.extend(versions.iter().map(|v| v.id.parse::<i64>().unwrap()));
        match next_cursor {
            Some(id) => cursor = Some(id.parse().unwrap()),
            None => break,
        }
    }
    assert!(shortened);
    assert_eq!(ids, (high..high + 90).rev().collect::<Vec<_>>());
}

#[test]
fn quitting_refuses_validated_save_without_an_incomplete_row() {
    let dir = tempfile::tempdir().unwrap();
    let engine = Engine::open(dir.path().join("db.sqlite")).unwrap();
    let quitting = std::sync::atomic::AtomicBool::new(true);
    let error = engine
        .strategies_unless_quitting(
            Request::Save {
                json: EXAMPLE.into(),
                reviewed_json: EXAMPLE.into(),
            },
            &quitting,
        )
        .unwrap_err();
    assert_eq!(error.code, "quitting");
    assert!(list(&engine, None).0.is_empty());
    quitting.store(false, std::sync::atomic::Ordering::SeqCst);
    save(&engine, EXAMPLE);
}
