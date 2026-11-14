use metten_engine::{Engine, strategy::*};
use serde_json::{Value, json};
const EXAMPLE: &str = include_str!("../../../examples/learning.json");
fn rules() -> Value {
    serde_json::from_str(EXAMPLE).unwrap()
}
#[test]
fn immutable_versions_survive_edit_reopen_and_existing_history() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.sqlite");
    let first;
    {
        let engine = Engine::open(&path).unwrap();
        first = engine.import_strategy(EXAMPLE).unwrap();
        let mut edit = rules();
        edit["purchase"]["condition"]["comparisons"][0]["value"] = json!(45);
        let second = engine.import_strategy(&edit.to_string()).unwrap();
        assert!(second.id > first.id);
        assert_ne!(second.explanation, first.explanation);
        assert!(engine.import_strategy("{}").is_err());
        let connection = rusqlite::Connection::open(&path).unwrap();
        assert!(
            connection
                .execute("UPDATE strategy_versions SET json='{}'", [])
                .is_err()
        );
        assert!(
            connection
                .execute("DELETE FROM strategy_versions", [])
                .is_err()
        );
        engine.stop_and_save().unwrap();
    }
    let reopened = Engine::open(&path).unwrap();
    assert!(!reopened.snapshot().running);
    assert!(reopened.snapshot().history.len() >= 3);
    let versions = reopened.strategy_versions().unwrap();
    assert_eq!(versions.len(), 2);
    assert_eq!(versions[1].json, first.json);
    assert_eq!(versions[1].explanation, first.explanation);
    assert_eq!(
        Strategy::parse(&versions[1].json).unwrap().explanation(),
        versions[1].explanation
    );
}

fn format2() -> Value {
    json!({"format":2,"name":"Controlled asks","inputs":[{"id":"ask","source":"kalshi","instrument":"current_contract","field":"buyable_ask","unit":"contract_cents","side":"selected_side"}],"checkpoints":[{"id":"early","during":{"start_seconds":0,"end_seconds":60},"remember_side":{"type":"unique_side"},"when":{"type":"compare","left":{"reference":"ask"},"comparison":"less_than","right":{"value":"40","unit":"contract_cents"}}}],"purchase_options":[{"id":"opposite","during":{"start_seconds":60,"end_seconds":120},"side":{"type":"checkpoint_side","checkpoint":"early","relation":"opposite"},"when":{"type":"compare","left":{"reference":"ask"},"comparison":"at_most","right":{"value":"30","unit":"contract_cents"}}},{"id":"same","during":{"start_seconds":60,"end_seconds":120},"side":{"type":"checkpoint_side","checkpoint":"early","relation":"same"},"when":{"type":"compare","left":{"reference":"ask"},"comparison":"at_most","right":{"value":"30","unit":"contract_cents"}}}]})
}
fn descriptor(id: &str, open: i64) -> MarketDescriptor {
    MarketDescriptor {
        series_id: "btc15m".into(),
        market_id: id.into(),
        contract_id: format!("contract_{id}"),
        open_ms: open,
        close_ms: open + 900000,
    }
}
fn binding(id: u32, input: &str, market: &str, side: Side) -> BindingDescriptor {
    BindingDescriptor {
        binding_id: id,
        input_id: input.into(),
        scope: BindingScope::Contract {
            market_id: market.into(),
            contract_id: format!("contract_{market}"),
            side,
        },
    }
}
fn context() -> RunContext {
    RunContext {
        strategy_version: "version_1".into(),
        run_id: "controlled".into(),
        go_time: "2026-11-13T00:00:00Z".into(),
        series_id: "btc15m".into(),
        markets: vec![descriptor("one", 1)],
        bindings: vec![
            binding(1, "ask", "one", Side::Yes),
            binding(2, "ask", "one", Side::No),
        ],
    }
}
fn objective() -> ObjectiveView {
    ObjectiveView {
        market_id: "one".into(),
        requested_quantity: 10,
        confirmed_quantity: 0,
        locked_side: None,
        attempt_state: AttemptState::None,
        safeguards: Safeguards {
            allowed: true,
            yes: PriceRange {
                minimum: Decimal::parse("0").unwrap(),
                maximum: Decimal::parse("100").unwrap(),
            },
            no: PriceRange {
                minimum: Decimal::parse("0").unwrap(),
                maximum: Decimal::parse("100").unwrap(),
            },
        },
    }
}
fn batch(time: u64, number: u64, changes: Vec<EvidenceChange>) -> AdvanceBatch {
    AdvanceBatch {
        received_through_ms: time,
        target_ms: time,
        transactions: if changes.is_empty() {
            vec![]
        } else {
            vec![NormalizedTransaction {
                number,
                at_ms: time,
                changes,
            }]
        },
        coverage: vec![],
        current_admission: Admission::Allowed,
    }
}
fn prices(yes: &str, no: &str, number: u64) -> Vec<EvidenceChange> {
    vec![
        EvidenceChange::Observation {
            binding_id: 1,
            observation_number: number,
            value: Decimal::parse(yes).unwrap(),
        },
        EvidenceChange::Observation {
            binding_id: 2,
            observation_number: number,
            value: Decimal::parse(no).unwrap(),
        },
    ]
}
fn finish(e: &mut Evaluator, b: AdvanceBatch, o: Option<ObjectiveView>) -> Decision {
    e.begin(b).unwrap();
    loop {
        match e.process_slice(SliceControl::Continue, o.clone()) {
            SliceResult::Complete(d) => return d,
            SliceResult::Pending { .. } => (),
            other => panic!("{other:?}"),
        }
    }
}
fn entered(r: Value) -> Evaluator {
    let mut e =
        Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), context()).unwrap();
    let validity = vec![
        EvidenceChange::Validity {
            binding_id: 1,
            valid_until_ms: 900001,
            reason: None,
        },
        EvidenceChange::Validity {
            binding_id: 2,
            valid_until_ms: 900001,
            reason: None,
        },
    ];
    let mut changes = prices("39", "61", 1);
    changes.push(EvidenceChange::MarketBoundary {
        market_id: "one".into(),
    });
    let mut initial = batch(1, 1, changes);
    initial.transactions.insert(
        0,
        NormalizedTransaction {
            number: 0,
            at_ms: 0,
            changes: validity,
        },
    );
    initial.transactions[0].number = 1;
    initial.transactions[1].number = 2;
    finish(&mut e, initial, Some(objective()));
    e
}
#[test]
fn phase1_checkpoint_priority_ask_boundaries_and_objective() {
    let mut e = entered(format2());
    assert_eq!(e.snapshot().checkpoints["early"].side, Some(Side::Yes));
    assert!(e.snapshot().candidate.is_none());
    let d = finish(
        &mut e,
        batch(60001, 3, prices("29", "35", 2)),
        Some(objective()),
    );
    assert_eq!(d.candidate.unwrap().option_id, "same");
    let d = finish(
        &mut e,
        batch(60002, 4, prices("29", "29", 3)),
        Some(objective()),
    );
    assert_eq!(d.candidate.unwrap().side, Side::No);
    let mut o = objective();
    o.confirmed_quantity = 4;
    o.locked_side = Some(Side::Yes);
    let d = finish(
        &mut e,
        batch(60003, 5, prices("29", "29", 4)),
        Some(o.clone()),
    );
    let c = d.candidate.unwrap();
    assert_eq!(c.side, Side::Yes);
    assert_eq!(c.remaining_quantity, 6);
    o.attempt_state = AttemptState::Unknown;
    assert!(
        finish(&mut e, batch(60004, 6, prices("29", "29", 5)), Some(o))
            .candidate
            .is_none()
    );
    assert!(
        finish(
            &mut e,
            batch(120001, 7, prices("29", "29", 6)),
            Some(objective())
        )
        .candidate
        .is_none()
    );
}
#[test]
fn phase1_rejection_releases_admission_and_stop_is_terminal() {
    let mut e = entered(format2());
    let checkpoints = e.snapshot().checkpoints.clone();
    let mut invalid = batch(2, 3, prices("20", "80", 2));
    invalid.target_ms = 0;
    e.begin(invalid).unwrap();
    loop {
        match e.process_slice(SliceControl::Continue, Some(objective())) {
            SliceResult::Pending { .. } => (),
            SliceResult::Rejected(error) => {
                assert_eq!(error.code, "invalid_time");
                break;
            }
            other => panic!("{other:?}"),
        }
    }
    assert!(e.snapshot().candidate.is_none());
    assert_eq!(e.snapshot().checkpoints, checkpoints);
    finish(
        &mut e,
        batch(2, 3, prices("20", "80", 2)),
        Some(objective()),
    );
    e.begin(batch(600000, 3, vec![])).unwrap();
    assert!(matches!(
        e.process_slice(SliceControl::Continue, Some(objective())),
        SliceResult::Pending { .. }
    ));
    assert_eq!(e.begin(batch(600001, 4, vec![])).unwrap_err().code, "busy");
    assert_eq!(
        e.process_slice(SliceControl::Stop, None),
        SliceResult::Stopped
    );
    assert_eq!(
        e.process_slice(SliceControl::Continue, None),
        SliceResult::Stopped
    );
    assert_eq!(
        e.begin(batch(600001, 4, vec![])).unwrap_err().code,
        "stopped"
    );
}

