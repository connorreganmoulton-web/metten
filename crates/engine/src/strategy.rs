//! Bounded declarative rules; no feeds or execution.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Yes,
    No,
}
impl Side {
    pub fn opposite(self) -> Self {
        match self {
            Self::Yes => Self::No,
            Self::No => Self::Yes,
        }
    }
}
const SCALE: i128 = 1_000_000;
const LIMIT: i128 = 1_000_000_000_000_000_000_000_000;
const RESERVE: usize = 8 * 1024 * 1024;
const MEMORY_LIMIT: usize = 256 * 1024 * 1024;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct RuleError {
    pub path: String,
    pub code: &'static str,
    pub message: &'static str,
    pub accepted_form: &'static str,
}
fn rule_error(path: &str, code: &'static str) -> RuleError {
    fixed_work(1);
    RuleError {
        path: path.into(),
        code,
        message: match code {
            "numeric_range_exceeded" => {
                "This calculation exceeds the format-2 numeric range; dependent values are unavailable."
            }
            "zero_denominator" => {
                "The reference value is zero; this percentage and its dependents are unavailable."
            }
            "unavailable" => "This value needs valid current inputs and complete required history.",
            "resource_limit" => "The rule exceeds the rules_2_initial resource profile.",
            "reference" => "An identifier is missing, duplicated or cannot be used here.",
            "incompatible_units" => "Operands must identify compatible quantities and units.",
            "unreachable_duration" => {
                "This duration cannot complete before the exclusive interval end."
            }
            "unbound_side" => "This condition needs an enclosing side selector.",
            "unsupported_binding" => {
                "This source, instrument, field, unit and side combination is unsupported."
            }
            "cycle" => "References must form an acyclic dependency graph.",
            "unknown" => "This object contains an unsupported field.",
            _ => "Rule is outside the supported contract.",
        },
        accepted_form: match code {
            "resource_limit" => "Reduce the named workload to fit rules_2_initial.",
            "unreachable_duration" => {
                "Shorten the hold or extend its allowed interval so completion occurs strictly before the end."
            }
            "incompatible_units" => {
                "Use operands with the same physical quantity, unit and compatible instrument identity."
            }
            "unknown" => {
                "Remove unsupported fields; use only the documented fields for this object."
            }
            _ => "Use the format-2 contract at this field.",
        },
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleUnit {
    ContractCents,
    UsdPerBtc,
    Percent,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
enum ValueSpec {
    Reference { reference: String },
    Literal { value: String, unit: RuleUnit },
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct InputSpec {
    id: String,
    source: String,
    instrument: String,
    field: String,
    unit: RuleUnit,
    side: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum Calculation {
    Difference {
        left: ValueSpec,
        right: ValueSpec,
    },
    AbsoluteDifference {
        left: ValueSpec,
        right: ValueSpec,
    },
    PercentageDifference {
        left: ValueSpec,
        reference: ValueSpec,
    },
    Change {
        of: ValueSpec,
        lookback_seconds: u32,
    },
    PercentageChange {
        of: ValueSpec,
        lookback_seconds: u32,
    },
    RollingAverage {
        of: ValueSpec,
        lookback_seconds: u32,
    },
    RollingMinimum {
        of: ValueSpec,
        lookback_seconds: u32,
    },
    RollingMaximum {
        of: ValueSpec,
        lookback_seconds: u32,
    },
    RollingRange {
        of: ValueSpec,
        lookback_seconds: u32,
    },
}
#[derive(Clone, Debug)]
struct MeasurementSpec {
    id: String,
    calculation: Calculation,
}
impl<'de> Deserialize<'de> for MeasurementSpec {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut fields = serde_json::Map::<String, serde_json::Value>::deserialize(d)?;
        let id = fields
            .remove("id")
            .and_then(|x| x.as_str().map(str::to_owned))
            .ok_or_else(|| serde::de::Error::custom("measurement id required"))?;
        let calculation = serde_json::from_value(fields.into())
            .map_err(|_| serde::de::Error::custom("invalid measurement"))?;
        Ok(Self { id, calculation })
    }
}
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ComparisonOp {
    LessThan,
    AtMost,
    EqualTo,
    AtLeast,
    GreaterThan,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum Predicate {
    Compare {
        left: ValueSpec,
        comparison: ComparisonOp,
        right: ValueSpec,
    },
    All {
        conditions: Vec<Predicate>,
    },
    Any {
        conditions: Vec<Predicate>,
    },
    CrossedBelow {
        left: ValueSpec,
        right: ValueSpec,
    },
    CrossedAbove {
        left: ValueSpec,
        right: ValueSpec,
    },
    HeldFor {
        condition: Box<Predicate>,
        duration_seconds: u32,
    },
    ConfirmedUpdates {
        condition: Box<Predicate>,
        count: u32,
        count_input: String,
    },
    CheckpointReached {
        checkpoint: String,
    },
    AfterCheckpointUpdate {
        checkpoint: String,
        count_input: String,
    },
}
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum Selector {
    Fixed {
        side: Side,
    },
    CheckpointSide {
        checkpoint: String,
        relation: RuleRelation,
    },
    UniqueSide,
}
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RuleRelation {
    Same,
    Opposite,
}
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Interval {
    start_seconds: u32,
    end_seconds: u32,
}
impl Default for Interval {
    fn default() -> Self {
        Self {
            start_seconds: 0,
            end_seconds: 900,
        }
    }
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointSpec {
    id: String,
    when: Predicate,
    #[serde(default)]
    during: Interval,
    remember_side: Option<Selector>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionSpec {
    id: String,
    when: Predicate,
    #[serde(default)]
    during: Interval,
    options: Option<Vec<OptionSpec>>,
    side: Option<Selector>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    format: u32,
    name: String,
    inputs: Vec<InputSpec>,
    #[serde(default)]
    measurements: Vec<MeasurementSpec>,
    #[serde(default)]
    checkpoints: Vec<CheckpointSpec>,
    purchase_options: Vec<OptionSpec>,
}

/// Exact six-place values. No binary floating point or feed rounding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Decimal(i128);
impl Decimal {
    pub fn parse(text: &str) -> Result<Self, RuleError> {
        let fail = || rule_error("$", "range");
        let s = text.strip_prefix('-').unwrap_or(text);
        let mut parts = s.split('.');
        let whole = parts.next().ok_or_else(fail)?;
        let frac = parts.next();
        if whole.is_empty()
            || whole.len() > 12
            || !whole
                .bytes()
                .inspect(inspect_entry)
                .all(|b| b.is_ascii_digit())
            || (whole.len() > 1 && whole.starts_with('0'))
            || parts.next().is_some()
        {
            return Err(fail());
        }
        let f = frac.unwrap_or("");
        if (frac.is_some() && f.is_empty())
            || f.len() > 6
            || !f.bytes().inspect(inspect_entry).all(|b| b.is_ascii_digit())
        {
            return Err(fail());
        }
        fixed_work(1);
        vector_copy_work(whole.len() + f.len());
        let mut n = whole.parse::<i128>().map_err(|_| fail())? * SCALE;
        if !f.is_empty() {
            n += f.parse::<i128>().map_err(|_| fail())? * 10_i128.pow(6 - f.len() as u32);
        }
        if text.starts_with('-') {
            if n == 0 {
                return Err(fail());
            }
            n = -n;
        }
        Ok(Self(n))
    }
    pub fn millionths(self) -> i128 {
        self.0
    }
}
impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.0.abs();
        let fraction = format!("{:06}", n % SCALE);
        let fraction = fraction.trim_end_matches('0');
        write!(
            f,
            "{}{}{}{}",
            if self.0 < 0 { "-" } else { "" },
            n / SCALE,
            if fraction.is_empty() { "" } else { "." },
            fraction
        )
    }
}
fn divide_even(a: i128, b: i128) -> Option<i128> {
    fixed_work(1);
    let q = a.checked_div(b)?;
    let r = a.checked_rem(b)?;
    let twice = r.checked_abs()?.checked_mul(2)?;
    let den = b.checked_abs()?;
    if twice > den || (twice == den && q % 2 != 0) {
        q.checked_add(if (a < 0) == (b < 0) { 1 } else { -1 })
    } else {
        Some(q)
    }
}

// This visitor rejects duplicates before serde's tagged/untagged buffering can erase them.
struct StrictJson;
impl<'de> serde::de::Visitor<'de> for StrictJson {
    type Value = serde_json::Value;
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "bounded JSON without duplicate keys or null")
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(v.into())
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v.into())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v.into())
    }
    fn visit_f64<E: serde::de::Error>(self, _: f64) -> Result<Self::Value, E> {
        Err(E::custom("integer token required"))
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        if v.len() > 1024 {
            Err(E::custom("string limit"))
        } else {
            Ok(v.into())
        }
    }
    fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
        let mut v = Vec::new();
        while let Some(x) = a.next_element::<StrictValue>()? {
            v.push(x.0);
        }
        Ok(v.into())
    }
    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
        let mut v = serde_json::Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if k.len() > 1024 {
                return Err(serde::de::Error::custom("string limit"));
            }
            if v.contains_key(&k) {
                return Err(serde::de::Error::custom("duplicate_key"));
            }
            v.insert(k, a.next_value::<StrictValue>()?.0);
        }
        Ok(v.into())
    }
}
struct StrictValue(serde_json::Value);
impl<'de> Deserialize<'de> for StrictValue {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(StrictJson).map(Self)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Requirements {
    pub support_profile: &'static str,
    pub planned_bytes: usize,
    pub startup_ms: std::collections::BTreeMap<String, u64>,
    pub per_node_bytes: std::collections::BTreeMap<String, usize>,
    pub shared_underlying_bytes: usize,
    pub per_market_bytes: usize,
    pub implicit_buyable_asks: Vec<String>,
}
#[derive(Clone, Copy, Debug)]
struct NumericInfo {
    dependencies: u32,
    horizons: [u64; 32],
    market: bool,
    selected: bool,
}
#[derive(Clone, Debug)]
pub struct ValidatedStrategy {
    text: String,
    document: Document,
    order: Vec<usize>,
    checkpoint_order: Vec<usize>,
    numeric_scopes: Vec<(bool, bool)>,
    numeric_info: std::collections::BTreeMap<String, NumericInfo>,
    requirements: Requirements,
    explanation: String,
}
impl ValidatedStrategy {
    pub fn json(&self) -> &str {
        &self.text
    }
    pub fn name(&self) -> &str {
        &self.document.name
    }
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
    pub fn requirements(&self) -> &Requirements {
        &self.requirements
    }
    pub fn parse(text: &str) -> Result<Self, Vec<RuleError>> {
        Self::validate(text).map_err(|e| vec![e])
    }
    fn validate(text: &str) -> Result<Self, RuleError> {
        let err = |code| rule_error("$", code);
        if text.len() > 256 * 1024 {
            return Err(err("resource_limit"));
        }
        let mut depth = 0;
        let mut quoted = false;
        let mut escaped = false;
        for b in text.bytes() {
            if quoted {
                if escaped {
                    escaped = false;
                } else if b == b'\\' {
                    escaped = true;
                } else if b == b'"' {
                    quoted = false;
                }
            } else {
                match b {
                    b'"' => quoted = true,
                    b'{' | b'[' => {
                        depth += 1;
                        if depth > 48 {
                            return Err(err("resource_limit"));
                        }
                    }
                    b'}' | b']' => depth -= 1,
                    _ => (),
                }
            }
        }
        let strict: StrictValue = serde_json::from_str(text).map_err(|e| {
            err(if e.to_string().starts_with("duplicate_key") {
                "duplicate"
            } else {
                "malformed"
            })
        })?;
        validate_shape(&strict.0, "document", "$")?;
        let document: Document = serde_json::from_value(strict.0).map_err(|_| err("malformed"))?;
        if document.format != 2 {
            return Err(rule_error("$.format", "unsupported_format"));
        }
        if document.name.trim().is_empty()
            || document.name.chars().count() > 120
            || document.name.chars().any(char::is_control)
        {
            return Err(rule_error("$.name", "range"));
        }
        if document.inputs.is_empty()
            || document.inputs.len() > 32
            || document.measurements.len() > 64
            || document.checkpoints.len() > 32
            || document.purchase_options.is_empty()
        {
            return Err(err("resource_limit"));
        }
        let mut ids = BTreeSet::new();
        for (i, input) in document.inputs.iter().enumerate() {
            let p = format!("$.inputs[{i}]");
            check_id(&input.id, &mut ids, &p)?;
            let valid = matches!(
                (
                    input.source.as_str(),
                    input.instrument.as_str(),
                    input.field.as_str(),
                    input.unit,
                    input.side.as_deref(),
                ),
                (
                    "kalshi",
                    "current_contract",
                    "buyable_ask" | "bid" | "last_trade",
                    RuleUnit::ContractCents,
                    Some("yes" | "no" | "selected_side"),
                ) | ("brti", "btc_usd", "index_value", RuleUnit::UsdPerBtc, None)
                    | (
                        "coinbase" | "kraken",
                        "btc_usd",
                        "last_trade",
                        RuleUnit::UsdPerBtc,
                        None
                    )
                    | (
                        "kalshi",
                        "current_market",
                        "opening_reference" | "settlement_reference",
                        RuleUnit::UsdPerBtc,
                        None,
                    )
            );
            if !valid {
                return Err(rule_error(&p, "unsupported_binding"));
            }
        }
        for (i, m) in document.measurements.iter().enumerate() {
            check_id(&m.id, &mut ids, &format!("$.measurements[{i}].id"))?;
        }
        for (i, c) in document.checkpoints.iter().enumerate() {
            check_id(&c.id, &mut ids, &format!("$.checkpoints[{i}].id"))?;
        }
        let mut visiting = BTreeSet::new();
        let mut resolved = std::collections::BTreeMap::new();
        let mut order = Vec::new();
        let mut bytes = RESERVE;
        let mut startup = std::collections::BTreeMap::new();
        for (i, input) in document.inputs.iter().enumerate() {
            resolved.insert(
                input.id.clone(),
                (
                    input.unit,
                    0u64,
                    input.side.as_deref() == Some("selected_side"),
                ),
            );
            startup.insert(input.id.clone(), 0);
            let _ = i;
        }
        for i in 0..document.measurements.len() {
            resolve_measurement(
                i,
                &document,
                &mut visiting,
                &mut resolved,
                &mut order,
                0,
                &mut bytes,
            )?;
        }
        for m in &document.measurements {
            if graph_depth(&m.id, &document, 0) > 12 {
                return Err(rule_error("$.measurements", "resource_limit"));
            }
        }
        let expanded = document
            .inputs
            .iter()
            .map(|x| {
                if x.side.as_deref() == Some("selected_side") {
                    2
                } else {
                    1
                }
            })
            .sum::<usize>()
            + document
                .measurements
                .iter()
                .map(|x| if resolved[&x.id].2 { 2 } else { 1 })
                .sum::<usize>();
        if expanded + 2 > 128 || bytes > MEMORY_LIMIT {
            return Err(err("resource_limit"));
        }
        // Propagate cumulative source horizons through every operand path.
        for m in &document.measurements {
            source_horizons(&m.id, 0, &document, &mut startup)?;
        }
        let mut count = 0;
        let mut options = BTreeSet::new();
        let mut cp_earliest = std::collections::BTreeMap::new();
        let mut cp_visiting = BTreeSet::new();
        let mut checkpoint_order = Vec::new();
        for i in 0..document.checkpoints.len() {
            checkpoint_earliest(
                i,
                &document,
                &resolved,
                &mut cp_earliest,
                &mut cp_visiting,
                &mut count,
                &mut checkpoint_order,
            )?;
        }
        let mut leaves = 0;
        for (i, o) in document.purchase_options.iter().enumerate() {
            validate_option(
                o,
                Interval::default(),
                0,
                &format!("$.purchase_options[{i}]"),
                &document,
                &resolved,
                &cp_earliest,
                &mut options,
                &mut count,
                &mut leaves,
            )?;
        }
        for (i, o) in document.purchase_options.iter().enumerate() {
            option_selector_reachability(
                o,
                Interval::default(),
                &[],
                &format!("$.purchase_options[{i}]"),
                &document,
                &resolved,
                &cp_earliest,
            )?;
        }
        if count > 512 || leaves == 0 {
            return Err(err("resource_limit"));
        }
        if condition_instances(&document) > 1024 {
            return Err(rule_error("$.purchase_options", "resource_limit"));
        }
        // Resolve shared dependencies once; runtime traversal must not expand a reused DAG.
        let mut numeric_info = std::collections::BTreeMap::new();
        for (i, input) in document.inputs.iter().enumerate() {
            numeric_info.insert(
                input.id.clone(),
                NumericInfo {
                    dependencies: 1u32 << i,
                    horizons: [0; 32],
                    market: input.instrument != "btc_usd",
                    selected: input.side.as_deref() == Some("selected_side"),
                },
            );
        }
        for &i in &order {
            let m = &document.measurements[i];
            let mut info = NumericInfo {
                dependencies: 0,
                horizons: [0; 32],
                market: false,
                selected: false,
            };
            for value in calculation_values(&m.calculation) {
                if let ValueSpec::Reference { reference } = value {
                    let operand = numeric_info[reference];
                    for (index, horizon) in info.horizons.iter_mut().enumerate() {
                        if operand.dependencies & (1u32 << index) != 0 {
                            *horizon =
                                (*horizon).max(operand.horizons[index] + lookback(&m.calculation));
                        }
                    }
                    info.dependencies |= operand.dependencies;
                    info.market |= operand.market;
                    info.selected |= operand.selected;
                }
            }
            numeric_info.insert(m.id.clone(), info);
        }
        let mut per_node_bytes = std::collections::BTreeMap::new();
        let mut shared_underlying_bytes = 0usize;
        let mut per_market_bytes = 0usize;
        for m in &document.measurements {
            let cost = history_bytes(&m.calculation)
                .checked_mul(if resolved[&m.id].2 { 2 } else { 1 })
                .ok_or_else(|| err("resource_limit"))?;
            per_node_bytes.insert(m.id.clone(), cost);
            let total = if numeric_info[&m.id].market {
                &mut per_market_bytes
            } else {
                &mut shared_underlying_bytes
            };
            *total = total
                .checked_add(cost)
                .ok_or_else(|| err("resource_limit"))?;
        }
        let implicit_buyable_asks = ["yes", "no"]
            .into_iter()
            .filter(|side| {
                !document.inputs.iter().any(|i| {
                    i.field == "buyable_ask"
                        && (i.side.as_deref() == Some(*side)
                            || i.side.as_deref() == Some("selected_side"))
                })
            })
            .map(|side| format!("__buyable_ask_{side}"))
            .collect();
        let requirements = Requirements {
            support_profile: "rules_2_initial",
            planned_bytes: bytes,
            startup_ms: startup,
            per_node_bytes,
            shared_underlying_bytes,
            per_market_bytes,
            implicit_buyable_asks,
        };
        let explanation = explain(&document, &requirements);
        if explanation.len() > 128 * 1024 {
            return Err(err("resource_limit"));
        }
        let numeric_scopes = document
            .measurements
            .iter()
            .map(|m| (numeric_info[&m.id].market, resolved[&m.id].2))
            .collect();
        Ok(Self {
            text: text.into(),
            document,
            order,
            checkpoint_order,
            numeric_scopes,
            numeric_info,
            requirements,
            explanation,
        })
    }
}
fn check_id(id: &str, ids: &mut BTreeSet<String>, p: &str) -> Result<(), RuleError> {
    if id.is_empty()
        || id.len() > 64
        || !id.as_bytes()[0].is_ascii_lowercase()
        || !id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        || !ids.insert(id.into())
    {
        Err(rule_error(p, "reference"))
    } else {
        Ok(())
    }
}
type Resolved = std::collections::BTreeMap<String, (RuleUnit, u64, bool)>;
fn calculation_values(c: &Calculation) -> Vec<&ValueSpec> {
    fixed_work(1);
    let values = match c {
        Calculation::Difference { left, right }
        | Calculation::AbsoluteDifference { left, right } => vec![left, right],
        Calculation::PercentageDifference { left, reference } => vec![left, reference],
        Calculation::Change { of, .. }
        | Calculation::PercentageChange { of, .. }
        | Calculation::RollingAverage { of, .. }
        | Calculation::RollingMinimum { of, .. }
        | Calculation::RollingMaximum { of, .. }
        | Calculation::RollingRange { of, .. } => vec![of],
    };
    vector_copy_work(values.len());
    values
}
fn lookback(c: &Calculation) -> u64 {
    fixed_work(1);
    u64::from(match c {
        Calculation::Change {
            lookback_seconds, ..
        }
        | Calculation::PercentageChange {
            lookback_seconds, ..
        }
        | Calculation::RollingAverage {
            lookback_seconds, ..
        }
        | Calculation::RollingMinimum {
            lookback_seconds, ..
        }
        | Calculation::RollingMaximum {
            lookback_seconds, ..
        }
        | Calculation::RollingRange {
            lookback_seconds, ..
        } => *lookback_seconds,
        _ => 0,
    }) * 1000
}
fn value_type(v: &ValueSpec, r: &Resolved, p: &str) -> Result<(RuleUnit, u64, bool), RuleError> {
    match v {
        ValueSpec::Reference { reference } => r
            .get(reference)
            .copied()
            .ok_or_else(|| rule_error(p, "reference")),
        ValueSpec::Literal { value, unit } => {
            Decimal::parse(value).map_err(|_| rule_error(p, "range"))?;
            Ok((*unit, 0, false))
        }
    }
}
#[allow(clippy::too_many_arguments)]
fn resolve_measurement(
    i: usize,
    d: &Document,
    vis: &mut BTreeSet<usize>,
    r: &mut Resolved,
    order: &mut Vec<usize>,
    depth: usize,
    bytes: &mut usize,
) -> Result<(), RuleError> {
    let m = &d.measurements[i];
    let p = format!("$.measurements[{i}]");
    if r.contains_key(&m.id) {
        return Ok(());
    }
    if depth >= 12 || !vis.insert(i) {
        return Err(rule_error(&p, "cycle"));
    }
    let vals = calculation_values(&m.calculation);
    for v in &vals {
        if let ValueSpec::Reference { reference } = v
            && !r.contains_key(reference)
        {
            let n = d
                .measurements
                .iter()
                .position(|x| x.id == *reference)
                .ok_or_else(|| rule_error(&p, "reference"))?;
            resolve_measurement(n, d, vis, r, order, depth + 1, bytes)?;
        }
    }
    let types = vals
        .iter()
        .map(|v| value_type(v, r, &p))
        .collect::<Result<Vec<_>, _>>()?;
    if types.iter().any(|t| t.0 != types[0].0) {
        return Err(rule_error(&p, "incompatible_units"));
    }
    let w = lookback(&m.calculation);
    if w > 3600000
        || (vals.len() == 1 && (w == 0 || !matches!(vals[0], ValueSpec::Reference { .. })))
    {
        return Err(rule_error(&p, "range"));
    }
    let warm = types.iter().map(|t| t.1).max().unwrap_or(0) + w;
    if warm > 14400000 {
        return Err(rule_error(&p, "resource_limit"));
    }
    let selected = types.iter().any(|t| t.2);
    let cost = history_bytes(&m.calculation);
    *bytes = bytes
        .checked_add(cost * if selected { 2 } else { 1 })
        .ok_or_else(|| rule_error(&p, "resource_limit"))?;
    let unit = if matches!(
        m.calculation,
        Calculation::PercentageDifference { .. } | Calculation::PercentageChange { .. }
    ) {
        RuleUnit::Percent
    } else {
        types[0].0
    };
    r.insert(m.id.clone(), (unit, warm, selected));
    vis.remove(&i);
    order.push(i);
    Ok(())
}
fn source_horizons(
    id: &str,
    extra: u64,
    d: &Document,
    out: &mut std::collections::BTreeMap<String, u64>,
) -> Result<(), RuleError> {
    if let Some(m) = d.measurements.iter().find(|m| m.id == id) {
        let n = extra + lookback(&m.calculation);
        for v in calculation_values(&m.calculation) {
            if let ValueSpec::Reference { reference } = v {
                source_horizons(reference, n, d, out)?;
            }
        }
    } else if let Some(x) = out.get_mut(id) {
        *x = (*x).max(extra);
    }
    Ok(())
}
fn interval(i: Interval, parent: Interval, p: &str) -> Result<Interval, RuleError> {
    if i.start_seconds >= i.end_seconds || i.end_seconds > 900 {
        return Err(rule_error(p, "interval"));
    }
    let x = Interval {
        start_seconds: i.start_seconds.max(parent.start_seconds),
        end_seconds: i.end_seconds.min(parent.end_seconds),
    };
    if x.start_seconds >= x.end_seconds {
        Err(rule_error(p, "interval"))
    } else {
        Ok(x)
    }
}
fn selector_valid(s: &Selector, d: &Document, p: &str) -> Result<(), RuleError> {
    if let Selector::CheckpointSide { checkpoint, .. } = s
        && !d
            .checkpoints
            .iter()
            .any(|c| c.id == *checkpoint && c.remember_side.is_some())
    {
        return Err(rule_error(p, "reference"));
    }
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn predicate_bound(
    c: &Predicate,
    d: &Document,
    r: &Resolved,
    cp: &std::collections::BTreeMap<String, u64>,
    start: u64,
    end: u64,
    bound: bool,
    depth: usize,
    count: &mut usize,
    p: &str,
) -> Result<u64, RuleError> {
    *count += 1;
    if depth >= 12 {
        return Err(rule_error(p, "resource_limit"));
    }
    let pair = |a: &ValueSpec, b: &ValueSpec| -> Result<(), RuleError> {
        let a = value_type(a, r, p)?;
        let b = value_type(b, r, p)?;
        if a.0 != b.0 {
            return Err(rule_error(p, "incompatible_units"));
        }
        if !bound && (a.2 || b.2) {
            return Err(rule_error(p, "unbound_side"));
        }
        Ok(())
    };
    let earliest = match c {
        Predicate::Compare { left, right, .. }
        | Predicate::CrossedBelow { left, right }
        | Predicate::CrossedAbove { left, right } => {
            pair(left, right)?;
            start
        }
        Predicate::All { conditions } | Predicate::Any { conditions } => {
            if conditions.is_empty() {
                return Err(rule_error(p, "range"));
            }
            let mut times = Vec::new();
            for (i, c) in conditions.iter().enumerate() {
                times.push(predicate_bound(
                    c,
                    d,
                    r,
                    cp,
                    start,
                    end,
                    bound,
                    depth + 1,
                    count,
                    &format!("{p}.conditions[{i}]"),
                )?);
            }
            if matches!(c, Predicate::All { .. }) {
                *times.iter().max().unwrap()
            } else {
                *times.iter().min().unwrap()
            }
        }
        Predicate::HeldFor {
            condition,
            duration_seconds,
        } => {
            if *duration_seconds == 0 || *duration_seconds > 86400 {
                return Err(rule_error(p, "range"));
            }
            predicate_bound(
                condition,
                d,
                r,
                cp,
                start,
                end,
                bound,
                depth + 1,
                count,
                &format!("{p}.condition"),
            )? + u64::from(*duration_seconds) * 1000
        }
        Predicate::ConfirmedUpdates {
            condition,
            count: n,
            count_input,
        } => {
            if *n == 0 || *n > 10000 || !d.inputs.iter().any(|i| i.id == *count_input) {
                return Err(rule_error(p, "reference"));
            }
            predicate_bound(
                condition,
                d,
                r,
                cp,
                start,
                end,
                bound,
                depth + 1,
                count,
                &format!("{p}.condition"),
            )?
        }
        Predicate::CheckpointReached { checkpoint }
        | Predicate::AfterCheckpointUpdate { checkpoint, .. } => {
            if let Predicate::AfterCheckpointUpdate { count_input, .. } = c
                && !d.inputs.iter().any(|i| i.id == *count_input)
            {
                return Err(rule_error(p, "reference"));
            }
            start.max(
                *cp.get(checkpoint)
                    .ok_or_else(|| rule_error(p, "reference"))?,
            )
        }
    };
    if earliest >= end {
        return Err(rule_error(p, "unreachable_duration"));
    }
    Ok(earliest)
}
fn predicate_checkpoints(c: &Predicate, out: &mut BTreeSet<String>) {
    match c {
        Predicate::CheckpointReached { checkpoint }
        | Predicate::AfterCheckpointUpdate { checkpoint, .. } => {
            out.insert(checkpoint.clone());
        }
        Predicate::All { conditions } | Predicate::Any { conditions } => {
            for c in conditions {
                predicate_checkpoints(c, out)
            }
        }
        Predicate::HeldFor { condition, .. } | Predicate::ConfirmedUpdates { condition, .. } => {
            predicate_checkpoints(condition, out)
        }
        _ => (),
    }
}
fn checkpoint_earliest(
    i: usize,
    d: &Document,
    r: &Resolved,
    cp: &mut std::collections::BTreeMap<String, u64>,
    vis: &mut BTreeSet<usize>,
    count: &mut usize,
    order: &mut Vec<usize>,
) -> Result<(), RuleError> {
    let c = &d.checkpoints[i];
    let p = format!("$.checkpoints[{i}]");
    if cp.contains_key(&c.id) {
        return Ok(());
    }
    if !vis.insert(i) {
        return Err(rule_error(&p, "cycle"));
    }
    let mut refs = BTreeSet::new();
    predicate_checkpoints(&c.when, &mut refs);
    if let Some(Selector::CheckpointSide { checkpoint, .. }) = &c.remember_side {
        refs.insert(checkpoint.clone());
    }
    for id in refs {
        let j = d
            .checkpoints
            .iter()
            .position(|c| c.id == id)
            .ok_or_else(|| rule_error(&p, "reference"))?;
        checkpoint_earliest(j, d, r, cp, vis, count, order)?;
    }
    if let Some(s) = &c.remember_side {
        selector_valid(s, d, &p)?;
    }
    let window = interval(c.during, Interval::default(), &p)?;
    let earliest = predicate_bound(
        &c.when,
        d,
        r,
        cp,
        (u64::from(window.start_seconds) * 1000).max(match &c.remember_side {
            Some(Selector::CheckpointSide { checkpoint, .. }) => cp[checkpoint],
            _ => 0,
        }),
        u64::from(window.end_seconds) * 1000,
        c.remember_side.is_some(),
        0,
        count,
        &p,
    )?;
    cp.insert(c.id.clone(), earliest);
    vis.remove(&i);
    order.push(i);
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn validate_option(
    o: &OptionSpec,
    parent: Interval,
    depth: usize,
    p: &str,
    d: &Document,
    r: &Resolved,
    cp: &std::collections::BTreeMap<String, u64>,
    ids: &mut BTreeSet<String>,
    count: &mut usize,
    leaves: &mut usize,
) -> Result<(), RuleError> {
    check_id(&o.id, ids, p)?;
    *count += 1;
    if depth >= 8 {
        return Err(rule_error(p, "resource_limit"));
    }
    let w = interval(o.during, parent, p)?;
    predicate_bound(
        &o.when,
        d,
        r,
        cp,
        u64::from(w.start_seconds) * 1000,
        u64::from(w.end_seconds) * 1000,
        true,
        0,
        count,
        p,
    )?;
    match (&o.options, &o.side) {
        (Some(children), None) if !children.is_empty() => {
            for (i, c) in children.iter().enumerate() {
                validate_option(
                    c,
                    w,
                    depth + 1,
                    &format!("{p}.options[{i}]"),
                    d,
                    r,
                    cp,
                    ids,
                    count,
                    leaves,
                )?;
            }
        }
        (None, Some(s)) => {
            selector_valid(s, d, p)?;
            *leaves += 1;
        }
        _ => return Err(rule_error(p, "malformed")),
    }
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn option_selector_reachability(
    o: &OptionSpec,
    parent: Interval,
    ancestors: &[(&Predicate, Interval, String)],
    path: &str,
    d: &Document,
    resolved: &Resolved,
    cp: &std::collections::BTreeMap<String, u64>,
) -> Result<(), RuleError> {
    let window = interval(o.during, parent, path)?;
    let mut conditions = ancestors.to_vec();
    conditions.push((&o.when, window, path.into()));
    if let Some(children) = &o.options {
        for (i, child) in children.iter().enumerate() {
            option_selector_reachability(
                child,
                window,
                &conditions,
                &format!("{path}.options[{i}]"),
                d,
                resolved,
                cp,
            )?;
        }
    } else {
        let side_ready = match &o.side {
            Some(Selector::CheckpointSide { checkpoint, .. }) => cp[checkpoint],
            _ => 0,
        };
        for (condition, own_window, p) in conditions {
            predicate_bound(
                condition,
                d,
                resolved,
                cp,
                (u64::from(own_window.start_seconds) * 1000).max(side_ready),
                u64::from(window.end_seconds) * 1000,
                true,
                0,
                &mut 0,
                &p,
            )?;
        }
    }
    Ok(())
}
fn explain_value(v: &ValueSpec) -> String {
    match v {
        ValueSpec::Reference { reference } => reference.clone(),
        ValueSpec::Literal { value, unit } => format!("{value} {unit:?}"),
    }
}
fn explain_condition(c: &Predicate) -> String {
    match c {
        Predicate::Compare {
            left,
            comparison,
            right,
        } => format!(
            "{} {} {}",
            explain_value(left),
            match comparison {
                ComparisonOp::LessThan => "less than",
                ComparisonOp::AtMost => "at most",
                ComparisonOp::EqualTo => "equal to",
                ComparisonOp::AtLeast => "at least",
                ComparisonOp::GreaterThan => "greater than",
            },
            explain_value(right)
        ),
        Predicate::All { conditions } | Predicate::Any { conditions } => format!(
            "{} ({})",
            if matches!(c, Predicate::All { .. }) {
                "all of"
            } else {
                "any of"
            },
            conditions
                .iter()
                .map(explain_condition)
                .collect::<Vec<_>>()
                .join("; ")
        ),
        Predicate::CrossedBelow { left, right } | Predicate::CrossedAbove { left, right } => {
            format!(
                "{} crosses {} {} on a genuine observation after a continuous observed baseline",
                explain_value(left),
                if matches!(c, Predicate::CrossedBelow { .. }) {
                    "strictly below"
                } else {
                    "strictly above"
                },
                explain_value(right)
            )
        }
        Predicate::HeldFor {
            condition,
            duration_seconds,
        } => format!(
            "({}) remains true with complete continuous evidence for {duration_seconds} seconds; false or unavailable resets the hold",
            explain_condition(condition)
        ),
        Predicate::ConfirmedUpdates {
            condition,
            count,
            count_input,
        } => format!(
            "({}) on {count} consecutive distinct observations of {count_input}; false or unavailable resets the count",
            explain_condition(condition)
        ),
        Predicate::CheckpointReached { checkpoint } => {
            format!("checkpoint {checkpoint} is latched")
        }
        Predicate::AfterCheckpointUpdate {
            checkpoint,
            count_input,
        } => format!(
            "checkpoint {checkpoint} is latched and {count_input} has a genuinely new observation in a later transaction beyond its captured observation number"
        ),
    }
}
fn explain_selector(s: &Selector) -> String {
    match s {Selector::Fixed{side}=>format!("fixed {side:?}"),Selector::CheckpointSide{checkpoint,relation}=>format!("the {} side of {checkpoint}",if matches!(relation,RuleRelation::Same){"same"}else{"opposite"}),Selector::UniqueSide=>"exactly one qualifying side with the other false; both true is ambiguous and either unavailable prevents selection".into()}
}
fn explain(d: &Document, r: &Requirements) -> String {
    let mut s = format!(
        "{} — metten_rules_2. Calculations use six decimal places and one-millisecond time steps. Each named calculation rounds to nearest with ties to even. Source access is unverified.\n",
        d.name
    );
    for i in &d.inputs {
        s.push_str(&format!(
            "{}: {} {} {}, {:?}, side {:?}. Required post-Go history {} milliseconds.\n",
            i.id, i.source, i.instrument, i.field, i.unit, i.side, r.startup_ms[&i.id]
        ));
    }
    for m in &d.measurements {
        let operands = calculation_values(&m.calculation)
            .into_iter()
            .map(explain_value)
            .collect::<Vec<_>>();
        let meaning = match m.calculation {
            Calculation::Difference { .. } => format!("{} minus {}", operands[0], operands[1]),
            Calculation::AbsoluteDifference { .. } => {
                format!("absolute value of {} minus {}", operands[0], operands[1])
            }
            Calculation::PercentageDifference { .. } => format!(
                "100 times ({} minus {}) divided by {}; a zero reference is unavailable",
                operands[0], operands[1], operands[1]
            ),
            Calculation::Change { .. } => format!(
                "current {} minus its exact lookback-boundary value",
                operands[0]
            ),
            Calculation::PercentageChange { .. } => format!(
                "100 times the change in {} divided by its signed nonzero lookback-boundary value",
                operands[0]
            ),
            Calculation::RollingAverage { .. } => format!(
                "time-weighted average of {} over every completed millisecond cell",
                operands[0]
            ),
            Calculation::RollingMinimum { .. } => format!(
                "minimum of {} over every completed millisecond cell",
                operands[0]
            ),
            Calculation::RollingMaximum { .. } => format!(
                "maximum of {} over every completed millisecond cell",
                operands[0]
            ),
            Calculation::RollingRange { .. } => format!(
                "maximum minus minimum of {} over every completed millisecond cell",
                operands[0]
            ),
        };
        s.push_str(&format!("{}: {meaning}; lookback {} milliseconds. Missing required coverage makes this value unavailable.\n",m.id,lookback(&m.calculation)));
    }
    for c in &d.checkpoints {
        s.push_str(&format!("Checkpoint {}: during [{}, {}) seconds, latch when {}; remember {}. A completed checkpoint survives gaps until market reset.\n",c.id,c.during.start_seconds,c.during.end_seconds,explain_condition(&c.when),c.remember_side.as_ref().map(explain_selector).unwrap_or_else(||"a Boolean only".into())));
    }
    fn options(nodes: &[OptionSpec], s: &mut String, depth: usize) {
        for o in nodes {
            s.push_str(&format!(
                "{}Option {}: during [{}, {}) seconds and when {}",
                "  ".repeat(depth),
                o.id,
                o.during.start_seconds,
                o.during.end_seconds,
                explain_condition(&o.when)
            ));
            if let Some(side) = &o.side {
                s.push_str(&format!(", consider buying {}.\n", explain_selector(side)));
            } else {
                s.push_str("; gate every child for that child's side.\n");
            }
            if let Some(children) = &o.options {
                options(children, s, depth + 1);
            }
        }
    }
    options(&d.purchase_options, &mut s, 0);
    s.push_str("Intervals include start and exclude end; nested windows intersect. The first eligible leaf in depth-first array order wins. Missing evidence is unavailable only to dependent paths. Verified quiet coverage adds no observations, confirmations or crossings. Completed checkpoints survive gaps. Collect all required post-Go history, then wait for the next full market; collection becoming ready exactly at open waits for the following open. Market entry clears checkpoints and temporal/objective state while preserving that exact entering contract's history and compatible underlying history; another contract's history cannot substitute. Unknown/in-flight attempts block all purchases. Confirmed zero fill permits reselection; any fill locks the side and leaves only the original remaining quantity. Every candidate requires its current buyable ask, user price range and common safeguards. No trade is executed.\n");
    for (node, bytes) in &r.per_node_bytes {
        s.push_str(&format!("Calculation {node}: {bytes} history bytes per resolved market instance (including side expansion).\n"));
    }
    s.push_str(&format!("Shared underlying history: {} bytes; history replicated per concrete market: {} bytes; fixed non-history reserve: {} bytes.\n", r.shared_underlying_bytes, r.per_market_bytes, RESERVE));
    for input in &r.implicit_buyable_asks {
        s.push_str(&format!("Required implicit binding {input}: kalshi current_contract buyable_ask, contract_cents, exact market/contract and matching side. Source access is unverified.\n"));
    }
    s.push_str(&format!(
        "Support profile {}: planned {} bytes.\n",
        r.support_profile, r.planned_bytes
    ));
    s
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceError {
    pub code: &'static str,
}
fn evidence(code: &'static str) -> EvidenceError {
    fixed_work(1);
    EvidenceError { code }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketDescriptor {
    pub series_id: String,
    pub market_id: String,
    pub contract_id: String,
    pub open_ms: i64,
    pub close_ms: i64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindingScope {
    Underlying,
    Market {
        market_id: String,
    },
    Contract {
        market_id: String,
        contract_id: String,
        side: Side,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindingDescriptor {
    pub binding_id: u32,
    pub input_id: String,
    pub scope: BindingScope,
}
#[derive(Clone, Debug)]
pub struct RunContext {
    pub strategy_version: String,
    pub run_id: String,
    pub go_time: String,
    pub series_id: String,
    pub markets: Vec<MarketDescriptor>,
    pub bindings: Vec<BindingDescriptor>,
}
#[derive(Clone, Debug)]
pub enum EvidenceChange {
    Observation {
        binding_id: u32,
        observation_number: u64,
        value: Decimal,
    },
    Resynchronize {
        binding_id: u32,
        value: Decimal,
    },
    Validity {
        binding_id: u32,
        valid_until_ms: u64,
        reason: Option<String>,
    },
    Gap {
        binding_id: u32,
        start_ms: u64,
        end_ms: u64,
        reason: String,
    },
    RegisterMarket {
        descriptor: MarketDescriptor,
        bindings: Vec<BindingDescriptor>,
    },
    MarketBoundary {
        market_id: String,
    },
}
#[derive(Clone, Debug)]
pub struct NormalizedTransaction {
    pub number: u64,
    pub at_ms: u64,
    pub changes: Vec<EvidenceChange>,
}
#[derive(Clone, Debug)]
pub struct CoverageFact {
    pub binding_id: u32,
    pub start_ms: u64,
    pub end_ms: u64,
}
#[derive(Clone, Debug)]
pub enum Admission {
    Allowed,
    Blocked(&'static str),
}
#[derive(Clone, Debug)]
pub struct AdvanceBatch {
    pub received_through_ms: u64,
    pub target_ms: u64,
    pub transactions: Vec<NormalizedTransaction>,
    pub coverage: Vec<CoverageFact>,
    pub current_admission: Admission,
}
impl AdvanceBatch {
    /// Owned ingress storage, including spare capacity and nested metadata.
    pub fn ingress_bytes(&self) -> Result<usize, EvidenceError> {
        fn charge(total: &mut usize, count: usize, size: usize) -> Result<(), EvidenceError> {
            *total = count
                .checked_mul(size)
                .and_then(|n| total.checked_add(n))
                .filter(|n| *n <= RESERVE)
                .ok_or_else(|| evidence("resource_limit"))?;
            Ok(())
        }
        let mut bytes = std::mem::size_of::<Self>();
        charge(
            &mut bytes,
            self.transactions.capacity(),
            std::mem::size_of::<NormalizedTransaction>(),
        )?;
        charge(
            &mut bytes,
            self.coverage.capacity(),
            std::mem::size_of::<CoverageFact>(),
        )?;
        for transaction in &self.transactions {
            charge(
                &mut bytes,
                transaction.changes.capacity(),
                std::mem::size_of::<EvidenceChange>(),
            )?;
            for change in &transaction.changes {
                match change {
                    EvidenceChange::Validity {
                        reason: Some(reason),
                        ..
                    }
                    | EvidenceChange::Gap { reason, .. } => {
                        charge(&mut bytes, reason.capacity(), 1)?
                    }
                    EvidenceChange::MarketBoundary { market_id } => {
                        charge(&mut bytes, market_id.capacity(), 1)?
                    }
                    EvidenceChange::RegisterMarket {
                        descriptor,
                        bindings,
                    } => {
                        for id in [
                            &descriptor.series_id,
                            &descriptor.market_id,
                            &descriptor.contract_id,
                        ] {
                            charge(&mut bytes, id.capacity(), 1)?;
                        }
                        charge(
                            &mut bytes,
                            bindings.capacity(),
                            std::mem::size_of::<BindingDescriptor>(),
                        )?;
                        for binding in bindings {
                            charge(&mut bytes, binding.input_id.capacity(), 1)?;
                            match &binding.scope {
                                BindingScope::Underlying => (),
                                BindingScope::Market { market_id } => {
                                    charge(&mut bytes, market_id.capacity(), 1)?
                                }
                                BindingScope::Contract {
                                    market_id,
                                    contract_id,
                                    ..
                                } => {
                                    charge(&mut bytes, market_id.capacity(), 1)?;
                                    charge(&mut bytes, contract_id.capacity(), 1)?;
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok(bytes)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttemptState {
    None,
    InFlight,
    Unknown,
    Finished,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PriceRange {
    pub minimum: Decimal,
    pub maximum: Decimal,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Safeguards {
    pub allowed: bool,
    pub yes: PriceRange,
    pub no: PriceRange,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectiveView {
    pub market_id: String,
    pub requested_quantity: u64,
    pub confirmed_quantity: u64,
    pub locked_side: Option<Side>,
    pub attempt_state: AttemptState,
    pub safeguards: Safeguards,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    pub market_id: String,
    pub option_id: String,
    pub side: Side,
    pub remaining_quantity: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckpointSnapshot {
    pub side: Option<Side>,
    pub at_ms: u64,
    pub transaction: u64,
    pub observations: std::collections::BTreeMap<u32, u64>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeafResult {
    pub option_id: String,
    pub eligible: bool,
    pub reason: &'static str,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryProgress {
    pub binding_id: u32,
    pub input_id: String,
    pub required_ms: u64,
    pub collected_ms: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    pub phase: &'static str,
    pub progress: Vec<HistoryProgress>,
    pub evaluated_through_ms: u64,
    pub checkpoints: std::collections::BTreeMap<String, CheckpointSnapshot>,
    pub leaves: Vec<LeafResult>,
    pub candidate: Option<Candidate>,
    pub reasons: Vec<RuleError>,
    pub repeated_waits: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliceControl {
    Continue,
    Stop,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SliceResult {
    Pending {
        phase: &'static str,
        evaluated_through_ms: u64,
    },
    Complete(Decision),
    Stopped,
    Rejected(EvidenceError),
    Failed {
        code: &'static str,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Truth {
    True,
    False,
    Unavailable,
}
#[derive(Clone, Debug, Default)]
struct Temporal {
    since: Option<u64>,
    count: u32,
    last_number: u64,
    baseline: Option<(i128, i128)>,
    baseline_ms: u64,
}
#[derive(Clone, Debug)]
struct RawState {
    dependencies: u32,
    selected: bool,
    descriptor: BindingDescriptor,
    value: Option<i128>,
    number: u64,
    valid_until: u64,
    last_observed_value: Option<i128>,
    observed_transaction: u64,
    covered_previous: bool,
    cell_covered: bool,
    covered_cells: u64,
    bad_cell: bool,
}
#[derive(Debug)]
struct History {
    ring: Vec<i128>,
    ring_len: usize,
    cursor: usize,
    count: u64,
    sum: i128,
    bad: Option<u64>,
    minimum: std::collections::VecDeque<(i128, u64)>,
    maximum: std::collections::VecDeque<(i128, u64)>,
    insertion_stage: u8,
}
impl History {
    fn new(c: &Calculation) -> Result<Self, EvidenceError> {
        fixed_work(1);
        let w = lookback(c) as usize;
        let delay = matches!(
            c,
            Calculation::Change { .. } | Calculation::PercentageChange { .. }
        );
        let ring_len = if delay {
            w + 1
        } else if matches!(c, Calculation::RollingAverage { .. }) {
            w
        } else {
            0
        };
        let mut ring = Vec::new();
        ring.try_reserve_exact(ring_len)
            .map_err(|_| evidence("resource_limit"))?;
        let mut minimum = std::collections::VecDeque::new();
        let mut maximum = std::collections::VecDeque::new();
        if matches!(
            c,
            Calculation::RollingMinimum { .. } | Calculation::RollingRange { .. }
        ) {
            minimum
                .try_reserve_exact(w + 1)
                .map_err(|_| evidence("resource_limit"))?;
        }
        if matches!(
            c,
            Calculation::RollingMaximum { .. } | Calculation::RollingRange { .. }
        ) {
            maximum
                .try_reserve_exact(w + 1)
                .map_err(|_| evidence("resource_limit"))?;
        }
        Ok(Self {
            ring,
            ring_len,
            cursor: 0,
            count: 0,
            sum: 0,
            bad: None,
            minimum,
            maximum,
            insertion_stage: 0,
        })
    }
    fn push_step(&mut self, c: &Calculation, value: Option<i128>, tick: u64) -> Option<bool> {
        fixed_work(1);
        let w = lookback(c);
        let v = value.unwrap_or(0);
        match self.insertion_stage {
            0 => {
                if value.is_none() {
                    self.bad = Some(tick);
                }
                if self.ring_len != 0 {
                    let old = self
                        .ring
                        .get(self.cursor)
                        .inspect(inspect_entry)
                        .copied()
                        .unwrap_or(0);
                    self.sum = self.sum.checked_sub(old)?.checked_add(v)?;
                    if self.ring.len() < self.ring_len {
                        // Capacity is reserved before collection; write cells only as they arrive.
                        inspect_entry(&());
                        self.ring.push(v);
                    } else {
                        inspect_entry(&());
                        self.ring[self.cursor] = v;
                    }
                    self.cursor = (self.cursor + 1) % self.ring_len;
                }
                self.insertion_stage = 1;
            }
            1 => {
                if matches!(
                    c,
                    Calculation::RollingMinimum { .. } | Calculation::RollingRange { .. }
                ) {
                    if self
                        .minimum
                        .back()
                        .inspect(inspect_entry)
                        .is_some_and(|x| x.0 >= v)
                    {
                        self.minimum.pop_back().inspect(inspect_entry);
                        return Some(false);
                    }
                    inspect_entry(&());
                    self.minimum.push_back((v, tick));
                }
                self.insertion_stage = 2;
            }
            2 => {
                if self
                    .minimum
                    .front()
                    .inspect(inspect_entry)
                    .is_some_and(|x| x.1 + w <= tick)
                {
                    self.minimum.pop_front().inspect(inspect_entry);
                    return Some(false);
                }
                self.insertion_stage = 3;
            }
            3 => {
                if matches!(
                    c,
                    Calculation::RollingMaximum { .. } | Calculation::RollingRange { .. }
                ) {
                    if self
                        .maximum
                        .back()
                        .inspect(inspect_entry)
                        .is_some_and(|x| x.0 <= v)
                    {
                        self.maximum.pop_back().inspect(inspect_entry);
                        return Some(false);
                    }
                    inspect_entry(&());
                    self.maximum.push_back((v, tick));
                }
                self.insertion_stage = 4;
            }
            _ => {
                if self
                    .maximum
                    .front()
                    .inspect(inspect_entry)
                    .is_some_and(|x| x.1 + w <= tick)
                {
                    self.maximum.pop_front().inspect(inspect_entry);
                    return Some(false);
                }
                self.count += 1;
                self.insertion_stage = 0;
                return Some(true);
            }
        }
        Some(false)
    }
    fn result(&self, c: &Calculation, current: Option<i128>, tick: u64) -> Option<i128> {
        fixed_work(1);
        let w = lookback(c);
        if self.count < w || self.bad.is_some_and(|bad| bad + w >= tick) {
            return None;
        }
        match c {
            Calculation::RollingAverage { .. } => divide_even(self.sum, i128::from(w)),
            Calculation::RollingMinimum { .. } => {
                self.minimum.front().inspect(inspect_entry).map(|x| x.0)
            }
            Calculation::RollingMaximum { .. } => {
                self.maximum.front().inspect(inspect_entry).map(|x| x.0)
            }
            Calculation::RollingRange { .. } => self
                .maximum
                .front()
                .inspect(inspect_entry)?
                .0
                .checked_sub(self.minimum.front().inspect(inspect_entry)?.0),
            Calculation::Change { .. } | Calculation::PercentageChange { .. } => {
                let boundary =
                    self.ring[(self.cursor + self.ring.len() - w as usize) % self.ring.len()];
                let diff = current?.checked_sub(boundary)?;
                if matches!(c, Calculation::PercentageChange { .. }) {
                    divide_even(diff.checked_mul(100 * SCALE)?, boundary)
                } else {
                    Some(diff)
                }
            }
            _ => None,
        }
    }
}
#[derive(Debug)]
struct Series {
    market: Option<String>,
    side: Side,
    measurement: usize,
    current: Option<i128>,
    previous: Option<i128>,
    cell_valid: bool,
    continuous: bool,
    history: Option<History>,
    unavailable_reason: &'static str,
}
#[derive(Default)]
struct CoverageCursor {
    raw: usize,
    fact: usize,
    transaction: usize,
    member: usize,
    found: bool,
    gaps: bool,
}
#[derive(Clone, Copy)]
enum ComputeEvent {
    Clock,
    Transaction(usize),
    Final,
}
fn condition_instances(document: &Document) -> usize {
    fn nodes(c: &Predicate) -> usize {
        1 + match c {
            Predicate::All { conditions } | Predicate::Any { conditions } => {
                conditions.iter().map(nodes).sum()
            }
            Predicate::HeldFor { condition, .. }
            | Predicate::ConfirmedUpdates { condition, .. } => nodes(condition),
            _ => 0,
        }
    }
    fn sides(selector: Option<&Selector>) -> usize {
        if matches!(selector, None | Some(Selector::Fixed { .. })) {
            1
        } else {
            2
        }
    }
    fn options(o: &OptionSpec, ancestor_nodes: usize) -> usize {
        let count = ancestor_nodes + nodes(&o.when);
        if let Some(children) = &o.options {
            children.iter().map(|o| options(o, count)).sum()
        } else {
            count * sides(o.side.as_ref())
        }
    }
    document
        .checkpoints
        .iter()
        .map(|c| nodes(&c.when) * sides(c.remember_side.as_ref()))
        .sum::<usize>()
        + document
            .purchase_options
            .iter()
            .map(|o| options(o, 0))
            .sum::<usize>()
}
#[derive(Clone)]
struct LeafNode {
    id: String,
    during: Interval,
    side: Option<Selector>,
}
impl From<&OptionSpec> for LeafNode {
    fn from(option: &OptionSpec) -> Self {
        Self {
            id: option.id.clone(),
            during: option.during,
            side: option.side.clone(),
        }
    }
}
struct LeafPlan {
    leaf: LeafNode,
    ancestors: Vec<LeafNode>,
    path: String,
}
fn leaf_plans_fn(nodes: &[OptionSpec], ancestors: &[LeafNode], out: &mut Vec<LeafPlan>) {
    fn collect(
        nodes: &[OptionSpec],
        ancestors: &[LeafNode],
        parent: &str,
        out: &mut Vec<LeafPlan>,
    ) {
        for (index, o) in nodes.iter().enumerate() {
            let path = format!("{parent}[{index}]");
            if let Some(children) = &o.options {
                let mut ancestors = ancestors.to_vec();
                ancestors.push(LeafNode::from(o));
                collect(children, &ancestors, &format!("{path}.options"), out);
            } else {
                out.push(LeafPlan {
                    leaf: LeafNode::from(o),
                    ancestors: ancestors.to_vec(),
                    path: path.clone(),
                });
            }
        }
    }
    collect(nodes, ancestors, "$.purchase_options", out);
}
impl Decision {
    // Destroy one owned entry at a time, including nested checkpoint observations.
    fn discard_step(&mut self) -> bool {
        fixed_work(1);
        let checkpoints = self.checkpoints.len();
        tree_work(checkpoints, false);
        if let Some(mut checkpoint) = self.checkpoints.first_entry() {
            tree_work(checkpoint.get().observations.len(), true);
            if checkpoint.get_mut().observations.pop_first().is_none() {
                tree_work(checkpoints, true);
                checkpoint.remove_entry();
            }
        } else if self.progress.pop().is_some()
            || self.leaves.pop().is_some()
            || self.reasons.pop().is_some()
            || self.candidate.take().is_some()
        {
        } else {
            return true;
        }
        #[cfg(test)]
        OUTPUT_DISCARD_PROBES.with(|n| n.set(n.get() + 1));
        false
    }
}
struct DecisionCursor {
    result: Decision,
    publication: Decision,
    stage: u8,
    index: usize,
    unchanged: bool,
}
impl DecisionCursor {
    fn new(phase: &'static str) -> Self {
        fixed_work(1);
        let result = Decision {
            phase,
            progress: vec![],
            evaluated_through_ms: 0,
            checkpoints: Default::default(),
            leaves: vec![],
            candidate: None,
            reasons: vec![],
            repeated_waits: 0,
        };
        Self {
            publication: result.clone(),
            result,
            stage: 0,
            index: 0,
            unchanged: true,
        }
    }
    fn reason(&mut self, reason: RuleError, old: &Decision) {
        fixed_work(1);
        self.unchanged &= old.reasons.get(self.result.reasons.len()) == Some(&reason);
        vector_growth(&self.publication.reasons);
        self.publication.reasons.push(reason.clone());
        vector_growth(&self.result.reasons);
        self.result.reasons.push(reason);
    }
}
#[derive(Clone)]
struct ConditionNode {
    steps: Vec<(Predicate, String)>,
    path: String,
    during: Interval,
}
impl ConditionNode {
    fn new(condition: &Predicate, path: String, during: Interval) -> Self {
        fn compile(c: &Predicate, path: &str, out: &mut Vec<(Predicate, String)>) {
            let placeholder = || Predicate::CheckpointReached {
                checkpoint: String::new(),
            };
            let shallow = match c {
                Predicate::All { conditions } | Predicate::Any { conditions } => {
                    for (i, child) in conditions.iter().enumerate() {
                        compile(child, &format!("{path}.{i}"), out);
                    }
                    let children = (0..conditions.len()).map(|_| placeholder()).collect();
                    if matches!(c, Predicate::All { .. }) {
                        Predicate::All {
                            conditions: children,
                        }
                    } else {
                        Predicate::Any {
                            conditions: children,
                        }
                    }
                }
                Predicate::HeldFor {
                    condition,
                    duration_seconds,
                } => {
                    compile(condition, &format!("{path}.condition"), out);
                    Predicate::HeldFor {
                        condition: Box::new(placeholder()),
                        duration_seconds: *duration_seconds,
                    }
                }
                Predicate::ConfirmedUpdates {
                    condition,
                    count,
                    count_input,
                } => {
                    compile(condition, &format!("{path}.condition"), out);
                    Predicate::ConfirmedUpdates {
                        condition: Box::new(placeholder()),
                        count: *count,
                        count_input: count_input.clone(),
                    }
                }
                _ => c.clone(),
            };
            // Parent evaluation reads cached children; keep only their arity here.
            out.push((shallow, path.to_owned()));
        }
        let mut steps = Vec::new();
        compile(condition, &path, &mut steps);
        Self {
            steps,
            path,
            during,
        }
    }
}
struct ConditionPlan {
    checkpoint: Option<CheckpointSpec>,
    selector: Option<Selector>,
    nodes: Vec<ConditionNode>,
}
#[derive(Default)]
struct ConditionCursor {
    plan: usize,
    side: usize,
    node: usize,
    sides: Option<Vec<Side>>,
    step: usize,
    child: usize,
    aggregate: Option<(Truth, bool)>,
    checkpoint: Option<CheckpointSnapshot>,
    observation: usize,
}
fn condition_plans(strategy: &ValidatedStrategy) -> Vec<ConditionPlan> {
    fn options<'a>(
        o: &'a OptionSpec,
        parent: Interval,
        ancestors: &mut Vec<(&'a OptionSpec, Interval)>,
        out: &mut Vec<ConditionPlan>,
    ) {
        let during = Interval {
            start_seconds: parent.start_seconds.max(o.during.start_seconds),
            end_seconds: parent.end_seconds.min(o.during.end_seconds),
        };
        ancestors.push((o, during));
        if let Some(children) = &o.options {
            for child in children {
                options(child, during, ancestors, out);
            }
        } else {
            let nodes = ancestors
                .iter()
                .map(|(node, during)| {
                    ConditionNode::new(&node.when, format!("option.{}.{}", o.id, node.id), *during)
                })
                .collect();
            out.push(ConditionPlan {
                checkpoint: None,
                selector: o.side.clone(),
                nodes,
            });
        }
        ancestors.pop();
    }
    let mut out = Vec::new();
    for &i in &strategy.checkpoint_order {
        let c = &strategy.document.checkpoints[i];
        out.push(ConditionPlan {
            checkpoint: Some(c.clone()),
            selector: c.remember_side.clone(),
            nodes: vec![ConditionNode::new(
                &c.when,
                format!("checkpoint.{}", c.id),
                c.during,
            )],
        });
    }
    for o in &strategy.document.purchase_options {
        options(o, Interval::default(), &mut Vec::new(), &mut out);
    }
    out
}
#[derive(Default)]
struct GapCursor {
    transaction: usize,
    member: usize,
    series: Option<usize>,
}
struct Pending {
    cleaning: bool,
    rejection: Option<EvidenceError>,
    finished: Option<(Decision, Decision)>,
    initial_frontier: u64,
    gap_cursor: GapCursor,
    gaps_ready: bool,
    batch: AdvanceBatch,
    phase: &'static str,
    validation_cursor: usize,
    staging_cursor: usize,
    transaction_cursor: usize,
    cell_cursor: Option<usize>,
    clock_due: bool,
    coverage_ready: bool,
    coverage_cursor: CoverageCursor,
    validation_member: usize,
    registration: RegistrationCursor,
    apply_member: usize,
    transfer_cursor: usize,
    compute_event: Option<ComputeEvent>,
    compute_cursor: usize,
    final_evaluated: bool,
    conditions: Option<ConditionCursor>,
    output: Option<DecisionCursor>,
    output_objective: Option<ObjectiveView>,
    discarding_output: bool,
    binding_lookup: usize,
    member_bindings: BTreeSet<u32>,
    catalog_context: RunContext,
    catalog_raw: Vec<RawState>,
    reserved_series: Vec<Series>,
    allocation: AllocationCursor,
}
impl Pending {
    fn discard_resources_step(&mut self) -> bool {
        fixed_work(1);
        if let Some(transaction) = self.batch.transactions.last_mut() {
            if let Some(EvidenceChange::RegisterMarket { bindings, .. }) =
                transaction.changes.last_mut()
                && bindings.pop().is_some()
            {
            } else if transaction.changes.pop().is_none() {
                self.batch.transactions.pop();
            }
        } else if self.batch.coverage.pop().is_some()
            || self.catalog_context.markets.pop().is_some()
            || self.catalog_context.bindings.pop().is_some()
            || self.catalog_raw.pop().is_some()
            || self.reserved_series.pop().is_some()
        {
        } else if !self.member_bindings.is_empty() {
            tree_work(self.member_bindings.len(), true);
            self.member_bindings.pop_first();
        } else if self.allocation.candidate.take().is_some() {
        } else {
            return true;
        }
        #[cfg(test)]
        CLEANUP_PROBES.with(|n| n.set(n.get() + 1));
        false
    }
}
// Owned heap capacity only; inline storage is charged by its owning container.
trait HeapBytes {
    fn heap_bytes(&self) -> usize;
}
fn sum_bytes(values: impl IntoIterator<Item = usize>) -> usize {
    values
        .into_iter()
        .try_fold(0usize, usize::checked_add)
        .unwrap_or(usize::MAX)
}
fn capacity_bytes<T>(capacity: usize) -> usize {
    capacity
        .checked_mul(std::mem::size_of::<T>())
        .filter(|bytes| *bytes <= RESERVE)
        .unwrap_or(RESERVE + 1)
}
// Rust B-trees have at least five keys per non-root node, up to eleven keys,
// and at most twelve edges. Charge every node as an internal node, including
// the root retained after removing the final entry (Rust 1.100.0).
fn tree_bytes<K, V>(entries: usize) -> usize {
    let nodes = entries.saturating_sub(1) / 5 + 1;
    nodes
        .checked_mul(sum_bytes([
            128,
            capacity_bytes::<K>(11),
            capacity_bytes::<V>(11),
        ]))
        .filter(|bytes| *bytes <= RESERVE)
        .unwrap_or(RESERVE + 1)
}
impl HeapBytes for String {
    fn heap_bytes(&self) -> usize {
        self.capacity()
    }
}
impl<T: HeapBytes> HeapBytes for Vec<T> {
    fn heap_bytes(&self) -> usize {
        sum_bytes(
            std::iter::once(capacity_bytes::<T>(self.capacity()))
                .chain(self.iter().map(HeapBytes::heap_bytes)),
        )
    }
}
impl<T: HeapBytes> HeapBytes for Option<T> {
    fn heap_bytes(&self) -> usize {
        self.as_ref().map_or(0, HeapBytes::heap_bytes)
    }
}
impl<T: HeapBytes> HeapBytes for Box<T> {
    fn heap_bytes(&self) -> usize {
        sum_bytes([std::mem::size_of::<T>(), (**self).heap_bytes()])
    }
}
impl<A: HeapBytes, B: HeapBytes> HeapBytes for (A, B) {
    fn heap_bytes(&self) -> usize {
        sum_bytes([self.0.heap_bytes(), self.1.heap_bytes()])
    }
}
impl<K: HeapBytes, V: HeapBytes> HeapBytes for std::collections::BTreeMap<K, V> {
    fn heap_bytes(&self) -> usize {
        sum_bytes(
            std::iter::once(tree_bytes::<K, V>(self.len())).chain(
                self.iter()
                    .map(|(k, v)| sum_bytes([k.heap_bytes(), v.heap_bytes()])),
            ),
        )
    }
}
macro_rules! heap_fields {
    ($ty:ty: $($field:ident),+ $(,)?) => {
        impl HeapBytes for $ty { fn heap_bytes(&self) -> usize { sum_bytes([$(self.$field.heap_bytes()),+]) } }
    };
}
macro_rules! no_heap {
    ($($ty:ty),+) => { $(impl HeapBytes for $ty { fn heap_bytes(&self) -> usize { 0 } })+ };
}
no_heap!(usize, u64, bool, NumericInfo);
heap_fields!(InputSpec: id, source, instrument, field, side);
heap_fields!(MeasurementSpec: id, calculation);
heap_fields!(CheckpointSpec: id, when, remember_side);
heap_fields!(OptionSpec: id, when, options, side);
heap_fields!(Document: name, inputs, measurements, checkpoints, purchase_options);
heap_fields!(Requirements: startup_ms, per_node_bytes, implicit_buyable_asks);
heap_fields!(ValidatedStrategy: text, document, order, checkpoint_order, numeric_scopes, numeric_info, requirements, explanation);
heap_fields!(LeafNode: id, side);
heap_fields!(LeafPlan: leaf, ancestors, path);
heap_fields!(ConditionNode: steps, path);
heap_fields!(ConditionPlan: checkpoint, selector, nodes);
heap_fields!(MarketDescriptor: series_id, market_id, contract_id);
heap_fields!(BindingDescriptor: input_id, scope);
heap_fields!(RunContext: strategy_version, run_id, go_time, series_id, markets, bindings);
impl HeapBytes for BindingScope {
    fn heap_bytes(&self) -> usize {
        match self {
            Self::Underlying => 0,
            Self::Market { market_id } => market_id.heap_bytes(),
            Self::Contract {
                market_id,
                contract_id,
                ..
            } => sum_bytes([market_id.heap_bytes(), contract_id.heap_bytes()]),
        }
    }
}
impl HeapBytes for ValueSpec {
    fn heap_bytes(&self) -> usize {
        match self {
            Self::Reference { reference } => reference.heap_bytes(),
            Self::Literal { value, .. } => value.heap_bytes(),
        }
    }
}
impl HeapBytes for Calculation {
    fn heap_bytes(&self) -> usize {
        sum_bytes(
            calculation_values(self)
                .into_iter()
                .map(HeapBytes::heap_bytes),
        )
    }
}
impl HeapBytes for Selector {
    fn heap_bytes(&self) -> usize {
        match self {
            Self::CheckpointSide { checkpoint, .. } => checkpoint.heap_bytes(),
            _ => 0,
        }
    }
}
impl HeapBytes for Predicate {
    fn heap_bytes(&self) -> usize {
        match self {
            Self::Compare { left, right, .. }
            | Self::CrossedBelow { left, right }
            | Self::CrossedAbove { left, right } => {
                sum_bytes([left.heap_bytes(), right.heap_bytes()])
            }
            Self::All { conditions } | Self::Any { conditions } => conditions.heap_bytes(),
            Self::HeldFor { condition, .. } => condition.heap_bytes(),
            Self::ConfirmedUpdates {
                condition,
                count_input,
                ..
            } => sum_bytes([condition.heap_bytes(), count_input.heap_bytes()]),
            Self::CheckpointReached { checkpoint } => checkpoint.heap_bytes(),
            Self::AfterCheckpointUpdate {
                checkpoint,
                count_input,
            } => sum_bytes([checkpoint.heap_bytes(), count_input.heap_bytes()]),
        }
    }
}

// Compiler ownership: final steps/nodes/leaves plus one recursive path and the
// borrowed ancestor stack. Vec push growth is at most four slots per item.
fn compilation_bytes(strategy: &ValidatedStrategy) -> usize {
    let states = condition_instances(&strategy.document);
    let base_path = 7 + 64 + 1 + 64; // option.<leaf>.<node>
    let step_path = base_path + 12 * ".condition".len();
    let per_state = sum_bytes([
        4 * std::mem::size_of::<(Predicate, String)>(),
        std::mem::size_of::<Predicate>(), // one child edge/box per non-root
        128,                              // at most two cloned identifier/literal payloads
        step_path,
        2 * base_path, // formatted node path capacity
        std::mem::size_of::<ConditionNode>(),
        std::mem::size_of::<LeafNode>(),
        128, // compiled full purchase-option field path
        128, // leaf id and optional checkpoint-side id
    ]);
    sum_bytes([
        states * per_state,
        capacity_bytes::<ConditionPlan>(1024),
        capacity_bytes::<LeafPlan>(1024),
        strategy.document.checkpoints.heap_bytes(), // retained checkpoint definitions
        12 * 2 * step_path, // temporary paths during recursive condition compilation
        8 * 16 * (std::mem::size_of::<LeafNode>() + 128), // leaf traversal ancestor clones
        capacity_bytes::<(&OptionSpec, Interval)>(8),
    ])
}

/// Owns controlled evidence only. It is never attached to desktop Start/Pause.
pub struct Evaluator {
    metadata_reservation: usize,
    strategy: ValidatedStrategy,
    context: RunContext,
    raw: Vec<RawState>,
    series: Vec<Series>,
    frontier: u64,
    last_transaction: u64,
    market: Option<String>,
    ready_at: Option<u64>,
    checkpoints: std::collections::BTreeMap<String, CheckpointSnapshot>,
    temporal: std::collections::BTreeMap<(String, usize), Temporal>,
    pending: Option<Pending>,
    stopped: bool,
    failed: bool,
    published: Decision,
    retired: Option<Decision>,
    last_objective: Option<ObjectiveView>,
    truths: std::collections::BTreeMap<(String, usize), Truth>,
    continuities: std::collections::BTreeMap<(String, usize), bool>,
    condition_plans: Vec<ConditionPlan>,
    leaf_plans: Vec<LeafPlan>,
    #[cfg(test)]
    fail_after_mutation: bool,
    #[cfg(test)]
    yield_each_step: bool,
}
impl Evaluator {
    pub fn new(strategy: ValidatedStrategy, context: RunContext) -> Result<Self, RuleError> {
        if context.strategy_version.is_empty()
            || context.run_id.is_empty()
            || context.series_id.is_empty()
            || context.markets.len() > 128
        {
            return Err(rule_error("$.context", "incompatible_market"));
        }
        if sum_bytes([
            context.heap_bytes(),
            strategy.heap_bytes(),
            compilation_bytes(&strategy),
        ]) > RESERVE
        {
            return Err(rule_error("$.context", "resource_limit"));
        }
        let condition_plans = condition_plans(&strategy);
        let mut leaf_plans = Vec::new();
        leaf_plans_fn(&strategy.document.purchase_options, &[], &mut leaf_plans);
        let mut evaluator = Self {
            metadata_reservation: 0,
            strategy,
            context,
            raw: Vec::new(),
            series: Vec::new(),
            frontier: 0,
            last_transaction: 0,
            market: None,
            ready_at: None,
            checkpoints: Default::default(),
            temporal: Default::default(),
            pending: None,
            stopped: false,
            failed: false,
            published: Decision {
                phase: "Prefill — collecting live data",
                progress: Vec::new(),
                evaluated_through_ms: 0,
                checkpoints: Default::default(),
                leaves: Vec::new(),
                candidate: None,
                reasons: Vec::new(),
                repeated_waits: 0,
            },
            retired: None,
            last_objective: None,
            truths: Default::default(),
            continuities: Default::default(),
            condition_plans,
            leaf_plans,
            #[cfg(test)]
            fail_after_mutation: false,
            #[cfg(test)]
            yield_each_step: false,
        };
        evaluator.metadata_reservation = evaluator.plan_metadata_bytes();
        if sum_bytes([
            evaluator.metadata_reservation,
            evaluator.context.heap_bytes(),
        ]) > RESERVE
        {
            return Err(rule_error("$.context", "resource_limit"));
        }
        // Supplied duplicates/spare capacities are charged above, never copied wholesale.
        let markets = std::mem::take(&mut evaluator.context.markets);
        let bindings = std::mem::take(&mut evaluator.context.bindings);
        for descriptor in markets {
            evaluator
                .register(descriptor, Vec::new())
                .map_err(|_| rule_error("$.context", "incompatible_market"))?;
        }
        for b in bindings {
            evaluator
                .add_binding(b)
                .map_err(|_| rule_error("$.context", "unregistered_binding"))?;
        }
        evaluator
            .allocate_series()
            .map_err(|_| rule_error("$.context", "resource_limit"))?;
        if evaluator
            .strategy
            .requirements
            .startup_ms
            .values()
            .all(|w| *w == 0)
        {
            evaluator.ready_at = Some(0);
        }
        Ok(evaluator)
    }
    fn plan_metadata_bytes(&self) -> usize {
        let mut path_bytes = 0usize;
        let mut states = 0usize;
        let mut scratch = 0usize;
        for plan in &self.condition_plans {
            let sides = if matches!(plan.selector, None | Some(Selector::Fixed { .. })) {
                1
            } else {
                2
            };
            for node in &plan.nodes {
                for (predicate, path) in &node.steps {
                    states += sides;
                    path_bytes = sum_bytes([path_bytes, path.len() * sides]);
                    scratch = scratch.max(sum_bytes([
                        std::mem::size_of::<Predicate>(),
                        predicate.heap_bytes(),
                        path.len(),
                    ]));
                }
            }
        }
        let inputs = &self.strategy.document.inputs;
        let input_id = inputs
            .iter()
            .map(|input| input.id.len())
            .max()
            .unwrap_or(0)
            .max("__buyable_ask_yes".len());
        let checkpoint_count = self.strategy.document.checkpoints.len();
        let checkpoint_keys = self
            .strategy
            .document
            .checkpoints
            .iter()
            .map(|checkpoint| checkpoint.id.len())
            .sum::<usize>();
        let leaves = self.leaf_plans.len();
        let leaf_names = self
            .leaf_plans
            .iter()
            .map(|leaf| leaf.leaf.id.len())
            .sum::<usize>();
        let leaf_id = self
            .leaf_plans
            .iter()
            .map(|leaf| leaf.leaf.id.len())
            .max()
            .unwrap_or(0);
        // Vec growth from an arbitrary admitted length can approach twice 128.
        let catalogs = sum_bytes([
            2 * capacity_bytes::<MarketDescriptor>(256),
            2 * 128 * 3 * 128,
            2 * capacity_bytes::<BindingDescriptor>(256),
            2 * capacity_bytes::<RawState>(256),
            4 * 128 * (input_id + 256),
            2 * capacity_bytes::<Series>(256),
            2 * 128 * 128,
            2 * sum_bytes([
                self.context.strategy_version.capacity(),
                self.context.run_id.capacity(),
                self.context.go_time.capacity(),
                self.context.series_id.capacity(),
            ]),
        ]);
        let maps = sum_bytes([
            tree_bytes::<(String, usize), Temporal>(states),
            tree_bytes::<(String, usize), Truth>(states),
            tree_bytes::<(String, usize), bool>(states),
            3 * path_bytes,
            tree_bytes::<u32, ()>(128),
        ]);
        let checkpoints = sum_bytes([
            tree_bytes::<String, CheckpointSnapshot>(checkpoint_count),
            checkpoint_keys,
            checkpoint_count * tree_bytes::<u32, u64>(128),
        ]);
        // Latest caller result + publication + two private outputs. Retired output
        // is drained before private output starts; changed objectives drain both first.
        let decision = sum_bytes([
            checkpoints,
            capacity_bytes::<HistoryProgress>(256),
            128 * input_id,
            capacity_bytes::<LeafResult>(2 * leaves.max(4)),
            leaf_names,
            capacity_bytes::<RuleError>(2 * (leaves + 128).max(4)),
            (leaves + 128) * 256,
            128 + leaf_id,
        ]);
        sum_bytes([
            std::mem::size_of::<Self>(),
            std::mem::size_of::<Pending>(),
            self.strategy.heap_bytes(),
            self.condition_plans.heap_bytes(),
            self.leaf_plans.heap_bytes(),
            catalogs,
            maps,
            checkpoints,
            4 * decision,
            // A non-Boolean step owns H + 5P + 158 bytes at most, where
            // scratch >= size_of::<Predicate>() + H + P; eight slots cover this.
            8 * scratch,
            // Phase-exclusive auxiliary peaks: registration <= 640, numeric <= 336,
            // selection <= 282, plus one 128-byte supplied objective and tiny cursors.
            4096,
            3 * 128,
        ])
    }
    pub fn allocated_bytes(&self) -> usize {
        RESERVE + self.series.iter().map(series_bytes).sum::<usize>()
    }
    pub fn snapshot(&self) -> &Decision {
        &self.published
    }
    pub fn begin(&mut self, batch: AdvanceBatch) -> Result<(), EvidenceError> {
        if self.failed {
            return Err(evidence("internal_evaluation_failure"));
        }
        if self.stopped {
            return Err(evidence("stopped"));
        }
        if self.pending.is_some() {
            return Err(evidence("busy"));
        }
        if batch.transactions.len() > 4096
            || batch
                .ingress_bytes()?
                .checked_add(self.metadata_reservation)
                .is_none_or(|bytes| bytes > RESERVE)
        {
            return Err(evidence("resource_limit"));
        }
        self.published.candidate = None;
        self.pending = Some(Pending {
            cleaning: false,
            rejection: None,
            finished: None,
            initial_frontier: self.frontier,
            gap_cursor: GapCursor::default(),
            gaps_ready: false,
            batch,
            phase: "validating",
            validation_cursor: 0,
            staging_cursor: 0,
            transaction_cursor: 0,
            cell_cursor: None,
            clock_due: false,
            coverage_ready: false,
            coverage_cursor: CoverageCursor::default(),
            validation_member: 0,
            registration: RegistrationCursor::default(),
            apply_member: 0,
            transfer_cursor: 0,
            compute_event: None,
            compute_cursor: 0,
            final_evaluated: false,
            conditions: None,
            output: None,
            output_objective: None,
            discarding_output: false,
            binding_lookup: 0,
            member_bindings: BTreeSet::new(),
            catalog_context: self.context.clone(),
            catalog_raw: self.raw.clone(),
            reserved_series: Vec::new(),
            allocation: AllocationCursor::default(),
        });
        Ok(())
    }
    pub fn process_slice(
        &mut self,
        control: SliceControl,
        objective: Option<ObjectiveView>,
    ) -> SliceResult {
        #[cfg(test)]
        let _work_probe = SliceWorkProbe(processing_entries());
        fixed_work(1);
        if self.failed {
            return SliceResult::Failed {
                code: "internal_evaluation_failure",
            };
        }
        if self.stopped || control == SliceControl::Stop {
            self.stopped = true;
            // Retain cancelled ownership until evaluator destruction: Stop must not
            // synchronously destroy every member of a large admitted packet.
            self.published.candidate = None;
            return SliceResult::Stopped;
        }
        let objective = objective.filter(|view| view.market_id.capacity() <= 128);
        let Some(mut pending) = self.pending.take() else {
            return SliceResult::Rejected(evidence("no_pending_batch"));
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.slice(&mut pending, objective)
        }));
        match result {
            Ok(Ok(Some((d, publication)))) => {
                self.retired = Some(std::mem::replace(&mut self.published, publication));
                SliceResult::Complete(d)
            }
            Ok(Ok(None)) => {
                let phase = pending.phase;
                self.pending = Some(pending);
                SliceResult::Pending {
                    phase,
                    evaluated_through_ms: self.frontier,
                }
            }
            Ok(Err(e)) if pending.phase != "evaluating" => {
                if pending.cleaning {
                    SliceResult::Rejected(e)
                } else {
                    pending.cleaning = true;
                    pending.rejection = Some(e);
                    let phase = pending.phase;
                    self.pending = Some(pending);
                    SliceResult::Pending {
                        phase,
                        evaluated_through_ms: self.frontier,
                    }
                }
            }
            _ => {
                self.pending = Some(pending);
                self.failed = true;
                self.published.candidate = None;
                SliceResult::Failed {
                    code: "internal_evaluation_failure",
                }
            }
        }
    }
    fn slice(
        &mut self,
        p: &mut Pending,
        objective: Option<ObjectiveView>,
    ) -> Result<Option<(Decision, Decision)>, EvidenceError> {
        fixed_work(1);
        if let Some(view) = objective.as_ref()
            && self.last_objective.as_ref() != Some(view)
            && self.objective_valid(view)
        {
            // Trusted fills remain binding even if pending evidence is later rejected.
            self.last_objective = Some(view.clone());
            return Ok(None);
        }
        // Reserve the wrapper, objective-retention prologue and final publication
        // separately from loop dispatch and entry/library work.
        let mut budget = 4096usize - 64;
        #[cfg(test)]
        let mut steps = 0;
        // One dispatch and its bounded scalar/helper control use at most 64
        // record primitives; entry/library work reserves the separate task bound.
        while budget >= 64 {
            fixed_work(1);
            #[cfg(test)]
            if self.yield_each_step {
                if steps > 0 {
                    return Ok(None);
                }
                steps += 1;
            }
            budget -= 64;
            if let Some(retired) = &mut self.retired {
                if budget < 512 {
                    return Ok(None);
                }
                budget -= 512;
                if retired.discard_step() {
                    self.retired = None;
                }
                continue;
            }
            if p.cleaning
                && p.finished.is_some()
                && (p.discarding_output || p.output_objective != objective)
            {
                p.discarding_output = true;
                if budget < 512 {
                    return Ok(None);
                }
                budget -= 512;
                let (result, publication) = p.finished.as_mut().unwrap();
                if result.discard_step() && publication.discard_step() {
                    p.finished = None;
                    p.cleaning = false;
                    p.final_evaluated = true;
                    p.discarding_output = false;
                }
                continue;
            }
            if p.cleaning {
                if budget < 256 {
                    return Ok(None);
                }
                budget -= 256;
                if !p.discard_resources_step() {
                    continue;
                }
                if let Some(error) = p.rejection.take() {
                    return Err(error);
                }
                return Ok(p.finished.take());
            }
            if p.phase == "validating" {
                if p.batch.target_ms < self.frontier
                    || p.batch.target_ms > p.batch.received_through_ms
                {
                    return Err(evidence("invalid_time"));
                }
                if let Some(t) = p.batch.transactions.get(p.validation_cursor) {
                    let previous = p
                        .validation_cursor
                        .checked_sub(1)
                        .map(|i| &p.batch.transactions[i]);
                    if t.at_ms < self.frontier
                        || t.at_ms > p.batch.target_ms
                        || t.number <= previous.map_or(self.last_transaction, |x| x.number)
                        || previous.is_some_and(|x| x.at_ms > t.at_ms)
                    {
                        return Err(evidence("invalid_transaction"));
                    }
                    if let Some(c) = t.changes.get(p.validation_member) {
                        if let EvidenceChange::RegisterMarket {
                            descriptor,
                            bindings,
                        } = c
                        {
                            // A binding admission scans at most 128 raw and market entries
                            // and 32 declared inputs; never clone an entire registration.
                            if budget < 1024 {
                                return Ok(None);
                            }
                            budget -= 1024;
                            if !p.registration.step(
                                &self.strategy,
                                &mut p.catalog_context,
                                &mut p.catalog_raw,
                                descriptor,
                                bindings,
                            )? {
                                continue;
                            }
                        }
                        if let Some(id) = change_binding(c) {
                            let r = p
                                .catalog_raw
                                .get(p.binding_lookup)
                                .ok_or_else(|| evidence("unregistered_binding"))?;
                            #[cfg(test)]
                            CATALOG_PROBES.with(|n| n.set(n.get() + 1));
                            if r.descriptor.binding_id != id {
                                p.binding_lookup += 1;
                                continue;
                            }
                            if budget < 256 {
                                return Ok(None);
                            }
                            budget -= 256;
                            tree_work(p.member_bindings.len(), true);
                            if !p.member_bindings.insert(id) {
                                return Err(evidence("unregistered_binding"));
                            }
                        }
                        if let EvidenceChange::Gap {
                            start_ms, end_ms, ..
                        } = c
                            && (start_ms >= end_ms || *end_ms > p.batch.received_through_ms)
                        {
                            return Err(evidence("invalid_coverage"));
                        }
                        if matches!(c, EvidenceChange::MarketBoundary { .. }) {
                            if budget < 128 {
                                return Ok(None);
                            }
                            budget -= 128;
                        }
                        if let EvidenceChange::MarketBoundary { market_id } = c
                            && !p
                                .catalog_context
                                .markets
                                .iter()
                                .inspect(inspect_entry)
                                .any(|m| {
                                    m.market_id == *market_id
                                        && i128::from(m.open_ms) == i128::from(t.at_ms)
                                })
                        {
                            return Err(evidence("incompatible_market"));
                        }
                        p.validation_member += 1;
                        p.binding_lookup = 0;
                        continue;
                    }
                    if !p.member_bindings.is_empty() {
                        if budget < 256 {
                            return Ok(None);
                        }
                        budget -= 256;
                        tree_work(p.member_bindings.len(), true);
                        p.member_bindings.pop_first();
                        inspect_entry(&());
                        continue;
                    }
                    p.validation_member = 0;
                    p.validation_cursor += 1;
                    continue;
                }
                p.phase = "staging";
                continue;
            }
            if p.phase == "staging" {
                if let Some(c) = p.batch.coverage.get(p.staging_cursor) {
                    if c.start_ms >= c.end_ms || c.end_ms > p.batch.received_through_ms {
                        return Err(evidence("invalid_coverage"));
                    }
                    let r = p
                        .catalog_raw
                        .get(p.binding_lookup)
                        .ok_or_else(|| evidence("invalid_coverage"))?;
                    #[cfg(test)]
                    CATALOG_PROBES.with(|n| n.set(n.get() + 1));
                    if r.descriptor.binding_id != c.binding_id {
                        p.binding_lookup += 1;
                        continue;
                    }
                    p.binding_lookup = 0;
                    p.staging_cursor += 1;
                    continue;
                }
                if budget < 512 {
                    return Ok(None);
                }
                budget -= 512;
                if !p.allocation.step(
                    &self.strategy,
                    &p.catalog_context,
                    &self.series,
                    &mut p.reserved_series,
                )? {
                    continue;
                }
                p.phase = "evaluating";
                return Ok(None);
            }
            if !p.gaps_ready {
                let cursor = &mut p.gap_cursor;
                if let Some(t) = p.batch.transactions.get(cursor.transaction) {
                    if let Some(change) = t.changes.get(cursor.member) {
                        if let EvidenceChange::Gap {
                            binding_id,
                            start_ms,
                            end_ms,
                            ..
                        } = change
                            && (*start_ms == self.frontier
                                || (self.frontier == p.initial_frontier
                                    && *start_ms < self.frontier))
                        {
                            if budget < 1024 {
                                return Ok(None);
                            }
                            budget -= 1024;
                            if let Some(index) = cursor.series {
                                if index < self.series.len() {
                                    self.invalidate_series(*binding_id, *start_ms, *end_ms, index);
                                    cursor.series = Some(index + 1);
                                    continue;
                                }
                            } else {
                                let Some(raw) = self
                                    .raw
                                    .iter_mut()
                                    .inspect(inspect_entry)
                                    .find(|r| r.descriptor.binding_id == *binding_id)
                                else {
                                    cursor.member += 1;
                                    continue;
                                };
                                raw.covered_cells = 0;
                                raw.covered_previous = false;
                                raw.value = None;
                                raw.valid_until = 0;
                                raw.bad_cell = true;
                                cursor.series = Some(0);
                                continue;
                            }
                        }
                        cursor.member += 1;
                        cursor.series = None;
                    } else {
                        cursor.transaction += 1;
                        cursor.member = 0;
                    }
                    continue;
                }
                p.gaps_ready = true;
                continue;
            }
            if let Some(event) = p.compute_event {
                if p.compute_cursor < self.series.len() {
                    if budget < 2048 {
                        return Ok(None);
                    }
                    budget -= 2048;
                    self.compute_node(p.compute_cursor, &p.batch);
                    p.compute_cursor += 1;
                    continue;
                }
                if p.conditions.is_none() {
                    if budget < 256 {
                        return Ok(None);
                    }
                    self.finish_compute();
                    p.conditions = Some(ConditionCursor::default());
                    return Ok(None);
                }
                if budget < 3968 {
                    return Ok(None);
                }
                budget -= 3968;
                let transaction = match event {
                    ComputeEvent::Transaction(n) => Some(&p.batch.transactions[n]),
                    _ => None,
                };
                if !self.evaluate_step(
                    p.conditions.as_mut().unwrap(),
                    transaction,
                    objective.as_ref(),
                    &p.batch,
                ) {
                    continue;
                }
                p.conditions = None;
                p.final_evaluated |= matches!(event, ComputeEvent::Final);
                p.compute_event = None;
                p.compute_cursor = 0;
                return Ok(None);
            }
            if p.clock_due {
                p.clock_due = false;
                if !p
                    .batch
                    .transactions
                    .get(p.transaction_cursor)
                    .is_some_and(|t| t.at_ms == self.frontier)
                {
                    p.compute_event = Some(ComputeEvent::Clock);
                    continue;
                }
            }
            let next = p.batch.transactions.get(p.transaction_cursor);
            if next.is_some_and(|t| t.at_ms == self.frontier) {
                let t = &p.batch.transactions[p.transaction_cursor];
                if let Some(change) = t.changes.get(p.apply_member) {
                    if budget < 1024 {
                        return Ok(None);
                    }
                    budget -= 1024;
                    if let EvidenceChange::RegisterMarket {
                        descriptor,
                        bindings,
                    } = change
                    {
                        if !p.registration.step(
                            &self.strategy,
                            &mut self.context,
                            &mut self.raw,
                            descriptor,
                            bindings,
                        )? {
                            continue;
                        }
                    } else if !matches!(change, EvidenceChange::Gap { .. }) {
                        if matches!(change, EvidenceChange::MarketBoundary { .. })
                            && self.ready_at.is_some_and(|ready| ready < self.frontier)
                        {
                            tree_work(self.checkpoints.len(), true);
                            if let Some((_, checkpoint)) = self.checkpoints.pop_first() {
                                tree_clone_work(checkpoint.observations.len());
                                continue;
                            }
                            tree_work(self.temporal.len(), true);
                            if self.temporal.pop_first().is_some() {
                                continue;
                            }
                            tree_work(self.truths.len(), true);
                            if self.truths.pop_first().is_some() {
                                continue;
                            }
                            tree_work(self.continuities.len(), true);
                            if self.continuities.pop_first().is_some() {
                                continue;
                            }
                        }
                        self.apply(change, t.number)?;
                    }
                    p.apply_member += 1;
                    continue;
                }
                if let Some(series) = p.reserved_series.get(p.transfer_cursor) {
                    if budget < 512 {
                        return Ok(None);
                    }
                    budget -= 512;
                    if series.market.as_deref().is_none_or(|market| {
                        self.context
                            .markets
                            .iter()
                            .inspect(inspect_entry)
                            .any(|m| m.market_id == market)
                    }) {
                        vector_growth(&self.series);
                        vector_copy_work(p.reserved_series.len() - p.transfer_cursor - 1);
                        self.series
                            .push(p.reserved_series.remove(p.transfer_cursor));
                    } else {
                        p.transfer_cursor += 1;
                    }
                    continue;
                }
                self.last_transaction = t.number;
                p.apply_member = 0;
                p.transfer_cursor = 0;
                p.compute_event = Some(ComputeEvent::Transaction(p.transaction_cursor));
                p.transaction_cursor += 1;
                continue;
            }
            if self.frontier == p.batch.target_ms {
                if p.batch.transactions.is_empty()
                    && self.published.evaluated_through_ms == self.frontier
                    && !p.final_evaluated
                {
                    p.compute_event = Some(ComputeEvent::Final);
                    continue;
                }
                if p.output.is_some() && (p.discarding_output || p.output_objective != objective) {
                    p.discarding_output = true;
                    if budget < 512 {
                        return Ok(None);
                    }
                    budget -= 512;
                    let output = p.output.as_mut().unwrap();
                    if output.result.discard_step() && output.publication.discard_step() {
                        p.output = None;
                        p.discarding_output = false;
                    }
                    continue;
                }
                if p.output.is_none() {
                    p.output_objective = objective.clone();
                    let phase = if self.market.is_some() {
                        "Active"
                    } else if self.ready_at.is_some() {
                        "Ready — waiting for next market"
                    } else {
                        "Prefill — collecting live data"
                    };
                    p.output = Some(DecisionCursor::new(phase));
                }
                if budget < 3968 {
                    return Ok(None);
                }
                budget -= 3968;
                if self.decision_step(p.output.as_mut().unwrap(), objective.as_ref(), &p.batch) {
                    let output = p.output.take().unwrap();
                    p.finished = Some((output.result, output.publication));
                    p.cleaning = true;
                    continue;
                }
                continue;
            }
            if !p.coverage_ready {
                let c = &mut p.coverage_cursor;
                if c.raw == self.raw.len() {
                    p.coverage_ready = true;
                    continue;
                }
                let binding = self.raw[c.raw].descriptor.binding_id;
                if !c.gaps {
                    if let Some(fact) = p.batch.coverage.get(c.fact) {
                        #[cfg(test)]
                        COVERAGE_PROBES.with(|n| n.set(n.get() + 1));
                        c.fact += 1;
                        if fact.binding_id == binding
                            && fact.start_ms <= self.frontier
                            && self.frontier < fact.end_ms
                        {
                            c.found = true;
                            c.gaps = true;
                        }
                    } else {
                        c.gaps = true;
                    }
                    continue;
                }
                if let Some(t) = p.batch.transactions.get(c.transaction) {
                    if let Some(change) = t.changes.get(c.member) {
                        #[cfg(test)]
                        COVERAGE_PROBES.with(|n| n.set(n.get() + 1));
                        c.member += 1;
                        if matches!(change, EvidenceChange::Gap {binding_id, start_ms, end_ms, ..}
                            if *binding_id == binding && *start_ms <= self.frontier && self.frontier < *end_ms)
                        {
                            c.found = false;
                            c.transaction = p.batch.transactions.len();
                        }
                    } else {
                        c.transaction += 1;
                        c.member = 0;
                    }
                    continue;
                }
                self.raw[c.raw].cell_covered = c.found;
                *c = CoverageCursor {
                    raw: c.raw + 1,
                    ..Default::default()
                };
                continue;
            }
            if p.cell_cursor.is_none() {
                if budget < 128 {
                    return Ok(None);
                }
                budget -= 128;
                for series in self.series.iter_mut().inspect(inspect_entry) {
                    series.previous = series.current;
                }
                p.cell_cursor = Some(0);
            }
            let n = p.cell_cursor.unwrap();
            if n < self.series.len() {
                if budget < 1024 {
                    return Ok(None);
                }
                budget -= 1024;
                let series = &self.series[n];
                let c = &self.strategy.document.measurements[series.measurement].calculation;
                let cell_valid = lookback(c) > 0
                    || calculation_values(c).into_iter().all(|v| {
                        self.value(
                            v,
                            series.market.as_deref(),
                            series.side,
                            true,
                            true,
                            &p.batch,
                        )
                        .is_some()
                    });
                self.series[n].cell_valid = cell_valid;
                let series = &self.series[n];
                let c = &self.strategy.document.measurements[series.measurement].calculation;
                if lookback(c) == 0 {
                    p.cell_cursor = Some(n + 1);
                    continue;
                }
                let value = self.value(
                    calculation_values(c)[0],
                    series.market.as_deref(),
                    series.side,
                    true,
                    true,
                    &p.batch,
                );
                let c = c.clone();
                if self.series[n]
                    .history
                    .as_mut()
                    .ok_or_else(|| evidence("internal_evaluation_failure"))?
                    .push_step(&c, value, self.frontier)
                    .ok_or_else(|| evidence("internal_evaluation_failure"))?
                {
                    p.cell_cursor = Some(n + 1);
                }
                continue;
            }
            if budget < 128 {
                return Ok(None);
            }
            // This is the last work in the slice; the reservation check covers the sweep.
            p.cell_cursor = None;
            for raw in self.raw.iter_mut().inspect(inspect_entry) {
                raw.covered_previous = !raw.bad_cell
                    && raw.value.is_some()
                    && raw.valid_until > self.frontier
                    && raw.cell_covered;
                raw.covered_cells = if raw.covered_previous {
                    raw.covered_cells.saturating_add(1)
                } else {
                    0
                };
                raw.bad_cell = false;
            }
            self.frontier += 1;
            p.gaps_ready = false;
            p.gap_cursor = GapCursor::default();
            p.coverage_ready = false;
            p.coverage_cursor = CoverageCursor::default();
            #[cfg(test)]
            if self.fail_after_mutation {
                panic!("controlled internal failure");
            }
            p.clock_due = true;
            return Ok(None);
        }
        Ok(None)
    }
    fn add_binding(&mut self, b: BindingDescriptor) -> Result<(), EvidenceError> {
        add_binding_to(&self.strategy, &mut self.context, &mut self.raw, b)
    }
    fn register(
        &mut self,
        d: MarketDescriptor,
        bindings: Vec<BindingDescriptor>,
    ) -> Result<(), EvidenceError> {
        register_to(
            &self.strategy,
            &mut self.context,
            &mut self.raw,
            d,
            bindings,
        )
    }
    fn allocate_series(&mut self) -> Result<(), EvidenceError> {
        let added = allocate_new(&self.strategy, &self.context, &self.series)?;
        self.series.extend(added);
        Ok(())
    }
    fn apply(&mut self, change: &EvidenceChange, transaction: u64) -> Result<(), EvidenceError> {
        fixed_work(1);
        self.last_transaction = transaction;
        match change {
            EvidenceChange::Observation {
                binding_id,
                observation_number,
                value,
            } => {
                let r = self
                    .raw
                    .iter_mut()
                    .inspect(inspect_entry)
                    .find(|r| r.descriptor.binding_id == *binding_id)
                    .ok_or_else(|| evidence("unregistered_binding"))?;
                let contract = matches!(r.descriptor.scope, BindingScope::Contract { .. });
                if (contract && !(0..=100 * SCALE).contains(&value.0))
                    || (!contract && value.0 <= 0)
                    || *observation_number < r.number
                    || (*observation_number == r.number && r.last_observed_value != Some(value.0))
                {
                    r.value = None;
                    r.bad_cell = true;
                    return Ok(());
                }
                if *observation_number > r.number {
                    r.number = *observation_number;
                    r.last_observed_value = Some(value.0);
                    r.observed_transaction = transaction;
                    r.value = Some(value.0);
                }
            }
            EvidenceChange::Resynchronize { binding_id, value } => {
                let r = self
                    .raw
                    .iter_mut()
                    .inspect(inspect_entry)
                    .find(|r| r.descriptor.binding_id == *binding_id)
                    .ok_or_else(|| evidence("unregistered_binding"))?;
                let contract = matches!(r.descriptor.scope, BindingScope::Contract { .. });
                r.value = if (contract && !(0..=100 * SCALE).contains(&value.0))
                    || (!contract && value.0 <= 0)
                {
                    None
                } else {
                    Some(value.0)
                };
                if r.value.is_none() {
                    r.bad_cell = true;
                }
            }
            EvidenceChange::Validity {
                binding_id,
                valid_until_ms,
                reason,
            } => {
                let r = self
                    .raw
                    .iter_mut()
                    .inspect(inspect_entry)
                    .find(|r| r.descriptor.binding_id == *binding_id)
                    .ok_or_else(|| evidence("unregistered_binding"))?;
                r.valid_until = if reason.is_none() { *valid_until_ms } else { 0 };
                if r.valid_until <= self.frontier {
                    r.bad_cell = true;
                }
            }
            EvidenceChange::MarketBoundary { market_id } => {
                let m = self
                    .context
                    .markets
                    .iter()
                    .inspect(inspect_entry)
                    .find(|m| m.market_id == *market_id)
                    .ok_or_else(|| evidence("incompatible_market"))?;
                if i128::from(m.open_ms) != i128::from(self.frontier) {
                    return Err(evidence("incompatible_market"));
                }
                if self.ready_at.is_some_and(|ready| ready < self.frontier) {
                    self.market = Some(market_id.clone());
                    self.checkpoints.clear();
                    self.temporal.clear();
                    self.truths.clear();
                    self.continuities.clear();
                    self.last_objective = None;
                }
            }
            EvidenceChange::RegisterMarket { .. } | EvidenceChange::Gap { .. } => {
                return Err(evidence("internal_evaluation_failure"));
            }
        }
        Ok(())
    }
    fn invalidate_series(&mut self, binding_id: u32, start: u64, end: u64, index: usize) {
        fixed_work(1);
        let Some(raw) = self
            .raw
            .iter()
            .inspect(inspect_entry)
            .find(|r| r.descriptor.binding_id == binding_id)
        else {
            return;
        };
        let series = &mut self.series[index];
        let m = &self.strategy.document.measurements[series.measurement];
        if scope_market(&raw.descriptor.scope)
            .is_some_and(|market| series.market.as_deref() != Some(market))
            || (self.strategy.numeric_selected(&raw.descriptor.input_id)
                && matches!(raw.descriptor.scope, BindingScope::Contract { side, .. } if side != series.side))
            || !calculation_values(&m.calculation)
                .into_iter()
                .any(|v| self.strategy.value_depends(v, &raw.descriptor.input_id))
        {
            return;
        }
        tree_work(self.strategy.numeric_info.len(), false);
        let source = self.strategy.numeric_info[&raw.descriptor.input_id]
            .dependencies
            .trailing_zeros() as usize;
        tree_work(self.strategy.numeric_info.len(), false);
        let horizon = self.strategy.numeric_info[&m.id].horizons[source];
        if start < self.frontier && end > self.frontier.saturating_sub(horizon) {
            if let Some(history) = &mut series.history {
                // Recollect contradicted coverage without rewriting prior decisions.
                history.count = 0;
            }
            series.current = None;
            series.previous = None;
        }
    }
    fn raw_value(
        &self,
        id: &str,
        market: Option<&str>,
        side: Side,
        cell: bool,
        _batch: &AdvanceBatch,
    ) -> Option<i128> {
        fixed_work(1);
        let r = self.raw.iter().inspect(inspect_entry).find(|r| {
            r.descriptor.input_id == id
                && scope_market(&r.descriptor.scope).is_none_or(|m| Some(m) == market)
                && match r.descriptor.scope {
                    BindingScope::Contract { side: s, .. } => !r.selected || s == side,
                    _ => true,
                }
        })?;
        if r.valid_until <= self.frontier {
            return None;
        }
        if cell && (r.bad_cell || !r.cell_covered) {
            return None;
        }
        r.value
    }
    fn value(
        &self,
        v: &ValueSpec,
        market: Option<&str>,
        side: Side,
        previous: bool,
        cell: bool,
        batch: &AdvanceBatch,
    ) -> Option<i128> {
        fixed_work(1);
        match v {
            ValueSpec::Literal { value, .. } => Decimal::parse(value).ok().map(|v| v.0),
            ValueSpec::Reference { reference } => {
                let selected = self.strategy.numeric_selected(reference);
                if let Some(i) = self
                    .strategy
                    .document
                    .measurements
                    .iter()
                    .inspect(inspect_entry)
                    .position(|m| m.id == *reference)
                {
                    let s = self.series.iter().inspect(inspect_entry).find(|s| {
                        s.measurement == i
                            && (s.market.is_none() || s.market.as_deref() == market)
                            && (!selected || s.side == side)
                    })?;
                    if previous {
                        if cell && !s.cell_valid {
                            return None;
                        }
                        s.previous
                    } else {
                        s.current
                    }
                } else {
                    self.raw_value(reference, market, side, cell, batch)
                }
            }
        }
    }
    fn unavailable_reason(
        &self,
        value: &ValueSpec,
        market: Option<&str>,
        side: Side,
    ) -> &'static str {
        fixed_work(1);
        let ValueSpec::Reference { reference } = value else {
            return "unavailable";
        };
        let selected = self.strategy.numeric_selected(reference);
        let Some(i) = self
            .strategy
            .document
            .measurements
            .iter()
            .inspect(inspect_entry)
            .position(|m| m.id == *reference)
        else {
            return "unavailable";
        };
        self.series
            .iter()
            .inspect(inspect_entry)
            .find(|s| {
                s.measurement == i
                    && (s.market.is_none() || s.market.as_deref() == market)
                    && (!selected || s.side == side)
            })
            .map_or("unavailable", |s| s.unavailable_reason)
    }
    fn compute_node(&mut self, n: usize, batch: &AdvanceBatch) {
        fixed_work(1);
        let s = &self.series[n];
        let m = &self.strategy.document.measurements[s.measurement];
        let c = m.calculation.clone();
        let market = s.market.clone();
        let side = s.side;
        let values = calculation_values(&c)
            .into_iter()
            .map(|v| self.value(v, market.as_deref(), side, false, false, batch))
            .collect::<Vec<_>>();
        vector_copy_work(values.len());
        let mut reason = calculation_values(&c)
            .into_iter()
            .zip(&values)
            .filter(|(_, value)| value.is_none())
            .map(|(operand, _)| self.unavailable_reason(operand, market.as_deref(), side))
            .find(|reason| *reason != "unavailable")
            .unwrap_or("unavailable");
        let result = if lookback(&c) > 0 {
            self.series[n]
                .history
                .as_ref()
                .and_then(|h| h.result(&c, values[0], self.frontier))
        } else {
            match (values[0], values[1]) {
                (Some(a), Some(b)) => {
                    if matches!(c, Calculation::PercentageDifference { .. }) && b == 0 {
                        reason = "zero_denominator";
                    }
                    let diff = a.checked_sub(b);
                    match c {
                        Calculation::Difference { .. } => diff,
                        Calculation::AbsoluteDifference { .. } => diff.and_then(i128::checked_abs),
                        Calculation::PercentageDifference { .. } => diff
                            .and_then(|v| v.checked_mul(100 * SCALE))
                            .and_then(|v| divide_even(v, b)),
                        _ => None,
                    }
                }
                _ => None,
            }
        };
        if result.is_some_and(|value| value.unsigned_abs() > LIMIT as u128) {
            reason = "numeric_range_exceeded";
        }
        self.series[n].current = result.filter(|x| x.unsigned_abs() <= LIMIT as u128);
        self.series[n].unavailable_reason = reason;
        self.series[n].continuous = if lookback(&c) > 0 {
            self.series[n].current.is_some() && self.series[n].previous.is_some()
        } else {
            calculation_values(&c)
                .into_iter()
                .all(|v| self.value_continuous(v, market.as_deref(), side))
        };
    }
    fn finish_compute(&mut self) {
        fixed_work(1);
        if self.ready_at.is_none()
            && self
                .series
                .iter()
                .inspect(inspect_entry)
                .filter(|s| s.history.is_some())
                .all(|s| s.current.is_some())
            && self
                .strategy
                .requirements
                .startup_ms
                .values()
                .inspect(inspect_entry)
                .copied()
                .max()
                .unwrap_or(0)
                <= self.frontier
        {
            self.ready_at = Some(self.frontier);
        }
    }
    fn predicate(
        &mut self,
        c: &Predicate,
        path: &str,
        side: Side,
        transaction: Option<&NormalizedTransaction>,
        batch: &AdvanceBatch,
    ) -> Truth {
        fixed_work(1);
        let result = self.predicate_eval(c, path, side, transaction, batch);
        tree_work(self.continuities.len(), true);
        self.continuities.insert(
            (path.to_owned(), if side == Side::Yes { 0 } else { 1 }),
            self.condition_continuous(c, path, side),
        );
        tree_work(self.truths.len(), true);
        self.truths.insert(
            (path.to_owned(), if side == Side::Yes { 0 } else { 1 }),
            result,
        );
        result
    }
    fn child_truth(
        &self,
        _c: &Predicate,
        path: &str,
        side: Side,
        _transaction: Option<&NormalizedTransaction>,
        _batch: &AdvanceBatch,
    ) -> Truth {
        fixed_work(1);
        tree_work(self.truths.len(), false);
        self.truths
            .get(&(path.to_owned(), if side == Side::Yes { 0 } else { 1 }))
            .copied()
            .unwrap_or(Truth::Unavailable)
    }
    fn value_continuous(&self, v: &ValueSpec, market: Option<&str>, side: Side) -> bool {
        fixed_work(1);
        match v {
            ValueSpec::Literal { .. } => true,
            ValueSpec::Reference { reference } => {
                let selected = self.strategy.numeric_selected(reference);
                if let Some(i) = self
                    .strategy
                    .document
                    .measurements
                    .iter()
                    .inspect(inspect_entry)
                    .position(|m| m.id == *reference)
                {
                    self.series
                        .iter()
                        .inspect(inspect_entry)
                        .find(|s| {
                            s.measurement == i
                                && (s.market.is_none() || s.market.as_deref() == market)
                                && (!selected || s.side == side)
                        })
                        .is_some_and(|s| s.continuous)
                } else {
                    self.raw.iter().inspect(inspect_entry).find(|r|r.descriptor.input_id==*reference && scope_market(&r.descriptor.scope).is_none_or(|m|Some(m)==market) && (!selected || matches!(r.descriptor.scope,BindingScope::Contract{side:s,..} if s==side))).is_some_and(|r|r.covered_previous)
                }
            }
        }
    }
    fn condition_continuous(&self, c: &Predicate, path: &str, side: Side) -> bool {
        fixed_work(1);
        let cached = |path: String| {
            tree_work(self.continuities.len(), false);
            self.continuities
                .get(&(path, if side == Side::Yes { 0 } else { 1 }))
                .copied()
                .unwrap_or(false)
        };
        match c {
            Predicate::Compare { left, right, .. }
            | Predicate::CrossedBelow { left, right }
            | Predicate::CrossedAbove { left, right } => {
                self.value_continuous(left, self.market.as_deref(), side)
                    && self.value_continuous(right, self.market.as_deref(), side)
            }
            Predicate::All { .. } | Predicate::Any { .. } => cached(path.to_owned()),
            Predicate::HeldFor { .. } | Predicate::ConfirmedUpdates { .. } => {
                cached(format!("{path}.condition"))
            }
            _ => true,
        }
    }
    fn predicate_eval(
        &mut self,
        c: &Predicate,
        path: &str,
        side: Side,
        transaction: Option<&NormalizedTransaction>,
        batch: &AdvanceBatch,
    ) -> Truth {
        fixed_work(1);
        let market = self.market.clone();
        let values = |a: &ValueSpec, b: &ValueSpec| {
            self.value(a, market.as_deref(), side, false, false, batch)
                .zip(self.value(b, market.as_deref(), side, false, false, batch))
        };
        let truth = |b| if b { Truth::True } else { Truth::False };
        let key = (path.to_string(), if side == Side::Yes { 0 } else { 1 });
        match c {
            Predicate::Compare {
                left,
                comparison,
                right,
            } => values(left, right).map_or(Truth::Unavailable, |(a, b)| {
                truth(match comparison {
                    ComparisonOp::LessThan => a < b,
                    ComparisonOp::AtMost => a <= b,
                    ComparisonOp::EqualTo => a == b,
                    ComparisonOp::AtLeast => a >= b,
                    ComparisonOp::GreaterThan => a > b,
                })
            }),
            Predicate::All { .. } | Predicate::Any { .. } => {
                unreachable!("Boolean children are processed by the condition continuation")
            }
            Predicate::CrossedBelow { left, right } | Predicate::CrossedAbove { left, right } => {
                let pair = values(left, right);
                let continuous = self.value_continuous(left, self.market.as_deref(), side)
                    && self.value_continuous(right, self.market.as_deref(), side);
                let dependencies = [left, right].into_iter().fold(0, |mask, value| {
                    mask | match value {
                        ValueSpec::Reference { reference } => {
                            tree_work(self.strategy.numeric_info.len(), false);
                            self.strategy
                                .numeric_info
                                .get(reference)
                                .map_or(0, |info| info.dependencies)
                        }
                        ValueSpec::Literal { .. } => 0,
                    }
                });
                tree_work(self.temporal.len(), true);
                let state = self.temporal.entry(key).or_default();
                let Some((a, b)) = pair else {
                    state.baseline = None;
                    return Truth::Unavailable;
                };
                if !transaction.is_some_and(|t| {
                    self.raw.iter().inspect(inspect_entry).any(|r| {
                        r.observed_transaction == t.number
                            && scope_market(&r.descriptor.scope).is_none_or(|m| Some(m) == self.market.as_deref())
                            && (!r.selected
                                || matches!(r.descriptor.scope, BindingScope::Contract { side: s, .. } if s == side))
                            && dependencies & r.dependencies != 0
                    })
                }) {
                    return Truth::False;
                }
                if !continuous && state.baseline_ms < self.frontier {
                    state.baseline = None;
                }
                let crossed = state.baseline.is_some_and(|(x, y)| {
                    if matches!(c, Predicate::CrossedBelow { .. }) {
                        x >= y && a < b
                    } else {
                        x <= y && a > b
                    }
                });
                state.baseline = Some((a, b));
                state.baseline_ms = self.frontier;
                truth(crossed)
            }
            Predicate::HeldFor {
                condition,
                duration_seconds,
            } => {
                let inner = self.child_truth(
                    condition,
                    &format!("{path}.condition"),
                    side,
                    transaction,
                    batch,
                );
                let continuous = self.condition_continuous(c, path, side);
                tree_work(self.temporal.len(), true);
                let state = self.temporal.entry(key).or_default();
                if !continuous && state.since.is_some_and(|since| since < self.frontier) {
                    state.since = None;
                }
                if inner != Truth::True {
                    state.since = None;
                    return inner;
                }
                let start = *state.since.get_or_insert(self.frontier);
                truth(self.frontier - start >= u64::from(*duration_seconds) * 1000)
            }
            Predicate::ConfirmedUpdates {
                condition,
                count,
                count_input,
            } => {
                let inner = self.child_truth(
                    condition,
                    &format!("{path}.condition"),
                    side,
                    transaction,
                    batch,
                );
                let number = transaction.and_then(|t| {
                    self.raw.iter().inspect(inspect_entry).find(|r| r.observed_transaction == t.number
                        && r.descriptor.input_id == *count_input
                        && scope_market(&r.descriptor.scope).is_none_or(|m| Some(m) == self.market.as_deref())
                        && (!r.selected
                            || matches!(r.descriptor.scope, BindingScope::Contract { side: s, .. } if s == side)))
                        .map(|r| r.number)
                });
                tree_work(self.temporal.len(), true);
                let state = self.temporal.entry(key).or_default();
                if inner != Truth::True {
                    state.count = 0;
                    if let Some(n) = number {
                        state.last_number = n;
                    }
                    return inner;
                }
                if let Some(n) = number
                    && n > state.last_number
                {
                    state.last_number = n;
                    state.count = state.count.saturating_add(1).min(*count);
                }
                truth(state.count >= *count)
            }
            Predicate::CheckpointReached { checkpoint } => {
                tree_work(self.checkpoints.len(), false);
                truth(self.checkpoints.contains_key(checkpoint))
            }
            Predicate::AfterCheckpointUpdate {
                checkpoint,
                count_input,
            } => {
                tree_work(self.checkpoints.len(), false);
                let Some(cp) = self.checkpoints.get(checkpoint) else {
                    return Truth::False;
                };
                truth(self.raw.iter().inspect(inspect_entry).any(|r| {
                    r.observed_transaction > cp.transaction
                        && r.descriptor.input_id == *count_input
                        && scope_market(&r.descriptor.scope)
                            .is_none_or(|m| Some(m) == self.market.as_deref())
                        && (!r.selected
                            || matches!(r.descriptor.scope, BindingScope::Contract {side:s,..} if s == side))
                        && {
                            tree_work(cp.observations.len(), false);
                            r.number > cp.observations.get(&r.descriptor.binding_id).copied().unwrap_or(0)
                        }
                }))
            }
        }
    }
    fn selected(&self, selector: &Selector) -> Option<Side> {
        fixed_work(1);
        match selector {
            Selector::Fixed { side } => Some(*side),
            Selector::CheckpointSide {
                checkpoint,
                relation,
            } => {
                tree_work(self.checkpoints.len(), false);
                self.checkpoints.get(checkpoint)?.side.map(|s| {
                    if matches!(relation, RuleRelation::Opposite) {
                        s.opposite()
                    } else {
                        s
                    }
                })
            }
            Selector::UniqueSide => None,
        }
    }
    fn in_window(&self, w: Interval) -> bool {
        fixed_work(1);
        self.market
            .as_ref()
            .and_then(|id| {
                self.context
                    .markets
                    .iter()
                    .inspect(inspect_entry)
                    .find(|m| m.market_id == *id)
            })
            .is_some_and(|m| {
                let time = self.frontier as i128 - i128::from(m.open_ms);
                time >= i128::from(w.start_seconds) * 1000
                    && time < i128::from(w.end_seconds) * 1000
            })
    }
    fn evaluate_step(
        &mut self,
        cursor: &mut ConditionCursor,
        transaction: Option<&NormalizedTransaction>,
        objective: Option<&ObjectiveView>,
        batch: &AdvanceBatch,
    ) -> bool {
        fixed_work(1);
        if self.market.is_none() || cursor.plan == self.condition_plans.len() {
            if let Some(o) = objective
                && self.objective_valid(o)
            {
                self.last_objective = Some(o.clone());
            }
            return true;
        }
        let plan = &self.condition_plans[cursor.plan];
        if let Some(snapshot) = &mut cursor.checkpoint {
            if let Some(raw) = self.raw.get(cursor.observation) {
                inspect_entry(&raw);
                tree_work(snapshot.observations.len(), true);
                snapshot
                    .observations
                    .insert(raw.descriptor.binding_id, raw.number);
                cursor.observation += 1;
                return false;
            }
            tree_work(self.checkpoints.len(), true);
            self.checkpoints.insert(
                plan.checkpoint.as_ref().unwrap().id.clone(),
                cursor.checkpoint.take().unwrap(),
            );
            cursor.plan += 1;
            cursor.side = 0;
            cursor.node = 0;
            cursor.sides = None;
            cursor.observation = 0;
            return false;
        }
        if cursor.sides.is_none() {
            if plan.checkpoint.as_ref().is_some_and(|c| {
                tree_work(self.checkpoints.len(), false);
                self.checkpoints.contains_key(&c.id) || !self.in_window(c.during)
            }) {
                cursor.plan += 1;
                return false;
            }
            cursor.sides = Some(match &plan.selector {
                None => vec![Side::Yes],
                Some(Selector::UniqueSide) => vec![Side::Yes, Side::No],
                Some(selector) => self.selected(selector).into_iter().collect(),
            });
            vector_copy_work(cursor.sides.as_ref().unwrap().len());
            return false;
        }
        let sides = cursor.sides.as_ref().unwrap();
        if let Some(&side) = sides.get(cursor.side) {
            if let Some(node) = plan.nodes.get(cursor.node) {
                if !self.in_window(node.during) || cursor.step == node.steps.len() {
                    cursor.node += 1;
                    cursor.step = 0;
                    return false;
                }
                let (c, path) = &node.steps[cursor.step];
                if let Predicate::All { conditions } | Predicate::Any { conditions } = c {
                    let all = matches!(c, Predicate::All { .. });
                    let (truth, continuous) = cursor.aggregate.get_or_insert(if all {
                        (Truth::True, true)
                    } else {
                        (Truth::False, false)
                    });
                    if cursor.child < conditions.len() {
                        let key = (
                            format!("{path}.{}", cursor.child),
                            if side == Side::Yes { 0 } else { 1 },
                        );
                        tree_work(self.truths.len(), false);
                        tree_work(self.continuities.len(), false);
                        let child = self.truths.get(&key).copied().unwrap_or(Truth::Unavailable);
                        let continuity = self.continuities.get(&key).copied().unwrap_or(false);
                        if all {
                            *truth = match (*truth, child) {
                                (Truth::False, _) | (_, Truth::False) => Truth::False,
                                (Truth::Unavailable, _) | (_, Truth::Unavailable) => {
                                    Truth::Unavailable
                                }
                                _ => Truth::True,
                            };
                            *continuous &= continuity;
                        } else {
                            *truth = match (*truth, child) {
                                (Truth::True, _) | (_, Truth::True) => Truth::True,
                                (Truth::Unavailable, _) | (_, Truth::Unavailable) => {
                                    Truth::Unavailable
                                }
                                _ => Truth::False,
                            };
                            *continuous |= child == Truth::True && continuity;
                        }
                        cursor.child += 1;
                        #[cfg(test)]
                        BOOLEAN_CHILD_PROBES.with(|n| n.set(n.get() + 1));
                        return false;
                    }
                    let key = (path.clone(), if side == Side::Yes { 0 } else { 1 });
                    tree_work(self.truths.len(), true);
                    tree_work(self.continuities.len(), true);
                    self.truths.insert(key.clone(), *truth);
                    self.continuities.insert(key, *continuous);
                    cursor.aggregate = None;
                    cursor.child = 0;
                } else {
                    let (c, path) = (c.clone(), path.clone());
                    self.predicate(&c, &path, side, transaction, batch);
                }
                cursor.step += 1;
                return false;
            }
            cursor.side += 1;
            cursor.node = 0;
            return false;
        }
        if let Some(c) = &plan.checkpoint {
            let path = &plan.nodes[0].path;
            let truth = |side| self.child_truth(&c.when, path, side, transaction, batch);
            let selected = if matches!(plan.selector, Some(Selector::UniqueSide)) {
                match (truth(Side::Yes), truth(Side::No)) {
                    (Truth::True, Truth::False) => Some(Some(Side::Yes)),
                    (Truth::False, Truth::True) => Some(Some(Side::No)),
                    _ => None,
                }
            } else {
                sides
                    .first()
                    .filter(|s| truth(**s) == Truth::True)
                    .map(|s| plan.selector.as_ref().map(|_| *s))
            };
            if let Some(side) = selected {
                cursor.checkpoint = Some(CheckpointSnapshot {
                    side,
                    at_ms: self.frontier,
                    transaction: transaction.map_or(self.last_transaction, |t| t.number),
                    observations: Default::default(),
                });
                return false;
            }
        }
        cursor.plan += 1;
        cursor.side = 0;
        cursor.node = 0;
        cursor.sides = None;
        false
    }
    fn objective_valid(&self, o: &ObjectiveView) -> bool {
        fixed_work(1);
        self.market.as_deref() == Some(o.market_id.as_str())
            && o.requested_quantity > 0
            && o.confirmed_quantity <= o.requested_quantity
            && ((o.confirmed_quantity > 0) == o.locked_side.is_some())
            && self.last_objective.as_ref().is_none_or(|old| {
                old.requested_quantity == o.requested_quantity
                    && !(matches!(
                        old.attempt_state,
                        AttemptState::InFlight | AttemptState::Unknown
                    ) && o.attempt_state == AttemptState::None)
                    && old.confirmed_quantity <= o.confirmed_quantity
                    && (old.locked_side.is_none() || old.locked_side == o.locked_side)
            })
    }
    fn decision_step(
        &mut self,
        out: &mut DecisionCursor,
        objective: Option<&ObjectiveView>,
        batch: &AdvanceBatch,
    ) -> bool {
        fixed_work(1);
        match out.stage {
            0 => {
                if let Some(plan) = self.leaf_plans.get(out.index) {
                    self.choose(
                        &plan.leaf,
                        &plan.ancestors,
                        objective,
                        batch,
                        &mut out.result.leaves,
                        &mut out.result.candidate,
                    );
                    vector_growth(&out.publication.leaves);
                    out.publication
                        .leaves
                        .push(out.result.leaves.last().unwrap().clone());
                    out.index += 1;
                    return false;
                }
                out.publication.candidate = out.result.candidate.clone();
                out.unchanged &= self.published.phase == out.result.phase
                    && self.published.candidate == out.result.candidate;
            }
            1 => {
                if let Some(leaf) = out.result.leaves.get(out.index) {
                    if !leaf.eligible {
                        out.reason(
                            rule_error(&self.leaf_plans[out.index].path, leaf.reason),
                            &self.published,
                        );
                    }
                    out.index += 1;
                    return false;
                }
            }
            2 => {
                if let Some(series) = self.series.get(out.index) {
                    if series.current.is_none()
                        && (series.market.is_none() || series.market == self.market)
                    {
                        let id = &self.strategy.document.measurements[series.measurement].id;
                        out.reason(
                            rule_error(
                                &format!("$.measurements[{}].{id}", series.measurement),
                                series.unavailable_reason,
                            ),
                            &self.published,
                        );
                    }
                    out.index += 1;
                    return false;
                }
            }
            3 => {
                if let Some(raw) = self.raw.get(out.index) {
                    tree_work(self.strategy.requirements.startup_ms.len(), false);
                    let required_ms = self
                        .strategy
                        .requirements
                        .startup_ms
                        .get(&raw.descriptor.input_id)
                        .copied()
                        .unwrap_or(0);
                    let progress = HistoryProgress {
                        binding_id: raw.descriptor.binding_id,
                        input_id: raw.descriptor.input_id.clone(),
                        required_ms,
                        collected_ms: raw.covered_cells.min(required_ms),
                    };
                    vector_growth(&out.publication.progress);
                    out.publication.progress.push(progress.clone());
                    vector_growth(&out.result.progress);
                    out.result.progress.push(progress);
                    out.index += 1;
                    return false;
                }
            }
            4 => {
                if let Some((id, cp)) = self
                    .checkpoints
                    .iter()
                    .inspect(inspect_entry)
                    .nth(out.index)
                {
                    tree_work(self.published.checkpoints.len(), false);
                    tree_clone_work(cp.observations.len());
                    out.unchanged &= self.published.checkpoints.get(id) == Some(cp);
                    tree_work(out.result.checkpoints.len(), true);
                    tree_work(out.publication.checkpoints.len(), true);
                    tree_clone_work(cp.observations.len());
                    tree_clone_work(cp.observations.len());
                    out.result.checkpoints.insert(id.clone(), cp.clone());
                    out.publication.checkpoints.insert(id.clone(), cp.clone());
                    out.index += 1;
                    return false;
                }
            }
            _ => {
                out.unchanged &= self.published.reasons.len() == out.result.reasons.len()
                    && self.published.checkpoints.len() == out.result.checkpoints.len();
                let repeated = if out.unchanged {
                    self.published.repeated_waits.saturating_add(1)
                } else {
                    0
                };
                out.result.repeated_waits = repeated;
                out.publication.repeated_waits = repeated;
                out.result.evaluated_through_ms = self.frontier;
                out.publication.evaluated_through_ms = self.frontier;
                return true;
            }
        }
        out.stage += 1;
        out.index = 0;
        false
    }
    fn choose(
        &self,
        o: &LeafNode,
        ancestors: &[LeafNode],
        objective: Option<&ObjectiveView>,
        batch: &AdvanceBatch,
        leaves: &mut Vec<LeafResult>,
        candidate: &mut Option<Candidate>,
    ) {
        fixed_work(1);
        let market_elapsed = self.market.as_ref().and_then(|id| {
            self.context
                .markets
                .iter()
                .inspect(inspect_entry)
                .find(|m| m.market_id == *id)
                .map(|m| i128::from(self.frontier) - i128::from(m.open_ms))
        });
        let mut reason = "condition_false";
        let mut side = None;
        if let Some(selector) = &o.side {
            let choices = if matches!(selector, Selector::UniqueSide) {
                vec![Side::Yes, Side::No]
            } else {
                self.selected(selector).into_iter().collect()
            };
            vector_copy_work(choices.len());
            let mut results = Vec::new();
            for s in choices {
                let mut result = Truth::True;
                for n in ancestors
                    .iter()
                    .inspect(inspect_entry)
                    .chain(std::iter::once(o))
                {
                    if !market_elapsed.is_some_and(|time| {
                        time >= i128::from(n.during.start_seconds) * 1000
                            && time < i128::from(n.during.end_seconds) * 1000
                    }) {
                        result = Truth::False;
                        continue;
                    }
                    tree_work(self.truths.len(), false);
                    let value = self
                        .truths
                        .get(&(
                            format!("option.{}.{}", o.id, n.id),
                            if s == Side::Yes { 0 } else { 1 },
                        ))
                        .copied()
                        .unwrap_or(Truth::Unavailable);
                    if value == Truth::False {
                        result = Truth::False;
                    } else if value == Truth::Unavailable && result == Truth::True {
                        result = Truth::Unavailable;
                    }
                }
                vector_growth(&results);
                results.push((s, result));
            }
            if matches!(selector, Selector::UniqueSide) {
                if results.len() == 2 {
                    side = match (results[0].1, results[1].1) {
                        (Truth::True, Truth::False) => Some(Side::Yes),
                        (Truth::False, Truth::True) => Some(Side::No),
                        _ => None,
                    };
                    if results
                        .iter()
                        .inspect(inspect_entry)
                        .any(|(_, v)| *v == Truth::Unavailable)
                    {
                        reason = "unavailable";
                    } else if results
                        .iter()
                        .inspect(inspect_entry)
                        .all(|(_, v)| *v == Truth::True)
                    {
                        reason = "ambiguous_side";
                    }
                }
            } else {
                if results
                    .first()
                    .is_some_and(|(_, value)| *value == Truth::Unavailable)
                {
                    reason = "unavailable";
                }
                side = results
                    .into_iter()
                    .find(|(_, v)| *v == Truth::True)
                    .map(|(s, _)| s);
            }
        }
        let mut eligible = false;
        if let Some(s) = side {
            reason = "objective_blocked";
            if let Some(objective) = objective
                && self.objective_valid(objective)
                && objective.safeguards.allowed
                && objective.confirmed_quantity < objective.requested_quantity
                && !matches!(
                    objective.attempt_state,
                    AttemptState::InFlight | AttemptState::Unknown
                )
                && objective.locked_side.is_none_or(|locked| locked == s)
                && matches!(batch.current_admission, Admission::Allowed)
            {
                let range = if s == Side::Yes {
                    &objective.safeguards.yes
                } else {
                    &objective.safeguards.no
                };
                let ask_id = self
                    .strategy
                    .document
                    .inputs
                    .iter()
                    .inspect(inspect_entry)
                    .find(|i| {
                        i.field == "buyable_ask"
                            && (i.side.as_deref() == Some("selected_side")
                                || i.side.as_deref()
                                    == Some(if s == Side::Yes { "yes" } else { "no" }))
                    })
                    .map(|i| i.id.as_str())
                    .unwrap_or(if s == Side::Yes {
                        "__buyable_ask_yes"
                    } else {
                        "__buyable_ask_no"
                    });
                if let Some(ask) = self.raw_value(ask_id, self.market.as_deref(), s, false, batch)
                    && range.minimum.0 >= 0
                    && range.maximum.0 <= 100 * SCALE
                    && range.minimum.0 <= ask
                    && ask <= range.maximum.0
                {
                    eligible = true;
                    reason = if candidate.is_some() {
                        "lower_priority"
                    } else {
                        "eligible"
                    };
                    if candidate.is_none() {
                        *candidate = Some(Candidate {
                            market_id: objective.market_id.clone(),
                            option_id: o.id.clone(),
                            side: s,
                            remaining_quantity: objective.requested_quantity
                                - objective.confirmed_quantity,
                        });
                    }
                }
            }
        }
        vector_growth(leaves);
        leaves.push(LeafResult {
            option_id: o.id.clone(),
            eligible,
            reason,
        });
    }
}
fn scope_market(scope: &BindingScope) -> Option<&str> {
    fixed_work(1);
    match scope {
        BindingScope::Underlying => None,
        BindingScope::Market { market_id } | BindingScope::Contract { market_id, .. } => {
            Some(market_id)
        }
    }
}
fn change_binding(c: &EvidenceChange) -> Option<u32> {
    fixed_work(1);
    match c {
        EvidenceChange::Observation { binding_id, .. }
        | EvidenceChange::Resynchronize { binding_id, .. }
        | EvidenceChange::Validity { binding_id, .. }
        | EvidenceChange::Gap { binding_id, .. } => Some(*binding_id),
        _ => None,
    }
}
impl ValidatedStrategy {
    fn numeric_selected(&self, id: &str) -> bool {
        fixed_work(1);
        tree_work(self.numeric_info.len(), false);
        self.numeric_info.get(id).is_some_and(|info| info.selected)
    }
    fn value_depends(&self, value: &ValueSpec, input: &str) -> bool {
        fixed_work(1);
        let ValueSpec::Reference { reference } = value else {
            return false;
        };
        tree_work(self.numeric_info.len(), false);
        tree_work(self.numeric_info.len(), false);
        self.numeric_info
            .get(reference)
            .zip(self.numeric_info.get(input))
            .is_some_and(|(value, input)| value.dependencies & input.dependencies != 0)
    }
}

fn validate_binding_text(binding: &BindingDescriptor) -> Result<(), EvidenceError> {
    fixed_work(1);
    let scope_fits = match &binding.scope {
        BindingScope::Underlying => true,
        BindingScope::Market { market_id } => market_id.len() <= 128,
        BindingScope::Contract {
            market_id,
            contract_id,
            ..
        } => market_id.len() <= 128 && contract_id.len() <= 128,
    };
    if binding.input_id.len() > 64 || !scope_fits {
        return Err(evidence("unregistered_binding"));
    }
    Ok(())
}
fn add_binding_to(
    strategy: &ValidatedStrategy,
    context: &mut RunContext,
    raw: &mut Vec<RawState>,
    b: BindingDescriptor,
) -> Result<(), EvidenceError> {
    fixed_work(1);
    validate_binding_text(&b)?;
    // Normalize caller-owned spare capacity before retaining this definition.
    let b = b.clone();
    if let Some(old) = raw.iter().find(|r| {
        #[cfg(test)]
        REGISTRATION_PROBES.with(|n| n.set(n.get() + 1));
        r.descriptor.binding_id == b.binding_id
    }) {
        return if old.descriptor == b {
            Ok(())
        } else {
            Err(evidence("unregistered_binding"))
        };
    }
    let input = strategy.document.inputs.iter().find(|i| {
        #[cfg(test)]
        REGISTRATION_PROBES.with(|n| n.set(n.get() + 1));
        i.id == b.input_id
    });
    if let Some(i) = input {
        match (&b.scope, i.instrument.as_str(), i.side.as_deref()) {
            (BindingScope::Underlying, "btc_usd", None) => (),
            (BindingScope::Market { market_id }, "current_market", None)
                if context.markets.iter().any(|m| {
                    #[cfg(test)]
                    REGISTRATION_PROBES.with(|n| n.set(n.get() + 1));
                    m.market_id == *market_id
                }) => {}
            (
                BindingScope::Contract {
                    market_id,
                    contract_id,
                    side,
                },
                "current_contract",
                Some(s),
            ) if context.markets.iter().any(|m| {
                #[cfg(test)]
                REGISTRATION_PROBES.with(|n| n.set(n.get() + 1));
                m.market_id == *market_id && m.contract_id == *contract_id
            }) && (s == "selected_side"
                || s == if *side == Side::Yes { "yes" } else { "no" }) => {}
            _ => return Err(evidence("incompatible_market")),
        }
    } else {
        let expected_side = match b.input_id.as_str() {
            "__buyable_ask_yes" => Side::Yes,
            "__buyable_ask_no" => Side::No,
            _ => return Err(evidence("unregistered_binding")),
        };
        if !matches!(&b.scope, BindingScope::Contract { market_id, contract_id, side }
            if *side == expected_side && context.markets.iter().inspect(inspect_entry).any(|m|
                m.market_id == *market_id && m.contract_id == *contract_id))
        {
            return Err(evidence("incompatible_market"));
        }
    }
    if raw.iter().any(|r| {
        #[cfg(test)]
        REGISTRATION_PROBES.with(|n| n.set(n.get() + 1));
        r.descriptor.input_id == b.input_id && r.descriptor.scope == b.scope
    }) {
        return Err(evidence("unregistered_binding"));
    }
    if raw.len() >= 128 {
        return Err(evidence("resource_limit"));
    }
    vector_growth(raw);
    tree_work(strategy.numeric_info.len(), false);
    raw.push(RawState {
        dependencies: strategy
            .numeric_info
            .get(&b.input_id)
            .map_or(0, |i| i.dependencies),
        selected: strategy.numeric_selected(&b.input_id),
        descriptor: b.clone(),
        value: None,
        number: 0,
        valid_until: 0,
        last_observed_value: None,
        observed_transaction: 0,
        covered_previous: false,
        cell_covered: false,
        covered_cells: 0,
        bad_cell: false,
    });
    vector_growth(&context.bindings);
    context.bindings.push(b);
    Ok(())
}

#[derive(Default)]
struct RegistrationCursor {
    descriptor_done: bool,
    binding: usize,
}
impl RegistrationCursor {
    fn step(
        &mut self,
        strategy: &ValidatedStrategy,
        context: &mut RunContext,
        raw: &mut Vec<RawState>,
        d: &MarketDescriptor,
        bindings: &[BindingDescriptor],
    ) -> Result<bool, EvidenceError> {
        fixed_work(1);
        if !self.descriptor_done {
            let valid_id = |x: &str| {
                !x.is_empty()
                    && x.len() <= 128
                    && !x.chars().inspect(inspect_entry).any(char::is_control)
            };
            if !valid_id(&d.series_id)
                || !valid_id(&d.market_id)
                || !valid_id(&d.contract_id)
                || d.series_id != context.series_id
                || d.close_ms.checked_sub(d.open_ms) != Some(900000)
            {
                return Err(evidence("incompatible_market"));
            }
            if let Some(old) = context.markets.iter().find(|m| {
                #[cfg(test)]
                REGISTRATION_PROBES.with(|n| n.set(n.get() + 1));
                m.market_id == d.market_id
            }) {
                if old != d {
                    return Err(evidence("incompatible_market"));
                }
            } else {
                if context.markets.len() >= 128 {
                    return Err(evidence("resource_limit"));
                }
                vector_growth(&context.markets);
                context.markets.push(d.clone());
            }
            self.descriptor_done = true;
            return Ok(false);
        }
        if let Some(b) = bindings.get(self.binding) {
            validate_binding_text(b)?;
            add_binding_to(strategy, context, raw, b.clone())?;
            self.binding += 1;
            return Ok(false);
        }
        *self = Self::default();
        Ok(true)
    }
}
fn register_to(
    strategy: &ValidatedStrategy,
    context: &mut RunContext,
    raw: &mut Vec<RawState>,
    d: MarketDescriptor,
    bindings: Vec<BindingDescriptor>,
) -> Result<(), EvidenceError> {
    let mut cursor = RegistrationCursor::default();
    while !cursor.step(strategy, context, raw, &d, &bindings)? {}
    Ok(())
}

#[derive(Default)]
struct AllocationCursor {
    measurement: usize,
    market: usize,
    side: usize,
    existing: usize,
    candidate: Option<(usize, Option<String>, Side)>,
    instances: usize,
    planned: usize,
    allocated: usize,
    accounted_existing: usize,
}
impl AllocationCursor {
    // Each step examines one existing instance or one concrete node/market/side.
    fn step(
        &mut self,
        strategy: &ValidatedStrategy,
        context: &RunContext,
        existing: &[Series],
        added: &mut Vec<Series>,
    ) -> Result<bool, EvidenceError> {
        fixed_work(1);
        if let Some(s) = existing.get(self.accounted_existing) {
            #[cfg(test)]
            ALLOCATION_PROBES.with(|n| n.set(n.get() + 1));
            self.allocated = self
                .allocated
                .checked_add(series_bytes(s))
                .ok_or_else(|| evidence("resource_limit"))?;
            self.accounted_existing += 1;
            return Ok(false);
        }
        if let Some((i, market, side)) = &self.candidate {
            if let Some(s) = existing.get(self.existing) {
                #[cfg(test)]
                ALLOCATION_PROBES.with(|n| n.set(n.get() + 1));
                self.existing += 1;
                if s.measurement == *i && s.market == *market && s.side == *side {
                    self.candidate = None;
                }
                return Ok(false);
            }
            let c = &strategy.document.measurements[*i].calculation;
            // Existing later nodes still own their buffers while this earlier node is staged.
            self.allocated
                .checked_add(history_bytes(c))
                .filter(|bytes| *bytes <= MEMORY_LIMIT - RESERVE)
                .ok_or_else(|| evidence("resource_limit"))?;
            let history = if lookback(c) > 0 {
                Some(History::new(c)?)
            } else {
                None
            };
            let s = Series {
                market: market.clone(),
                side: *side,
                measurement: *i,
                current: None,
                previous: None,
                cell_valid: false,
                continuous: false,
                history,
                unavailable_reason: "unavailable",
            };
            self.allocated = self
                .allocated
                .checked_add(series_bytes(&s))
                .filter(|n| *n <= MEMORY_LIMIT - RESERVE)
                .ok_or_else(|| evidence("resource_limit"))?;
            vector_growth(added);
            added.push(s);
            self.candidate = None;
            return Ok(false);
        }
        let Some(&i) = strategy.order.get(self.measurement) else {
            return Ok(true);
        };
        let (market_dependent, selected) = strategy.numeric_scopes[i];
        let market_count = if market_dependent {
            context.markets.len()
        } else {
            1
        };
        if self.market == market_count {
            self.measurement += 1;
            self.market = 0;
            return Ok(false);
        }
        #[cfg(test)]
        ALLOCATION_PROBES.with(|n| n.set(n.get() + 1));
        let market = if market_dependent {
            Some(context.markets[self.market].market_id.clone())
        } else {
            None
        };
        let side = if self.side == 0 { Side::Yes } else { Side::No };
        self.side += 1;
        if self.side == if selected { 2 } else { 1 } {
            self.side = 0;
            self.market += 1;
        }
        self.instances += 1;
        self.planned = self
            .planned
            .checked_add(history_bytes(
                &strategy.document.measurements[i].calculation,
            ))
            .filter(|n| *n <= MEMORY_LIMIT - RESERVE)
            .ok_or_else(|| evidence("resource_limit"))?;
        if self.instances + context.bindings.len() > 128 {
            return Err(evidence("resource_limit"));
        }
        self.candidate = Some((i, market, side));
        self.existing = 0;
        Ok(false)
    }
}
fn allocate_new(
    strategy: &ValidatedStrategy,
    context: &RunContext,
    existing: &[Series],
) -> Result<Vec<Series>, EvidenceError> {
    let mut cursor = AllocationCursor::default();
    let mut added = Vec::new();
    while !cursor.step(strategy, context, existing, &mut added)? {}
    Ok(added)
}
fn series_bytes(s: &Series) -> usize {
    fixed_work(1);
    s.history.as_ref().map_or(0, |h| {
        h.ring.capacity() * 16 + (h.minimum.capacity() + h.maximum.capacity()) * 32
    })
}

fn graph_depth(id: &str, d: &Document, depth: usize) -> usize {
    if depth > 12 {
        return depth;
    }
    d.measurements
        .iter()
        .find(|m| m.id == id)
        .map_or(depth, |m| {
            calculation_values(&m.calculation)
                .into_iter()
                .map(|v| {
                    if let ValueSpec::Reference { reference } = v {
                        graph_depth(reference, d, depth + 1)
                    } else {
                        depth + 1
                    }
                })
                .max()
                .unwrap_or(depth + 1)
        })
}

fn history_bytes(c: &Calculation) -> usize {
    fixed_work(1);
    let w = lookback(c) as usize;
    match c {
        Calculation::Change { .. } | Calculation::PercentageChange { .. } => (w + 1) * 16,
        Calculation::RollingAverage { .. } => w * 16,
        Calculation::RollingMinimum { .. } | Calculation::RollingMaximum { .. } => (w + 1) * 32,
        Calculation::RollingRange { .. } => 2 * (w + 1) * 32,
        _ => 0,
    }
}

fn validate_shape(v: &serde_json::Value, kind: &str, p: &str) -> Result<(), RuleError> {
    let object = v.as_object().ok_or_else(|| rule_error(p, "malformed"))?;
    let tag = |field: &str| {
        object
            .get(field)
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| rule_error(&format!("{p}.{field}"), "malformed"))
    };
    let (required, optional): (&[&str], &[&str]) = match kind {
        "document" => (
            &["format", "name", "inputs", "purchase_options"],
            &["measurements", "checkpoints"],
        ),
        "input" => (&["id", "source", "instrument", "field", "unit"], &["side"]),
        "value" if object.contains_key("reference") => (&["reference"], &[]),
        "value" => (&["value", "unit"], &[]),
        "interval" => (&["start_seconds", "end_seconds"], &[]),
        "checkpoint" => (&["id", "when"], &["during", "remember_side"]),
        "option" => (&["id", "when"], &["during", "options", "side"]),
        "selector" => match tag("type")? {
            "fixed" => (&["type", "side"], &[]),
            "checkpoint_side" => (&["type", "checkpoint", "relation"], &[]),
            "unique_side" => (&["type"], &[]),
            _ => return Err(rule_error(&format!("{p}.type"), "unsupported_operation")),
        },
        "measurement" => match tag("operation")? {
            "difference" | "absolute_difference" => (&["id", "operation", "left", "right"], &[]),
            "percentage_difference" => (&["id", "operation", "left", "reference"], &[]),
            "change" | "percentage_change" | "rolling_average" | "rolling_minimum"
            | "rolling_maximum" | "rolling_range" => {
                (&["id", "operation", "of", "lookback_seconds"], &[])
            }
            _ => {
                return Err(rule_error(
                    &format!("{p}.operation"),
                    "unsupported_operation",
                ));
            }
        },
        "condition" => match tag("type")? {
            "compare" => (&["type", "left", "comparison", "right"], &[]),
            "all" | "any" => (&["type", "conditions"], &[]),
            "crossed_below" | "crossed_above" => (&["type", "left", "right"], &[]),
            "held_for" => (&["type", "condition", "duration_seconds"], &[]),
            "confirmed_updates" => (&["type", "condition", "count", "count_input"], &[]),
            "checkpoint_reached" => (&["type", "checkpoint"], &[]),
            "after_checkpoint_update" => (&["type", "checkpoint", "count_input"], &[]),
            _ => return Err(rule_error(&format!("{p}.type"), "unsupported_operation")),
        },
        _ => return Err(rule_error(p, "malformed")),
    };
    if object
        .keys()
        .any(|key| !required.contains(&key.as_str()) && !optional.contains(&key.as_str()))
    {
        return Err(rule_error(p, "unknown"));
    }
    for key in required {
        if !object.contains_key(*key) {
            return Err(rule_error(&format!("{p}.{key}"), "malformed"));
        }
    }
    for (key, value) in object {
        let child = match key.as_str() {
            "inputs" => Some(("input", true)),
            "measurements" => Some(("measurement", true)),
            "checkpoints" => Some(("checkpoint", true)),
            "purchase_options" | "options" => Some(("option", true)),
            "conditions" => Some(("condition", true)),
            "when" | "condition" => Some(("condition", false)),
            "left" | "right" | "of" => Some(("value", false)),
            "reference" if kind == "measurement" => Some(("value", false)),
            "during" => Some(("interval", false)),
            "remember_side" => Some(("selector", false)),
            "side" if kind == "option" => Some(("selector", false)),
            _ => None,
        };
        if let Some((child, array)) = child {
            let path = format!("{p}.{key}");
            if array {
                let entries = value
                    .as_array()
                    .ok_or_else(|| rule_error(&path, "malformed"))?;
                for (i, value) in entries.iter().enumerate() {
                    validate_shape(value, child, &format!("{path}[{i}]"))?;
                }
            } else {
                validate_shape(value, child, &path)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
fn processing_entries() -> usize {
    FIXED_WORK_PROBES.with(|n| n.get())
        + ENTRY_PROBES.with(|n| n.get())
        + TREE_WORK_PROBES.with(|n| n.get())
        + COPY_WORK_PROBES.with(|n| n.get())
        + CATALOG_PROBES.with(|n| n.get())
        + REGISTRATION_PROBES.with(|n| n.get())
        + ALLOCATION_PROBES.with(|n| n.get())
        + BOOLEAN_CHILD_PROBES.with(|n| n.get())
        + OUTPUT_DISCARD_PROBES.with(|n| n.get())
        + CLEANUP_PROBES.with(|n| n.get())
        + COVERAGE_PROBES.with(|n| n.get())
}

#[cfg(test)]
struct SliceWorkProbe(usize);
#[cfg(test)]
impl Drop for SliceWorkProbe {
    fn drop(&mut self) {
        let work = processing_entries() - self.0;
        MAX_SLICE_WORK.with(|maximum| maximum.set(maximum.get().max(work)));
        assert!(
            work <= 4096,
            "executed record primitives plus library bounds exceeded one slice: {work}"
        );
    }
}

// A record primitive performs one acyclic operation on fixed-width scalars and
// bounded identity fields. Loops, container entries and library movements are
// charged separately at their actual execution sites; see the resource proof.
#[inline]
fn fixed_work(records: usize) {
    #[cfg(test)]
    FIXED_WORK_PROBES.with(|n| n.set(n.get() + records));
    #[cfg(not(test))]
    let _ = records;
}

// Rust 1.100.0 (0ef1bc386): 11 keys/12 edges, minimum 5 non-root keys.
// Paired key/value moves are one entry. Lookup <=11h; split/insert <=62h,
// merge/remove <=77h, including node/edge visits and allocation calls; 80h
// covers either mutation. Re-derive these bounds when changing the toolchain.
#[inline]
fn tree_work(entries: usize, mutation: bool) {
    #[cfg(test)]
    {
        let mut height = 1;
        let mut minimum = 11usize;
        while entries >= minimum {
            height += 1;
            minimum = minimum.saturating_mul(6).saturating_add(5);
        }
        TREE_WORK_PROBES.with(|n| n.set(n.get() + height * if mutation { 80 } else { 11 }));
    }
    #[cfg(not(test))]
    let _ = (entries, mutation);
}

#[inline]
fn tree_clone_work(entries: usize) {
    #[cfg(test)]
    TREE_WORK_PROBES.with(|n| n.set(n.get() + entries + 1 + entries / 5));
    #[cfg(not(test))]
    let _ = entries;
}

#[inline]
fn vector_copy_work(entries: usize) {
    #[cfg(test)]
    COPY_WORK_PROBES.with(|n| n.set(n.get() + entries));
    #[cfg(not(test))]
    let _ = entries;
}

#[inline]
fn vector_growth<T>(values: &Vec<T>) {
    // A growing Vec may relocate every initialized entry, even if realloc stays in place.
    vector_copy_work(
        1 + if values.len() == values.capacity() {
            values.len()
        } else {
            0
        },
    );
}

// Independently count actual entries visited by explicit scans.
#[inline]
fn inspect_entry<T>(_: &T) {
    #[cfg(test)]
    ENTRY_PROBES.with(|n| n.set(n.get() + 1));
}

#[cfg(test)]
std::thread_local! {
    static FIXED_WORK_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static MAX_SLICE_WORK: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static COPY_WORK_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static TREE_WORK_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static ENTRY_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    // Actual entries inspected, independent of the continuation's outer allowance.
    static CATALOG_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static REGISTRATION_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static ALLOCATION_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static BOOLEAN_CHILD_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static OUTPUT_DISCARD_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static CLEANUP_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static COVERAGE_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

// Thread-local accounting isolates controlled tests from the parallel test runner.
#[cfg(test)]
mod heap_probe {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;

    std::thread_local! {
        static ENABLED: Cell<bool> = const { Cell::new(false) };
        static LIVE: Cell<isize> = const { Cell::new(0) };
        static PEAK: Cell<usize> = const { Cell::new(0) };
    }
    struct Allocator;
    #[global_allocator]
    static ALLOCATOR: Allocator = Allocator;
    fn record(delta: isize) {
        if ENABLED.try_with(Cell::get).unwrap_or(false) {
            LIVE.with(|live| {
                let bytes = live.get() + delta;
                live.set(bytes);
                PEAK.with(|peak| peak.set(peak.get().max(bytes.max(0) as usize)));
            });
        }
    }
    // Delegation preserves System's allocation/layout contract; counters allocate nothing.
    unsafe impl GlobalAlloc for Allocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let pointer = unsafe { System.alloc(layout) };
            if !pointer.is_null() {
                record(layout.size() as isize);
            }
            pointer
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            let pointer = unsafe { System.alloc_zeroed(layout) };
            if !pointer.is_null() {
                record(layout.size() as isize);
            }
            pointer
        }
        unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
            record(-(layout.size() as isize));
            unsafe { System.dealloc(pointer, layout) };
        }
        unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
            let replacement = unsafe { System.realloc(pointer, layout, size) };
            if !replacement.is_null() {
                record(size as isize - layout.size() as isize);
            }
            replacement
        }
    }
    pub fn live() -> usize {
        LIVE.with(Cell::get).try_into().expect("tracked ownership")
    }
    pub fn measure(work: impl FnOnce()) -> usize {
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                ENABLED.with(|enabled| enabled.set(false));
            }
        }
        assert!(!ENABLED.with(|enabled| enabled.replace(true)));
        let _reset = Reset;
        LIVE.with(|live| live.set(0));
        PEAK.with(|peak| peak.set(0));
        work();
        assert_eq!(
            LIVE.with(Cell::get),
            0,
            "measurement must own and release its allocations"
        );
        PEAK.with(Cell::get)
    }
}

#[cfg(test)]
mod controlled_failure_tests {
    use super::*;
    #[test]
    fn constructor_rejects_context_before_duplicating_over_budget_text() {
        let text = r#"{"format":2,"name":"Constructor ownership","inputs":[{"id":"usd","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}],"purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"usd"},"comparison":"greater_than","right":{"value":"1","unit":"usd_per_btc"}}}]}"#;
        let mut nested: serde_json::Value = serde_json::from_str(text).unwrap();
        let condition = nested["purchase_options"][0]["when"].clone();
        let mut option = nested["purchase_options"][0].clone();
        for depth in (0..7).rev() {
            option = serde_json::json!({"id": format!("group_{depth:058}"),
                "when": if depth == 0 {serde_json::json!({"type":"all","conditions":vec![condition.clone(); 494]})} else {condition.clone()},
                "options":[option]});
        }
        nested["purchase_options"] = serde_json::json!([option]);
        let nested = nested.to_string();
        for (text, context_bytes) in [(text, RESERVE - 64 * 1024), (nested.as_str(), 0)] {
            let strategy = ValidatedStrategy::parse(text).unwrap();
            let compiler_peak = heap_probe::measure(|| {
                let plans = condition_plans(&strategy);
                let mut leaves = Vec::new();
                leaf_plans_fn(&strategy.document.purchase_options, &[], &mut leaves);
                drop((plans, leaves));
            });
            assert!(compiler_peak <= compilation_bytes(&strategy));
            eprintln!(
                "compiler peak={compiler_peak}, bound={}",
                compilation_bytes(&strategy)
            );
            drop(strategy);
            let peak = heap_probe::measure(|| {
                let strategy = ValidatedStrategy::parse(text).unwrap();
                let context = RunContext {
                    strategy_version: "fixed".into(),
                    run_id: "x".repeat(if context_bytes == 0 {
                        (RESERVE - strategy.heap_bytes()) / 2 - 1024
                    } else {
                        context_bytes
                    }),
                    go_time: "controlled".into(),
                    series_id: "btc15m".into(),
                    markets: vec![],
                    bindings: vec![],
                };
                assert!(
                    matches!(Evaluator::new(strategy, context), Err(error) if error.code == "resource_limit")
                );
            });
            eprintln!("constructor rejected-context peak={peak}");
            assert!(peak <= RESERVE, "constructor peak {peak} exceeds {RESERVE}");
        }
    }

    #[test]
    fn memory_constructor_streams_supplied_duplicate_and_spare_capacity_bindings() {
        let text = r#"{"format":2,"name":"Supplied ownership","inputs":[{"id":"usd","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}],"purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"usd"},"comparison":"greater_than","right":{"value":"1","unit":"usd_per_btc"}}}]}"#;
        for mode in 0..4 {
            let peak = heap_probe::measure(|| {
                let strategy = ValidatedStrategy::parse(text).unwrap();
                let binding = BindingDescriptor {
                    binding_id: 1,
                    input_id: "usd".into(),
                    scope: BindingScope::Underlying,
                };
                let mut bindings = if mode == 0 || mode == 3 {
                    vec![binding; 60_000]
                } else {
                    let mut bindings = Vec::with_capacity(if mode == 1 { 60_000 } else { 1 });
                    bindings.push(binding);
                    bindings
                };
                if mode == 2 {
                    bindings[0].input_id.reserve_exact(4 * 1024 * 1024);
                }
                if mode == 3 {
                    bindings.last_mut().unwrap().scope = BindingScope::Market {
                        market_id: "unregistered".into(),
                    };
                }
                let result = Evaluator::new(
                    strategy,
                    RunContext {
                        strategy_version: "fixed".into(),
                        run_id: "duplicates".into(),
                        go_time: "controlled".into(),
                        series_id: "btc15m".into(),
                        markets: Vec::new(),
                        bindings,
                    },
                );
                if mode == 3 {
                    assert!(matches!(result, Err(error) if error.code == "unregistered_binding"));
                } else {
                    let evaluator = result.unwrap();
                    assert_eq!(evaluator.raw.len(), 1);
                    assert_eq!(evaluator.context.bindings.len(), 1);
                    assert_eq!(evaluator.context.bindings[0].input_id.capacity(), 3);
                    assert!(evaluator.context.bindings.capacity() <= 4);
                }
            });
            eprintln!("streamed constructor mode={mode}, independently measured peak={peak}");
            assert!(peak <= RESERVE);
        }
    }

    #[test]
    fn memory_latest_return_and_changed_outputs_remain_bounded_on_complete_stop_and_failure() {
        let input_id = format!("usd_{:060}", 0);
        let condition = serde_json::json!({"type":"compare","left":{"reference":input_id},
            "comparison":"greater_than","right":{"value":"0","unit":"usd_per_btc"}});
        let text = serde_json::json!({"format":2,"name":"Returned ownership",
            "inputs":(0..31).map(|n|serde_json::json!({"id":format!("usd_{n:060}"),
                "source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}))
                .chain(std::iter::once(serde_json::json!({"id":"ask","source":"kalshi",
                    "instrument":"current_contract","field":"buyable_ask","unit":"contract_cents","side":"selected_side"})))
                .collect::<Vec<_>>(),
            "checkpoints":(0..32).map(|n|serde_json::json!({"id":format!("checkpoint_{n:053}"),"when":condition})).collect::<Vec<_>>(),
            "purchase_options":(0..200).map(|n|serde_json::json!({"id":format!("leaf_{n:059}"),
                "side":{"type":"fixed","side":"yes"},"when":condition})).collect::<Vec<_>>()}).to_string();
        let market_id = format!("market_{:0121}", 0);
        let objective = ObjectiveView {
            market_id: market_id.clone(),
            requested_quantity: 10,
            confirmed_quantity: 0,
            locked_side: None,
            attempt_state: AttemptState::None,
            safeguards: Safeguards {
                allowed: true,
                yes: PriceRange {
                    minimum: Decimal(0),
                    maximum: Decimal(100 * SCALE),
                },
                no: PriceRange {
                    minimum: Decimal(0),
                    maximum: Decimal(100 * SCALE),
                },
            },
        };
        // Warm panic reporting's process/thread cache outside evaluator ownership.
        drop(std::panic::catch_unwind(|| {
            panic!("controlled allocator-probe unwind warmup")
        }));
        for terminal in 0..3 {
            let mut inventory = (0, 0, 0, 0, 0);
            let peak = heap_probe::measure(|| {
                let markets = (0..48)
                    .map(|n| MarketDescriptor {
                        series_id: "btc15m".into(),
                        market_id: format!("market_{n:0121}"),
                        contract_id: format!("contract_{n:0119}"),
                        open_ms: 1,
                        close_ms: 900001,
                    })
                    .collect::<Vec<_>>();
                let mut bindings = (0..31)
                    .map(|n| BindingDescriptor {
                        binding_id: n,
                        input_id: format!("usd_{n:060}"),
                        scope: BindingScope::Underlying,
                    })
                    .collect::<Vec<_>>();
                for market in &markets {
                    for side in [Side::Yes, Side::No] {
                        bindings.push(BindingDescriptor {
                            binding_id: bindings.len() as u32,
                            input_id: "ask".into(),
                            scope: BindingScope::Contract {
                                market_id: market.market_id.clone(),
                                contract_id: market.contract_id.clone(),
                                side,
                            },
                        });
                    }
                }
                let mut e = Evaluator::new(
                    ValidatedStrategy::parse(&text).unwrap(),
                    RunContext {
                        strategy_version: "fixed".into(),
                        run_id: "return".into(),
                        go_time: "controlled".into(),
                        series_id: "btc15m".into(),
                        markets,
                        bindings,
                    },
                )
                .unwrap();
                let initial = AdvanceBatch {
                    target_ms: 1,
                    received_through_ms: 1,
                    coverage: vec![],
                    current_admission: Admission::Allowed,
                    transactions: vec![
                        NormalizedTransaction {
                            number: 1,
                            at_ms: 0,
                            changes: (0..127)
                                .map(|binding_id| EvidenceChange::Validity {
                                    binding_id,
                                    valid_until_ms: 100,
                                    reason: None,
                                })
                                .collect(),
                        },
                        NormalizedTransaction {
                            number: 2,
                            at_ms: 1,
                            changes: std::iter::once(EvidenceChange::MarketBoundary {
                                market_id: market_id.clone(),
                            })
                            .chain((0..127).map(|binding_id| EvidenceChange::Observation {
                                binding_id,
                                observation_number: 1,
                                value: Decimal(30 * SCALE),
                            }))
                            .collect(),
                        },
                    ],
                };
                e.begin(initial).unwrap();
                let latest = loop {
                    match e.process_slice(SliceControl::Continue, Some(objective.clone())) {
                        SliceResult::Pending { .. } => (),
                        SliceResult::Complete(d) => break d,
                        other => panic!("{other:?}"),
                    }
                };
                assert_eq!(latest.checkpoints.len(), 32);
                assert_eq!(latest.leaves.len(), 200);
                let mut next = AdvanceBatch {
                    target_ms: if terminal == 2 { 2 } else { 1 },
                    received_through_ms: 2,
                    coverage: vec![],
                    current_admission: Admission::Allowed,
                    transactions: vec![NormalizedTransaction {
                        number: 3,
                        at_ms: 1,
                        changes: vec![EvidenceChange::Validity {
                            binding_id: 126,
                            valid_until_ms: 100,
                            reason: None,
                        }],
                    }],
                };
                let spare = RESERVE - e.metadata_reservation - next.ingress_bytes().unwrap();
                if let EvidenceChange::Validity { reason, .. } =
                    &mut next.transactions[0].changes[0]
                {
                    *reason = Some(String::with_capacity(spare));
                }
                e.begin(next).unwrap();
                if terminal == 2 {
                    e.fail_after_mutation = true;
                }
                if terminal == 2 {
                    loop {
                        match e.process_slice(SliceControl::Continue, Some(objective.clone())) {
                            SliceResult::Pending { .. } => (),
                            SliceResult::Failed { .. } => break,
                            other => panic!("{other:?}"),
                        }
                    }
                    assert!(e.pending.is_some());
                    assert_eq!(e.snapshot().checkpoints, latest.checkpoints);
                } else {
                    loop {
                        assert!(matches!(
                            e.process_slice(SliceControl::Continue, Some(objective.clone())),
                            SliceResult::Pending { .. }
                        ));
                        if e.pending
                            .as_ref()
                            .unwrap()
                            .output
                            .as_ref()
                            .is_some_and(|out| out.stage == 4 && out.index == 16)
                        {
                            break;
                        }
                    }
                    let mut changed = objective.clone();
                    changed.safeguards.allowed = false;
                    loop {
                        assert!(matches!(
                            e.process_slice(SliceControl::Continue, Some(changed.clone())),
                            SliceResult::Pending { .. }
                        ));
                        if e.pending.as_ref().unwrap().discarding_output {
                            break;
                        }
                    }
                    drop(changed);
                    if terminal == 1 {
                        assert_eq!(
                            e.process_slice(SliceControl::Stop, None),
                            SliceResult::Stopped
                        );
                        assert!(e.pending.as_ref().unwrap().output.is_some());
                    } else {
                        loop {
                            match e.process_slice(SliceControl::Continue, Some(objective.clone())) {
                                SliceResult::Pending { .. } => (),
                                SliceResult::Complete(d) => {
                                    assert_eq!(d.candidate, latest.candidate);
                                    assert_eq!(d.leaves, latest.leaves);
                                    assert_eq!(d.checkpoints, latest.checkpoints);
                                    break;
                                }
                                other => panic!("{other:?}"),
                            }
                        }
                    }
                    assert_eq!(e.snapshot().checkpoints, latest.checkpoints);
                }
                inventory = (
                    e.metadata_reservation,
                    spare,
                    heap_probe::live(),
                    latest.progress.capacity(),
                    latest
                        .checkpoints
                        .values()
                        .map(|cp| cp.observations.len())
                        .sum::<usize>(),
                );
            });
            eprintln!(
                "latest-result/changed-output mode={terminal}, independently measured peak={peak}, metadata/diagnostic/live/progress-capacity/observation-count={inventory:?}"
            );
            assert!(
                peak <= RESERVE,
                "ownership peak {peak} exceeds shared reserve"
            );
        }
    }

    #[test]
    fn memory_runtime_scratch_inventory_covers_each_exclusive_phase() {
        let predicate = std::mem::size_of::<Predicate>();
        let numeric = 128
            + 128
            + 2 * std::mem::size_of::<Option<i128>>()
            + 2 * std::mem::size_of::<&ValueSpec>();
        let registration = 2 * (64 + 128 + 128);
        let selection = 2 * (7 + 64 + 1 + 64)
            + 2 * std::mem::size_of::<Side>()
            + 4 * std::mem::size_of::<(Side, Truth)>();
        // Compiled conditions are shallow: Boolean arity never reaches predicate().
        // The maximal non-Boolean payload has two IDs, or one placeholder box plus ID.
        for payload in [128, predicate + 64] {
            for path in [1, 75, 136, 185, 246] {
                let owned = payload + 5 * path + 158;
                assert!(owned <= 8 * (predicate + payload + path));
            }
        }
        let auxiliary = numeric.max(registration).max(selection) + 128 + 2;
        assert!(auxiliary <= 4096);
        eprintln!(
            "scratch inventory Predicate={predicate}, numeric={numeric}, registration={registration}, selection={selection}, auxiliary simultaneous bound={auxiliary}"
        );
    }

    #[test]
    fn tree_reservation_covers_measured_insert_clone_remove_and_empty_root() {
        heap_probe::measure(|| {
            let mut map = std::collections::BTreeMap::<(String, usize), Temporal>::new();
            for n in 0..1024 {
                map.insert((format!("condition_{n:04}"), n), Temporal::default());
                assert!(
                    heap_probe::live()
                        <= tree_bytes::<(String, usize), Temporal>(map.len()) + map.len() * 14
                );
            }
            let cloned = map.clone();
            assert!(
                heap_probe::live()
                    <= 2 * (tree_bytes::<(String, usize), Temporal>(1024) + 1024 * 14)
            );
            drop(cloned);
            while map.pop_first().is_some() {
                assert!(
                    heap_probe::live()
                        <= tree_bytes::<(String, usize), Temporal>(map.len()) + map.len() * 14
                );
            }
            assert!(heap_probe::live() > 0, "the empty root remains allocated");
        });
    }

    #[test]
    fn widest_boolean_condition_yields_between_actual_child_lookups() {
        MAX_SLICE_WORK.with(|n| n.set(0));
        for operator in ["all", "any"] {
            let compare = serde_json::json!({"type":"compare","left":{"value":"1","unit":"usd_per_btc"},
                "comparison":"equal_to","right":{"value":"1","unit":"usd_per_btc"}});
            let text = serde_json::json!({"format":2,"name":"Wide Boolean",
                "inputs":[{"id":"usd","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}],
                "purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},
                    "when":{"type":operator,"conditions":vec![compare;510]}}]}).to_string();
            let make = || {
                let mut e = Evaluator::new(
                    ValidatedStrategy::parse(&text).unwrap(),
                    RunContext {
                        strategy_version: "fixed".into(),
                        run_id: "boolean".into(),
                        go_time: "controlled".into(),
                        series_id: "btc15m".into(),
                        markets: vec![MarketDescriptor {
                            series_id: "btc15m".into(),
                            market_id: "one".into(),
                            contract_id: "one".into(),
                            open_ms: 1,
                            close_ms: 900001,
                        }],
                        bindings: vec![],
                    },
                )
                .unwrap();
                e.begin(AdvanceBatch {
                    received_through_ms: 1,
                    target_ms: 1,
                    coverage: vec![],
                    current_admission: Admission::Allowed,
                    transactions: vec![NormalizedTransaction {
                        number: 1,
                        at_ms: 1,
                        changes: vec![EvidenceChange::MarketBoundary {
                            market_id: "one".into(),
                        }],
                    }],
                })
                .unwrap();
                e
            };
            let run = |e: &mut Evaluator| {
                let mut children = 0;
                loop {
                    BOOLEAN_CHILD_PROBES.with(|n| n.set(0));
                    let result = e.process_slice(SliceControl::Continue, None);
                    let inspected = BOOLEAN_CHILD_PROBES.with(|n| n.get());
                    assert!(inspected <= 1);
                    children += inspected;
                    match result {
                        SliceResult::Pending { .. } => (),
                        SliceResult::Complete(d) => {
                            assert_eq!(children, 510);
                            return d;
                        }
                        other => panic!("{other:?}"),
                    }
                }
            };
            let expected = run(&mut make());
            let mut yielded = make();
            yielded.yield_each_step = true;
            assert_eq!(run(&mut yielded), expected);
            let mut stopped = make();
            let published = stopped.snapshot().clone();
            loop {
                assert!(matches!(
                    stopped.process_slice(SliceControl::Continue, None),
                    SliceResult::Pending { .. }
                ));
                if stopped
                    .pending
                    .as_ref()
                    .unwrap()
                    .conditions
                    .as_ref()
                    .is_some_and(|c| c.child > 0)
                {
                    break;
                }
            }
            assert_eq!(
                stopped.process_slice(SliceControl::Stop, None),
                SliceResult::Stopped
            );
            assert_eq!(stopped.snapshot(), &published);
        }
        eprintln!(
            "widest_boolean_condition_yields_between_actual_child_lookups maximum observed work plus library bound={}",
            MAX_SLICE_WORK.with(|n| n.get())
        );
    }

    #[test]
    fn staged_allocation_refuses_before_allocating_over_existing_later_nodes() {
        let text = r#"{"format":2,"name":"Staged ownership","inputs":[
            {"id":"ask","source":"kalshi","instrument":"current_contract","field":"buyable_ask","unit":"contract_cents","side":"yes"},
            {"id":"usd","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}],
            "measurements":[{"id":"average","operation":"rolling_average","of":{"reference":"ask"},"lookback_seconds":3600},
                {"id":"range","operation":"rolling_range","of":{"reference":"usd"},"lookback_seconds":2700}],
            "purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare",
                "left":{"reference":"ask"},"comparison":"less_than","right":{"value":"40","unit":"contract_cents"}}}]}"#;
        let peak = heap_probe::measure(|| {
            let market = |id: &str| MarketDescriptor {
                series_id: "btc15m".into(),
                market_id: id.into(),
                contract_id: id.into(),
                open_ms: 1,
                close_ms: 900001,
            };
            let mut e = Evaluator::new(
                ValidatedStrategy::parse(text).unwrap(),
                RunContext {
                    strategy_version: "fixed".into(),
                    run_id: "allocation".into(),
                    go_time: "controlled".into(),
                    series_id: "btc15m".into(),
                    markets: vec![market("one")],
                    bindings: vec![],
                },
            )
            .unwrap();
            assert_eq!(e.allocated_bytes() - RESERVE, 230_400_064);
            e.begin(AdvanceBatch {
                target_ms: 0,
                received_through_ms: 0,
                coverage: vec![],
                current_admission: Admission::Allowed,
                transactions: vec![NormalizedTransaction {
                    number: 1,
                    at_ms: 0,
                    changes: vec![EvidenceChange::RegisterMarket {
                        descriptor: market("two"),
                        bindings: vec![],
                    }],
                }],
            })
            .unwrap();
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
            assert_eq!(e.context.markets.len(), 1);
            assert_eq!(e.allocated_bytes() - RESERVE, 230_400_064);
            e.begin(AdvanceBatch {
                target_ms: 0,
                received_through_ms: 0,
                coverage: vec![],
                transactions: vec![],
                current_admission: Admission::Allowed,
            })
            .unwrap();
        });
        eprintln!("rejected registration independently measured peak={peak}");
        assert!(peak <= 230_400_064 + RESERVE);
    }

    #[test]
    fn independent_heap_combines_history_catalog_conditions_checkpoints_and_ingress() {
        MAX_SLICE_WORK.with(|n| n.set(0));
        let compare = serde_json::json!({"type":"compare","left":{"reference":format!("usd_{:060}", 0)},
            "comparison":"greater_than","right":{"value":"0","unit":"usd_per_btc"}});
        let mut inputs = (0..31)
            .map(|n| {
                serde_json::json!({"id":format!("usd_{n:060}"),
            "source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"})
            })
            .collect::<Vec<_>>();
        inputs.push(
            serde_json::json!({"id":"ask","source":"kalshi","instrument":"current_contract",
            "field":"buyable_ask","unit":"contract_cents","side":"selected_side"}),
        );
        let mut measurements = (0..31)
            .map(|n| {
                serde_json::json!({"id":format!("difference_{n:053}"),
            "operation":"difference","left":{"reference":format!("usd_{n:060}")},
            "right":{"value":"0","unit":"usd_per_btc"}})
            })
            .collect::<Vec<_>>();
        measurements.push(
            serde_json::json!({"id":"hour_range","operation":"rolling_range",
            "of":{"reference":format!("usd_{:060}", 0)},"lookback_seconds":3600}),
        );
        let text = serde_json::json!({"format":2,"name":"Combined ownership", "inputs":inputs,
            "measurements":measurements,
            "checkpoints":(0..32).map(|n|serde_json::json!({"id":format!("checkpoint_{n:053}"),
                "when":{"type":"confirmed_updates","count":1,"count_input":format!("usd_{:060}", 0),"condition":compare}})).collect::<Vec<_>>(),
            "purchase_options":[{"id":"group","when":{"type":"all","conditions":[compare.clone(),compare.clone()]},"options":(0..120).map(|n|serde_json::json!({
                "id":format!("leaf_{n:059}"),"side":{"type":"unique_side"},"when":compare})).collect::<Vec<_>>()}]}).to_string();
        let peak = heap_probe::measure(|| {
            let strategy = ValidatedStrategy::parse(&text).unwrap();
            assert_eq!(condition_instances(&strategy.document), 1024);
            let markets = (0..128)
                .map(|n| MarketDescriptor {
                    series_id: "btc15m".into(),
                    market_id: format!("market_{n:0121}"),
                    contract_id: format!("contract_{n:0119}"),
                    open_ms: 1,
                    close_ms: 900001,
                })
                .collect::<Vec<_>>();
            let mut bindings = (0..31)
                .map(|n| BindingDescriptor {
                    binding_id: n,
                    input_id: format!("usd_{n:060}"),
                    scope: BindingScope::Underlying,
                })
                .collect::<Vec<_>>();
            for market in markets.iter().take(32) {
                for side in [Side::Yes, Side::No] {
                    bindings.push(BindingDescriptor {
                        binding_id: bindings.len() as u32,
                        input_id: "ask".into(),
                        scope: BindingScope::Contract {
                            market_id: market.market_id.clone(),
                            contract_id: market.contract_id.clone(),
                            side,
                        },
                    });
                }
            }
            let context = RunContext {
                strategy_version: "fixed".into(),
                run_id: "combined".into(),
                go_time: "controlled".into(),
                series_id: "btc15m".into(),
                markets,
                bindings,
            };
            let mut e = Evaluator::new(strategy, context).unwrap();
            assert_eq!(e.raw.len() + e.series.len(), 127);
            // Isolate ownership from the separately tested one-hour clock; no history/result is invented.
            e.ready_at = Some(0);
            let mut changes = vec![EvidenceChange::MarketBoundary {
                market_id: format!("market_{:0121}", 0),
            }];
            for id in 0..95 {
                changes.push(EvidenceChange::Observation {
                    binding_id: id,
                    observation_number: 1,
                    value: Decimal::parse("30").unwrap(),
                });
            }
            let mut batch = AdvanceBatch {
                received_through_ms: 1,
                target_ms: 1,
                coverage: vec![],
                current_admission: Admission::Allowed,
                transactions: vec![
                    NormalizedTransaction {
                        number: 1,
                        at_ms: 0,
                        changes: (0..95)
                            .map(|id| EvidenceChange::Validity {
                                binding_id: id,
                                valid_until_ms: 100,
                                reason: None,
                            })
                            .collect(),
                    },
                    NormalizedTransaction {
                        number: 2,
                        at_ms: 1,
                        changes,
                    },
                ],
            };
            // One nonessential binding carries the largest admitted owned diagnostic.
            let remaining = RESERVE - e.metadata_reservation - batch.ingress_bytes().unwrap();
            if let EvidenceChange::Validity { reason, .. } = &mut batch.transactions[0].changes[94]
            {
                *reason = Some("x".repeat(remaining));
            }
            let history = e.allocated_bytes() - RESERVE;
            assert_eq!(history, 230_400_064);
            e.begin(batch).unwrap();
            loop {
                match e.process_slice(SliceControl::Continue, None) {
                    SliceResult::Pending { .. } => (),
                    SliceResult::Complete(d) => {
                        assert_eq!(d.checkpoints.len(), 32);
                        assert_eq!(d.leaves.len(), 120);
                        assert!(d.candidate.is_none());
                        break;
                    }
                    other => panic!("{other:?}"),
                }
            }
            assert_eq!(e.truths.len(), 1024);
            assert_eq!(e.continuities.len(), 1024);
            assert_eq!(e.temporal.len(), 32);
            let mut discarded = 0;
            let mut maximum = 0;
            for _ in 0..2 {
                e.begin(AdvanceBatch {
                    received_through_ms: 1,
                    target_ms: 1,
                    coverage: vec![],
                    transactions: vec![],
                    current_admission: Admission::Allowed,
                })
                .unwrap();
                loop {
                    OUTPUT_DISCARD_PROBES.with(|n| n.set(0));
                    let result = e.process_slice(SliceControl::Continue, None);
                    let entries = OUTPUT_DISCARD_PROBES.with(|n| n.get());
                    discarded += entries;
                    maximum = maximum.max(entries);
                    assert!(entries <= 7);
                    assert_eq!(e.snapshot().checkpoints.len(), 32);
                    match result {
                        SliceResult::Pending { .. } => (),
                        SliceResult::Complete(d) => {
                            assert_eq!(d.checkpoints.len(), 32);
                            break;
                        }
                        other => panic!("{other:?}"),
                    }
                }
            }
            assert!(discarded >= 32 * 96);
            assert_eq!(maximum, 7);
        });
        eprintln!(
            "combined independently measured heap peak={peak}, history=230400064, non-history ceiling={RESERVE}"
        );
        assert!(peak <= 230_400_064 + RESERVE);
        eprintln!(
            "independent_heap_combines_history_catalog_conditions_checkpoints_and_ingress maximum observed work plus library bound={}",
            MAX_SLICE_WORK.with(|n| n.get())
        );
    }

    #[test]
    fn resolved_dependencies_share_diamonds_and_preserve_the_last_input_bit() {
        let inputs = (0..32)
            .map(|i| {
                serde_json::json!({
                    "id":format!("input_{i}"),"source":"kalshi","instrument":"current_contract",
                    "field":"buyable_ask","unit":"contract_cents",
                    "side":if i == 0 { "selected_side" } else { "yes" }
                })
            })
            .collect::<Vec<_>>();
        let measurements = (0..10)
            .rev()
            .map(|i| {
                let (left, right) = if i == 0 {
                    ("input_0".to_string(), "input_31".to_string())
                } else {
                    (format!("node_{}", i - 1), format!("node_{}", i - 1))
                };
                serde_json::json!({"id":format!("node_{i}"),"operation":"difference",
                "left":{"reference":left},"right":{"reference":right}})
            })
            .collect::<Vec<_>>();
        let text = serde_json::json!({"format":2,"name":"Shared dependencies", "inputs":inputs,
            "measurements":measurements,"purchase_options":[{"id":"buy",
                "side":{"type":"fixed","side":"yes"},"when":{"type":"compare",
                    "left":{"reference":"node_9"},"comparison":"equal_to",
                    "right":{"value":"0","unit":"contract_cents"}}}]})
        .to_string();
        let strategy = ValidatedStrategy::parse(&text).unwrap();
        for i in 0..10 {
            let reference = format!("node_{i}");
            let value = ValueSpec::Reference {
                reference: reference.clone(),
            };
            for input in 0..32 {
                assert_eq!(
                    strategy.value_depends(&value, &format!("input_{input}")),
                    input == 0 || input == 31
                );
            }
            assert!(strategy.numeric_selected(&reference));
            assert!(strategy.numeric_info[&reference].market);
        }
        assert!(!strategy.value_depends(
            &ValueSpec::Literal {
                value: "1".into(),
                unit: RuleUnit::ContractCents
            },
            "input_0"
        ));
    }
    #[test]
    fn numerical_continuity_uses_the_concrete_preopen_market() {
        let strategy = ValidatedStrategy::parse(r#"{"format":2,"name":"Concrete continuity",
            "inputs":[{"id":"ask","source":"kalshi","instrument":"current_contract","field":"buyable_ask","unit":"contract_cents","side":"yes"}],
            "measurements":[{"id":"adjusted","operation":"difference","left":{"reference":"ask"},"right":{"value":"1","unit":"contract_cents"}}],
            "purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"adjusted"},"comparison":"less_than","right":{"value":"40","unit":"contract_cents"}}}]}"#).unwrap();
        let mut e = Evaluator::new(
            strategy,
            RunContext {
                strategy_version: "immutable".into(),
                run_id: "continuity".into(),
                go_time: "controlled".into(),
                series_id: "btc15m".into(),
                markets: ["one", "two"]
                    .into_iter()
                    .map(|id| MarketDescriptor {
                        series_id: "btc15m".into(),
                        market_id: id.into(),
                        contract_id: id.into(),
                        open_ms: 1,
                        close_ms: 900001,
                    })
                    .collect(),
                bindings: ["one", "two"]
                    .into_iter()
                    .enumerate()
                    .map(|(index, id)| BindingDescriptor {
                        binding_id: index as u32,
                        input_id: "ask".into(),
                        scope: BindingScope::Contract {
                            market_id: id.into(),
                            contract_id: id.into(),
                            side: Side::Yes,
                        },
                    })
                    .collect(),
            },
        )
        .unwrap();
        e.market = Some("one".into());
        for raw in &mut e.raw {
            raw.value = Some(30 * SCALE);
            raw.valid_until = 100;
        }
        e.raw[1].covered_previous = true;
        let batch = AdvanceBatch {
            received_through_ms: 0,
            target_ms: 0,
            transactions: vec![],
            coverage: vec![],
            current_admission: Admission::Allowed,
        };
        for index in 0..e.series.len() {
            e.compute_node(index, &batch);
        }
        assert!(
            !e.series
                .iter()
                .find(|s| s.market.as_deref() == Some("one"))
                .unwrap()
                .continuous
        );
        assert!(
            e.series
                .iter()
                .find(|s| s.market.as_deref() == Some("two"))
                .unwrap()
                .continuous
        );
    }

    #[test]
    fn expanded_ancestor_conditions_obey_the_exact_state_cap() {
        let compare = serde_json::json!({"type":"compare","left":{"reference":"ask"},"comparison":"less_than","right":{"value":"40","unit":"contract_cents"}});
        let mut value = serde_json::json!({"format":2,"name":"Expanded conditions",
            "inputs":[{"id":"ask","source":"kalshi","instrument":"current_contract","field":"buyable_ask","unit":"contract_cents","side":"selected_side"}],
            "purchase_options":[{"id":"group","when":{"type":"all","conditions":[compare.clone(),compare.clone()]},
            "options":(0..128).map(|n|serde_json::json!({"id":format!("leaf_{n}"),"side":{"type":"unique_side"},"when":compare})).collect::<Vec<_>>()}]});
        let valid = ValidatedStrategy::parse(&value.to_string()).unwrap();
        assert_eq!(condition_instances(&valid.document), 1024);
        value["purchase_options"][0]["options"]
            .as_array_mut()
            .unwrap()
            .push(
                serde_json::json!({"id":"one_more","side":{"type":"unique_side"},"when":compare}),
            );
        assert_eq!(
            ValidatedStrategy::parse(&value.to_string()).unwrap_err()[0].code,
            "resource_limit"
        );
    }
    #[test]
    fn condition_and_output_continuations_preserve_results_stop_and_latest_objective() {
        let text = serde_json::json!({"format":2,"name":"Many eligible leaves",
            "inputs":[{"id":"ask","source":"kalshi","instrument":"current_contract",
                "field":"buyable_ask","unit":"contract_cents","side":"selected_side"}],
            "purchase_options":(0..200).map(|n|serde_json::json!({"id":format!("buy_{n}"),
                "side":{"type":"fixed","side":"yes"},"when":{"type":"compare",
                "left":{"reference":"ask"},"comparison":"less_than",
                "right":{"value":"40","unit":"contract_cents"}}})).collect::<Vec<_>>()})
        .to_string();
        let make = || {
            Evaluator::new(
                ValidatedStrategy::parse(&text).unwrap(),
                RunContext {
                    strategy_version: "immutable".into(),
                    run_id: "continuations".into(),
                    go_time: "controlled".into(),
                    series_id: "btc15m".into(),
                    markets: vec![MarketDescriptor {
                        series_id: "btc15m".into(),
                        market_id: "one".into(),
                        contract_id: "contract".into(),
                        open_ms: 1,
                        close_ms: 900001,
                    }],
                    bindings: [Side::Yes, Side::No]
                        .into_iter()
                        .enumerate()
                        .map(|(n, side)| BindingDescriptor {
                            binding_id: n as u32,
                            input_id: "ask".into(),
                            scope: BindingScope::Contract {
                                market_id: "one".into(),
                                contract_id: "contract".into(),
                                side,
                            },
                        })
                        .collect(),
                },
            )
            .unwrap()
        };
        let input = || AdvanceBatch {
            target_ms: 1,
            received_through_ms: 1,
            coverage: vec![],
            current_admission: Admission::Allowed,
            transactions: vec![
                NormalizedTransaction {
                    number: 1,
                    at_ms: 0,
                    changes: vec![EvidenceChange::Validity {
                        binding_id: 0,
                        valid_until_ms: 100,
                        reason: None,
                    }],
                },
                NormalizedTransaction {
                    number: 2,
                    at_ms: 1,
                    changes: vec![
                        EvidenceChange::MarketBoundary {
                            market_id: "one".into(),
                        },
                        EvidenceChange::Observation {
                            binding_id: 0,
                            observation_number: 1,
                            value: Decimal::parse("30").unwrap(),
                        },
                    ],
                },
            ],
        };
        let measured_peak = heap_probe::measure(|| {
            let mut evaluator = make();
            let mut batch = input();
            let remaining =
                RESERVE - evaluator.metadata_reservation - batch.ingress_bytes().unwrap();
            if let EvidenceChange::Validity { reason, .. } = &mut batch.transactions[0].changes[0] {
                *reason = Some("x".repeat(remaining));
            }
            evaluator.begin(batch).unwrap();
            loop {
                match evaluator.process_slice(SliceControl::Continue, None) {
                    SliceResult::Pending { .. } => (),
                    SliceResult::Complete(_) => break,
                    other => panic!("{other:?}"),
                }
            }
        });
        eprintln!("independent exact-boundary peak={measured_peak}");
        assert!(
            measured_peak <= RESERVE,
            "combined non-history peak {measured_peak} exceeds {RESERVE}"
        );
        let mut admission = make();
        let allowance = RESERVE - admission.metadata_reservation;
        let packet = |extra| {
            let mut batch = input();
            let overhead = batch.ingress_bytes().unwrap();
            if let EvidenceChange::Validity { reason, .. } = &mut batch.transactions[0].changes[0] {
                *reason = Some(String::with_capacity(allowance - overhead + extra));
            }
            batch
        };
        let before = admission.snapshot().clone();
        assert_eq!(
            admission.begin(packet(1)).unwrap_err().code,
            "resource_limit"
        );
        assert_eq!(admission.snapshot(), &before);
        let exact = packet(0);
        assert_eq!(
            exact.ingress_bytes().unwrap() + admission.metadata_reservation,
            RESERVE
        );
        admission.begin(exact).unwrap();
        assert_eq!(
            admission.process_slice(SliceControl::Stop, None),
            SliceResult::Stopped
        );
        eprintln!(
            "metadata reservation={}, exact combined ingress allowance={allowance}",
            admission.metadata_reservation
        );
        let objective = ObjectiveView {
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
        };
        let drain = |e: &mut Evaluator, o: &ObjectiveView| {
            for _ in 0..100_000 {
                match e.process_slice(SliceControl::Continue, Some(o.clone())) {
                    SliceResult::Pending { .. } => (),
                    SliceResult::Complete(d) => return d,
                    other => panic!("{other:?}"),
                }
            }
            panic!("continuation did not finish");
        };
        let mut ordinary = make();
        ordinary.begin(input()).unwrap();
        let expected = drain(&mut ordinary, &objective);
        assert_eq!(expected.candidate.as_ref().unwrap().option_id, "buy_0");
        assert_eq!(expected.leaves.len(), 200);
        let mut yielded = make();
        yielded.yield_each_step = true;
        yielded.begin(input()).unwrap();
        assert_eq!(drain(&mut yielded, &objective), expected);
        for stage in 0..6 {
            let mut e = make();
            let before = e.snapshot().clone();
            e.begin(input()).unwrap();
            loop {
                assert!(matches!(
                    e.process_slice(SliceControl::Continue, Some(objective.clone())),
                    SliceResult::Pending { .. }
                ));
                let p = e.pending.as_ref().unwrap();
                if if stage >= 2 {
                    p.cleaning && p.finished.is_some()
                } else if stage == 1 {
                    p.output.as_ref().is_some_and(|o| o.index > 0)
                } else {
                    p.conditions.as_ref().is_some_and(|c| c.step > 0)
                } {
                    break;
                }
            }
            assert_eq!(e.snapshot(), &before);
            if stage >= 4 {
                let mut blocked = objective.clone();
                blocked.safeguards.allowed = false;
                OUTPUT_DISCARD_PROBES.with(|n| n.set(0));
                while !e.pending.as_ref().unwrap().discarding_output {
                    assert!(matches!(
                        e.process_slice(SliceControl::Continue, Some(blocked.clone())),
                        SliceResult::Pending { .. }
                    ));
                }
                assert!(OUTPUT_DISCARD_PROBES.with(|n| n.get()) <= 16);
                if stage == 4 {
                    // Returning to the original objective must rebuild a partially discarded result.
                    assert_eq!(drain(&mut e, &objective), expected);
                } else {
                    assert_eq!(
                        e.process_slice(SliceControl::Stop, None),
                        SliceResult::Stopped
                    );
                    assert_eq!(e.snapshot(), &before);
                }
            } else if stage == 1 || stage == 2 {
                let mut blocked = objective.clone();
                blocked.safeguards.allowed = false;
                assert!(drain(&mut e, &blocked).candidate.is_none());
            } else {
                assert_eq!(
                    e.process_slice(SliceControl::Stop, Some(objective.clone())),
                    SliceResult::Stopped
                );
                assert_eq!(e.snapshot(), &before);
            }
        }
        for during_cleanup in [false, true] {
            for regress_during_disposal in [false, true] {
                for extra_yields in [false, true] {
                    let mut e = make();
                    e.yield_each_step = extra_yields;
                    e.begin(input()).unwrap();
                    loop {
                        assert!(matches!(
                            e.process_slice(SliceControl::Continue, Some(objective.clone())),
                            SliceResult::Pending { .. }
                        ));
                        let pending = e.pending.as_ref().unwrap();
                        if if during_cleanup {
                            pending.cleaning && pending.finished.is_some()
                        } else {
                            pending.output.as_ref().is_some_and(|o| o.index >= 100)
                        } {
                            break;
                        }
                    }
                    let mut filled = objective.clone();
                    filled.confirmed_quantity = 4;
                    filled.locked_side = Some(Side::Yes);
                    filled.attempt_state = AttemptState::Finished;
                    assert!(matches!(
                        e.process_slice(SliceControl::Continue, Some(filled.clone())),
                        SliceResult::Pending { .. }
                    ));
                    if regress_during_disposal {
                        // Observe the fill while discarding, then supply the stale pre-fill view.
                        assert!(matches!(
                            e.process_slice(SliceControl::Continue, Some(filled.clone())),
                            SliceResult::Pending { .. }
                        ));
                        assert!(e.pending.as_ref().unwrap().discarding_output);
                        let blocked = drain(&mut e, &objective);
                        assert_eq!(blocked.leaves.len(), 200);
                        assert!(blocked.candidate.is_none());
                    } else {
                        let reduced = drain(&mut e, &filled);
                        let candidate = reduced.candidate.unwrap();
                        assert_eq!(candidate.remaining_quantity, 6);
                        assert_eq!(candidate.side, Side::Yes);
                    }
                    let next = || AdvanceBatch {
                        target_ms: 1,
                        received_through_ms: 1,
                        coverage: vec![],
                        transactions: vec![],
                        current_admission: Admission::Allowed,
                    };
                    e.begin(next()).unwrap();
                    assert!(drain(&mut e, &objective).candidate.is_none());
                    let mut opposite = filled.clone();
                    opposite.locked_side = Some(Side::No);
                    e.begin(next()).unwrap();
                    assert!(drain(&mut e, &opposite).candidate.is_none());
                    e.begin(next()).unwrap();
                    let recovered = drain(&mut e, &filled);
                    assert_eq!(recovered.candidate.unwrap().remaining_quantity, 6);
                }
            }
        }
    }
    #[test]
    fn processing_maxima_compose_with_deep_priority_equal_times_and_extra_yields() {
        MAX_SLICE_WORK.with(|n| n.set(0));
        let gate = serde_json::json!({"type":"compare","left":{"reference":"ask"},
            "comparison":"less_than","right":{"value":"40","unit":"contract_cents"}});
        let compare = serde_json::json!({"type":"compare","left":{"reference":"measurement_63"},
            "comparison":"less_than","right":{"value":"0","unit":"percent"}});
        let leaves = (0..60)
            .map(|n| {
                serde_json::json!({"id":format!("leaf_{n}"),
            "side":{"type":"unique_side"},"when":compare})
            })
            .collect::<Vec<_>>();
        let mut option = serde_json::json!({"id":"group_6","when":gate,"options":leaves});
        for n in (0..6).rev() {
            option = serde_json::json!({"id":format!("group_{n}"),"when":gate,"options":[option]});
        }
        let mut inputs = (0..31)
            .map(|n| {
                serde_json::json!({"id":format!("usd_{n}"),
            "source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"})
            })
            .collect::<Vec<_>>();
        inputs.push(
            serde_json::json!({"id":"ask","source":"kalshi","instrument":"current_contract",
            "field":"buyable_ask","unit":"contract_cents","side":"selected_side"}),
        );
        let text = serde_json::json!({"format":2,"name":"Combined processing maxima","inputs":inputs,
            "measurements":(0..64).map(|n|serde_json::json!({"id":format!("measurement_{n}"),
                "operation":"percentage_difference","left":{"reference":"usd_0"},
                "reference":{"value":"999999999999.999999","unit":"usd_per_btc"}})).collect::<Vec<_>>(),
            "checkpoints":(0..32).map(|n|serde_json::json!({"id":format!("checkpoint_{n}"),
                "when":{"type":"confirmed_updates","count":1,"count_input":"usd_0","condition":compare}})).collect::<Vec<_>>(),
            "purchase_options":[option]}).to_string();
        let context = RunContext {
            strategy_version: "fixed".into(),
            run_id: "processing".into(),
            go_time: "controlled".into(),
            series_id: "btc15m".into(),
            markets: (0..128)
                .map(|n| MarketDescriptor {
                    series_id: "btc15m".into(),
                    market_id: format!("market_{n:0121}"),
                    contract_id: format!("contract_{n:0119}"),
                    open_ms: 1,
                    close_ms: 900001,
                })
                .collect(),
            bindings: vec![],
        };
        let mut context = context;
        context.bindings = (0..30)
            .map(|n| BindingDescriptor {
                binding_id: n,
                input_id: format!("usd_{n}"),
                scope: BindingScope::Underlying,
            })
            .collect();
        for m in context.markets.iter().take(17) {
            for side in [Side::Yes, Side::No] {
                context.bindings.push(BindingDescriptor {
                    binding_id: context.bindings.len() as u32,
                    input_id: "ask".into(),
                    scope: BindingScope::Contract {
                        market_id: m.market_id.clone(),
                        contract_id: m.contract_id.clone(),
                        side,
                    },
                });
            }
        }
        let strategy = ValidatedStrategy::parse(&text).unwrap();
        assert_eq!(condition_instances(&strategy.document), 1024);
        let make = || {
            let mut e = Evaluator::new(strategy.clone(), context.clone()).unwrap();
            assert_eq!(e.raw.len() + e.series.len(), 128);
            // This test isolates a maximal processing tick; startup clock behavior has public tests.
            e.ready_at = Some(0);
            e
        };
        let mut changes = vec![EvidenceChange::MarketBoundary {
            market_id: context.markets[0].market_id.clone(),
        }];
        for b in &context.bindings {
            changes.push(EvidenceChange::Observation {
                binding_id: b.binding_id,
                observation_number: 1,
                value: Decimal::parse(
                    if matches!(b.scope, BindingScope::Contract { side: Side::No, .. }) {
                        "70"
                    } else {
                        "30"
                    },
                )
                .unwrap(),
            });
        }
        let batch = AdvanceBatch {
            received_through_ms: 2,
            target_ms: 2,
            coverage: context
                .bindings
                .iter()
                .map(|b| CoverageFact {
                    binding_id: b.binding_id,
                    start_ms: 0,
                    end_ms: 2,
                })
                .collect(),
            transactions: vec![
                NormalizedTransaction {
                    number: 1,
                    at_ms: 0,
                    changes: context
                        .bindings
                        .iter()
                        .map(|b| EvidenceChange::Validity {
                            binding_id: b.binding_id,
                            valid_until_ms: 10,
                            reason: None,
                        })
                        .collect(),
                },
                NormalizedTransaction {
                    number: 2,
                    at_ms: 1,
                    changes,
                },
                NormalizedTransaction {
                    number: 3,
                    at_ms: 1,
                    changes: vec![EvidenceChange::Observation {
                        binding_id: 0,
                        observation_number: 2,
                        value: Decimal::parse("20").unwrap(),
                    }],
                },
                NormalizedTransaction {
                    number: 4,
                    at_ms: 1,
                    changes: vec![EvidenceChange::Observation {
                        binding_id: 0,
                        observation_number: 3,
                        value: Decimal::parse("30").unwrap(),
                    }],
                },
            ],
            current_admission: Admission::Allowed,
        };
        let objective = ObjectiveView {
            market_id: context.markets[0].market_id.clone(),
            requested_quantity: 10,
            confirmed_quantity: 0,
            locked_side: None,
            attempt_state: AttemptState::Finished,
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
        };
        let drain = |e: &mut Evaluator| loop {
            match e.process_slice(SliceControl::Continue, Some(objective.clone())) {
                SliceResult::Pending { .. } => (),
                SliceResult::Complete(d) => break d,
                other => panic!("{other:?}"),
            }
        };
        let mut ordinary = make();
        ordinary.begin(batch.clone()).unwrap();
        let expected = drain(&mut ordinary);
        assert_eq!(expected.checkpoints.len(), 32);
        assert_eq!(expected.leaves.len(), 60);
        assert_eq!(expected.candidate.as_ref().unwrap().remaining_quantity, 10);
        let mut extra = make();
        extra.yield_each_step = true;
        extra.begin(batch.clone()).unwrap();
        assert_eq!(drain(&mut extra), expected);
        let mut stopped = make();
        stopped.begin(batch).unwrap();
        assert!(matches!(
            stopped.process_slice(SliceControl::Continue, Some(objective)),
            SliceResult::Pending { .. }
        ));
        assert_eq!(
            stopped.process_slice(SliceControl::Stop, None),
            SliceResult::Stopped
        );
        eprintln!(
            "processing combined128numeric1024condition128market depth8 equal-time max={}",
            MAX_SLICE_WORK.with(|n| n.get())
        );
    }

    #[test]
    fn repeated_registration_resumes_in_validation_and_application() {
        MAX_SLICE_WORK.with(|n| n.set(0));
        let text = serde_json::json!({
            "format":2,"name":"Registration continuation",
            "inputs":[{"id":"ask","source":"kalshi","instrument":"current_contract",
                "field":"buyable_ask","unit":"contract_cents","side":"selected_side"}],
            "purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},
                "when":{"type":"compare","left":{"reference":"ask"},
                    "comparison":"less_than","right":{"value":"40","unit":"contract_cents"}}}]
        })
        .to_string();
        let markets = (0..64)
            .map(|n| MarketDescriptor {
                series_id: "btc15m".into(),
                market_id: format!("market_{n}"),
                contract_id: format!("contract_{n}"),
                open_ms: 1,
                close_ms: 900001,
            })
            .collect::<Vec<_>>();
        let bindings = markets
            .iter()
            .enumerate()
            .flat_map(|(n, m)| {
                [Side::Yes, Side::No]
                    .into_iter()
                    .enumerate()
                    .map(move |(i, side)| BindingDescriptor {
                        binding_id: (n * 2 + i) as u32,
                        input_id: "ask".into(),
                        scope: BindingScope::Contract {
                            market_id: m.market_id.clone(),
                            contract_id: m.contract_id.clone(),
                            side,
                        },
                    })
            })
            .collect::<Vec<_>>();
        let context = RunContext {
            strategy_version: "immutable".into(),
            run_id: "registration".into(),
            go_time: "controlled".into(),
            series_id: "btc15m".into(),
            markets: markets.clone(),
            bindings: bindings.clone(),
        };
        let make =
            || Evaluator::new(ValidatedStrategy::parse(&text).unwrap(), context.clone()).unwrap();
        let batch = |repetitions| AdvanceBatch {
            target_ms: 0,
            received_through_ms: 0,
            coverage: vec![],
            current_admission: Admission::Allowed,
            transactions: vec![NormalizedTransaction {
                number: 1,
                at_ms: 0,
                changes: (0..repetitions)
                    .map(|_| EvidenceChange::RegisterMarket {
                        descriptor: markets[0].clone(),
                        bindings: bindings.clone(),
                    })
                    .collect(),
            }],
        };
        let mut reference = make();
        reference.begin(batch(1)).unwrap();
        let compact = loop {
            match reference.process_slice(SliceControl::Continue, None) {
                SliceResult::Pending { .. } => (),
                SliceResult::Complete(d) => break d,
                other => panic!("{other:?}"),
            }
        };
        let mut e = make();
        let before = e.snapshot().clone();
        let input = batch(100);
        assert!(input.ingress_bytes().unwrap() < RESERVE);
        e.begin(input).unwrap();
        let mut maximum = 0;
        let mut total = 0;
        let mut phases = BTreeSet::new();
        let mut cleanup_total = 0;
        let mut cleanup_maximum = 0;
        loop {
            REGISTRATION_PROBES.with(|n| n.set(0));
            CLEANUP_PROBES.with(|n| n.set(0));
            let result = e.process_slice(SliceControl::Continue, None);
            let probes = REGISTRATION_PROBES.with(|n| n.get());
            let cleanup = CLEANUP_PROBES.with(|n| n.get());
            cleanup_total += cleanup;
            cleanup_maximum = cleanup_maximum.max(cleanup);
            assert!(cleanup <= 256);
            maximum = maximum.max(probes);
            total += probes;
            assert!(
                probes <= 4096,
                "registration scan entries in one slice: {probes}"
            );
            match result {
                SliceResult::Pending { phase, .. } => {
                    assert_eq!(e.snapshot(), &before);
                    if probes > 0 {
                        phases.insert(phase);
                    }
                }
                SliceResult::Complete(d) => {
                    assert_eq!(d, compact);
                    break;
                }
                other => panic!("{other:?}"),
            }
        }
        assert_eq!(phases, BTreeSet::from(["validating", "evaluating"]));
        assert!(total > 1_000_000);
        assert_eq!(cleanup_total, 12_901 + 64 + 128 + 128);
        eprintln!("cleanup entries total={cleanup_total}, maximum per slice={cleanup_maximum}");
        for phase in phases {
            let mut stopped = make();
            stopped.begin(batch(100)).unwrap();
            loop {
                if matches!(stopped.process_slice(SliceControl::Continue,None),
                    SliceResult::Pending {phase:p,..} if p==phase)
                {
                    break;
                }
            }
            assert_eq!(
                stopped.process_slice(SliceControl::Stop, None),
                SliceResult::Stopped
            );
            assert_eq!(stopped.snapshot(), &before);
        }
        eprintln!("registration entries total={total}, maximum per slice={maximum}");
        eprintln!(
            "repeated_registration_resumes_in_validation_and_application maximum observed work plus library bound={}",
            MAX_SLICE_WORK.with(|n| n.get())
        );
    }
    #[test]
    fn admitted_allocation_scans_resume_and_stop_before_publication() {
        let text = serde_json::json!({
            "format": 2, "name": "Allocation budget",
            "inputs": [{"id":"ask","source":"kalshi","instrument":"current_contract",
                "field":"buyable_ask","unit":"contract_cents","side":"selected_side"}],
            "measurements": (0..62).map(|n| serde_json::json!({
                "id":format!("node_{n}"),"operation":"difference",
                "left":{"reference":"ask"},"right":{"value":"1","unit":"contract_cents"}
            })).collect::<Vec<_>>(),
            "purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},
                "when":{"type":"compare","left":{"reference":"ask"},
                    "comparison":"less_than","right":{"value":"40","unit":"contract_cents"}}}]
        })
        .to_string();
        let make = || {
            Evaluator::new(
                ValidatedStrategy::parse(&text).unwrap(),
                RunContext {
                    strategy_version: "immutable".into(),
                    run_id: "allocation".into(),
                    go_time: "controlled".into(),
                    series_id: "btc15m".into(),
                    markets: vec![MarketDescriptor {
                        series_id: "btc15m".into(),
                        market_id: "one".into(),
                        contract_id: "contract_one".into(),
                        open_ms: 1,
                        close_ms: 900001,
                    }],
                    bindings: [Side::Yes, Side::No]
                        .into_iter()
                        .enumerate()
                        .map(|(n, side)| BindingDescriptor {
                            binding_id: n as u32 + 1,
                            input_id: "ask".into(),
                            scope: BindingScope::Contract {
                                market_id: "one".into(),
                                contract_id: "contract_one".into(),
                                side,
                            },
                        })
                        .collect(),
                },
            )
            .unwrap()
        };
        let batch = || AdvanceBatch {
            received_through_ms: 0,
            target_ms: 0,
            transactions: vec![],
            coverage: vec![],
            current_admission: Admission::Allowed,
        };
        let mut stopped = make();
        let before = stopped.snapshot().clone();
        stopped.begin(batch()).unwrap();
        ALLOCATION_PROBES.with(|n| n.set(0));
        assert!(matches!(
            stopped.process_slice(SliceControl::Continue, None),
            SliceResult::Pending {
                phase: "staging",
                ..
            }
        ));
        assert!(ALLOCATION_PROBES.with(|n| n.get()) > 0);
        assert_eq!(
            stopped.process_slice(SliceControl::Stop, None),
            SliceResult::Stopped
        );
        assert_eq!(stopped.snapshot(), &before);

        let mut e = make();
        assert_eq!(e.series.len(), 124);
        e.begin(batch()).unwrap();
        let mut total = 0;
        let mut maximum = 0;
        let compact = loop {
            ALLOCATION_PROBES.with(|n| n.set(0));
            let result = e.process_slice(SliceControl::Continue, None);
            let probes = ALLOCATION_PROBES.with(|n| n.get());
            maximum = maximum.max(probes);
            total += probes;
            assert!(
                probes <= 4096,
                "actual allocation entries inspected: {probes}"
            );
            match result {
                SliceResult::Pending { .. } => assert_eq!(e.snapshot(), &before),
                SliceResult::Complete(d) => {
                    assert!(d.candidate.is_none());
                    assert_eq!(e.series.len(), 124);
                    break d;
                }
                other => panic!("{other:?}"),
            }
        };
        assert!(total > 4096, "fixture must require multiple slices");
        eprintln!(
            "maximum_allocation_entry_probes_per_slice={maximum}, total_allocation_entry_probes={total}"
        );
        let mut e = make();
        let mut dense = batch();
        dense.received_through_ms = 1;
        dense.coverage = (0..4097)
            .map(|_| CoverageFact {
                binding_id: 2,
                start_ms: 0,
                end_ms: 1,
            })
            .collect();
        e.begin(dense).unwrap();
        let mut catalog_total = 0;
        let mut maximum_combined = 0;
        loop {
            CATALOG_PROBES.with(|n| n.set(0));
            ALLOCATION_PROBES.with(|n| n.set(0));
            COVERAGE_PROBES.with(|n| n.set(0));
            let result = e.process_slice(SliceControl::Continue, None);
            let catalog = CATALOG_PROBES.with(|n| n.get());
            catalog_total += catalog;
            let probes =
                catalog + ALLOCATION_PROBES.with(|n| n.get()) + COVERAGE_PROBES.with(|n| n.get());
            maximum_combined = maximum_combined.max(probes);
            assert!(probes <= 4096);
            match result {
                SliceResult::Pending { .. } => assert_eq!(e.snapshot(), &before),
                SliceResult::Complete(d) => {
                    assert_eq!(d, compact);
                    break;
                }
                other => panic!("{other:?}"),
            }
        }
        assert_eq!(catalog_total, 8194);
        let accepted = e.snapshot().clone();
        let mut rejected = batch();
        rejected.received_through_ms = 1;
        rejected.coverage = (0..4097)
            .map(|_| CoverageFact {
                binding_id: 2,
                start_ms: 0,
                end_ms: 1,
            })
            .collect();
        rejected.coverage.push(CoverageFact {
            binding_id: 999,
            start_ms: 0,
            end_ms: 1,
        });
        e.begin(rejected).unwrap();
        loop {
            match e.process_slice(SliceControl::Continue, None) {
                SliceResult::Pending { .. } => assert_eq!(e.snapshot(), &accepted),
                SliceResult::Rejected(error) => {
                    assert_eq!(error.code, "invalid_coverage");
                    break;
                }
                other => panic!("{other:?}"),
            }
        }
        assert_eq!(e.snapshot(), &accepted);
        e.begin(batch()).unwrap();
        loop {
            match e.process_slice(SliceControl::Continue, None) {
                SliceResult::Pending { .. } => (),
                SliceResult::Complete(d) => {
                    assert_eq!(d.progress, accepted.progress);
                    assert_eq!(d.checkpoints, accepted.checkpoints);
                    assert_eq!(d.leaves, accepted.leaves);
                    assert!(d.candidate.is_none());
                    break;
                }
                other => panic!("{other:?}"),
            }
        }
        eprintln!(
            "catalog_entry_probes={catalog_total}, maximum_combined_scan_probes_per_slice={maximum_combined}"
        );
    }
    #[test]
    fn admitted_coverage_scan_must_fit_the_slice_budget() {
        let text = r#"{"format":2,"name":"Coverage budget","inputs":[{"id":"price","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}],"purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"price"},"comparison":"greater_than","right":{"value":"1","unit":"usd_per_btc"}}}]}"#;
        let mut e = Evaluator::new(
            ValidatedStrategy::parse(text).unwrap(),
            RunContext {
                strategy_version: "immutable".into(),
                run_id: "budget".into(),
                go_time: "controlled".into(),
                series_id: "btc15m".into(),
                markets: vec![],
                bindings: vec![BindingDescriptor {
                    binding_id: 1,
                    input_id: "price".into(),
                    scope: BindingScope::Underlying,
                }],
            },
        )
        .unwrap();
        let mut coverage = vec![
            CoverageFact {
                binding_id: 1,
                start_ms: 1,
                end_ms: 2
            };
            4096
        ];
        coverage.push(CoverageFact {
            binding_id: 1,
            start_ms: 0,
            end_ms: 2,
        });
        let b = AdvanceBatch {
            received_through_ms: 2,
            target_ms: 1,
            transactions: vec![
                NormalizedTransaction {
                    number: 1,
                    at_ms: 0,
                    changes: vec![EvidenceChange::Validity {
                        binding_id: 1,
                        valid_until_ms: 3,
                        reason: None,
                    }],
                },
                NormalizedTransaction {
                    number: 2,
                    at_ms: 0,
                    changes: vec![EvidenceChange::Observation {
                        binding_id: 1,
                        observation_number: 1,
                        value: Decimal::parse("100").unwrap(),
                    }],
                },
            ],
            coverage,
            current_admission: Admission::Allowed,
        };
        assert!(b.ingress_bytes().unwrap() < RESERVE);
        let mut reference = Evaluator::new(e.strategy.clone(), e.context.clone()).unwrap();
        let mut stopped = Evaluator::new(e.strategy.clone(), e.context.clone()).unwrap();
        stopped.begin(b.clone()).unwrap();
        loop {
            COVERAGE_PROBES.with(|n| n.set(0));
            assert!(matches!(
                stopped.process_slice(SliceControl::Continue, None),
                SliceResult::Pending { .. }
            ));
            if COVERAGE_PROBES.with(|n| n.get()) != 0 {
                break;
            }
        }
        let published = stopped.snapshot().clone();
        assert_eq!(
            stopped.process_slice(SliceControl::Stop, None),
            SliceResult::Stopped
        );
        assert_eq!(stopped.snapshot(), &published);
        let mut compact = b.clone();
        compact.coverage = vec![compact.coverage.pop().unwrap()];
        reference.begin(compact).unwrap();
        let expected = loop {
            match reference.process_slice(SliceControl::Continue, None) {
                SliceResult::Pending { .. } => (),
                SliceResult::Complete(d) => break d,
                other => panic!("{other:?}"),
            }
        };
        e.begin(b).unwrap();
        let mut maximum = 0;
        loop {
            COVERAGE_PROBES.with(|n| n.set(0));
            let result = e.process_slice(SliceControl::Continue, None);
            let probes = COVERAGE_PROBES.with(|n| n.get());
            maximum = maximum.max(probes);
            assert!(
                probes <= 4096,
                "one slice inspected {probes} coverage/change entries; other primitive work is additional"
            );
            match result {
                SliceResult::Pending { .. } => assert!(e.snapshot().candidate.is_none()),
                SliceResult::Complete(d) => {
                    assert_eq!(d, expected);
                    println!("maximum_coverage_entry_probes_per_slice={maximum}");
                    break;
                }
                other => panic!("{other:?}"),
            }
        }
    }

    #[test]
    fn reserved_history_writes_only_arriving_cells_and_matches_dense_oracle() {
        for operation in ["rolling_average", "change", "percentage_change"] {
            let c: Calculation = serde_json::from_value(serde_json::json!({
                "operation": operation, "of": {"reference":"price"}, "lookback_seconds":1
            }))
            .unwrap();
            let mut h = History::new(&c).unwrap();
            let capacity = h.ring.capacity();
            assert!(capacity >= 1000);
            assert!(h.ring.is_empty());
            let mut cells = Vec::new();
            for tick in 0..3000 {
                let value = (i128::from(tick % 7) + 1) * SCALE;
                let before = h.ring.len();
                let mut steps = 0;
                loop {
                    steps += 1;
                    if h.push_step(&c, Some(value), tick).unwrap() {
                        break;
                    }
                }
                assert_eq!(steps, 5);
                assert_eq!(h.ring.capacity(), capacity);
                assert_eq!(h.ring.len(), (before + 1).min(h.ring_len));
                cells.push(value);
                let at = tick + 1;
                let current = 10 * SCALE;
                let expected = if at < 1000 {
                    None
                } else {
                    let window = &cells[cells.len() - 1000..];
                    Some(match operation {
                        "rolling_average" => divide_even(window.iter().sum(), 1000).unwrap(),
                        "change" => current - window[0],
                        _ => divide_even((current - window[0]) * 100 * SCALE, window[0]).unwrap(),
                    })
                };
                assert_eq!(
                    h.result(&c, Some(current), at),
                    expected,
                    "{operation} at {at}"
                );
            }
        }
        let c: Calculation = serde_json::from_value(serde_json::json!({
            "operation":"rolling_average", "of":{"reference":"price"}, "lookback_seconds":3600
        }))
        .unwrap();
        let h = History::new(&c).unwrap();
        assert_eq!(h.ring_len, 3_600_000);
        assert!(h.ring.capacity() >= 3_600_000);
        assert!(h.ring.is_empty());
    }

    #[test]
    fn extrema_mass_removal_resumes_one_entry_at_a_time_and_matches_dense_windows() {
        for operation in ["rolling_minimum", "rolling_maximum", "rolling_range"] {
            let calculation: Calculation = serde_json::from_value(serde_json::json!({
                "operation":operation,"of":{"reference":"price"},"lookback_seconds":1
            }))
            .unwrap();
            let mut history = History::new(&calculation).unwrap();
            let capacities = (history.minimum.capacity(), history.maximum.capacity());
            let mut dense = Vec::new();
            let mut longest_insertion = 0;
            for tick in 0..3000 {
                let value = if tick == 1000 {
                    -1
                } else {
                    i128::from(tick + 1)
                };
                let value = if operation == "rolling_maximum" {
                    -value
                } else {
                    value
                };
                dense.push(value);
                let mut steps = 0;
                loop {
                    let before = history.minimum.len() + history.maximum.len();
                    let done = history.push_step(&calculation, Some(value), tick).unwrap();
                    let after = history.minimum.len() + history.maximum.len();
                    assert!(before.abs_diff(after) <= 1);
                    steps += 1;
                    if done {
                        break;
                    }
                }
                longest_insertion = longest_insertion.max(steps);
                assert_eq!(
                    (history.minimum.capacity(), history.maximum.capacity()),
                    capacities
                );
                let expected = if tick < 999 {
                    None
                } else {
                    let window = &dense[dense.len() - 1000..];
                    let minimum = *window.iter().min().unwrap();
                    let maximum = *window.iter().max().unwrap();
                    Some(match operation {
                        "rolling_minimum" => minimum,
                        "rolling_maximum" => maximum,
                        _ => maximum - minimum,
                    })
                };
                assert_eq!(history.result(&calculation, None, tick + 1), expected);
            }
            assert!(
                longest_insertion >= 1005,
                "{operation}: {longest_insertion}"
            );
        }
    }

    #[test]
    fn terminal_cleanup_drains_catalog_and_coverage_before_returning() {
        MAX_SLICE_WORK.with(|n| n.set(0));
        let text = r#"{"format":2,"name":"Terminal cleanup","inputs":[{"id":"price","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}],"purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"price"},"comparison":"greater_than","right":{"value":"1","unit":"usd_per_btc"}}}]}"#;
        let make = || {
            Evaluator::new(
                ValidatedStrategy::parse(text).unwrap(),
                RunContext {
                    strategy_version: "fixed".into(),
                    run_id: "cleanup".into(),
                    go_time: "controlled".into(),
                    series_id: "btc15m".into(),
                    markets: (0..128)
                        .map(|n| MarketDescriptor {
                            series_id: "btc15m".into(),
                            market_id: format!("{n:0128}"),
                            contract_id: format!("{n:0128}"),
                            open_ms: 1,
                            close_ms: 900001,
                        })
                        .collect(),
                    bindings: vec![BindingDescriptor {
                        binding_id: 1,
                        input_id: "price".into(),
                        scope: BindingScope::Underlying,
                    }],
                },
            )
            .unwrap()
        };
        let batch = |invalid| {
            let mut coverage = vec![
                CoverageFact {
                    binding_id: 1,
                    start_ms: 0,
                    end_ms: 1
                };
                4097
            ];
            if invalid {
                coverage.last_mut().unwrap().binding_id = 2;
            }
            AdvanceBatch {
                received_through_ms: 1,
                target_ms: 0,
                transactions: vec![],
                coverage,
                current_admission: Admission::Allowed,
            }
        };
        for invalid in [false, true] {
            let mut results = vec![];
            for extra_yields in [false, true] {
                let mut e = make();
                e.yield_each_step = extra_yields;
                let before = e.snapshot().clone();
                e.begin(batch(invalid)).unwrap();
                let mut cleanup_total = 0;
                let mut cleanup_slices = 0;
                let result = loop {
                    let before_entries = CLEANUP_PROBES.with(|n| n.get());
                    let result = e.process_slice(SliceControl::Continue, None);
                    let entries = CLEANUP_PROBES.with(|n| n.get()) - before_entries;
                    assert!(entries <= if extra_yields { 1 } else { 256 });
                    cleanup_total += entries;
                    cleanup_slices += usize::from(entries > 0);
                    if matches!(result, SliceResult::Pending { .. }) {
                        assert_eq!(e.snapshot(), &before);
                    } else {
                        break result;
                    }
                };
                assert_eq!(cleanup_total, 4097 + 128 + 1 + 1);
                assert!(cleanup_slices > 1);
                assert!(e.pending.is_none());
                if invalid {
                    assert_eq!(result, SliceResult::Rejected(evidence("invalid_coverage")));
                    assert_eq!(e.snapshot(), &before);
                    e.begin(batch(false)).unwrap();
                    loop {
                        match e.process_slice(SliceControl::Continue, None) {
                            SliceResult::Pending { .. } => (),
                            SliceResult::Complete(_) => break,
                            other => panic!("{other:?}"),
                        }
                    }
                }
                results.push(result);
            }
            assert_eq!(results[0], results[1]);
            let mut stopped = make();
            let before = stopped.snapshot().clone();
            stopped.begin(batch(invalid)).unwrap();
            loop {
                let result = stopped.process_slice(SliceControl::Continue, None);
                assert!(matches!(result, SliceResult::Pending { .. }));
                if stopped
                    .pending
                    .as_ref()
                    .is_some_and(|p| p.cleaning && p.batch.coverage.len() < 4097)
                {
                    break;
                }
            }
            assert_eq!(
                stopped.process_slice(SliceControl::Stop, None),
                SliceResult::Stopped
            );
            assert_eq!(stopped.snapshot(), &before);
        }
        eprintln!(
            "terminal_cleanup_drains_catalog_and_coverage_before_returning maximum observed work plus library bound={}",
            MAX_SLICE_WORK.with(|n| n.get())
        );
    }

    #[test]
    fn unexpected_post_mutation_failure_is_terminal_and_keeps_published_snapshot() {
        let text = r#"{"format":2,"name":"Fault test","inputs":[{"id":"underlying","source":"coinbase","instrument":"btc_usd","field":"last_trade","unit":"usd_per_btc"}],"purchase_options":[{"id":"buy","side":{"type":"fixed","side":"yes"},"when":{"type":"compare","left":{"reference":"underlying"},"comparison":"greater_than","right":{"value":"1","unit":"usd_per_btc"}}}]}"#;
        let strategy = ValidatedStrategy::parse(text).unwrap();
        let context = RunContext {
            strategy_version: "immutable".into(),
            run_id: "fault".into(),
            go_time: "controlled".into(),
            series_id: "btc15m".into(),
            markets: vec![],
            bindings: vec![BindingDescriptor {
                binding_id: 1,
                input_id: "underlying".into(),
                scope: BindingScope::Underlying,
            }],
        };
        let mut e = Evaluator::new(strategy, context).unwrap();
        let old = e.snapshot().clone();
        e.begin(AdvanceBatch {
            received_through_ms: 1000,
            target_ms: 1000,
            transactions: vec![],
            coverage: vec![],
            current_admission: Admission::Allowed,
        })
        .unwrap();
        e.fail_after_mutation = true;
        let failure = SliceResult::Failed {
            code: "internal_evaluation_failure",
        };
        loop {
            let result = e.process_slice(SliceControl::Continue, None);
            if matches!(result, SliceResult::Pending { .. }) {
                continue;
            }
            assert_eq!(result, failure);
            break;
        }
        assert_eq!(e.snapshot(), &old);
        assert_eq!(e.strategy.json(), text);
        assert_eq!(e.process_slice(SliceControl::Continue, None), failure);
        assert_eq!(
            e.begin(AdvanceBatch {
                received_through_ms: 1001,
                target_ms: 1001,
                transactions: vec![],
                coverage: vec![],
                current_admission: Admission::Allowed
            })
            .unwrap_err()
            .code,
            "internal_evaluation_failure"
        );
    }
}
