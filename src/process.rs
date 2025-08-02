use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{Pid, System};

#[cfg(unix)]
use std::time::Duration;
#[cfg(unix)]
use sysinfo::Signal;

pub struct ProcessManager {
    system: System,
}

impl ProcessManager {
    pub fn new() -> Self {
        let system = System::new_all();
        Self { system }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    /// Kill a process by PID
    pub fn kill_process(&mut self, pid: u32) -> Result<bool> {
        self.refresh();

        let sysinfo_pid = Pid::from(pid as usize);
        let result = if let Some(process) = self.system.process(sysinfo_pid) {
            #[cfg(unix)]
            {
                let term_result = process.kill_with(Signal::Term);
                // Give process time to terminate gracefully
                std::thread::sleep(Duration::from_millis(100));

                // Check if still running, then force kill
                self.refresh();
                if self.system.process(sysinfo_pid).is_some() {
                    if let Some(process) = self.system.process(sysinfo_pid) {
                        Ok(process.kill_with(Signal::Kill).unwrap_or(false))
                    } else {
                        Ok(true)
                    }
                } else {
                    Ok(term_result.unwrap_or(false))
                }
            }

            #[cfg(windows)]
            {
                Ok(process.kill())
            }
        } else {
            Err(anyhow::anyhow!("Process with PID {pid} not found"))
        };

        result
    }

    /// Get detailed process information
    #[allow(dead_code)]
    pub fn get_process_info(&mut self, pid: u32) -> Option<ProcessInfo> {
        self.refresh();

        let sysinfo_pid = Pid::from(pid as usize);
        self.system.process(sysinfo_pid).map(|process| ProcessInfo {
            pid,
            name: process.name().to_string(),
            command: {
                let cmd_args = process.cmd().join(" ");
                format!("{} {cmd_args}", process.name())
            },
            memory_usage: process.memory(),
            cpu_usage: process.cpu_usage(),
            start_time: process.start_time(),
            runtime_duration: self.calculate_runtime(process.start_time()),
        })
    }

    /// Check if a process is a system-critical process that should not be killed
    pub fn is_system_critical(&self, process_name: &str) -> bool {
        let critical_processes = [
            "systemd",
            "kernel",
            "init",
            "kthreadd",
            "rcu_gp",
            "rcu_par_gp",
            "migration",
            "ksoftirqd",
            "watchdog",
            "sshd",
            "dbus",
            "networkd",
            "wpa_supplicant",
            "dhcpcd",
            "chronyd",
            "rsyslog",
            // Windows critical processes
            "System",
            "smss.exe",
            "csrss.exe",
            "wininit.exe",
            "winlogon.exe",
            "services.exe",
            "lsass.exe",
            "dwm.exe",
            "explorer.exe",
            // macOS critical processes
            "launchd",
            "kernel_task",
            "WindowServer",
            "loginwindow",
            "Dock",
        ];

        critical_processes.iter().any(|&critical| {
            process_name
                .to_lowercase()
                .contains(&critical.to_lowercase())
        })
    }

    #[allow(dead_code)]
    fn calculate_runtime(&self, start_time: u64) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let runtime_seconds = now.saturating_sub(start_time);
        format_duration(runtime_seconds)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub command: String,
    pub memory_usage: u64,
    pub cpu_usage: f32,
    pub start_time: u64,
    pub runtime_duration: String,
}

/// Format duration in seconds to human-readable format
pub fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else if minutes > 0 {
        format!("{minutes}m {secs}s")
    } else {
        format!("{secs}s")
    }
}

/// Format memory usage in bytes to human-readable format
pub fn format_memory(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        let size_u64 = size as u64;
        format!("{size_u64} {unit}", unit = UNITS[unit_index])
    } else {
        format!("{size:.1} {unit}", unit = UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30), "30s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3600), "1h 0m");
        assert_eq!(format_duration(90061), "1d 1h");
    }

    #[test]
    fn test_format_memory() {
        assert_eq!(format_memory(512), "512 B");
        assert_eq!(format_memory(1024), "1.0 KB");
        assert_eq!(format_memory(1536), "1.5 KB");
        assert_eq!(format_memory(1048576), "1.0 MB");
        assert_eq!(format_memory(1073741824), "1.0 GB");
    }

    #[test]
    fn test_process_manager_creation() {
        let _manager = ProcessManager::new();
        // Just verify we can create a manager without panicking
        // Just verify we can create a manager without panicking
    }

    #[test]
    fn test_is_system_critical() {
        let manager = ProcessManager::new();

        // Test system critical processes
        assert!(manager.is_system_critical("systemd"));
        assert!(manager.is_system_critical("kernel"));
        assert!(manager.is_system_critical("sshd"));
        assert!(manager.is_system_critical("System"));
        assert!(manager.is_system_critical("launchd"));

        // Test non-critical processes
        assert!(!manager.is_system_critical("node"));
        assert!(!manager.is_system_critical("python"));
        assert!(!manager.is_system_critical("java"));
        assert!(!manager.is_system_critical("firefox"));
    }
}
