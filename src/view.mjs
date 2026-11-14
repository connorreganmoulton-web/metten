export function presentation(snapshot) {
  const failed = snapshot.recording_error !== null;
  const busy = snapshot.pending > 0;
  return {
    status: failed ? 'Stopped — recording needs attention' : snapshot.running ? 'Running · controlled non-trading session' : busy ? 'Stopped · recording control change…' : snapshot.history[0].status === 'paused' ? 'Paused · no test events running' : 'Stopped · ready when you are',
    recording: failed ? 'Requested change not confirmed saved. History below shows the last confirmed records.' : busy ? 'Saving… This change is not yet confirmed saved.' : 'All displayed records are saved.',
    startDisabled: failed || busy || snapshot.running,
    advanceDisabled: failed || busy || !snapshot.running,
    pauseDisabled: failed || (!busy && !snapshot.running),
  };
}

export function strategyFailure(failure, saving = false) {
  const code = failure && typeof failure === 'object' ? failure.code : '';
  if (code === 'busy') return 'Strategy action already in progress. Wait and retry.';
  if (code === 'not_reviewed') return 'Preview the current text before saving.';
  if (code === 'quitting') return 'Quitting — wait for the stopped state to be saved.';
  if (code === 'resource_limit') return 'Strategy content is too large to process or show completely. Stored history is preserved.';
  return saving ? 'Save not confirmed — inspect saved versions before retrying.' : 'Local strategy action failed. Wait and retry.';
}
