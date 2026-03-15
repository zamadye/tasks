# Agent Retry Logic Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** When an agent exits non-zero, the server decides whether to retry (transition to Waiting with backoff) or give up (transition to Failed), based on retry count and progress — instead of always failing.

**Architecture:** The session manager emits a new `agent:exit` event with raw exit data. The run loop's failure handler reads the task, checks `retry_count < max_retries`, and either retries (Waiting + increment retry_count) or fails. The dispatcher's existing backoff logic handles re-dispatch timing.

**Tech Stack:** Rust, events crate, server crate, app crate

---

### Task 1: Add `AgentExit` event type

**Files:**
- Modify: `crates/events/src/event.rs`

**Step 1: Add the variant and string mappings**

In the `EventType` enum, add `AgentExit` after the existing agent events:

```rust
// Agent events
AgentMessage,
AgentQuestion,
AgentError,
AgentExit,
```

In `as_str()`, add:

```rust
Self::AgentExit => "agent:exit",
```

In `TryFrom<String>`, add:

```rust
"agent:exit" => Ok(Self::AgentExit),
```

**Step 2: Run tests**

Run: `cargo test -p events`
Expected: PASS — existing tests still work, new variant is wired up.

**Step 3: Commit**

```
feat(events): add AgentExit event type for raw agent exit reporting
```

---

### Task 2: Session manager emits `AgentExit` instead of deciding state

**Files:**
- Modify: `crates/session/src/manager.rs`

**Step 1: Update the existing tests first**

The test `exit_zero_maps_to_awaiting_merge` should still pass — exit(0) still emits `task:state:awaiting_merge`.

Change `exit_nonzero_maps_to_failed` to expect `AgentExit` instead of `TaskStateFailed`:

```rust
#[tokio::test]
async fn exit_nonzero_maps_to_agent_exit() {
    let (bus, mut rx) = test_event_bus().await;
    let exit = runtime::protocol::AgentExitEvent {
        code: Some(1),
        signal: None,
    };

    handle_exit("task-1", &exit, false, &bus).await;

    let received = rx.recv().await.unwrap();
    assert_eq!(received.event_type, events::EventType::AgentExit);
    assert_eq!(received.data["exit_code"], 1);
    assert_eq!(received.data["made_progress"], false);
}
```

Update `exit_includes_progress_flag` similarly — rename to `exit_nonzero_includes_progress_flag`:

```rust
#[tokio::test]
async fn exit_nonzero_includes_progress_flag() {
    let (bus, mut rx) = test_event_bus().await;
    let exit = runtime::protocol::AgentExitEvent {
        code: Some(1),
        signal: None,
    };

    handle_exit("task-1", &exit, true, &bus).await;

    let received = rx.recv().await.unwrap();
    assert_eq!(received.event_type, events::EventType::AgentExit);
    assert_eq!(received.data["made_progress"], true);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p tasks-session`
Expected: FAIL — `handle_exit` still emits `TaskStateFailed`.

**Step 3: Update `handle_exit` implementation**

Change the non-zero branch to emit `AgentExit` instead of `TaskStateFailed`:

```rust
async fn handle_exit(
    task_id: &str,
    exit: &runtime::protocol::AgentExitEvent,
    made_progress: bool,
    event_bus: &EventBus,
) {
    let success = exit.code == Some(0);

    let (event_type, data) = if success {
        (
            events::EventType::TaskStateAwaitingMerge,
            serde_json::json!({ "exit_code": 0 }),
        )
    } else {
        (
            events::EventType::AgentExit,
            serde_json::json!({
                "exit_code": exit.code,
                "signal": exit.signal,
                "made_progress": made_progress,
            }),
        )
    };

    let event = events::Event::new(event_type, task_id, events::Actor::System, data);
    let _ = event_bus.publish(event).await;
}
```

**Step 4: Run tests**

Run: `cargo test -p tasks-session`
Expected: PASS

**Step 5: Commit**

```
refactor(session): emit AgentExit event instead of deciding task state on failure
```

---

### Task 3: Add `max_retries` to config

**Files:**
- Modify: `crates/app/src/config.rs`

**Step 1: Add the field and env var**

Add to `AppConfig`:

```rust
/// Max task retries before marking as failed (default: 3, spec §13.2).
pub max_retries: u32,
```

In `from_env()`, add before the `Ok(Self { ... })`:

```rust
let max_retries = std::env::var("TASKS_MAX_RETRIES")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(3);
```

And include `max_retries` in the returned struct.

