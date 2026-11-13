export type Action = 'start' | 'pause' | 'advance';
export type Record = { id: number; status: 'stopped' | 'running' | 'paused'; tick: number; reason: string; recorded_at: string };
export type Snapshot = {
  running: boolean; pending: number; recording_error: string | null; history: Record[];
  writer_settings: { journal_mode: string; synchronous: number; fullfsync: number; checkpoint_fullfsync: number; sqlite_version: string };
};