#[test]
fn phase1_fills_seen_during_validation_or_staging_survive_stale_views_and_rejection() {
    for phase in ["validating", "staging"] {
        for reject in [false, true] {
            let mut e = entered(current_rule(ask_below()));
            let mut input = batch(1, 3, vec![]);
            if phase == "validating" {
                input.transactions = (3..1003)
                    .map(|number| NormalizedTransaction {
                        number,
                        at_ms: 1,
                        changes: vec![EvidenceChange::Validity {
                            binding_id: 1,
                            valid_until_ms: 900001,
                            reason: None,
                        }],
                    })
                    .collect();
                if reject {
                    input.transactions.last_mut().unwrap().number = 3;
                }
            } else {
                input.coverage = vec![
                    CoverageFact {
                        binding_id: 1,
                        start_ms: 0,
                        end_ms: 1,
                    };
                    1000
                ];
                if reject {
                    input.coverage.last_mut().unwrap().start_ms = 1;
                }
            }
            e.begin(input).unwrap();
            loop {
                match e.process_slice(SliceControl::Continue, Some(objective())) {
                    SliceResult::Pending { phase: reached, .. } if reached == phase => break,
                    SliceResult::Pending { .. } => (),
                    other => panic!("{phase}, reject={reject}: {other:?}"),
                }
            }
            let mut filled = objective();
            filled.confirmed_quantity = 4;
            filled.locked_side = Some(Side::Yes);
            filled.attempt_state = AttemptState::Finished;
            assert!(matches!(
                e.process_slice(SliceControl::Continue, Some(filled.clone())),
                SliceResult::Pending { .. }
            ));
            loop {
                match e.process_slice(SliceControl::Continue, Some(objective())) {
                    SliceResult::Pending { .. } => (),
                    SliceResult::Complete(decision) if !reject => {
                        assert!(decision.candidate.is_none(), "{phase}: {decision:?}");
                        break;
                    }
                    SliceResult::Rejected(error) if reject => {
                        assert_eq!(
                            error.code,
                            if phase == "validating" {
                                "invalid_transaction"
                            } else {
                                "invalid_coverage"
                            }
                        );
                        break;
                    }
                    other => panic!("{phase}, reject={reject}: {other:?}"),
                }
            }
            assert!(
                finish(&mut e, batch(1, 1003, vec![]), Some(objective()))
                    .candidate
                    .is_none(),
                "{phase}, reject={reject}: rejection must not erase a confirmed fill"
            );
            let recovered = finish(&mut e, batch(1, 1004, vec![]), Some(filled));
            let candidate = recovered.candidate.unwrap();
            assert_eq!(candidate.remaining_quantity, 6);
            assert_eq!(candidate.side, Side::Yes);
        }
    }
}
#[test]
fn phase1_grammar_decimal_and_reachability() {
    let r = format2();
    assert!(ValidatedStrategy::parse(&r.to_string()).is_ok());
    for bad in [
        "-0",
        "01",
        "1e2",
        "+1",
        " 1",
        "1.0000001",
        "NaN",
        "1000000000000",
    ] {
        assert!(Decimal::parse(bad).is_err(), "{bad}");
    }
    for good in ["0", "0.000001", "-1.23", "999999999999.999999"] {
        assert_eq!(Decimal::parse(good).unwrap().to_string(), good);
    }
    let mut bad = r.clone();
    bad["purchase_options"][0]["when"] = json!({"type":"held_for","duration_seconds":60,"condition":r["purchase_options"][0]["when"]});
    assert_eq!(
        ValidatedStrategy::parse(&bad.to_string()).unwrap_err()[0].code,
        "unreachable_duration"
    );
    let raw = r.to_string();
    assert!(
        ValidatedStrategy::parse(&raw.replacen("\"format\":2", "\"format\":2,\"format\":2", 1))
            .is_err()
    );
    let mut bad = r;
    bad["inputs"][0]["secret"] = "do not echo".into();
    let errors = ValidatedStrategy::parse(&bad.to_string()).unwrap_err();
    assert!(!format!("{errors:?}").contains("do not echo"));
}
#[test]
fn phase1_historical_composition_is_accepted_and_budgeted() {
    let mut r = format2();
    r["inputs"] = json!([{"id":"underlying","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}]);
    r["checkpoints"] = json!([]);
    r["purchase_options"] = json!([{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"average"},"comparison":"greater_than","right":{"value":"1","unit":"usd_per_btc"}}}]);
    r["measurements"] = json!([{"id":"average","operation":"rolling_average","of":{"reference":"underlying"},"lookback_seconds":3600}]);
    let v = ValidatedStrategy::parse(&r.to_string()).unwrap();
    assert_eq!(v.requirements().startup_ms["underlying"], 3600000);
    assert_eq!(v.requirements().planned_bytes, 8 * 1024 * 1024 + 57600000);
    assert!(
        v.explanation()
            .contains("Calculations use six decimal places and one-millisecond time steps.")
    );
    let m = r["measurements"][0].clone();
    r["measurements"] = Value::Array(
        (0..5)
            .map(|i| {
                let mut m = m.clone();
                m["id"] = json!(if i == 0 {
                    "average".into()
                } else {
                    format!("average_{i}")
                });
                m
            })
            .collect(),
    );
    assert_eq!(
        ValidatedStrategy::parse(&r.to_string()).unwrap_err()[0].code,
        "resource_limit"
    );
}

fn historical_rule(operation: &str, threshold: &str) -> Value {
    json!({"format":2,"name":"Controlled history","inputs":[{"id":"underlying","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"},{"id":"ask","source":"kalshi","instrument":"current_contract","field":"buyable_ask","unit":"contract_cents","side":"yes"}],"measurements":[{"id":"measured","operation":operation,"of":{"reference":"underlying"},"lookback_seconds":1}],"purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"measured"},"comparison":"equal_to","right":{"value":threshold,"unit":"usd_per_btc"}}}]})
}
fn historical_context() -> RunContext {
    let mut c = context();
    c.markets = vec![descriptor("one", 1001)];
    c.bindings = vec![
        BindingDescriptor {
            binding_id: 1,
            input_id: "underlying".into(),
            scope: BindingScope::Underlying,
        },
        binding(2, "ask", "one", Side::Yes),
    ];
    c
}
fn observation_change(id: u32, n: u64, v: &str) -> EvidenceChange {
    EvidenceChange::Observation {
        binding_id: id,
        observation_number: n,
        value: Decimal::parse(v).unwrap(),
    }
}
fn lease(id: u32) -> EvidenceChange {
    EvidenceChange::Validity {
        binding_id: id,
        valid_until_ms: 1000000,
        reason: None,
    }
}
#[test]
fn phase1_verified_quiet_history_is_staged_before_advancing() {
    for (operation, threshold) in [
        ("rolling_average", "104.002"),
        ("rolling_minimum", "100"),
        ("rolling_maximum", "110"),
        ("rolling_range", "10"),
        ("change", "10"),
    ] {
        let mut c = historical_context();
        c.markets = vec![descriptor("one", 5001)];
        let mut r = historical_rule(operation, threshold);
        r["measurements"][0]["lookback_seconds"] = json!(5);
        let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
        let mut b = batch(5001, 1, vec![]);
        b.transactions = vec![
            NormalizedTransaction {
                number: 1,
                at_ms: 0,
                changes: vec![lease(1), lease(2)],
            },
            NormalizedTransaction {
                number: 2,
                at_ms: 0,
                changes: vec![
                    observation_change(1, 1, "100"),
                    observation_change(2, 1, "25"),
                ],
            },
            NormalizedTransaction {
                number: 3,
                at_ms: 3000,
                changes: vec![observation_change(1, 2, "110")],
            },
            NormalizedTransaction {
                number: 4,
                at_ms: 5001,
                changes: vec![EvidenceChange::MarketBoundary {
                    market_id: "one".into(),
                }],
            },
        ];
        b.coverage = vec![CoverageFact {
            binding_id: 1,
            start_ms: 0,
            end_ms: 5001,
        }];
        let d = finish(&mut e, b, Some(objective()));
        // At 5001, the window is [1,5001): 2999 cells of 100 and 2001 of 110.
        assert!(d.candidate.is_some(), "{operation}: {d:?}");
    }
}
#[test]
fn phase1_gap_is_not_repaired_by_a_late_snapshot() {
    let mut e = Evaluator::new(
        ValidatedStrategy::parse(&historical_rule("rolling_average", "100").to_string()).unwrap(),
        historical_context(),
    )
    .unwrap();
    let mut b = batch(1000, 1, vec![]);
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "100"),
                observation_change(2, 1, "25"),
            ],
        },
    ];
    finish(&mut e, b, None);
    let mut b = batch(
        1001,
        3,
        vec![EvidenceChange::MarketBoundary {
            market_id: "one".into(),
        }],
    );
    b.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 1001,
    }];
    let d = finish(&mut e, b, Some(objective()));
    assert!(d.candidate.is_none());
    assert_eq!(d.phase, "Prefill — collecting live data");
}

fn ask_below() -> Value {
    json!({"type":"compare","left":{"reference":"ask"},"comparison":"less_than","right":{"value":"40","unit":"contract_cents"}})
}
fn current_rule(condition: Value) -> Value {
    let mut r = format2();
    r["checkpoints"] = json!([]);
    r["purchase_options"] =
        json!([{"id":"buy","side":{"type":"fixed","side":"yes"},"when":condition}]);
    r
}
fn covered_prices(time: u64, number: u64, yes: &str, no: &str) -> AdvanceBatch {
    let mut b = batch(time, number, prices(yes, no, number));
    b.coverage = vec![
        CoverageFact {
            binding_id: 1,
            start_ms: 0,
            end_ms: time.max(1),
        },
        CoverageFact {
            binding_id: 2,
            start_ms: 0,
            end_ms: time.max(1),
        },
    ];
    b
}
#[test]
fn phase1_hold_false_excursion_and_same_time_completion_preemption() {
    let hold = json!({"type":"held_for","condition":ask_below(),"duration_seconds":10});
    let mut r = current_rule(json!({"type":"checkpoint_reached","checkpoint":"held"}));
    r["checkpoints"] =
        json!([{"id":"held","remember_side":{"type":"fixed","side":"yes"},"when":hold}]);
    let mut e = entered(r);
    finish(
        &mut e,
        covered_prices(3001, 3, "60", "40"),
        Some(objective()),
    );
    finish(
        &mut e,
        covered_prices(4001, 4, "30", "70"),
        Some(objective()),
    );
    assert!(
        finish(
            &mut e,
            covered_prices(10001, 5, "30", "70"),
            Some(objective())
        )
        .checkpoints
        .is_empty()
    );
    assert!(
        finish(
            &mut e,
            covered_prices(14001, 6, "60", "40"),
            Some(objective())
        )
        .checkpoints
        .is_empty()
    );
    finish(
        &mut e,
        covered_prices(14002, 7, "30", "70"),
        Some(objective()),
    );
    let mut b = batch(24002, 8, vec![]);
    b.coverage = vec![
        CoverageFact {
            binding_id: 1,
            start_ms: 14002,
            end_ms: 24002,
        },
        CoverageFact {
            binding_id: 2,
            start_ms: 14002,
            end_ms: 24002,
        },
    ];
    assert!(
        finish(&mut e, b, Some(objective()))
            .checkpoints
            .contains_key("held")
    );
}
#[test]
fn phase1_distinct_identical_prices_count_but_replays_do_not() {
    let mut e = entered(current_rule(
        json!({"type":"confirmed_updates","condition":ask_below(),"count":3,"count_input":"ask"}),
    ));
    assert!(
        finish(&mut e, covered_prices(2, 3, "39", "61"), Some(objective()))
            .candidate
            .is_none()
    );
    // A new transaction containing the same observation numbers supplies no new observation.
    let replay = batch(2, 4, prices("39", "61", 3));
    assert!(
        finish(&mut e, replay, Some(objective()))
            .candidate
            .is_none()
    );
    assert!(
        finish(&mut e, covered_prices(3, 5, "39", "61"), Some(objective()))
            .candidate
            .is_some()
    );
}
#[test]
fn phase1_four_feed_percentage_rounding_executes_through_history() {
    let mut r = historical_rule("rolling_average", "0.001");
    r["inputs"][0]["id"] = json!("coinbase_price");
    let mut kraken = r["inputs"][0].clone();
    kraken["id"] = json!("kraken_price");
    kraken["source"] = json!("kraken");
    r["inputs"].as_array_mut().unwrap().push(kraken);
    r["measurements"] = json!([{"id":"feed_difference_percent","operation":"percentage_difference","left":{"reference":"coinbase_price"},"reference":{"reference":"kraken_price"}},{"id":"measured","operation":"rolling_average","of":{"reference":"feed_difference_percent"},"lookback_seconds":4}]);
    r["purchase_options"][0]["when"]["right"]["unit"] = json!("percent");
    let mut c = historical_context();
    c.markets = vec![descriptor("one", 4001)];
    c.bindings[0].input_id = "coinbase_price".into();
    c.bindings.push(BindingDescriptor {
        binding_id: 3,
        input_id: "kraken_price".into(),
        scope: BindingScope::Underlying,
    });
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut b = batch(4001, 1, vec![]);
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2), lease(3)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "100001"),
                observation_change(2, 1, "25"),
                observation_change(3, 1, "100000.000003"),
            ],
        },
    ];
    for (i, price) in ["100000.000019", "100000.000057", "100000.000063"]
        .iter()
        .enumerate()
    {
        b.transactions.push(NormalizedTransaction {
            number: i as u64 + 3,
            at_ms: (i as u64 + 1) * 1000,
            changes: vec![observation_change(3, i as u64 + 2, price)],
        });
    }
    b.transactions.push(NormalizedTransaction {
        number: 6,
        at_ms: 4001,
        changes: vec![EvidenceChange::MarketBoundary {
            market_id: "one".into(),
        }],
    });
    b.coverage = vec![
        CoverageFact {
            binding_id: 1,
            start_ms: 0,
            end_ms: 4001,
        },
        CoverageFact {
            binding_id: 3,
            start_ms: 0,
            end_ms: 4001,
        },
    ];
    assert!(finish(&mut e, b, Some(objective())).candidate.is_some());
}

