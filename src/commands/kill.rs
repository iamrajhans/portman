use crate::output::{
    confirm_action, display_error, display_info, display_success, display_warning,
};
use crate::process::ProcessManager;
use crate::scanner::PortScanner;
use anyhow::Result;

pub async fn execute(ports: Vec<u16>, force: bool) -> Result<()> {
    if ports.is_empty() {
        display_error("No ports specified");
        return Ok(());
    }

    let mut scanner = PortScanner::new();
    let mut process_manager = ProcessManager::new();
    let mut successful_kills = Vec::new();
    let mut failed_kills = Vec::new();

    for port in ports {
        match scanner.get_port_info(port) {
            Ok(Some(port_info)) => {
                // Check if this is a system-critical process
                if process_manager.is_system_critical(&port_info.process_name) {
                    display_warning(&format!(
                        "Skipping system-critical process on port {}: {} (PID: {})",
                        port, port_info.process_name, port_info.pid
                    ));
                    failed_kills.push((
                        port,
                        format!("System-critical process: {}", port_info.process_name),
                    ));
                    continue;
                }

                // Show what will be killed
                display_info(&format!(
                    "Process to kill:\n  Port: {}\n  PID: {}\n  Process: {} ({})",
                    port,
                    port_info.pid,
                    port_info.process_name,
                    truncate_command(&port_info.command, 50)
                ));

                // Ask for confirmation unless force flag is used
                let should_kill = if force {
                    true
                } else {
                    confirm_action(&format!(
                        "Kill process {} on port {}?",
                        port_info.process_name, port
                    ))
                };

                if should_kill {
                    match process_manager.kill_process(port_info.pid) {
                        Ok(true) => {
                            display_success(&format!(
                                "Successfully killed process on port {} (PID: {})",
                                port, port_info.pid
                            ));
                            successful_kills.push(port);
                        }
                        Ok(false) => {
                            display_error(&format!(
                                "Failed to kill process on port {} (PID: {})",
                                port, port_info.pid
                            ));
                            failed_kills.push((port, "Kill command failed".to_string()));
                        }
                        Err(e) => {
                            display_error(&format!(
                                "Error killing process on port {} (PID: {}): {}",
                                port, port_info.pid, e
                            ));
                            failed_kills.push((port, e.to_string()));
                        }
                    }
                } else {
                    display_info("Skipped");
                    failed_kills.push((port, "User cancelled".to_string()));
                }
            }
            Ok(None) => {
                display_warning(&format!("No process found on port {}", port));
                failed_kills.push((port, "Port not in use".to_string()));
            }
            Err(e) => {
                display_error(&format!("Error checking port {}: {}", port, e));
                failed_kills.push((port, e.to_string()));
            }
        }
    }

    // Summary
    if !successful_kills.is_empty() || !failed_kills.is_empty() {
        println!();
        display_info("Summary:");

        if !successful_kills.is_empty() {
            display_success(&format!(
                "Successfully killed processes on {} port(s): {}",
                successful_kills.len(),
                successful_kills
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        if !failed_kills.is_empty() {
            display_error(&format!(
                "Failed to kill processes on {} port(s):",
                failed_kills.len()
            ));
            for (port, reason) in failed_kills {
                display_error(&format!("  Port {}: {}", port, reason));
            }
        }
    }

    Ok(())
}

fn truncate_command(command: &str, max_length: usize) -> String {
    if command.len() <= max_length {
        command.to_string()
    } else {
        format!("{}...", &command[..max_length.saturating_sub(3)])
    }
}
