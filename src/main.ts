import { invoke } from '@tauri-apps/api/core';
import type { Action, Snapshot, StrategyRequest, StrategyResponse, StrategyDetail } from './contract';
import { presentation, strategyFailure } from './view.mjs';
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
let refreshSequence = 0;

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
  const sequence = ++refreshSequence;
  try {
    const snapshot = await invoke<Snapshot>('snapshot');
    if (sequence === refreshSequence) render(snapshot);
  } catch (failure) {
    if (sequence !== refreshSequence) return;
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

for (const operation of ['save', 'replace', 'remove', 'check']) {
  element(`credential-${operation}`).addEventListener('click', () => {
    const input = element<HTMLInputElement>('credential-value');
    const value = input.value;
    input.value = '';
    const buttons = ['save', 'replace', 'remove', 'check'].map(o => element<HTMLButtonElement>(`credential-${o}`));
    buttons.forEach(b => { b.disabled = true; });
    element('credential-status').textContent = 'Checking local credential action…';
    void invoke<string>('credential', { operation, ...(['save', 'replace'].includes(operation) ? { value } : {}) })
      .then(status => { element('credential-status').textContent = status; })
      .catch(failure => {
        // Only native finite messages are shown; unexpected framework errors stay private.
        element('credential-status').textContent = typeof failure === 'string' && /^(Unavailable|Locked|Verification unavailable|Already saved|Missing|Canceled|Authentication failed|Denied|Invalid input|Invalid request|Invalid credential|Quitting|Credential access unavailable|Mismatch) —?/.test(failure)
          ? failure : 'Credential action failed — reopen the isolated verification setup and retry.';
      })
      .finally(() => { buttons.forEach(b => { b.disabled = false; }); });
  });
}

const editor = element<HTMLTextAreaElement>('strategy-json');
const previewButton = element<HTMLButtonElement>('strategy-preview');
const saveButton = element<HTMLButtonElement>('strategy-save');
const moreButton = element<HTMLButtonElement>('strategy-more');
const strategyStatus = element('strategy-status');
let editorGeneration = 0;
let reviewedText: string | null = null;
let previewPending = false;
let savePending = false;
let listGeneration = 0;
let detailGeneration = 0;
let nextCursor: string | null = null;
let inspected: StrategyDetail | null = null;
function strategyControls() {
  previewButton.disabled = previewPending || savePending;
  saveButton.disabled = previewPending || savePending || reviewedText === null || reviewedText !== editor.value;
}
function changedText() {
  editorGeneration++;
  reviewedText = null;
  element('strategy-explanation').textContent = '';
  strategyStatus.textContent = 'Text changed. Preview before saving.';
  strategyControls();
}
editor.addEventListener('input', changedText);
element('strategy-file').addEventListener('change', () => {
  changedText();
  const generation = editorGeneration;
  const file = element<HTMLInputElement>('strategy-file').files?.[0];
  if (!file) return;
  if (file.size > 256 * 1024) { strategyStatus.textContent = 'Strategy exceeds 256 KiB.'; return; }
  void file.text().then(text => {
    if (generation !== editorGeneration) return;
    if (new TextEncoder().encode(text).length > 256 * 1024) { strategyStatus.textContent = 'Strategy exceeds 256 KiB.'; return; }
    editor.value = text; changedText();
  }).catch(() => { if (generation === editorGeneration) strategyStatus.textContent = 'Could not read file. Choose a local JSON file again.'; });
});
function strategyRequest(request: StrategyRequest) { return invoke<StrategyResponse>('strategies', request); }
function ruleErrors(result: Extract<StrategyResponse, { kind: 'rule_errors' }>) {
  return result.errors.map(error => `${error.path}: ${error.message} (${error.code})\nAccepted: ${error.accepted_form}`).join('\n\n') + (result.truncated ? '\nAdditional errors omitted.' : '');
}
previewButton.addEventListener('click', () => {
  if (previewPending || savePending) return;
  const generation = editorGeneration;
  const json = editor.value;
  if (new TextEncoder().encode(json).length > 256 * 1024) { strategyStatus.textContent = 'Strategy exceeds 256 KiB.'; strategyControls(); return; }
  previewPending = true; strategyControls();
  strategyStatus.textContent = 'Checking the current text…';
  void strategyRequest({ operation: 'preview', json }).then(result => {
    if (generation !== editorGeneration || json !== editor.value) return;
    if (result.kind === 'rule_errors') { reviewedText = null; element('strategy-explanation').textContent = ''; strategyStatus.textContent = ruleErrors(result); return; }
    if (result.kind !== 'preview') throw new Error('Unexpected response');
    reviewedText = json;
    element('strategy-explanation').textContent = result.explanation;
    strategyStatus.textContent = `${result.notice}\n${result.meaning_id} · ${result.requirements.support_profile}\nPlanned memory: ${result.requirements.planned_bytes} bytes; shared underlying: ${result.requirements.shared_underlying_bytes}; per market: ${result.requirements.per_market_bytes}.\nRequired history: ${Object.entries(result.requirements.startup_ms).map(([id, ms]) => `${id}: ${ms / 1000}s`).join(', ') || 'none'}.`;
  }).catch(failure => { if (generation === editorGeneration) strategyStatus.textContent = strategyFailure(failure); })
    .finally(() => { previewPending = false; strategyControls(); });
});
saveButton.addEventListener('click', () => {
  if (savePending || reviewedText === null || reviewedText !== editor.value) return;
  const json = editor.value;
  const reviewed_json = reviewedText;
  savePending = true; strategyControls();
  strategyStatus.textContent = 'Saving reviewed text…';
  void strategyRequest({ operation: 'save', json, reviewed_json }).then(result => {
    if (result.kind === 'rule_errors') { reviewedText = null; element('strategy-explanation').textContent = ''; strategyStatus.textContent = ruleErrors(result); return; }
    if (result.kind !== 'saved') throw new Error('Unexpected response');
    reviewedText = null;
    strategyStatus.textContent = `Saved immutable version ${result.version.id} · ${result.version.recorded_at}.`;
    void loadStrategies();
  }).catch(failure => { strategyStatus.textContent = strategyFailure(failure, true); })
    .finally(() => { savePending = false; strategyControls(); });
});
async function loadStrategies(before_id?: string) {
  const generation = ++listGeneration;
  element('strategy-list-status').textContent = 'Reading saved versions…';
  moreButton.disabled = true;
  try {
    const result = await strategyRequest({ operation: 'list', ...(before_id ? { before_id } : {}) });
    if (generation !== listGeneration) return;
    if (result.kind !== 'versions') throw new Error('Unexpected response');
    const items = result.versions.map(version => {
      const item = document.createElement('li');
      const button = document.createElement('button');
      button.textContent = `Inspect version ${version.id} · ${version.display_name ?? 'Unknown name'} · ${version.status} · ${version.recorded_at}`;
      button.addEventListener('click', () => void inspectStrategy(version.id));
      item.append(button); return item;
    });
    if (before_id) element('strategy-versions').append(...items);
    else element('strategy-versions').replaceChildren(...items);
    nextCursor = result.next_cursor;
    element('strategy-list-status').textContent = result.versions.length ? 'Saved versions. Select one to inspect.' : 'No saved versions.';
  } catch (failure) {
    if (generation === listGeneration) element('strategy-list-status').textContent = `Version refresh failed. ${strategyFailure(failure)} Any confirmed save above remains saved.`;
  } finally { if (generation === listGeneration) moreButton.disabled = nextCursor === null; }
}
async function inspectStrategy(version_id: string) {
  const generation = ++detailGeneration;
  inspected = null;
  element<HTMLButtonElement>('strategy-copy').disabled = true;
  element('strategy-detail').textContent = 'Reading version…';
  try {
    const result = await strategyRequest({ operation: 'detail', version_id });
    if (generation !== detailGeneration) return;
    if (result.kind !== 'detail') throw new Error('Unexpected response');
    inspected = result.version;
    element('strategy-detail').textContent = `Version ${inspected.summary.id} · ${inspected.summary.status}\n${inspected.notice}\n${inspected.meaning_id ?? 'Historical meaning not recorded'}\n\n${inspected.explanation ?? 'No stored explanation.'}\n\n${inspected.json ?? 'Full content unavailable for display.'}`;
    element<HTMLButtonElement>('strategy-copy').disabled = inspected.json === null;
  } catch (failure) { if (generation === detailGeneration) element('strategy-detail').textContent = strategyFailure(failure); }
}
element('strategy-copy').addEventListener('click', () => {
  if (inspected?.json === null || !inspected) return;
  editor.value = inspected.json; changedText(); editor.focus();
});
element('strategy-reload').addEventListener('click', () => void loadStrategies());
moreButton.addEventListener('click', () => { if (nextCursor) void loadStrategies(nextCursor); });
void loadStrategies();