**Step 2: Run check**

Run: `cargo check`
Expected: Compilation errors in run_loop.rs where `config` is destructured — fix by passing `config.max_retries` to the failure handler (done in Task 4).

**Step 3: Commit**

```
feat(config): add TASKS_MAX_RETRIES config (default: 3)
```

---

### Task 4: Add `Server::handle_agent_failure` method

**Files:**
- Modify: `crates/server/src/server.rs`

**Step 1: Write tests**

Add to the `tests` module in `server.rs`:

```rust
#[tokio::test]
async fn handle_failure_retries_when_under_max() {
    let store = tasks_store::Store::open_memory().unwrap();
    let dir = tempdir().unwrap();
    let event_store = EventStore::new(dir.path());
    let bus = EventBus::new(event_store, 64);
    let server = Server::with_store(bus, store);
    let mut rx = server.event_bus.subscribe();

    let project = Project::new("p1", "owner/repo");
    server.add_project(project).await;
    let task = Task::new("t1", TaskSource::Internal, "Test", "p1");
    server.add_task(task).await.unwrap();
    // Drain the TaskCreated event
    let _ = rx.recv().await;

    server.handle_agent_failure("t1", 3).await.unwrap();

    let task = server.get_task("t1").await.unwrap();
    assert_eq!(task.state, TaskState::Waiting);
    assert_eq!(task.retry_count, 1);
    assert!(task.last_failure_at.is_some());

    // Should have emitted task:state:waiting
    let event = rx.recv().await.unwrap();
    assert_eq!(event.event_type, EventType::TaskStateWaiting);
}

#[tokio::test]
async fn handle_failure_fails_when_retries_exhausted() {
    let store = tasks_store::Store::open_memory().unwrap();
    let dir = tempdir().unwrap();
    let event_store = EventStore::new(dir.path());
    let bus = EventBus::new(event_store, 64);
    let server = Server::with_store(bus, store);
    let mut rx = server.event_bus.subscribe();

    let project = Project::new("p1", "owner/repo");
    server.add_project(project).await;
    let mut task = Task::new("t1", TaskSource::Internal, "Test", "p1");
    task.retry_count = 3; // already at max
    server.add_task(task).await.unwrap();
    let _ = rx.recv().await;

    server.handle_agent_failure("t1", 3).await.unwrap();

    let task = server.get_task("t1").await.unwrap();
    assert_eq!(task.state, TaskState::Failed);
    assert_eq!(task.retry_count, 4); // incremented past max

    let event = rx.recv().await.unwrap();
    assert_eq!(event.event_type, EventType::TaskStateFailed);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test -p server handle_failure`
Expected: FAIL — method doesn't exist yet.

**Step 3: Implement `handle_agent_failure`**

Add to `impl Server`, after `sync_task_state`:

```rust
/// Handle an agent failure — decide whether to retry or mark as failed (spec §13, §18.2).
///
/// Increments retry_count and sets last_failure_at. If retries are exhausted,
/// transitions to Failed. Otherwise transitions to Waiting for re-dispatch
/// after backoff.
pub async fn handle_agent_failure(
    &self,
    task_id: &str,
    max_retries: u32,
) -> Result<(), ServerError> {
    let new_state = {
        let mut state = self.state.write().await;
        let task = state
            .tasks
            .get_mut(task_id)
            .ok_or_else(|| ServerError::TaskNotFound(task_id.to_string()))?;

        task.retry_count += 1;
        task.last_failure_at = Some(chrono::Utc::now());

        let new_state = if task.retry_count > max_retries {
            TaskState::Failed
        } else {
            TaskState::Waiting
        };
        task.set_state(new_state);

        // Write-through to store
        if let Some(ref store) = self.store {
            if let Ok(store) = store.lock() {
                let _ = store.save_task(task);
            }
        }

        new_state
    };

    // Emit the state transition event
    let event_type = match new_state {
        TaskState::Failed => EventType::TaskStateFailed,
        TaskState::Waiting => EventType::TaskStateWaiting,
        _ => unreachable!(),
    };
    let event = Event::new(event_type, task_id, Actor::System, serde_json::json!({}));
    self.event_bus.publish(event).await?;
    Ok(())
}
```

Note: This method emits its own state event (unlike `sync_task_state`), because the retry decision originates here — it's not syncing an external event.

**Step 4: Add `use chrono` if not already imported**

Check existing imports at top of `server.rs`. If `chrono::Utc` isn't imported, add it.

**Step 5: Run tests**

