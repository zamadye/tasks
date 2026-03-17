//! App configuration — reads from environment variables.

use std::time::Duration;

/// Expand tilde at the start of a path to the user's home directory.
///
/// Shell doesn't expand `~` in environment variable values, so we handle it manually.
/// Only expands `~` at the start of the path (e.g., `~/.tasks` -> `/home/user/.tasks`).
pub fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{home}/{rest}")
    } else if path == "~" {
        std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
    } else {
        path.to_string()
    }
}

/// Top-level app configuration.
pub struct AppConfig {
    /// Data directory (default: `~/.tasks`).
    pub data_dir: String,
    /// GitHub personal access token.
    pub github_token: String,
    /// Global max concurrent sessions (default: 5).
    pub max_sessions: u32,
    /// Maximum retry attempts for failed tasks (default: 3, spec §13.2/§14.1).
    pub max_retries: u32,
    /// GitHub poll interval (default: 60s).
    pub poll_interval: Duration,
    /// Dispatch tick interval (default: 30s).
    pub dispatch_interval: Duration,
    /// Container image for sessions.
    pub container_image: String,
    /// Container memory limit (default: 8G).
    pub container_memory: String,
    /// Session soft time limit (default: 1h).
    pub session_soft_limit: Duration,
    /// Session hard time limit (default: 1h15m).
    pub session_hard_limit: Duration,
    /// Minimum session duration to count as "progress" (default: 60s, spec §13.1).
    pub progress_threshold: Duration,
    /// Memory usage percentage at which to warn (default: 75%).
    pub memory_warn_pct: u8,
    /// Memory usage percentage at which to pause dispatch (default: 85%).
    pub memory_soft_limit_pct: u8,
    /// Memory usage percentage at which to emergency-stop sessions (default: 92%).
    pub memory_hard_limit_pct: u8,
    /// Whether to run the TUI.
    pub tui: bool,
    /// Whether to run the web UI.
    pub web: bool,
    /// Web server port (default: 4800).
    pub web_port: u16,
}

impl AppConfig {
    /// Read configuration from `.env` file (if present) and environment variables.
    pub fn from_env() -> Result<Self, String> {
        // Load .env file if it exists — doesn't error if missing.
        dotenvy::dotenv().ok();
        let data_dir = std::env::var("TASKS_DATA_DIR")
            .map(|p| expand_tilde(&p))
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                format!("{home}/.tasks")
            });

        let github_token = std::env::var("GITHUB_TOKEN")
            .map_err(|_| "GITHUB_TOKEN environment variable not set".to_string())?;

        let container_image = std::env::var("TASKS_CONTAINER_IMAGE")
            .unwrap_or_else(|_| "tasks-agent:latest".to_string());

        let container_memory =
            std::env::var("TASKS_CONTAINER_MEMORY").unwrap_or_else(|_| "8G".to_string());

        let max_sessions = std::env::var("TASKS_MAX_SESSIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let max_retries = std::env::var("TASKS_MAX_RETRIES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(server::DEFAULT_MAX_RETRIES);

        let poll_interval_secs = std::env::var("TASKS_POLL_INTERVAL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60u64);

        let dispatch_interval_secs = std::env::var("TASKS_DISPATCH_INTERVAL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30u64);

        let progress_threshold_secs = std::env::var("TASKS_PROGRESS_THRESHOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60u64);

        let memory_warn_pct = std::env::var("TASKS_MEMORY_WARN_PCT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(75u8);

        let memory_soft_limit_pct = std::env::var("TASKS_MEMORY_SOFT_LIMIT_PCT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(85u8);

        let memory_hard_limit_pct = std::env::var("TASKS_MEMORY_HARD_LIMIT_PCT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(92u8);

        if !(memory_warn_pct < memory_soft_limit_pct && memory_soft_limit_pct < memory_hard_limit_pct) {
            return Err(format!(
                "Memory thresholds must be ordered: warn ({memory_warn_pct}) < soft ({memory_soft_limit_pct}) < hard ({memory_hard_limit_pct})"
            ));
        }

        Ok(Self {
            data_dir,
            github_token,
            max_sessions,
            max_retries,
            poll_interval: Duration::from_secs(poll_interval_secs),
            dispatch_interval: Duration::from_secs(dispatch_interval_secs),
            container_image,
            container_memory,
            session_soft_limit: Duration::from_secs(3600),
            session_hard_limit: Duration::from_secs(4500),
            progress_threshold: Duration::from_secs(progress_threshold_secs),
            memory_warn_pct,
            memory_soft_limit_pct,
            memory_hard_limit_pct,
            tui: false, // set by CLI flag
            web: false, // set by CLI flag
            web_port: std::env::var("TASKS_WEB_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(4800),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_tilde_with_path() {
        // SAFETY: Test runs single-threaded, safe to modify env
        unsafe { std::env::set_var("HOME", "/home/testuser") };
        assert_eq!(expand_tilde("~/.tasks"), "/home/testuser/.tasks");
        assert_eq!(expand_tilde("~/foo/bar"), "/home/testuser/foo/bar");
    }

    #[test]
    fn expand_tilde_just_tilde() {
        // SAFETY: Test runs single-threaded, safe to modify env
        unsafe { std::env::set_var("HOME", "/home/testuser") };
        assert_eq!(expand_tilde("~"), "/home/testuser");
    }

    #[test]
    fn expand_tilde_no_tilde() {
        assert_eq!(expand_tilde("/absolute/path"), "/absolute/path");
        assert_eq!(expand_tilde("relative/path"), "relative/path");
        assert_eq!(expand_tilde(""), "");
    }

    #[test]
    fn expand_tilde_middle_tilde_unchanged() {
        // Tilde in the middle of a path should not be expanded
        assert_eq!(expand_tilde("/path/~/.tasks"), "/path/~/.tasks");
        assert_eq!(expand_tilde("foo~bar"), "foo~bar");
    }
}
