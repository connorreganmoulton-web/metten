import type { Snapshot } from './contract';
export function presentation(snapshot: Snapshot): { status: string; recording: string; startDisabled: boolean; advanceDisabled: boolean; pauseDisabled: boolean };
