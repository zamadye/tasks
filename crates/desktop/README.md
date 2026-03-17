# Tasks Desktop

Native GPUI desktop frontend for the Tasks platform.

## Overview

This crate provides a native desktop UI using [GPUI](https://www.gpui.rs/), the UI framework from [Zed](https://zed.dev/). It replicates the functionality of the web frontend (`web/`) as a native application.

## Platform Requirements

GPUI requires platform-specific dependencies:

### macOS

No additional dependencies needed. GPUI uses native macOS frameworks.

### Linux

Install the following system packages:

```bash
# Debian/Ubuntu
sudo apt install pkg-config libx11-dev libxcb1-dev libxkbcommon-dev libwayland-dev

# Fedora/RHEL
sudo dnf install pkg-config libX11-devel libxcb-devel libxkbcommon-devel wayland-devel

# Arch Linux
sudo pacman -S pkg-config libx11 libxcb libxkbcommon wayland
```

### Windows

GPUI has experimental Windows support. See the [Zed documentation](https://github.com/zed-industries/zed) for details.

## Building

```bash
# Check compilation
cargo check -p desktop

# Build
cargo build -p desktop

# Run
cargo run -p desktop
```

## Configuration

The desktop app connects to a running Tasks server. Configure the API URL:

```bash
# Default: http://localhost:4800
export TASKS_API_URL="http://localhost:4800"
cargo run -p desktop
```

## Architecture

```
src/
├── main.rs       # Entry point, window creation
├── lib.rs        # Module exports
├── api.rs        # HTTP API client
├── state.rs      # App state management (GPUI Model)
├── theme.rs      # Colors, typography, spacing
├── ui/           # Reusable UI components
│   ├── badge.rs  # Status badges
│   └── card.rs   # Card containers
└── views/        # View components
    └── dashboard.rs  # Main dashboard view
```

## Views

### Dashboard (`/`)

The main landing page displaying:
- **Stats cards**: Active sessions, running tasks, waiting tasks, merge queue count
- **Active tasks**: Tasks in running, question, testing, or awaiting_merge states
- **Recent events**: Latest 15 events from the event log

## Related Issues

- #145 - Epic: Create GPUI app for frontend UI
- #153 - Basic crate setup
- #157 - App state management
- #158 - UI primitive components
- #160 - Dashboard view (this implementation)