#[test]
fn phase1_registration_rejection_preserves_history_and_next_registration_works() {
    let mut e = entered(format2());
    let before = e.snapshot().clone();
    let bad = EvidenceChange::RegisterMarket {
        descriptor: descriptor("two", 2000),
        bindings: vec![binding(1, "ask", "two", Side::Yes)],
    };
    e.begin(batch(2, 3, vec![bad])).unwrap();
    loop {
        match e.process_slice(SliceControl::Continue, Some(objective())) {
            SliceResult::Pending { .. } => (),
            SliceResult::Rejected(error) => {
                assert_eq!(error.code, "unregistered_binding");
                break;
            }
            other => panic!("{other:?}"),
        }
    }
    assert_eq!(e.snapshot().checkpoints, before.checkpoints);
    assert_eq!(
        e.snapshot().evaluated_through_ms,
        before.evaluated_through_ms
    );
    finish(
        &mut e,
        batch(
            2,
            3,
            vec![EvidenceChange::RegisterMarket {
                descriptor: descriptor("two", 2000),
                bindings: vec![
                    binding(3, "ask", "two", Side::Yes),
                    binding(4, "ask", "two", Side::No),
                ],
            }],
        ),
        Some(objective()),
    );
    finish(
        &mut e,
        batch(3, 4, vec![lease(3), lease(4)]),
        Some(objective()),
    );
    let mut o = objective();
    o.market_id = "two".into();
    let d = finish(
        &mut e,
        batch(
            2000,
            5,
            vec![
                observation_change(3, 1, "61"),
                observation_change(4, 1, "39"),
                EvidenceChange::MarketBoundary {
                    market_id: "two".into(),
                },
            ],
        ),
        Some(o),
    );
    assert_eq!(d.checkpoints["early"].side, Some(Side::No));
    assert_eq!(d.checkpoints["early"].at_ms, 2000);
}
#[test]
fn phase1_entering_contract_retains_its_own_preopen_history() {
    let mut r = historical_rule("rolling_average", "39");
    r["inputs"].as_array_mut().unwrap().remove(0);
    r["measurements"][0]["of"] = json!({"reference":"ask"});
    r["purchase_options"][0]["when"]["right"]["unit"] = json!("contract_cents");
    let mut c = context();
    c.markets = vec![descriptor("one", 1), descriptor("two", 1001)];
    c.bindings = vec![
        binding(1, "ask", "one", Side::Yes),
        binding(2, "ask", "two", Side::Yes),
    ];
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut b = batch(1001, 1, vec![]);
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "20"),
                observation_change(2, 1, "39"),
            ],
        },
        NormalizedTransaction {
            number: 3,
            at_ms: 1001,
            changes: vec![EvidenceChange::MarketBoundary {
                market_id: "two".into(),
            }],
        },
    ];
    b.coverage = vec![
        CoverageFact {
            binding_id: 1,
            start_ms: 0,
            end_ms: 1001,
        },
        CoverageFact {
            binding_id: 2,
            start_ms: 0,
            end_ms: 1001,
        },
    ];
    let mut o = objective();
    o.market_id = "two".into();
    let d = finish(&mut e, b, Some(o));
    assert_eq!(d.candidate.unwrap().market_id, "two");
}
#[test]
fn phase1_oversized_registration_refuses_before_history_collection() {
    let mut r = historical_rule("rolling_average", "39");
    r["inputs"].as_array_mut().unwrap().remove(0);
    r["measurements"][0]["of"] = json!({"reference":"ask"});
    r["measurements"][0]["lookback_seconds"] = json!(3600);
    r["purchase_options"][0]["when"]["right"]["unit"] = json!("contract_cents");
    let mut c = context();
    c.bindings = vec![binding(1, "ask", "one", Side::Yes)];
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let allocated = e.allocated_bytes();
    assert_eq!(allocated, 8 * 1024 * 1024 + 57600000);
    let changes = (2..=5)
        .map(|n| EvidenceChange::RegisterMarket {
            descriptor: descriptor(&format!("market_{n}"), n as i64 * 900000),
            bindings: vec![binding(n, "ask", &format!("market_{n}"), Side::Yes)],
        })
        .collect();
    e.begin(batch(0, 1, changes)).unwrap();
    loop {
        match e.process_slice(SliceControl::Continue, None) {
            SliceResult::Pending { .. } => (),
            SliceResult::Rejected(error) => {
                assert_eq!(error.code, "resource_limit");
                break;
            }
            other => panic!("{other:?}"),
        }
    }
    assert_eq!(e.allocated_bytes(), allocated);
    finish(&mut e, batch(0, 1, vec![lease(1)]), None);
}
#[test]
#[ignore = "measured separately in release mode"]
fn phase1_hour_average_allocates_and_processes_admitted_workload() {
    let mut r = historical_rule("rolling_average", "100");
    r["measurements"][0]["lookback_seconds"] = json!(3600);
    let mut c = historical_context();
    c.markets = vec![descriptor("one", 3600001)];
    let started = std::time::Instant::now();
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut b = batch(3600001, 1, vec![]);
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![
                EvidenceChange::Validity {
                    binding_id: 1,
                    valid_until_ms: 4000000,
                    reason: None,
                },
                EvidenceChange::Validity {
                    binding_id: 2,
                    valid_until_ms: 4000000,
                    reason: None,
                },
            ],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "100"),
                observation_change(2, 1, "25"),
            ],
        },
        NormalizedTransaction {
            number: 3,
            at_ms: 3600001,
            changes: vec![EvidenceChange::MarketBoundary {
                market_id: "one".into(),
            }],
        },
    ];
    b.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 3600001,
    }];
    let d = finish(&mut e, b, Some(objective()));
    assert!(d.candidate.is_some());
    println!(
        "allocated_bytes={}, elapsed_ms={}",
        e.allocated_bytes(),
        started.elapsed().as_millis()
    );
}

#[test]
fn phase1_rounding_normalizes_each_named_operation_with_signed_ties_to_even() {
    for (left, expected) in [
        ("128.000016", "0.000012"),
        ("128.000048", "0.000038"),
        ("127.999984", "-0.000012"),
        ("127.999952", "-0.000038"),
    ] {
        let mut r = current_rule(
            json!({"type":"compare","left":{"reference":"percentage"},"comparison":"equal_to","right":{"value":expected,"unit":"percent"}}),
        );
        r["measurements"] = json!([{"id":"percentage","operation":"percentage_difference","left":{"value":left,"unit":"usd_per_btc"},"reference":{"value":"128","unit":"usd_per_btc"}}]);
        assert!(entered(r).snapshot().candidate.is_some(), "{left}");
    }
}
#[test]
fn phase1_crossings_require_genuine_continuous_observations() {
    let crossing = json!({"type":"crossed_below","left":{"reference":"ask"},"right":{"value":"40","unit":"contract_cents"}});
    let mut e = entered(current_rule(crossing));
    assert!(e.snapshot().candidate.is_none());
    finish(&mut e, covered_prices(2, 3, "50", "50"), Some(objective()));
    assert!(
        finish(&mut e, covered_prices(3, 4, "39", "61"), Some(objective()))
            .candidate
            .is_some()
    );
    let mut timer = batch(4, 5, vec![]);
    timer.coverage = vec![
        CoverageFact {
            binding_id: 1,
            start_ms: 0,
            end_ms: 4,
        },
        CoverageFact {
            binding_id: 2,
            start_ms: 0,
            end_ms: 4,
        },
    ];
    assert!(finish(&mut e, timer, Some(objective())).candidate.is_none());
    assert!(
        finish(
            &mut e,
            batch(
                5,
                5,
                vec![EvidenceChange::Resynchronize {
                    binding_id: 1,
                    value: Decimal::parse("20").unwrap()
                }]
            ),
            Some(objective())
        )
        .candidate
        .is_none()
    );
}

#[test]
fn phase1_composed_history_runs_on_complete_normalized_clock() {
    for (operation, expected) in [("change", "9.99"), ("rolling_average", "105.005")] {
        let mut r = historical_rule("rolling_average", expected);
        r["measurements"] = json!([{"id":"inner","operation":"rolling_average","of":{"reference":"underlying"},"lookback_seconds":1},{"id":"measured","operation":operation,"of":{"reference":"inner"},"lookback_seconds":1}]);
        let validated = ValidatedStrategy::parse(&r.to_string()).unwrap();
        assert_eq!(validated.requirements().startup_ms["underlying"], 2000);
        let mut c = historical_context();
        c.markets = vec![descriptor("one", 2001)];
        let mut e = Evaluator::new(validated, c).unwrap();
        let mut b = batch(2001, 1, vec![]);
        b.transactions = vec![
            NormalizedTransaction {
                number: 1,
                at_ms: 0,
                changes: vec![lease(1), lease(2)],
            },
            NormalizedTransaction {
                number: 2,
                at_ms: 0,
                changes: vec![
                    observation_change(1, 1, "100"),
                    observation_change(2, 1, "25"),
                ],
            },
            NormalizedTransaction {
                number: 3,
                at_ms: 1000,
                changes: vec![observation_change(1, 2, "110")],
            },
            NormalizedTransaction {
                number: 4,
                at_ms: 2001,
                changes: vec![EvidenceChange::MarketBoundary {
                    market_id: "one".into(),
                }],
            },
        ];
        b.coverage = vec![CoverageFact {
            binding_id: 1,
            start_ms: 0,
            end_ms: 2001,
        }];
        assert!(
            finish(&mut e, b, Some(objective())).candidate.is_some(),
            "{operation}"
        );
    }
}
#[test]
fn phase1_document_and_node_exact_caps_and_semantic_rejections() {
    let raw = format2().to_string();
    let exact = format!("{raw}{}", " ".repeat(256 * 1024 - raw.len()));
    assert!(ValidatedStrategy::parse(&exact).is_ok());
    assert_eq!(
        ValidatedStrategy::parse(&(exact + " ")).unwrap_err()[0].code,
        "resource_limit"
    );
    let mut r = current_rule(json!({"type":"all","conditions":vec![ask_below();510]}));
    assert!(ValidatedStrategy::parse(&r.to_string()).is_ok());
    r["purchase_options"][0]["when"]["conditions"]
        .as_array_mut()
        .unwrap()
        .push(ask_below());
    assert_eq!(
        ValidatedStrategy::parse(&r.to_string()).unwrap_err()[0].code,
        "resource_limit"
    );
    for measurements in [
        json!([{"id":"first","operation":"difference","left":{"reference":"second"},"right":{"value":"1","unit":"contract_cents"}},{"id":"second","operation":"difference","left":{"reference":"first"},"right":{"value":"1","unit":"contract_cents"}}]),
        json!([{"id":"first","operation":"difference","left":{"reference":"ask"},"right":{"value":"1","unit":"percent"}}]),
    ] {
        let mut r = current_rule(ask_below());
        r["measurements"] = measurements;
        assert!(ValidatedStrategy::parse(&r.to_string()).is_err());
    }
}
#[test]
fn phase1_forward_checkpoint_dependencies_latch_in_one_event() {
    let mut r = format2();
    r["checkpoints"] = json!([{"id":"dependent","when":{"type":"checkpoint_reached","checkpoint":"early"}},r["checkpoints"][0].clone()]);
    let e = entered(r);
    assert_eq!(e.snapshot().checkpoints.len(), 2);
}

