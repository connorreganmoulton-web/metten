import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { stripTypeScriptTypes } from 'node:module';
import { runInNewContext } from 'node:vm';
import { setImmediate } from 'node:timers/promises';
import { presentation } from '../src/view.mjs';

test('failed refresh does not assert stopped and still lets the user attempt Pause', async () => {
  const elements = new Map();
  function element(id) {
    if (!elements.has(id)) elements.set(id, {
      textContent: '', disabled: false, handlers: {},
      addEventListener(event, handler) { this.handlers[event] = handler; },
      append() {}, replaceChildren() {},
    });
    return elements.get(id);
  }
  const controls = [];
  let reads = 0;
  let nextPoll;
  const source = stripTypeScriptTypes(await readFile(new URL('../src/main.ts', import.meta.url), 'utf8'))
    .replace(/^import .*;\n/gm, '');
  runInNewContext(source, {
    document: { getElementById: element, createElement: element },
    window: { setTimeout(callback) { nextPoll = callback; } },
    presentation,
    async invoke(command, args) {
      if (command === 'control') { controls.push(args.action); return; }
      if (++reads > 1) throw new Error('Controlled status-read failure');
      return {
        running: true, pending: 0, recording_error: null,
        history: [{ status: 'running', tick: 1, reason: 'Started', recorded_at: '2026-11-13T00:00:00Z' }],
        writer_settings: {},
      };
    },
  });
  await setImmediate();
  assert.match(element('status').textContent, /Running/);
  nextPoll();
  await setImmediate();
  assert.equal(element('status').textContent, 'Session status unavailable');
  assert.equal(element('start').disabled, true);
  assert.equal(element('advance').disabled, true);
  assert.equal(element('pause').disabled, false);
  assert.deepEqual(controls, [], 'a failed read has not stopped the engine');
  element('pause').handlers.click();
  await setImmediate();
  assert.deepEqual(controls, ['pause']);
});
