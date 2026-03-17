# Tasks Desktop

GPUI-based native desktop application for the Tasks platform.

## Overview

This crate provides a native desktop UI for Tasks, built with [GPUI](https://gpui.rs/) (Zed's GPU-accelerated UI framework). It implements global application state management similar to React Context in the web frontend.

## Features

- **App State Management** (`state.rs`): Global reactive state using GPUI's Model/Entity system
  - Snapshot (projects, tasks, merge queue, slot utilization)
  - Events buffer (recent events, max 200)
  - Connection status tracking
  - Selected project filter with computed properties
- **HTTP API Client** (`api.rs`): Type-safe access to the Tasks REST API
- **SSE Client** (`sse.rs`): Real-time event streaming with automatic reconnection
- **Polling**: Automatic snapshot refresh every 5 seconds
- **SSE-triggered refreshes**: State-changing events trigger immediate updates

## Architecture

The app state follows GPUI's reactive patterns:

```
AppState (Model)
├── Snapshot - Full system state from /api/snapshot
├── Events buffer - Recent events from SSE
├── Connection status - SSE connection state
├── Selected project - Filter for views
└── Computed properties
    ├── filtered_tasks() - Tasks filtered by selected project
    ├── filtered_merge_queue() - Merge queue entries filtered by selected project
    └── active_tasks() - Running, question, testing, awaiting_merge states
```

## Dependencies

### Linux
```bash
# Debian/Ubuntu
sudo apt install pkg-config libx11-dev libxcb1-dev libxkbcommon-dev libwayland-dev

# Fedora/RHEL
sudo dnf install pkg-config libX11-devel libxcb-devel libxkbcommon-devel wayland-devel

# Arch
sudo pacman -S pkg-config libx11 libxcb libxkbcommon wayland
```

### macOS
No additional dependencies required (uses native frameworks).

## Building

```bash
# Check compilation
cargo check -p tasks-desktop

# Build
cargo build -p tasks-desktop

# Run (requires Tasks server running on localhost:4800)
TASKS_SERVER_URL=http://localhost:4800 cargo run -p tasks-desktop
```

## Environment Variables

- `TASKS_SERVER_URL`: Tasks server URL (default: `http://localhost:4800`)
- `RUST_LOG`: Log level (e.g., `info`, `debug`, `tasks_desktop=debug`)

## Reference Implementation

This implementation mirrors the web frontend's state management pattern:
- `web/src/hooks/use-app-state.ts` - React Context-based state
- `web/src/lib/api.ts` - TypeScript API client

## Testing

Tests require GUI libraries to be installed (see Dependencies above).

```bash
cargo test -p tasks-desktop
```