#[test]
fn phase1_ingress_exact_cap_nested_capacity_and_refusal_preserve_state() {
    const CAP: usize = 8 * 1024 * 1024;
    let mut e = entered(current_rule(ask_below()));
    let before = e.snapshot().clone();
    let make = |extra: usize| {
        let mut b = batch(
            1,
            3,
            vec![EvidenceChange::Validity {
                binding_id: 1,
                valid_until_ms: 10,
                reason: Some(String::new()),
            }],
        );
        let overhead = b.ingress_bytes().unwrap();
        if let EvidenceChange::Validity {
            reason: Some(reason),
            ..
        } = &mut b.transactions[0].changes[0]
        {
            // Spare capacity consumes memory even when the string is empty.
            reason.reserve_exact(CAP - overhead + extra);
        }
        b
    };
    let over = make(1);
    assert_eq!(over.ingress_bytes().unwrap_err().code, "resource_limit");
    assert_eq!(e.begin(over).unwrap_err().code, "resource_limit");
    assert_eq!(e.snapshot(), &before);
    let exact = make(0);
    assert_eq!(exact.ingress_bytes().unwrap(), CAP);
    assert_eq!(e.begin(exact).unwrap_err().code, "resource_limit");
    assert_eq!(e.snapshot(), &before);
    e.begin(batch(1, 3, vec![])).unwrap();
    assert!(e.snapshot().candidate.is_none());
    assert_eq!(
        e.process_slice(SliceControl::Stop, None),
        SliceResult::Stopped
    );

    for location in 0..6 {
        let mut bindings = vec![binding(9, "ask", "two", Side::Yes)];
        let mut d = descriptor("two", 1001);
        let huge = String::with_capacity(CAP);
        match location {
            0 => d.series_id = huge,
            1 => d.market_id = huge,
            2 => d.contract_id = huge,
            3 => bindings[0].input_id = huge,
            4 => {
                if let BindingScope::Contract { market_id, .. } = &mut bindings[0].scope {
                    *market_id = huge
                }
            }
            5 => {
                if let BindingScope::Contract { contract_id, .. } = &mut bindings[0].scope {
                    *contract_id = huge
                }
            }
            _ => unreachable!(),
        }
        let b = batch(
            1,
            3,
            vec![EvidenceChange::RegisterMarket {
                descriptor: d,
                bindings,
            }],
        );
        assert_eq!(
            b.ingress_bytes().unwrap_err().code,
            "resource_limit",
            "nested allocation {location}"
        );
    }
    let mut spare = batch(
        1,
        3,
        vec![EvidenceChange::RegisterMarket {
            descriptor: descriptor("two", 1001),
            bindings: Vec::with_capacity(CAP / std::mem::size_of::<BindingDescriptor>()),
        }],
    );
    assert_eq!(spare.ingress_bytes().unwrap_err().code, "resource_limit");
    spare.transactions.clear();
    // Unused top-level capacity remains charged; removed nested allocations do not.
    assert!(spare.ingress_bytes().unwrap() < CAP);
}

#[test]
fn phase1_implicit_asks_require_the_exact_contract_and_side() {
    let mut r = historical_rule("rolling_average", "100");
    r["inputs"].as_array_mut().unwrap().pop();
    for scope in [
        BindingScope::Underlying,
        BindingScope::Market {
            market_id: "one".into(),
        },
        BindingScope::Contract {
            market_id: "one".into(),
            contract_id: "wrong".into(),
            side: Side::Yes,
        },
        BindingScope::Contract {
            market_id: "one".into(),
            contract_id: "contract_one".into(),
            side: Side::No,
        },
    ] {
        let mut c = historical_context();
        c.bindings[1] = BindingDescriptor {
            binding_id: 2,
            input_id: "__buyable_ask_yes".into(),
            scope,
        };
        assert!(Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).is_err());
    }
    let mut c = historical_context();
    c.bindings[1] = binding(2, "__buyable_ask_yes", "one", Side::Yes);
    assert!(Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).is_ok());
}

#[test]
fn phase1_remembered_side_hold_starts_when_the_side_is_known() {
    let mut r =
        current_rule(json!({"type":"held_for","duration_seconds":1,"condition":ask_below()}));
    r["checkpoints"] = json!([{"id":"side_known","during":{"start_seconds":2,"end_seconds":10},"remember_side":{"type":"fixed","side":"yes"},"when":ask_below()}]);
    r["purchase_options"][0]["side"] =
        json!({"type":"checkpoint_side","checkpoint":"side_known","relation":"same"});
    let mut e = entered(r.clone());
    assert!(
        finish(
            &mut e,
            covered_prices(2001, 3, "30", "70"),
            Some(objective())
        )
        .candidate
        .is_none()
    );
    assert!(
        finish(
            &mut e,
            covered_prices(3000, 4, "30", "70"),
            Some(objective())
        )
        .candidate
        .is_none()
    );
    assert!(
        finish(
            &mut e,
            covered_prices(3001, 5, "30", "70"),
            Some(objective())
        )
        .candidate
        .is_some()
    );
    r["purchase_options"][0]["during"] = json!({"start_seconds":0,"end_seconds":3});
    assert_eq!(
        ValidatedStrategy::parse(&r.to_string()).unwrap_err()[0].code,
        "unreachable_duration"
    );
    r["purchase_options"][0]["during"]["end_seconds"] = json!(4);
    assert!(ValidatedStrategy::parse(&r.to_string()).is_ok());
    r["checkpoints"].as_array_mut().unwrap().push(json!({"id":"later","during":{"start_seconds":0,"end_seconds":3},"remember_side":{"type":"checkpoint_side","checkpoint":"side_known","relation":"opposite"},"when":{"type":"held_for","duration_seconds":1,"condition":ask_below()}}));
    assert_eq!(
        ValidatedStrategy::parse(&r.to_string()).unwrap_err()[0].code,
        "unreachable_duration"
    );
}

#[test]
fn phase1_requirements_separate_shared_and_market_history_and_implicit_asks() {
    let mut r = historical_rule("rolling_average", "100");
    r["measurements"].as_array_mut().unwrap().push(json!({"id":"contract_average","operation":"rolling_average","of":{"reference":"ask"},"lookback_seconds":2}));
    let v = ValidatedStrategy::parse(&r.to_string()).unwrap();
    let requirements = v.requirements();
    assert_eq!(requirements.per_node_bytes["measured"], 16000);
    assert_eq!(requirements.per_node_bytes["contract_average"], 32000);
    assert_eq!(requirements.shared_underlying_bytes, 16000);
    assert_eq!(requirements.per_market_bytes, 32000);
    assert_eq!(requirements.planned_bytes, 8 * 1024 * 1024 + 48000);
    assert_eq!(requirements.implicit_buyable_asks, ["__buyable_ask_no"]);
    assert!(v.explanation().contains("contract_average: 32000"));
    assert!(
        v.explanation()
            .contains("Required implicit binding __buyable_ask_no")
    );
    let mut c = historical_context();
    let one = Evaluator::new(v.clone(), c.clone())
        .unwrap()
        .allocated_bytes();
    c.markets.push(descriptor("two", 2001));
    c.bindings.push(binding(3, "ask", "two", Side::Yes));
    let two = Evaluator::new(v, c).unwrap().allocated_bytes();
    assert_eq!(two - one, 32000);
}

#[test]
fn phase1_late_contradiction_recollects_dependent_history_and_preserves_checkpoint() {
    let mut r = historical_rule("rolling_average", "100");
    r["checkpoints"] = json!([{"id":"established","when":{"type":"compare","left":{"reference":"measured"},"comparison":"equal_to","right":{"value":"100","unit":"usd_per_btc"}}}]);
    let mut e = Evaluator::new(
        ValidatedStrategy::parse(&r.to_string()).unwrap(),
        historical_context(),
    )
    .unwrap();
    let mut b = batch(1001, 1, vec![]);
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "100"),
                observation_change(2, 1, "30"),
            ],
        },
        NormalizedTransaction {
            number: 3,
            at_ms: 1001,
            changes: vec![EvidenceChange::MarketBoundary {
                market_id: "one".into(),
            }],
        },
    ];
    b.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 1001,
    }];
    let before = finish(&mut e, b, Some(objective()));
    assert!(before.candidate.is_some());
    let bad = batch(
        1001,
        4,
        vec![EvidenceChange::Gap {
            binding_id: 1,
            start_ms: 1000,
            end_ms: 999,
            reason: "reversed".into(),
        }],
    );
    e.begin(bad).unwrap();
    loop {
        match e.process_slice(SliceControl::Continue, Some(objective())) {
            SliceResult::Pending { .. } => (),
            SliceResult::Rejected(error) => {
                assert_eq!(error.code, "invalid_coverage");
                break;
            }
            other => panic!("{other:?}"),
        }
    }
    assert_eq!(e.snapshot().checkpoints, before.checkpoints);
    assert_eq!(e.snapshot().evaluated_through_ms, 1001);
    let gap = batch(
        1001,
        4,
        vec![EvidenceChange::Gap {
            binding_id: 1,
            start_ms: 500,
            end_ms: 600,
            reason: "contradicts prior certificate".into(),
        }],
    );
    let d = finish(&mut e, gap, Some(objective()));
    assert!(d.candidate.is_none());
    assert_eq!(d.checkpoints, before.checkpoints);
    finish(&mut e, batch(1001, 5, vec![lease(1)]), Some(objective()));
    finish(
        &mut e,
        batch(
            1001,
            6,
            vec![EvidenceChange::Resynchronize {
                binding_id: 1,
                value: Decimal::parse("100").unwrap(),
            }],
        ),
        Some(objective()),
    );
    for (time, eligible) in [(2001, false), (2002, true)] {
        let mut b = batch(time, 7, vec![]);
        b.coverage = vec![CoverageFact {
            binding_id: 1,
            start_ms: 1001,
            end_ms: time,
        }];
        let d = finish(&mut e, b, Some(objective()));
        assert_eq!(d.candidate.is_some(), eligible);
        assert_eq!(d.checkpoints, before.checkpoints);
    }
}

