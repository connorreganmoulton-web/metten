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
