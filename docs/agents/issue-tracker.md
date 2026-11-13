# Issue tracker: GitHub

Repository: `connorreganmoulton-web/metten`. GitHub Issues is the work tracker; the committed product specification is the behavioral source of truth. Issues link to it rather than maintain a competing specification.

Use `gh` for tracker operations and explicit repository arguments when outside this working copy. Read each issue's full body and comments before working. Use a UTF-8 body file with `--body-file` for multiline creation, edits or comments.

Publish approved tickets in prerequisite order. Record blockers in each issue body and GitHub native dependency links using the blocker's numeric database ID. If native dependencies are unavailable, retain visible body links and report the limitation. Work only on tickets whose blockers are complete. Do not infer live-operation authority from labels.

PRs as a request surface: no.

When a skill says to publish to the issue tracker, create a GitHub issue. When it says to fetch a ticket, read its body, comments, labels and blocking relationships. Do not create a second local issue tracker.
