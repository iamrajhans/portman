use anyhow::{Context, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::process::Command;
use sysinfo::{Pid, System};

#[cfg(target_os = "linux")]
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PortInfo {
    pub port: u16,
    pub pid: u32,
    pub process_name: String,
    pub command: String,
    pub start_time: u64,
    pub memory_usage: u64,
}

pub struct PortScanner {
    system: System,
}

impl PortScanner {
    pub fn new() -> Self {
        let system = System::new_all();
        Self { system }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    /// Get all occupied ports with their process information
    pub fn scan_all_ports(&mut self) -> Result<Vec<PortInfo>> {
        self.refresh();

        #[cfg(target_os = "windows")]
        return self.scan_ports_windows();

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        return self.scan_ports_unix();
    }

    /// Check if a specific port is available
    #[allow(dead_code)]
    pub fn is_port_available(&self, port: u16) -> bool {
        TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)).is_ok()
    }

    /// Get process information for a specific port
    pub fn get_port_info(&mut self, port: u16) -> Result<Option<PortInfo>> {
        let all_ports = self.scan_all_ports()?;
        Ok(all_ports.into_iter().find(|info| info.port == port))
    }

    #[cfg(target_os = "linux")]
    fn scan_ports_unix(&self) -> Result<Vec<PortInfo>> {
        let output = Command::new("netstat")
            .args(["-tulnp"])
            .output()
            .context("Failed to execute netstat command")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("netstat command failed"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut port_infos = Vec::new();
        let mut pid_to_port: HashMap<u32, Vec<u16>> = HashMap::new();

        // Parse netstat output
        for line in output_str.lines().skip(2) {
            if let Some(port_info) = self.parse_netstat_line(line) {
                if let Some(pid) = port_info.1 {
                    pid_to_port
                        .entry(pid)
                        .or_insert_with(Vec::new)
                        .push(port_info.0);
                }
            }
        }

        // Match PIDs with process information
        for (pid, ports) in pid_to_port {
            if let Some(process) = self.system.process(Pid::from(pid as usize)) {
                for port in ports {
                    port_infos.push(PortInfo {
                        port,
                        pid,
                        process_name: process.name().to_string(),
                        command: format!("{} {}", process.name(), process.cmd().join(" ")),
                        start_time: process.start_time(),
                        memory_usage: process.memory(),
                    });
                }
            }
        }

        Ok(port_infos)
    }

    #[cfg(target_os = "macos")]
    fn scan_ports_unix(&self) -> Result<Vec<PortInfo>> {
        // Use lsof on macOS as it's more reliable for getting process info
        let output = Command::new("lsof")
            .args(["-i", "TCP", "-P", "-n", "-s", "TCP:LISTEN"])
            .output()
            .context("Failed to execute lsof command. Please install lsof or run with elevated privileges")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("lsof command failed"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut port_infos = Vec::new();

        // Parse lsof output (format: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME)
        for line in output_str.lines().skip(1) {
            if let Some(port_info) = self.parse_lsof_line(line) {
                if let Some(process) = self.system.process(Pid::from(port_info.1 as usize)) {
                    port_infos.push(PortInfo {
                        port: port_info.0,
                        pid: port_info.1,
                        process_name: process.name().to_string(),
                        command: format!("{} {}", process.name(), process.cmd().join(" ")),
                        start_time: process.start_time(),
                        memory_usage: process.memory(),
                    });
                }
            }
        }

        Ok(port_infos)
    }

    #[cfg(target_os = "windows")]
    fn scan_ports_windows(&self) -> Result<Vec<PortInfo>> {
        let output = Command::new("netstat")
            .args(["-ano"])
            .output()
            .context("Failed to execute netstat command")?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("netstat command failed"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut port_infos = Vec::new();

        for line in output_str.lines().skip(4) {
            if line.trim().starts_with("TCP") {
                if let Some(port_info) = self.parse_netstat_line_windows(line) {
                    if let Some(process) = self.system.process(Pid::from(port_info.1 as usize)) {
                        port_infos.push(PortInfo {
                            port: port_info.0,
                            pid: port_info.1,
                            process_name: process.name().to_string(),
                            command: format!("{} {}", process.name(), process.cmd().join(" ")),
                            start_time: process.start_time(),
                            memory_usage: process.memory(),
                        });
                    }
                }
            }
        }

        Ok(port_infos)
    }

    #[cfg(target_os = "macos")]
    fn parse_lsof_line(&self, line: &str) -> Option<(u16, u32)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 {
            return None;
        }

        // Extract PID (second column, index 1)
        let pid = parts[1].parse::<u32>().ok()?;

        // Extract port from NAME column (index 8)
        // Format: "COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME (LISTEN)"
        let name = parts[8];

        // Extract port from address (format like "*:3000" or "127.0.0.1:3000")
        let port = if let Some(colon_pos) = name.rfind(':') {
            name[colon_pos + 1..].parse::<u16>().ok()?
        } else {
            return None;
        };

        Some((port, pid))
    }

    #[cfg(target_os = "linux")]
    fn parse_netstat_line(&self, line: &str) -> Option<(u16, Option<u32>)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            return None;
        }

        // Extract port from address (format: 0.0.0.0:8080 or :::8080)
        let address = parts[3];
        let port = if let Some(colon_pos) = address.rfind(':') {
            address[colon_pos + 1..].parse::<u16>().ok()?
        } else {
            return None;
        };

        // Extract PID (last column, format: "12345/process_name" or "-")
        let pid_info = parts.last()?;
        let pid = if *pid_info != "-" {
            if let Some(slash_pos) = pid_info.find('/') {
                pid_info[..slash_pos].parse::<u32>().ok()
            } else {
                pid_info.parse::<u32>().ok()
            }
        } else {
            None
        };

        Some((port, pid))
    }

    #[cfg(target_os = "windows")]
    fn parse_netstat_line_windows(&self, line: &str) -> Option<(u16, u32)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            return None;
        }

        // Extract port from local address
        let local_addr = parts[1];
        let port = if let Some(colon_pos) = local_addr.rfind(':') {
            local_addr[colon_pos + 1..].parse::<u16>().ok()?
        } else {
            return None;
        };

        // Extract PID (last column)
        let pid = parts[4].parse::<u32>().ok()?;

        Some((port, pid))
    }
}

