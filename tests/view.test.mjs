import { test } from 'node:test';
import assert from 'node:assert/strict';
import { presentation } from '../src/view.mjs';

test('pending and failed recording never claim saved success or permit another event', () => {
  const saved = { running: true, pending: 0, recording_error: null, history: [{ status: 'running' }] };
  assert.equal(presentation(saved).advanceDisabled, false);
  const pending = presentation({ ...saved, pending: 1 });
  assert.equal(pending.advanceDisabled, true);
  assert.equal(pending.pauseDisabled, false);
  assert.match(pending.recording, /not yet confirmed saved/);
  const failed = presentation({ ...saved, running: false, recording_error: 'disk failure' });
  assert.equal(failed.startDisabled, true);
  assert.equal(failed.advanceDisabled, true);
  assert.match(failed.status, /Stopped/);
  assert.match(failed.recording, /not confirmed saved/);
});