Run: `cargo test -p server handle_failure`
Expected: PASS

**Step 6: Commit**

```
feat(server): add handle_agent_failure with retry-or-fail logic
```

---

### Task 5: Wire failure handler into the run loop

**Files:**
- Modify: `crates/app/src/run_loop.rs`

**Step 1: Add `AgentExit` handling to the state sync loop**

The state sync loop currently matches on `task:state:*` events. Add a branch for `AgentExit` that calls `handle_agent_failure` instead of `sync_task_state`.

Replace the state sync loop body with:

```rust
let sync_handle = tokio::spawn(async move {
    loop {
        match sync_rx.recv().await {
            Ok(event) => {
                // Handle agent exit — retry decision (spec §18.2)
                if event.event_type == EventType::AgentExit {
                    if let Err(e) =
                        sync_server.handle_agent_failure(&event.task, max_retries).await
                    {
                        warn!(
                            task_id = %event.task,
                            error = %e,
                            "failed to handle agent failure"
                        );
                    }
                    continue;
                }

                // Sync task:state:* events to in-memory state + store
                let new_state = match event.event_type {
                    EventType::TaskStateRunning => Some(models::task::TaskState::Running),
                    EventType::TaskStateAwaitingMerge => {
                        Some(models::task::TaskState::AwaitingMerge)
                    }
                    EventType::TaskStateFailed => Some(models::task::TaskState::Failed),
                    EventType::TaskStateCompleted => {
                        Some(models::task::TaskState::Completed)
                    }
                    EventType::TaskStateCancelled => {
                        Some(models::task::TaskState::Cancelled)
                    }
                    EventType::TaskStateWaiting => Some(models::task::TaskState::Waiting),
                    EventType::TaskStateBlocked => Some(models::task::TaskState::Blocked),
                    EventType::TaskStateQuestion => Some(models::task::TaskState::Question),
                    EventType::TaskStateTesting => Some(models::task::TaskState::Testing),
                    EventType::TaskStateConflict => Some(models::task::TaskState::Conflict),
                    _ => None,
                };

                if let Some(state) = new_state {
                    if let Err(e) =
                        sync_server.sync_task_state(&event.task, state).await
                    {
                        warn!(
                            task_id = %event.task,
                            error = %e,
                            "failed to sync task state from event"
                        );
                    }
                }
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                warn!(skipped = n, "state sync loop lagged — some events may not have been synced");
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
        }
    }
});
```

**Step 2: Pass `max_retries` into the sync loop closure**

Above the sync loop spawn, capture `max_retries` from config:

```rust
let max_retries = config.max_retries;
```

This goes alongside the existing `let sync_server = server.clone();`.

**Step 3: Run full test suite**

Run: `cargo test --workspace`
Expected: PASS

**Step 4: Commit**

```
feat(app): wire AgentExit handler into state sync loop with retry logic
```

---

### Task 6: Update the spec

**Files:**
- Modify: `spec/spec.md`

**Step 1: Update §8.3 Event Types**

In the Agent events section, add:

```
- `agent:exit` — agent process exited (carries exit_code, signal, made_progress)
```

**Step 2: Update §18.2 handle_failure**

Replace the current pseudocode to clarify the architectural boundary:

```
function start_session(session, prompt):
    ...
    match event:
        agent:started -> emit task:state:running
        agent:stdout  -> emit agent:message, check for question patterns
        agent:stderr  -> log warning
        agent:exit(0) -> emit task:state:testing or task:state:awaiting_merge
        agent:exit(n) -> emit agent:exit {exit_code=n, signal, made_progress}

# Failure handling lives in the server, not the session (see §9.3).
# The state sync loop receives agent:exit events and calls handle_failure.

function handle_failure(server, task_id, max_retries):
    task = server.get_task(task_id)
    task.retry_count += 1
    task.last_failure_at = now()

    if task.retry_count > max_retries:
        set task.state = Failed
        emit task:state:failed
    else:
        set task.state = Waiting
        emit task:state:waiting
        # Dispatcher picks up after backoff (§13.2).
```

**Step 3: Commit**

```
docs(spec): clarify retry logic ownership — server handles agent:exit, not session
```

---

### Task 7: Final verification

**Step 1: Run full test suite**

Run: `cargo test --workspace`
Expected: All tests PASS.

**Step 2: Run cargo clippy**

Run: `cargo clippy --workspace`
Expected: No new warnings.

**Step 3: Verify the web frontend still builds**

Run: `cd web && bun run build`
Expected: PASS — no frontend changes needed.