#[test]
fn phase1_binding_and_measurement_explanation_matrix() {
    for (source, instrument, field, unit, side) in [
        ("brti", "btc_usd", "index_value", "usd_per_btc", None),
        ("coinbase", "btc_usd", "last_trade", "usd_per_btc", None),
        ("kraken", "btc_usd", "last_trade", "usd_per_btc", None),
        (
            "kalshi",
            "current_market",
            "opening_reference",
            "usd_per_btc",
            None,
        ),
        (
            "kalshi",
            "current_market",
            "settlement_reference",
            "usd_per_btc",
            None,
        ),
        (
            "kalshi",
            "current_contract",
            "buyable_ask",
            "contract_cents",
            Some("yes"),
        ),
        (
            "kalshi",
            "current_contract",
            "bid",
            "contract_cents",
            Some("no"),
        ),
        (
            "kalshi",
            "current_contract",
            "last_trade",
            "contract_cents",
            Some("selected_side"),
        ),
    ] {
        let mut r = current_rule(ask_below());
        let mut input =
            json!({"id":"ask","source":source,"instrument":instrument,"field":field,"unit":unit});
        if let Some(side) = side {
            input["side"] = json!(side);
        }
        r["inputs"] = json!([input]);
        r["purchase_options"][0]["when"]["right"]["unit"] = json!(unit);
        let v = ValidatedStrategy::parse(&r.to_string()).unwrap();
        assert!(
            v.explanation()
                .contains(&format!("{source} {instrument} {field}"))
        );
        r["inputs"][0]["instrument"] = json!("eth_usd");
        assert_eq!(
            ValidatedStrategy::parse(&r.to_string()).unwrap_err()[0].code,
            "unsupported_binding"
        );
    }
    for (operation, meaning) in [
        ("difference", "minus"),
        ("absolute_difference", "absolute value"),
        ("percentage_difference", "zero reference"),
        ("change", "exact lookback-boundary"),
        ("percentage_change", "signed nonzero"),
        ("rolling_average", "time-weighted average"),
        ("rolling_minimum", "minimum of"),
        ("rolling_maximum", "maximum of"),
        ("rolling_range", "maximum minus minimum"),
    ] {
        let mut r = historical_rule("rolling_average", "100");
        let mut measurement = json!({"id":"measured","operation":operation});
        if operation.ends_with("difference") {
            measurement["left"] = json!({"reference":"underlying"});
            measurement[if operation == "percentage_difference" {
                "reference"
            } else {
                "right"
            }] = json!({"value":"1","unit":"usd_per_btc"});
        } else {
            measurement["of"] = json!({"reference":"underlying"});
            measurement["lookback_seconds"] = json!(1);
        }
        r["measurements"] = json!([measurement]);
        if operation.starts_with("percentage") {
            r["purchase_options"][0]["when"]["right"]["unit"] = json!("percent");
        }
        let v = ValidatedStrategy::parse(&r.to_string()).unwrap();
        assert!(v.explanation().contains(meaning), "{operation}");
        r["measurements"][0]["extra_operand"] = json!({"reference":"underlying"});
        assert!(ValidatedStrategy::parse(&r.to_string()).is_err());
    }
}

#[test]
fn phase1_numeric_failure_has_identity_and_does_not_block_an_independent_option() {
    let mut r = current_rule(ask_below());
    r["measurements"] = json!([
        {"id":"zero_reference","operation":"percentage_difference","left":{"value":"1","unit":"percent"},"reference":{"value":"0","unit":"percent"}},
        {"id":"too_large","operation":"percentage_difference","left":{"value":"999999999999","unit":"percent"},"reference":{"value":"0.000001","unit":"percent"}},
        {"id":"dependent_zero","operation":"difference","left":{"reference":"zero_reference"},"right":{"value":"1","unit":"percent"}},
        {"id":"dependent_range","operation":"difference","left":{"reference":"too_large"},"right":{"value":"1","unit":"percent"}},
        {"id":"transitive_zero","operation":"difference","left":{"reference":"dependent_zero"},"right":{"value":"1","unit":"percent"}}
    ]);
    let e = entered(r);
    assert!(e.snapshot().candidate.is_some());
    for (id, code) in [
        ("zero_reference", "zero_denominator"),
        ("too_large", "numeric_range_exceeded"),
        ("dependent_zero", "zero_denominator"),
        ("dependent_range", "numeric_range_exceeded"),
        ("transitive_zero", "zero_denominator"),
    ] {
        assert!(
            e.snapshot()
                .reasons
                .iter()
                .any(|reason| reason.path.contains(id) && reason.code == code)
        );
    }
}

#[test]
fn phase1_invalid_resynchronization_never_supplies_a_buyable_price() {
    let mut e = entered(current_rule(ask_below()));
    for (n, value) in [(3, "-1"), (4, "100.000001")] {
        let d = finish(
            &mut e,
            batch(
                1,
                n,
                vec![EvidenceChange::Resynchronize {
                    binding_id: 1,
                    value: Decimal::parse(value).unwrap(),
                }],
            ),
            Some(objective()),
        );
        assert!(d.candidate.is_none());
    }
}

#[test]
fn phase1_invalid_counting_observation_does_not_confirm_an_independent_true_condition() {
    let mut r = current_rule(
        json!({"type":"confirmed_updates","count":1,"count_input":"counter","condition":ask_below()}),
    );
    r["inputs"].as_array_mut().unwrap().push(json!({"id":"counter","source":"brti","instrument":"btc_usd","field":"index_value","unit":"usd_per_btc"}));
    let mut c = context();
    c.bindings.push(BindingDescriptor {
        binding_id: 3,
        input_id: "counter".into(),
        scope: BindingScope::Underlying,
    });
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut b = batch(1, 1, vec![]);
    let mut changes = prices("30", "70", 1);
    changes.push(EvidenceChange::MarketBoundary {
        market_id: "one".into(),
    });
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2), lease(3)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 1,
            changes,
        },
    ];
    assert!(finish(&mut e, b, Some(objective())).candidate.is_none());
    assert!(
        finish(
            &mut e,
            batch(1, 3, vec![observation_change(3, 1, "-1")]),
            Some(objective())
        )
        .candidate
        .is_none()
    );
    assert!(
        finish(
            &mut e,
            batch(1, 4, vec![observation_change(3, 2, "100")]),
            Some(objective())
        )
        .candidate
        .is_some()
    );
}

#[test]
fn phase1_later_hour_history_registration_matches_initial_registration_and_stops() {
    let mut r = format2();
    r["checkpoints"] = json!([]);
    r["measurements"] = json!([{"id":"hour_average", "operation":"rolling_average", "of":{"reference":"ask"}, "lookback_seconds":3600}]);
    r["purchase_options"] = json!([{"id":"buy", "side":{"type":"fixed","side":"yes"}, "when":{"type":"compare","left":{"reference":"hour_average"},"comparison":"less_than","right":{"value":"40","unit":"contract_cents"}}}]);
    // One fixed side keeps the admitted workload to one hour ring.
    r["inputs"][0]["side"] = json!("yes");
    let strategy = ValidatedStrategy::parse(&r.to_string()).unwrap();
    let mut initial = context();
    initial.bindings.retain(|b| b.binding_id == 1);
    let mut direct = Evaluator::new(strategy.clone(), initial.clone()).unwrap();
    let mut empty = initial.clone();
    empty.markets.clear();
    empty.bindings.clear();
    let registration = batch(
        0,
        1,
        vec![EvidenceChange::RegisterMarket {
            descriptor: initial.markets[0].clone(),
            bindings: initial.bindings.clone(),
        }],
    );
    let mut late = Evaluator::new(strategy.clone(), empty.clone()).unwrap();
    finish(&mut direct, batch(0, 0, vec![]), None);
    finish(&mut late, registration.clone(), None);
    assert_eq!(late.allocated_bytes(), direct.allocated_bytes());
    assert!(late.allocated_bytes() >= 8 * 1024 * 1024 + 3_600_000 * 16);
    let mut input = batch(1001, 2, vec![]);
    input.transactions = vec![
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![lease(1)],
        },
        NormalizedTransaction {
            number: 3,
            at_ms: 0,
            changes: vec![observation_change(1, 1, "30")],
        },
    ];
    input.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 1001,
    }];
    let a = finish(&mut direct, input.clone(), None);
    let b = finish(&mut late, input, None);
    assert_eq!(a, b);
    assert_eq!(a.progress[0].collected_ms, 1001);
    assert!(a.candidate.is_none());
    let mut stopped = Evaluator::new(strategy, empty).unwrap();
    stopped.begin(registration).unwrap();
    assert!(matches!(
        stopped.process_slice(SliceControl::Continue, None),
        SliceResult::Pending { .. }
    ));
    let published = stopped.snapshot().clone();
    assert_eq!(
        stopped.process_slice(SliceControl::Stop, None),
        SliceResult::Stopped
    );
    assert_eq!(stopped.snapshot(), &published);
}

#[test]
fn phase1_hold_over_any_preserves_the_true_alternatives_continuity() {
    let mut e = entered(current_rule(json!({
        "type":"held_for", "duration_seconds":1,
        "condition":{"type":"any","conditions":[ask_below(),
            {"type":"compare","left":{"reference":"ask"},"comparison":"greater_than",
             "right":{"value":"90","unit":"contract_cents"}}]}
    })));
    assert!(e.snapshot().candidate.is_none());
    assert!(
        finish(
            &mut e,
            covered_prices(1000, 3, "39", "61"),
            Some(objective())
        )
        .candidate
        .is_none()
    );
    assert!(
        finish(
            &mut e,
            covered_prices(1001, 4, "39", "61"),
            Some(objective())
        )
        .candidate
        .is_some()
    );
}

#[test]
fn phase1_after_checkpoint_requires_an_accepted_observation_in_the_active_market() {
    let mut r = current_rule(
        json!({"type":"after_checkpoint_update","checkpoint":"early","count_input":"counter"}),
    );
    r["checkpoints"] =
        json!([{"id":"early","when":ask_below(),"remember_side":{"type":"fixed","side":"yes"}}]);
    r["inputs"].as_array_mut().unwrap().push(json!({"id":"counter","source":"kalshi","instrument":"current_contract","field":"bid","unit":"contract_cents","side":"yes"}));
    let mut c = context();
    c.markets.push(descriptor("other", 900001));
    c.bindings.extend([
        binding(3, "counter", "one", Side::Yes),
        binding(4, "counter", "other", Side::Yes),
    ]);
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut changes = prices("30", "70", 1);
    changes.push(observation_change(3, 1, "10"));
    changes.push(EvidenceChange::MarketBoundary {
        market_id: "one".into(),
    });
    let mut b = batch(1, 1, vec![]);
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2), lease(3), lease(4)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 1,
            changes,
        },
    ];
    assert!(finish(&mut e, b, Some(objective())).candidate.is_none());
    for (transaction, change) in [
        (3, observation_change(4, 1, "20")),
        (4, observation_change(3, 2, "-1")),
        (5, observation_change(3, 2, "20")),
        (6, observation_change(3, 2, "20")),
    ] {
        let d = finish(
            &mut e,
            batch(1, transaction, vec![change]),
            Some(objective()),
        );
        assert_eq!(
            d.candidate.is_some(),
            transaction >= 5,
            "transaction {transaction}"
        );
        assert_eq!(d.checkpoints["early"].transaction, 2);
    }
    assert!(
        finish(&mut e, batch(2, 7, vec![]), Some(objective()))
            .candidate
            .is_some()
    );
}

