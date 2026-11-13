import { invoke } from '@tauri-apps/api/core';
import type { Action, Snapshot } from './contract';
import { presentation } from './view.mjs';
import './style.css';

function element<T extends HTMLElement>(id: string): T {
  const value = document.getElementById(id);
  if (!value) throw new Error(`Missing interface element: ${id}`);
  return value as T;
}
const start = element<HTMLButtonElement>('start');
const pause = element<HTMLButtonElement>('pause');
const advance = element<HTMLButtonElement>('advance');
const error = element('error');
let requestError = '';

function render(snapshot: Snapshot) {
  const view = presentation(snapshot);
  element('status').textContent = view.status;
  element('recording').textContent = view.recording;
  element('clock').textContent = `Saved test clock: ${snapshot.history[0].tick} seconds`;
  start.disabled = view.startDisabled;
  pause.disabled = view.pauseDisabled;
  advance.disabled = view.advanceDisabled;
  error.textContent = snapshot.recording_error ?? requestError;
  const items = snapshot.history.map(record => {
    const item = document.createElement('li');
    item.textContent = `${record.reason} · ${record.status} · clock ${record.tick}s`;
    const time = document.createElement('time');
    time.dateTime = record.recorded_at;
    time.textContent = record.recorded_at;
    item.append(time);
    return item;
  });
  element('history').replaceChildren(...items);
  const settings = snapshot.writer_settings;
  element('settings').textContent = `SQLite ${settings.sqlite_version} · journal ${settings.journal_mode} · synchronous ${settings.synchronous} (FULL) · fullfsync ${settings.fullfsync} · checkpoint_fullfsync ${settings.checkpoint_fullfsync}. Verified on the writer connection.`;
}

async function refresh() {
  try { render(await invoke<Snapshot>('snapshot')); }
  catch (failure) {
    start.disabled = advance.disabled = true;
    pause.disabled = false;
    element('status').textContent = 'Session status unavailable';
    element('recording').textContent = 'Cannot confirm session state. Try Pause now, or quit to stop the session.';
    error.textContent = String(failure);
  }
}
async function control(action: Action) {
  start.disabled = advance.disabled = true;
  requestError = '';
  try { await invoke('control', { action }); }
  catch (failure) { requestError = String(failure); }
  await refresh();
}
start.addEventListener('click', () => void control('start'));
pause.addEventListener('click', () => void control('pause'));
advance.addEventListener('click', () => void control('advance'));
async function poll() { await refresh(); window.setTimeout(() => void poll(), 200); }
void poll();
