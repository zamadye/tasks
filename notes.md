# Repo Assist Memory — iamnbutler/tasks

## Last run: 2026-03-17 (run 23214828278)
Tasks 8 + 2 + 11 executed.

## Labelling
All 22 original open issues labelled on 2026-03-17 (run 23214476363).
New GPUI sub-issues (#153–#167) created 2026-03-17 — need labelling next run.
Cursor: should process #148-#152 and #153-#167 next run.

## PRs created
- `repo-assist/perf-parallel-poller-20260317`: Parallel issue+PR polling in RepoPoller
  (tokio::try_join!, updated poller tests to use body_string_contains matchers)

## comments_made
- #43: Jitter implementation options (deterministic hash-based vs next_retry_at field)
- #147: Root cause + fix path for project ID display (task.project shows UUID, need to resolve via snapshot.projects)

## Issues created (may not be findable via search — safeoutputs label may not exist)
- "[Repo Assist] Monthly Activity 2026-03" — created run 23214828278

## Project notes
- Rust codebase, apple/container isolation, Claude-backed orchestrator
- All components exist; critical path to e2e: #144, #143, #46, #147
- spec/spec.md says "TypeScript" but impl is Rust
- GPUI desktop app work started (epic #145), many sub-issues #153-#167

## Backlog
- Label #148-#152 and GPUI issues #153-#167 (many are unlabelled)
- Investigate #144 (unique branch names for tasks) — fixable bug
- Investigate #143 (remove origin branch when throwing away work) — fixable bug
- Comment on #91, #86, #84, #48, #47 — no RA comments yet