#[test]
fn phase1_gap_effective_time_precedes_conditions_without_early_invalidation() {
    fn run(report_at: u64, start: u64) -> Decision {
        let mut r = format2();
        r["checkpoints"] = json!([{
            "id":"held", "when":{"type":"held_for","duration_seconds":1,
                "condition":{"type":"compare","left":{"reference":"ask"},
                    "comparison":"less_than","right":{"value":"40","unit":"contract_cents"}}},
            "remember_side":{"type":"fixed","side":"yes"}
        }]);
        r["purchase_options"] = json!([{"id":"buy","side":{"type":"fixed","side":"yes"},
            "when":{"type":"checkpoint_reached","checkpoint":"held"}}]);
        let mut e = entered(r);
        let mut b = batch(2001, 3, vec![]);
        b.transactions = vec![NormalizedTransaction {
            number: 3,
            at_ms: report_at,
            changes: vec![EvidenceChange::Gap {
                binding_id: 1,
                start_ms: start,
                end_ms: 2001,
                reason: "controlled outage".into(),
            }],
        }];
        b.coverage = vec![CoverageFact {
            binding_id: 1,
            start_ms: 1,
            end_ms: 2001,
        }];
        finish(&mut e, b, Some(objective()))
    }
    // A report received after the hold deadline still describes a gap before it.
    assert!(!run(2001, 501).checkpoints.contains_key("held"));
    // Knowing a future outage must not erase a hold completed before that outage.
    assert!(run(2, 1501).checkpoints.contains_key("held"));
}

#[test]
fn phase1_every_condition_comparison_and_selector_has_its_generated_meaning() {
    let comparison = ask_below();
    let values = json!({"left":{"reference":"ask"},"right":{"value":"40","unit":"contract_cents"}});
    let mut cases = Vec::new();
    for (op, meaning) in [
        ("less_than", "less than"),
        ("at_most", "at most"),
        ("equal_to", "equal to"),
        ("at_least", "at least"),
        ("greater_than", "greater than"),
    ] {
        let mut c = comparison.clone();
        c["comparison"] = json!(op);
        cases.push((c, meaning));
    }
    for (op, meaning) in [
        ("crossed_below", "strictly below"),
        ("crossed_above", "strictly above"),
    ] {
        let mut c = values.clone();
        c["type"] = json!(op);
        cases.push((c, meaning));
    }
    cases.extend([
        (json!({"type":"all","conditions":[comparison.clone()]}),"all of"),
        (json!({"type":"any","conditions":[comparison.clone()]}),"any of"),
        (json!({"type":"held_for","condition":comparison.clone(),"duration_seconds":1}),"false or unavailable resets the hold"),
        (json!({"type":"confirmed_updates","condition":comparison,"count":2,"count_input":"ask"}),"2 consecutive distinct observations of ask"),
        (json!({"type":"checkpoint_reached","checkpoint":"early"}),"checkpoint early is latched"),
        (json!({"type":"after_checkpoint_update","checkpoint":"early","count_input":"ask"}),"later transaction beyond its captured observation number"),
    ]);
    for (condition, meaning) in cases {
        let mut r = format2();
        r["purchase_options"][0]["when"] = condition;
        let v = ValidatedStrategy::parse(&r.to_string()).unwrap();
        assert!(
            v.explanation().contains(meaning),
            "{meaning}: {}",
            v.explanation()
        );
    }
    for (selector, meaning) in [
        (json!({"type":"fixed","side":"yes"}), "fixed Yes"),
        (json!({"type":"fixed","side":"no"}), "fixed No"),
        (
            json!({"type":"checkpoint_side","checkpoint":"early","relation":"same"}),
            "the same side of early",
        ),
        (
            json!({"type":"checkpoint_side","checkpoint":"early","relation":"opposite"}),
            "the opposite side of early",
        ),
        (
            json!({"type":"unique_side"}),
            "both true is ambiguous and either unavailable prevents selection",
        ),
    ] {
        let mut r = format2();
        r["purchase_options"][0]["side"] = selector;
        let v = ValidatedStrategy::parse(&r.to_string()).unwrap();
        assert!(v.explanation().contains(meaning));
    }
}

#[test]
fn phase1_old_contradiction_blocks_only_the_transitive_window_it_overlaps() {
    let mut r = historical_rule("rolling_average", "100");
    r["measurements"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"longer","operation":"rolling_average",
        "of":{"reference":"measured"},"lookback_seconds":1}));
    let mut preferred = r["purchase_options"][0].clone();
    preferred["id"] = json!("longer_first");
    preferred["when"]["left"]["reference"] = json!("longer");
    r["purchase_options"]
        .as_array_mut()
        .unwrap()
        .insert(0, preferred);
    let mut b = batch(3001, 1, vec![]);
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "100"),
                observation_change(2, 1, "30"),
            ],
        },
        NormalizedTransaction {
            number: 3,
            at_ms: 3001,
            changes: vec![EvidenceChange::MarketBoundary {
                market_id: "one".into(),
            }],
        },
    ];
    // This synthetic descriptor opens after both windows have collected.
    let mut c = historical_context();
    c.markets[0] = descriptor("one", 3001);
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    b.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 3001,
    }];
    assert_eq!(
        finish(&mut e, b, Some(objective()))
            .candidate
            .unwrap()
            .option_id,
        "longer_first"
    );
    let d = finish(
        &mut e,
        batch(
            3001,
            4,
            vec![EvidenceChange::Gap {
                binding_id: 1,
                start_ms: 1500,
                end_ms: 1600,
                reason: "contradicted older cells".into(),
            }],
        ),
        Some(objective()),
    );
    assert_ne!(d.candidate.as_ref().unwrap().option_id, "longer_first");
    assert!(
        d.reasons
            .iter()
            .any(|reason| reason.path.contains("longer"))
    );
}

#[test]
fn phase1_hold_requires_certified_continuity_and_recovers_without_a_new_market() {
    let rule =
        current_rule(json!({"type":"held_for","duration_seconds":1,"condition":ask_below()}));
    let mut missing = entered(rule.clone());
    assert!(
        finish(&mut missing, batch(1001, 3, vec![]), Some(objective()))
            .candidate
            .is_none()
    );
    // A late certificate cannot turn an already processed hole into a completed hold.
    let mut recovery = batch(2000, 3, vec![]);
    recovery.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 2000,
    }];
    assert!(
        finish(&mut missing, recovery, Some(objective()))
            .candidate
            .is_none()
    );
    let mut recovery = batch(2001, 3, vec![]);
    recovery.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 2000,
        end_ms: 2001,
    }];
    assert!(
        finish(&mut missing, recovery, Some(objective()))
            .candidate
            .is_some()
    );
    let mut covered = entered(rule);
    let mut quiet = batch(1001, 3, vec![]);
    quiet.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 1,
        end_ms: 1001,
    }];
    assert!(
        finish(&mut covered, quiet, Some(objective()))
            .candidate
            .is_some()
    );
}

#[test]
fn phase1_transitive_contradictions_stay_with_their_market_and_side() {
    for affected in 1..=4 {
        let r = json!({"format":2,"name":"Concrete transitive histories",
            "inputs":[{"id":"ask","source":"kalshi","instrument":"current_contract","field":"buyable_ask",
                "unit":"contract_cents","side":"selected_side"}],
            "measurements":[{"id":"average","operation":"rolling_average","of":{"reference":"ask"},"lookback_seconds":1},
                {"id":"adjusted","operation":"difference","left":{"reference":"average"},"right":{"value":"0","unit":"contract_cents"}},
                {"id":"smoothed","operation":"rolling_average","of":{"reference":"adjusted"},"lookback_seconds":1}],
            "checkpoints":[{"id":"early","remember_side":{"type":"unique_side"},"when":ask_below()}],
            "purchase_options":[{"id":"yes_first","side":{"type":"fixed","side":"yes"},"when":{"type":"compare",
                "left":{"reference":"smoothed"},"comparison":"less_than","right":{"value":"80","unit":"contract_cents"}}},
                {"id":"no_next","side":{"type":"fixed","side":"no"},"when":{"type":"compare",
                "left":{"reference":"smoothed"},"comparison":"less_than","right":{"value":"80","unit":"contract_cents"}}}]});
        let mut c = context();
        c.markets = vec![descriptor("one", 3001), descriptor("two", 3002)];
        c.bindings = vec![
            binding(1, "ask", "one", Side::Yes),
            binding(2, "ask", "one", Side::No),
            binding(3, "ask", "two", Side::Yes),
            binding(4, "ask", "two", Side::No),
        ];
        let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
        let mut initial = batch(3001, 1, vec![]);
        initial.transactions = vec![
            NormalizedTransaction {
                number: 1,
                at_ms: 0,
                changes: (1..=4).map(lease).collect(),
            },
            NormalizedTransaction {
                number: 2,
                at_ms: 0,
                changes: vec![
                    observation_change(1, 1, "30"),
                    observation_change(2, 1, "70"),
                    observation_change(3, 1, "70"),
                    observation_change(4, 1, "30"),
                ],
            },
            NormalizedTransaction {
                number: 3,
                at_ms: 3001,
                changes: vec![EvidenceChange::MarketBoundary {
                    market_id: "one".into(),
                }],
            },
        ];
        initial.coverage = (1..=4)
            .map(|binding_id| CoverageFact {
                binding_id,
                start_ms: 0,
                end_ms: 3001,
            })
            .collect();
        let established = finish(&mut e, initial, Some(objective()));
        assert_eq!(established.candidate.as_ref().unwrap().side, Side::Yes);
        assert_eq!(established.checkpoints["early"].side, Some(Side::Yes));
        let contradicted = finish(
            &mut e,
            batch(
                3001,
                4,
                vec![EvidenceChange::Gap {
                    binding_id: affected,
                    start_ms: 2000,
                    end_ms: 2001,
                    reason: "controlled contradiction".into(),
                }],
            ),
            Some(objective()),
        );
        assert_eq!(contradicted.checkpoints, established.checkpoints);
        assert_eq!(
            contradicted.candidate.unwrap().side,
            if affected == 1 { Side::No } else { Side::Yes }
        );
        let mut next = batch(
            3002,
            5,
            vec![EvidenceChange::MarketBoundary {
                market_id: "two".into(),
            }],
        );
        next.coverage = (1..=4)
            .map(|binding_id| CoverageFact {
                binding_id,
                start_ms: 3001,
                end_ms: 3002,
            })
            .collect();
        let mut next_objective = objective();
        next_objective.market_id = "two".into();
        let next = finish(&mut e, next, Some(next_objective));
        assert_eq!(
            next.candidate.unwrap().side,
            if affected == 3 { Side::No } else { Side::Yes }
        );
        if affected <= 2 {
            assert_eq!(next.checkpoints["early"].side, Some(Side::No));
            assert_eq!(next.checkpoints["early"].at_ms, 3002);
        } else {
            assert!(next.checkpoints.is_empty());
        }
    }
}