/// Common development ports that developers typically use
pub const COMMON_DEV_PORTS: &[u16] = &[
    3000, 3001, 3002, 3003, // React, Next.js, Node.js
    4000, 4001, 4200, // Angular, development servers
    5000, 5001, 5173, // Flask, Vite
    8000, 8001, 8080, 8081, 8888, // Django, Java, Jupyter
    9000, 9001, 9090, // Various dev servers
];

pub fn parse_port_range(range_str: &str) -> Result<(u16, u16)> {
    let parts: Vec<&str> = range_str.split('-').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid range format. Use: START-END (e.g., 3000-9000)"
        ));
    }

    let start = parts[0]
        .parse::<u16>()
        .context("Invalid start port number")?;
    let end = parts[1].parse::<u16>().context("Invalid end port number")?;

    if start > end {
        return Err(anyhow::anyhow!(
            "Start port must be less than or equal to end port"
        ));
    }

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_port_range_valid() {
        assert_eq!(parse_port_range("3000-3010").unwrap(), (3000, 3010));
        assert_eq!(parse_port_range("80-8080").unwrap(), (80, 8080));
        assert_eq!(parse_port_range("1-65535").unwrap(), (1, 65535));
    }

    #[test]
    fn test_parse_port_range_invalid() {
        assert!(parse_port_range("3000").is_err());
        assert!(parse_port_range("3000-").is_err());
        assert!(parse_port_range("-3000").is_err());
        assert!(parse_port_range("3000-abc").is_err());
        assert!(parse_port_range("abc-3000").is_err());
        assert!(parse_port_range("3000-2999").is_err()); // start > end
    }

    #[test]
    fn test_common_dev_ports() {
        assert!(COMMON_DEV_PORTS.contains(&3000));
        assert!(COMMON_DEV_PORTS.contains(&8080));
        assert!(COMMON_DEV_PORTS.contains(&5000));
        assert!(!COMMON_DEV_PORTS.contains(&22)); // SSH port shouldn't be included
    }

    #[test]
    fn test_port_scanner_creation() {
        let _scanner = PortScanner::new();
        // Just verify we can create a scanner without panicking
        assert!(true);
    }
}
