export type Action = 'start' | 'pause' | 'advance';
export type Record = { id: number; status: 'stopped' | 'running' | 'paused'; tick: number; reason: string; recorded_at: string };
export type Snapshot = {
  running: boolean; pending: number; recording_error: string | null; history: Record[];
  writer_settings: { journal_mode: string; synchronous: number; fullfsync: number; checkpoint_fullfsync: number; sqlite_version: string };
};
export type StrategyRequest =
  | { operation: 'preview'; json: string }
  | { operation: 'save'; json: string; reviewed_json: string }
  | { operation: 'list'; before_id?: string }
  | { operation: 'detail'; version_id: string };
export type StrategySummary = { id: string; recorded_at: string; display_name: string | null; format_revision: number | null; status: 'supported' | 'legacy_read_only' | 'unsupported' | 'invalid_record' };
export type StrategyDetail = { summary: StrategySummary; json: string | null; explanation: string | null; meaning_id: string | null; notice: string };
export type RuleError = { path: string; code: string; message: string; accepted_form: string };
export type Requirements = { support_profile: string; planned_bytes: number; startup_ms: { [id: string]: number }; per_node_bytes: { [id: string]: number }; shared_underlying_bytes: number; per_market_bytes: number; implicit_buyable_asks: string[] };
export type StrategyResponse =
  | { kind: 'preview'; format_revision: number; meaning_id: string; explanation: string; requirements: Requirements; notice: string }
  | { kind: 'rule_errors'; errors: RuleError[]; truncated: boolean }
  | { kind: 'saved'; version: StrategySummary }
  | { kind: 'versions'; versions: StrategySummary[]; next_cursor: string | null }
  | { kind: 'detail'; version: StrategyDetail };