#[test]
fn phase1_atomic_updates_do_not_expose_intermediate_equal_time_values() {
    for atomic in [true, false] {
        let mut r = current_rule(json!({"type":"checkpoint_reached","checkpoint":"equal_asks"}));
        r["inputs"][0]["side"] = json!("yes");
        let mut no = r["inputs"][0].clone();
        no["id"] = json!("no_ask");
        no["side"] = json!("no");
        r["inputs"].as_array_mut().unwrap().push(no);
        r["checkpoints"] = json!([{"id":"equal_asks","when":{"type":"compare","left":{"reference":"ask"},"comparison":"equal_to","right":{"reference":"no_ask"}}}]);
        let mut c = context();
        c.bindings[1].input_id = "no_ask".into();
        let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
        let mut initial = batch(1, 2, prices("39", "61", 1));
        initial.transactions[0]
            .changes
            .push(EvidenceChange::MarketBoundary {
                market_id: "one".into(),
            });
        initial.transactions.insert(
            0,
            NormalizedTransaction {
                number: 1,
                at_ms: 0,
                changes: vec![lease(1), lease(2)],
            },
        );
        assert!(
            finish(&mut e, initial, Some(objective()))
                .checkpoints
                .is_empty()
        );
        let mut updates = batch(2, 3, prices("61", "39", 2));
        if !atomic {
            let last = updates.transactions[0].changes.pop().unwrap();
            updates.transactions.push(NormalizedTransaction {
                number: 4,
                at_ms: 2,
                changes: vec![last],
            });
        }
        let decision = finish(&mut e, updates, Some(objective()));
        assert_eq!(decision.checkpoints.contains_key("equal_asks"), !atomic);
        assert_eq!(decision.candidate.is_some(), !atomic);
        if !atomic {
            assert_eq!(decision.checkpoints["equal_asks"].transaction, 3);
        }
    }
}

#[test]
fn phase1_unique_side_uses_ancestor_conditions_and_requires_proven_uniqueness() {
    let always = json!({"type":"compare","left":{"value":"1","unit":"contract_cents"},"comparison":"equal_to","right":{"value":"1","unit":"contract_cents"}});
    let mut r = current_rule(always.clone());
    r["purchase_options"] = json!([{"id":"group","when":ask_below(),"options":[{"id":"unique","side":{"type":"unique_side"},"when":always}]}]);
    let mut e = entered(r);
    assert_eq!(e.snapshot().candidate.as_ref().unwrap().side, Side::Yes);
    for (number, yes, no, expected) in [
        (3, "30", "30", None),
        (4, "50", "50", None),
        (5, "50", "30", Some(Side::No)),
        (6, "30", "50", Some(Side::Yes)),
    ] {
        let d = finish(
            &mut e,
            batch(2, number, prices(yes, no, number)),
            Some(objective()),
        );
        assert_eq!(d.candidate.map(|candidate| candidate.side), expected);
    }
    let unavailable = batch(
        2,
        7,
        vec![EvidenceChange::Validity {
            binding_id: 2,
            valid_until_ms: 1000,
            reason: Some("controlled unavailable side".into()),
        }],
    );
    assert!(
        finish(&mut e, unavailable, Some(objective()))
            .candidate
            .is_none()
    );
}

#[test]
fn phase1_finished_zero_fill_reselects_nested_priority_after_unresolved_attempt() {
    let mut r = current_rule(ask_below());
    r["purchase_options"] = json!([
        {"id":"group","when":ask_below(),"options":[{"id":"preferred","side":{"type":"fixed","side":"no"},"when":ask_below()}]},
        {"id":"fallback","side":{"type":"fixed","side":"yes"},"when":ask_below()}
    ]);
    let mut e = entered(r);
    assert_eq!(
        e.snapshot().candidate.as_ref().unwrap().option_id,
        "fallback"
    );
    for (number, state) in [(3, AttemptState::InFlight), (4, AttemptState::Unknown)] {
        let mut o = objective();
        o.attempt_state = state;
        assert!(
            finish(
                &mut e,
                batch(2, number, prices("30", "30", number)),
                Some(o)
            )
            .candidate
            .is_none()
        );
    }
    let mut o = objective();
    o.attempt_state = AttemptState::Finished;
    let d = finish(&mut e, batch(2, 5, prices("30", "30", 5)), Some(o));
    let candidate = d.candidate.unwrap();
    assert_eq!(candidate.option_id, "preferred");
    assert_eq!(candidate.side, Side::No);
    assert_eq!(candidate.remaining_quantity, 10);
}

#[test]
fn phase1_history_ready_exactly_at_open_waits_for_a_later_boundary() {
    let r = historical_rule("rolling_average", "100");
    let mut c = historical_context();
    c.markets = vec![descriptor("one", 1000), descriptor("two", 1001)];
    c.bindings.push(binding(3, "ask", "two", Side::Yes));
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut b = batch(1000, 1, vec![]);
    b.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2), lease(3)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "100"),
                observation_change(2, 1, "25"),
                observation_change(3, 1, "25"),
            ],
        },
        NormalizedTransaction {
            number: 3,
            at_ms: 1000,
            changes: vec![EvidenceChange::MarketBoundary {
                market_id: "one".into(),
            }],
        },
    ];
    b.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 1000,
    }];
    let d = finish(&mut e, b, Some(objective()));
    assert_eq!(d.phase, "Ready — waiting for next market");
    assert!(d.candidate.is_none());
    let mut next = batch(
        1001,
        4,
        vec![EvidenceChange::MarketBoundary {
            market_id: "two".into(),
        }],
    );
    next.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 1000,
        end_ms: 1001,
    }];
    let mut o = objective();
    o.market_id = "two".into();
    assert_eq!(
        finish(&mut e, next, Some(o)).candidate.unwrap().market_id,
        "two"
    );
}

#[test]
fn phase1_quiet_derived_excursion_resets_a_hold_between_source_observations() {
    let mut r = historical_rule("rolling_average", "100");
    r["checkpoints"] = json!([{"id":"held","when":{"type":"held_for","duration_seconds":1,"condition":{"type":"compare","left":{"reference":"measured"},"comparison":"less_than","right":{"value":"160","unit":"usd_per_btc"}}}}]);
    r["purchase_options"][0]["when"] = json!({"type":"checkpoint_reached","checkpoint":"held"});
    let mut e = Evaluator::new(
        ValidatedStrategy::parse(&r.to_string()).unwrap(),
        historical_context(),
    )
    .unwrap();
    let mut initial = batch(1001, 1, vec![]);
    initial.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "100"),
                observation_change(2, 1, "25"),
            ],
        },
        NormalizedTransaction {
            number: 3,
            at_ms: 250,
            changes: vec![observation_change(1, 2, "200")],
        },
        NormalizedTransaction {
            number: 4,
            at_ms: 750,
            changes: vec![observation_change(1, 3, "100")],
        },
        NormalizedTransaction {
            number: 5,
            at_ms: 1000,
            changes: vec![observation_change(1, 4, "150")],
        },
        NormalizedTransaction {
            number: 6,
            at_ms: 1001,
            changes: vec![EvidenceChange::MarketBoundary {
                market_id: "one".into(),
            }],
        },
    ];
    initial.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 1001,
    }];
    assert!(
        finish(&mut e, initial, Some(objective()))
            .checkpoints
            .is_empty()
    );
    // The average is below 160 at both endpoints, but reaches 162.5 at 1250 ms.
    // It falls strictly below 160 again at 1301 ms; no new source update occurs.
    let mut previous = 1001;
    for (time, expected) in [(2001, false), (2300, false), (2301, true)] {
        let mut quiet = batch(time, 7, vec![]);
        quiet.coverage = vec![CoverageFact {
            binding_id: 1,
            start_ms: previous,
            end_ms: time,
        }];
        let d = finish(&mut e, quiet, Some(objective()));
        assert_eq!(d.checkpoints.contains_key("held"), expected, "time {time}");
        assert_eq!(d.candidate.is_some(), expected, "time {time}");
        previous = time;
    }
}

#[test]
fn phase1_strict_json_rejects_duplicate_tagged_fields_and_numeric_coercions() {
    let raw = current_rule(ask_below()).to_string();
    for duplicate in [
        raw.replacen("\"format\":2", "\"format\":2,\"format\":2", 1),
        raw.replacen(
            "\"type\":\"compare\"",
            "\"type\":\"compare\",\"type\":\"compare\"",
            1,
        ),
    ] {
        let errors = ValidatedStrategy::parse(&duplicate).unwrap_err();
        assert_eq!(errors[0].code, "duplicate");
    }
    for malformed in [
        raw.replacen("\"format\":2", "\"format\":2.0", 1),
        raw.replacen("\"format\":2", "\"format\":2e0", 1),
        raw.replacen("\"format\":2", "\"format\":\"2\"", 1),
        raw.replacen("\"checkpoints\":[]", "\"checkpoints\":null", 1),
        raw.replacen("\"value\":\"40\"", "\"value\":40", 1),
        format!("{raw} null"),
        format!("// comment\n{raw}"),
    ] {
        assert!(ValidatedStrategy::parse(&malformed).is_err(), "{malformed}");
    }
}

