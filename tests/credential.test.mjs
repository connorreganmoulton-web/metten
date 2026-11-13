import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { stripTypeScriptTypes } from 'node:module';
import { runInNewContext } from 'node:vm';
import { setImmediate } from 'node:timers/promises';

test('credential submission clears input immediately and hides unexpected framework errors', async () => {
  const elements = new Map();
  const element = id => {
    if (!elements.has(id)) elements.set(id, { value: '', textContent: '', handlers: {}, disabled: false,
      addEventListener(name, handler) { this.handlers[name] = handler; } });
    return elements.get(id);
  };
  let rejectRequest;
  let submitted;
  const source = stripTypeScriptTypes(await readFile(new URL('../src/main.ts', import.meta.url), 'utf8')).replace(/^import .*;\n/gm, '');
  runInNewContext(source, {
    document: { getElementById: element }, window: { setTimeout() {} },
    invoke(command, args) {
      if (command === 'snapshot') return Promise.reject('Controlled snapshot failure');
      submitted = args;
      return new Promise((_, reject) => { rejectRequest = reject; });
    },
  });
  const marker = 'metten-dummy-test-only-marker';
  element('credential-value').value = marker;
  element('credential-save').handlers.click();
  assert.equal(element('credential-value').value, '');
  assert.equal(submitted.value, marker);
  assert.equal(element('credential-save').disabled, true);
  rejectRequest(`unexpected framework failure with ${marker}`);
  await setImmediate();
  assert.ok(!element('credential-status').textContent.includes(marker));
  assert.match(element('credential-status').textContent, /reopen/);
  assert.equal(element('credential-save').disabled, false);
});
