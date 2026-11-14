# Evidence and checkpoint verification

## Find current evidence

Start with the current-state index in the issue's `docs/verification/` record, then read the linked acceptance and proof sections. Dated failures and continuation prompts preserve history; they do not override later accepted evidence. Issue #4's [index](../verification/issue-4.md#current-evidence-index) separates those records.

For large documents, locate headings with `rg -n '^#'` and read explicit line ranges. Size tool output to include each requested range completely. If output truncates, retrieve the missing range before claiming the source was read. Summarize findings with source locations instead of repeatedly printing whole reports.

When delegation is requested or otherwise authorized, give each agent a bounded goal, owned functions/files, required source pointers, checks and stopping condition. Require implemented corrections and compact evidence. The lead integrates results and verifies the final source; agents coordinate shared-file edits before making them.

## Verify a mixed-work checkpoint

When a checkpoint combines inherited changes, untracked files or partial staging:

1. Stage the intended files explicitly and inspect `git diff --cached --stat` and the staged diff.
2. Run the affected existing checks against the staged snapshot. For example:

   ```sh
   python3 scripts/check-staged.py /Users/connormoulton/.cargo/bin/cargo test -p metten-engine --locked
   ```

   The helper checks staged whitespace, exports the immutable staged tree through a temporary index, and runs the supplied command in that disposable folder. It preserves the real index and working files and removes the temporary folder on success or failure. The export contains no Git metadata, installed dependencies or build cache; choose checks that work on source alone, or use an explicit shell command to install required locked dependencies first. It does not commit or impose a new approval gate.
3. Record the printed tree identity and actual check outcome. Commit only when the required affected checks succeed; if staging changes afterward, verify the changed evidence again.

Use this for mixed-work checkpoints rather than every documentation edit. A normal check of the original working folder cannot establish that unstaged dependencies were included.

## Handle check failures

Run dependent checks and mutations in separate tool calls or with fail-fast execution. A later successful command does not clear an earlier failure. The staged helper exits nonzero on a whitespace or supplied-command failure and does not execute the check after failed whitespace validation.

Markdown hard breaks and verbatim source captures can intentionally contain whitespace. Preserve established archived records. If importing a new archival record requires an exception, identify the exact files and whitespace in the evidence, run an explicitly scoped check with that exception, and report the exception as preserved formatting. The strict helper will continue to reject those staged warnings; do not describe that run as passing or silently disable checks for code or other files. Resolve the archival import separately before checking a mixed code checkpoint.

## Review responsibilities

Reviewers check whether claimed behavior and resource bounds cover the actual changed paths and final source. Passing sample workloads do not replace a general argument. Changes to the pinned Rust toolchain require reviewing the standard-library layout and movement assumptions in the processing and memory proofs. Keep these judgement calls here; mechanical failures belong in executable checks.
