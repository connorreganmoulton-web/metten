import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { stripTypeScriptTypes } from 'node:module';
import { runInNewContext } from 'node:vm';
import { setImmediate } from 'node:timers/promises';
import { presentation, strategyFailure } from '../src/view.mjs';

async function harness() {
  const elements = new Map();
  function element(id) {
    if (!elements.has(id)) elements.set(id, {
      value: '', textContent: '', disabled: false, handlers: {}, children: [],
      addEventListener(event, handler) { this.handlers[event] = handler; },
      append(...items) { this.children.push(...items); },
      replaceChildren(...items) { this.children = items; }, focus() { this.focused = true; },
    });
    return elements.get(id);
  }
  const calls = [];
  const source = stripTypeScriptTypes(await readFile(new URL('../src/main.ts', import.meta.url), 'utf8')).replace(/^import .*;\n/gm, '');
  runInNewContext(source, {
    document: { getElementById: element, createElement: tag => { const key = Symbol(tag); return element(key); } },
    window: { setTimeout() {} }, TextEncoder, presentation, strategyFailure,
    invoke(command, args) {
      if (command === 'strategies') return new Promise((resolve, reject) => calls.push({ args, resolve, reject }));
      if (command === 'snapshot') return Promise.resolve({ running: false, pending: 0, recording_error: null, history: [{ status: 'stopped', tick: 0, reason: 'test', recorded_at: 'test' }], writer_settings: {} });
      return Promise.resolve();
    },
  });
  assert.equal(calls[0].args.operation, 'list');
  calls.shift().resolve({ kind: 'versions', versions: [], next_cursor: null });
  await setImmediate();
  return { element, calls, click: id => element(id).handlers.click(), edit(text) { element('strategy-json').value = text; element('strategy-json').handlers.input(); } };
}
const preview = { kind: 'preview', explanation: '<script>literal explanation</script>', meaning_id: 'metten_rules_2', notice: 'Source access unverified', requirements: { support_profile: 'rules_2_initial', planned_bytes: 1, shared_underlying_bytes: 0, per_market_bytes: 1, startup_ms: {} } };
async function reviewed(h, text = '{"exact":"text"}') { h.edit(text); h.click('strategy-preview'); const call = h.calls.shift(); assert.equal(call.args.json, text); call.resolve(preview); await setImmediate(); }

test('exact editor text is previewed, rendered literally, and stale previews cannot enable save', async () => {
  const h = await harness(); h.edit('old'); h.click('strategy-preview');
  h.edit('new'); h.calls.shift().resolve(preview); await setImmediate();
  assert.equal(h.element('strategy-save').disabled, true);
  assert.equal(h.element('strategy-explanation').textContent, '');
  await reviewed(h, 'new');
  assert.equal(h.element('strategy-explanation').textContent, preview.explanation);
  assert.equal(h.element('strategy-save').disabled, false);
  h.edit('new '); assert.equal(h.element('strategy-save').disabled, true);
});

test('save captures reviewed bytes, blocks duplicates, and retains committed identity on refresh failure', async () => {
  const h = await harness(); await reviewed(h);
  h.click('strategy-save'); h.click('strategy-save');
  assert.equal(h.calls.length, 1);
  const save = h.calls.shift(); assert.equal(save.args.json, save.args.reviewed_json);
  h.edit('correction during save');
  save.resolve({ kind: 'saved', version: { id: '9007199254740993', recorded_at: 'UTC' } }); await setImmediate();
  assert.match(h.element('strategy-status').textContent, /Saved immutable version 9007199254740993/);
  h.calls.shift().reject('secret-marker'); await setImmediate();
  assert.match(h.element('strategy-status').textContent, /Saved immutable version/);
  assert.match(h.element('strategy-list-status').textContent, /refresh failed/);
  assert.equal(h.element('strategy-json').value, 'correction during save');
});

test('Busy preserves current preview; uncertain save directs inspection without automatic retry', async () => {
  const h = await harness(); await reviewed(h);
  h.click('strategy-preview'); h.calls.shift().reject({ code: 'busy' }); await setImmediate();
  assert.equal(h.element('strategy-save').disabled, false);
  assert.equal(h.element('strategy-explanation').textContent, preview.explanation);
  h.click('strategy-save'); h.calls.shift().reject('private transport error'); await setImmediate();
  assert.match(h.element('strategy-status').textContent, /Save not confirmed — inspect saved versions before retrying/);
  assert.equal(h.calls.length, 0);
  assert.doesNotMatch(h.element('strategy-status').textContent, /private/);
});

test('file reads and edit-copy invalidate previews; stale file and detail responses are discarded', async () => {
  const h = await harness(); await reviewed(h);
  let finishFile;
  h.element('strategy-file').files = [{ size: 4, text: () => new Promise(resolve => { finishFile = resolve; }) }];
  h.element('strategy-file').handlers.change(); assert.equal(h.element('strategy-save').disabled, true);
  h.edit('typed while reading'); finishFile('file'); await setImmediate();
  assert.equal(h.element('strategy-json').value, 'typed while reading');
  h.click('strategy-reload'); h.calls.shift().resolve({ kind: 'versions', versions: [{ id: '12', display_name: '<img onerror=bad>', status: 'legacy_read_only', recorded_at: 'old' }, { id: '13', display_name: null, status: 'unsupported', recorded_at: 'old' }], next_cursor: '12' }); await setImmediate();
  const buttons = h.element('strategy-versions').children.map(item => item.children[0]);
  assert.match(buttons[0].textContent, /<img onerror=bad>/);
  buttons[0].handlers.click(); buttons[1].handlers.click();
  const old = h.calls.shift(); const latest = h.calls.shift();
  const detail = id => ({ kind: 'detail', version: { summary: { id, status: 'legacy_read_only' }, json: `original ${id}`, explanation: null, meaning_id: null, notice: 'No historical explanation' } });
  latest.resolve(detail('13')); await setImmediate(); old.resolve(detail('12')); await setImmediate();
  assert.match(h.element('strategy-detail').textContent, /Version 13/);
  h.click('strategy-copy'); assert.equal(h.element('strategy-json').value, 'original 13');
  assert.equal(h.element('strategy-save').disabled, true); assert.equal(h.element('strategy-json').focused, true);
  h.click('strategy-more'); assert.equal(h.calls.shift().args.before_id, '12');
});

test('invalid preview keeps correction text and oversized files never get read', async () => {
  const h = await harness(); h.edit('bad text'); h.click('strategy-preview');
  h.calls.shift().resolve({ kind: 'rule_errors', errors: [{ path: '$.name', code: 'malformed', message: 'Invalid name', accepted_form: 'nonempty name' }], truncated: false }); await setImmediate();
  assert.equal(h.element('strategy-json').value, 'bad text'); assert.equal(h.element('strategy-save').disabled, true);
  assert.match(h.element('strategy-status').textContent, /\$\.name/);
  h.element('strategy-file').files = [{ size: 262145, text() { assert.fail('must reject before reading'); } }];
  h.element('strategy-file').handlers.change(); assert.match(h.element('strategy-status').textContent, /256 KiB/);
});
