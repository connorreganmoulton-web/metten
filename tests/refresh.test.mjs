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

for (const olderFails of [false, true]) {
  for (const newerFails of [false, true]) {
    test(`older status ${olderFails ? 'failure' : 'success'} cannot overwrite newer ${newerFails ? 'failure' : 'Pause'}`, async () => {
      const elements = new Map();
      function element(id) {
        if (!elements.has(id)) elements.set(id, {
          textContent: '', disabled: false, handlers: {},
          addEventListener(event, handler) { this.handlers[event] = handler; },
          append() {}, replaceChildren() {},
        });
        return elements.get(id);
      }
      const reads = [];
      const source = stripTypeScriptTypes(await readFile(new URL('../src/main.ts', import.meta.url), 'utf8'))
        .replace(/^import .*;\n/gm, '');
      runInNewContext(source, {
        document: { getElementById: element, createElement: element },
        window: { setTimeout() {} }, presentation,
        invoke(command) {
          return command === 'control' ? Promise.resolve()
            : new Promise((resolve, reject) => reads.push({ resolve, reject }));
        },
      });
      element('pause').handlers.click();
      await setImmediate();
      assert.equal(reads.length, 2);
      const snapshot = status => ({
        running: status === 'running', pending: 0, recording_error: null,
        history: [{ status, tick: status === 'running' ? 1 : 2, reason: status, recorded_at: '2026-11-13T00:00:00Z' }],
        writer_settings: {},
      });
      if (newerFails) reads[1].reject('Latest status failure');
      else reads[1].resolve(snapshot('paused'));
      await setImmediate();
      assert.equal(element('status').textContent, newerFails ? 'Session status unavailable' : 'Paused · no test events running');
      const displayed = [...elements].map(([id, value]) => [id, value.textContent, value.disabled]);
      if (olderFails) reads[0].reject('Outdated status failure');
      else reads[0].resolve(snapshot('running'));
      await setImmediate();
      assert.deepEqual([...elements].map(([id, value]) => [id, value.textContent, value.disabled]), displayed);
      assert.equal(element('advance').disabled, true);
      assert.equal(element('pause').disabled, newerFails ? false : true);
    });
  }
}