#[test]
fn phase1_checkpoint_only_source_can_disappear_but_bid_and_trade_never_replace_ask() {
    let mut r = current_rule(
        json!({"type":"all","conditions":[{"type":"checkpoint_reached","checkpoint":"signal_seen"},{"type":"compare","left":{"reference":"ask"},"comparison":"at_most","right":{"value":"30","unit":"contract_cents"}}]}),
    );
    r["inputs"].as_array_mut().unwrap().extend([
        json!({"id":"signal","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}),
        json!({"id":"bid","source":"kalshi","instrument":"current_contract","field":"bid","unit":"contract_cents","side":"yes"}),
        json!({"id":"trade","source":"kalshi","instrument":"current_contract","field":"last_trade","unit":"contract_cents","side":"yes"}),
    ]);
    r["checkpoints"] = json!([{"id":"signal_seen","remember_side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"signal"},"comparison":"greater_than","right":{"value":"100","unit":"usd_per_btc"}}}]);
    let mut c = context();
    c.bindings.extend([
        BindingDescriptor {
            binding_id: 3,
            input_id: "signal".into(),
            scope: BindingScope::Underlying,
        },
        binding(4, "bid", "one", Side::Yes),
        binding(5, "trade", "one", Side::Yes),
    ]);
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut initial = batch(
        1,
        2,
        vec![
            observation_change(1, 1, "35"),
            observation_change(2, 1, "65"),
            observation_change(3, 1, "101"),
            observation_change(4, 1, "25"),
            observation_change(5, 1, "28"),
            EvidenceChange::MarketBoundary {
                market_id: "one".into(),
            },
        ],
    );
    initial.transactions.insert(
        0,
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: (1..=5).map(lease).collect(),
        },
    );
    let before = finish(&mut e, initial, Some(objective()));
    assert!(before.candidate.is_none());
    assert_eq!(before.checkpoints["signal_seen"].side, Some(Side::Yes));
    let unavailable = batch(
        2,
        3,
        vec![EvidenceChange::Validity {
            binding_id: 3,
            valid_until_ms: 1000,
            reason: Some("controlled source loss".into()),
        }],
    );
    let d = finish(&mut e, unavailable, Some(objective()));
    assert_eq!(d.checkpoints, before.checkpoints);
    assert!(d.candidate.is_none());
    let d = finish(
        &mut e,
        batch(3, 4, prices("29", "71", 2)),
        Some(objective()),
    );
    assert_eq!(d.checkpoints, before.checkpoints);
    assert_eq!(d.candidate.unwrap().side, Side::Yes);
}

#[test]
fn phase1_confirmation_resets_when_a_different_dependency_fails() {
    let mut r = current_rule(
        json!({"type":"confirmed_updates","count":2,"count_input":"ask","condition":{"type":"all","conditions":[ask_below(),{"type":"compare","left":{"reference":"signal"},"comparison":"greater_than","right":{"value":"100","unit":"usd_per_btc"}}]}}),
    );
    r["inputs"].as_array_mut().unwrap().push(json!({"id":"signal","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}));
    let mut c = context();
    c.bindings.push(BindingDescriptor {
        binding_id: 3,
        input_id: "signal".into(),
        scope: BindingScope::Underlying,
    });
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut initial = batch(
        1,
        2,
        vec![
            observation_change(1, 1, "30"),
            observation_change(2, 1, "70"),
            observation_change(3, 1, "101"),
            EvidenceChange::MarketBoundary {
                market_id: "one".into(),
            },
        ],
    );
    initial.transactions.insert(
        0,
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2), lease(3)],
        },
    );
    assert!(
        finish(&mut e, initial, Some(objective()))
            .candidate
            .is_none()
    );
    for (transaction, change, eligible) in [
        (3, observation_change(3, 2, "99"), false),
        (4, observation_change(3, 3, "101"), false),
        (5, observation_change(1, 2, "30"), false),
        (6, observation_change(1, 2, "30"), false),
        (7, observation_change(1, 3, "30"), true),
    ] {
        assert_eq!(
            finish(
                &mut e,
                batch(1, transaction, vec![change]),
                Some(objective())
            )
            .candidate
            .is_some(),
            eligible,
            "transaction {transaction}"
        );
    }
}

#[test]
fn phase1_five_minute_readiness_waits_until_next_full_fifteen_minute_market() {
    let mut r = historical_rule("rolling_average", "100");
    r["measurements"][0]["lookback_seconds"] = json!(300);
    let mut c = historical_context();
    c.markets = vec![descriptor("one", 0), descriptor("two", 900000)];
    c.bindings.push(binding(3, "ask", "two", Side::Yes));
    let mut e = Evaluator::new(ValidatedStrategy::parse(&r.to_string()).unwrap(), c).unwrap();
    let mut initial = batch(300000, 1, vec![]);
    initial.transactions = vec![
        NormalizedTransaction {
            number: 1,
            at_ms: 0,
            changes: vec![lease(1), lease(2), lease(3)],
        },
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![
                observation_change(1, 1, "100"),
                observation_change(2, 1, "25"),
                observation_change(3, 1, "25"),
                EvidenceChange::MarketBoundary {
                    market_id: "one".into(),
                },
            ],
        },
    ];
    initial.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 300000,
    }];
    let ready = finish(&mut e, initial, None);
    assert_eq!(ready.phase, "Ready — waiting for next market");
    assert!(ready.candidate.is_none());
    let mut next = batch(
        900000,
        3,
        vec![EvidenceChange::MarketBoundary {
            market_id: "two".into(),
        }],
    );
    next.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 300000,
        end_ms: 900000,
    }];
    let mut o = objective();
    o.market_id = "two".into();
    let entered = finish(&mut e, next, Some(o));
    assert_eq!(entered.candidate.unwrap().market_id, "two");
}

#[test]
fn phase1_editing_rules_does_not_change_an_existing_evaluators_meaning() {
    let mut document = current_rule(ask_below());
    let original_text = document.to_string();
    let original = ValidatedStrategy::parse(&original_text).unwrap();
    let original_explanation = original.explanation().to_owned();
    let mut running = entered(document.clone());
    document["purchase_options"][0]["when"]["right"]["value"] = json!("20");
    let revised = ValidatedStrategy::parse(&document.to_string()).unwrap();
    assert_ne!(revised.explanation(), original_explanation);
    assert_eq!(original.json(), original_text);
    assert_eq!(original.explanation(), original_explanation);
    assert!(entered(document).snapshot().candidate.is_none());
    assert!(
        finish(
            &mut running,
            batch(2, 3, prices("30", "70", 2)),
            Some(objective())
        )
        .candidate
        .is_some()
    );
}

#[test]
fn phase1_oversized_matching_registration_identifier_rejects_and_releases_admission() {
    let mut c = context();
    c.markets.clear();
    c.bindings.clear();
    c.series_id = "s".repeat(1024 * 1024);
    let mut market = descriptor("one", 1);
    market.series_id = c.series_id.clone();
    let mut e = Evaluator::new(
        ValidatedStrategy::parse(&current_rule(ask_below()).to_string()).unwrap(),
        c,
    )
    .unwrap();
    let before = e.snapshot().clone();
    e.begin(batch(
        0,
        1,
        vec![EvidenceChange::RegisterMarket {
            descriptor: market,
            bindings: vec![],
        }],
    ))
    .unwrap();
    loop {
        match e.process_slice(SliceControl::Continue, None) {
            SliceResult::Pending { .. } => (),
            SliceResult::Rejected(error) => {
                assert_eq!(error.code, "incompatible_market");
                break;
            }
            other => panic!("{other:?}"),
        }
    }
    assert_eq!(e.snapshot(), &before);
    let recovered = finish(&mut e, batch(0, 1, vec![]), None);
    assert_eq!(recovered.evaluated_through_ms, 0);
    assert!(recovered.candidate.is_none());
    assert!(recovered.checkpoints.is_empty());
}

#[test]
fn phase1_nested_option_reasons_identify_the_actual_rule_path() {
    let mut rules = current_rule(ask_below());
    let mut leaf = rules["purchase_options"][0].clone();
    leaf["when"]["right"]["value"] = json!("20");
    let mut second = leaf.clone();
    second["id"] = json!("second");
    rules["purchase_options"] = json!([{
        "id":"group", "when":ask_below(), "options":[leaf, second]
    }]);
    let evaluator = entered(rules);
    assert!(evaluator.snapshot().candidate.is_none());
    assert_eq!(
        evaluator
            .snapshot()
            .reasons
            .iter()
            .map(|reason| reason.path.as_str())
            .collect::<Vec<_>>(),
        [
            "$.purchase_options[0].options[0]",
            "$.purchase_options[0].options[1]"
        ]
    );
}

#[test]
fn phase1_missing_evidence_is_reported_for_fixed_and_remembered_sides() {
    for (rules, time) in [(current_rule(ask_below()), 1), (format2(), 60001)] {
        let mut evaluator = entered(rules);
        let checkpoints = evaluator.snapshot().checkpoints.clone();
        let decision = finish(
            &mut evaluator,
            batch(time, 3, prices("-1", "101", 2)),
            Some(objective()),
        );
        assert!(decision.candidate.is_none());
        assert_eq!(decision.checkpoints, checkpoints);
        assert!(
            decision
                .leaves
                .iter()
                .all(|leaf| leaf.reason == "unavailable"),
            "{decision:?}"
        );
        assert!(
            decision
                .reasons
                .iter()
                .all(|reason| reason.code == "unavailable")
        );
    }
}

#[test]
#[ignore = "one controlled complete pre-open contract hour, run separately in release mode"]
fn phase1_registered_contract_hour_survives_its_exact_market_entry() {
    let mut rules = historical_rule("rolling_average", "39");
    rules["inputs"].as_array_mut().unwrap().remove(0);
    rules["measurements"][0]["of"] = json!({"reference":"ask"});
    rules["measurements"][0]["lookback_seconds"] = json!(3600);
    rules["purchase_options"][0]["when"]["right"]["unit"] = json!("contract_cents");
    rules["checkpoints"] = json!([{"id":"early", "when":ask_below()}]);
    let mut run = context();
    run.markets.clear();
    run.bindings.clear();
    let mut evaluator =
        Evaluator::new(ValidatedStrategy::parse(&rules.to_string()).unwrap(), run).unwrap();
    let started = std::time::Instant::now();
    finish(
        &mut evaluator,
        batch(
            0,
            1,
            vec![EvidenceChange::RegisterMarket {
                descriptor: descriptor("two", 3_600_001),
                bindings: vec![binding(1, "ask", "two", Side::Yes)],
            }],
        ),
        None,
    );
    let mut hour = batch(3_600_000, 2, vec![]);
    hour.transactions = vec![
        NormalizedTransaction {
            number: 2,
            at_ms: 0,
            changes: vec![EvidenceChange::Validity {
                binding_id: 1,
                valid_until_ms: 4_500_001,
                reason: None,
            }],
        },
        NormalizedTransaction {
            number: 3,
            at_ms: 0,
            changes: vec![observation_change(1, 1, "39")],
        },
    ];
    hour.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 0,
        end_ms: 3_600_000,
    }];
    let ready = finish(&mut evaluator, hour, None);
    assert_eq!(ready.phase, "Ready — waiting for next market");
    assert_eq!(ready.progress[0].collected_ms, 3_600_000);
    assert!(ready.checkpoints.is_empty());
    assert!(ready.candidate.is_none());
    let mut next = batch(
        3_600_001,
        4,
        vec![EvidenceChange::MarketBoundary {
            market_id: "two".into(),
        }],
    );
    next.coverage = vec![CoverageFact {
        binding_id: 1,
        start_ms: 3_600_000,
        end_ms: 3_600_001,
    }];
    let mut objective = objective();
    objective.market_id = "two".into();
    let active = finish(&mut evaluator, next, Some(objective));
    assert_eq!(active.phase, "Active");
    assert_eq!(active.progress[0].collected_ms, 3_600_000);
    assert_eq!(active.checkpoints["early"].at_ms, 3_600_001);
    let candidate = active.candidate.unwrap();
    assert_eq!(candidate.market_id, "two");
    assert_eq!(candidate.side, Side::Yes);
    assert_eq!(candidate.remaining_quantity, 10);
    println!(
        "allocated_bytes={} elapsed_ms={}",
        evaluator.allocated_bytes(),
        started.elapsed().as_millis()
    );
}
