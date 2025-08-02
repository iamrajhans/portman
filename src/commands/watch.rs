use crate::config::load_or_create_config;
use crate::output::{display_error, display_info, display_success, display_warning};
use crate::scanner::PortScanner;
use anyhow::Result;
use std::collections::HashSet;
use tokio::time::{interval, Duration};

pub async fn execute(config_path: Option<String>) -> Result<()> {
    let (config, config_file_path) = match load_or_create_config(config_path) {
        Ok(result) => result,
        Err(e) => {
            display_error(&format!("Failed to load config: {e}"));
            return Ok(());
        }
    };

    display_info(&format!(
        "Using config file: {path}",
        path = config_file_path.display()
    ));

    if let Some(project) = &config.project {
        display_info(&format!("Watching project: {project}"));
    }

    display_info(&format!(
        "Monitoring {len} ports: {ports:?}",
        len = config.ports.len(),
        ports = config.ports
    ));

    let watch_interval = config.watch_interval.unwrap_or(5);
    display_info(&format!("Check interval: {watch_interval}s"));
    display_info("Press Ctrl+C to stop watching");

    println!();

    let mut scanner = PortScanner::new();
    let mut interval_timer = interval(Duration::from_secs(watch_interval));
    let mut previously_occupied: HashSet<u16> = HashSet::new();
    let mut first_check = true;

    loop {
        interval_timer.tick().await;

        match scanner.scan_all_ports() {
            Ok(all_ports) => {
                let currently_occupied: HashSet<u16> = all_ports
                    .iter()
                    .filter(|port_info| config.ports.contains(&port_info.port))
                    .map(|port_info| port_info.port)
                    .collect();

                if first_check {
                    // Initial status
                    if currently_occupied.is_empty() {
                        display_success("All monitored ports are available");
                    } else {
                        let port_list = format_port_list(&currently_occupied);
                        display_warning(&format!("Currently occupied ports: {port_list}"));

                        // Show details for occupied ports
                        for port_info in &all_ports {
                            if config.ports.contains(&port_info.port) {
                                display_info(&format!(
                                    "  Port {port}: {name} (PID: {pid})",
                                    port = port_info.port,
                                    name = port_info.process_name,
                                    pid = port_info.pid
                                ));
                            }
                        }
                    }
                    first_check = false;
                } else {
                    // Check for changes
                    let newly_occupied: HashSet<_> = currently_occupied
                        .difference(&previously_occupied)
                        .collect();

                    let newly_freed: HashSet<_> = previously_occupied
                        .difference(&currently_occupied)
                        .collect();

                    for &&port in &newly_occupied {
                        if let Some(port_info) = all_ports.iter().find(|p| p.port == port) {
                            display_warning(&format!(
                                "Port {port} became occupied: {name} (PID: {pid})",
                                name = port_info.process_name,
                                pid = port_info.pid
                            ));
                        }
                    }

                    for &&port in &newly_freed {
                        display_success(&format!("Port {port} became available"));
                    }
                }

                previously_occupied = currently_occupied;
            }
            Err(e) => {
                display_error(&format!("Failed to scan ports: {e}"));
            }
        }
    }
}

fn format_port_list(ports: &HashSet<u16>) -> String {
    let mut sorted_ports: Vec<_> = ports.iter().collect();
    sorted_ports.sort();
    sorted_ports
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
