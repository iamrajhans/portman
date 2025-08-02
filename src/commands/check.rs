use crate::output::{display_error, display_info, display_success};
use crate::scanner::{parse_port_range, PortScanner};
use anyhow::Result;
use std::collections::HashSet;

pub async fn execute(port_args: Vec<String>) -> Result<bool> {
    if port_args.is_empty() {
        display_error("No ports specified to check");
        return Ok(false);
    }

    let mut scanner = PortScanner::new();
    let mut all_available = true;
    let mut ports_to_check = Vec::new();

    // Parse port arguments (can be individual ports or ranges)
    for arg in port_args {
        if arg.contains('-') {
            match parse_port_range(&arg) {
                Ok((start, end)) => {
                    for port in start..=end {
                        ports_to_check.push(port);
                    }
                }
                Err(e) => {
                    display_error(&format!("Invalid range '{}': {}", arg, e));
                    return Ok(false);
                }
            }
        } else {
            match arg.parse::<u16>() {
                Ok(port) => ports_to_check.push(port),
                Err(_) => {
                    display_error(&format!("Invalid port number: {}", arg));
                    return Ok(false);
                }
            }
        }
    }

    // Remove duplicates and sort
    ports_to_check.sort_unstable();
    ports_to_check.dedup();

    // Check each port by looking for actual processes using them
    let occupied_ports = match scanner.scan_all_ports() {
        Ok(ports) => ports
            .into_iter()
            .map(|info| info.port)
            .collect::<HashSet<_>>(),
        Err(e) => {
            display_error(&format!("Failed to scan ports: {}", e));
            return Ok(false);
        }
    };

    // Check each port
    for port in &ports_to_check {
        if occupied_ports.contains(port) {
            display_error(&format!("Port {} is occupied", port));
            all_available = false;
        } else {
            display_success(&format!("Port {} is available", port));
        }
    }

    // Summary message
    if all_available {
        if ports_to_check.len() == 1 {
            display_info("Port is available");
        } else {
            display_info(&format!("All {} ports are available", ports_to_check.len()));
        }
    } else {
        display_info("Some ports are occupied");
    }

    Ok(all_available)
}
