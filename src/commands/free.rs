use crate::commands::kill;
use crate::output::{display_error, display_info};
use crate::scanner::{PortScanner, COMMON_DEV_PORTS};
use anyhow::Result;

pub async fn execute(common: bool, force: bool) -> Result<()> {
    if !common {
        display_error("The 'free' command currently only supports --common flag");
        display_info("Usage: portman free --common [--force]");
        return Ok(());
    }

    let mut scanner = PortScanner::new();

    // Find which common dev ports are occupied
    let occupied_ports: Vec<u16> = match scanner.scan_all_ports() {
        Ok(all_ports) => all_ports
            .into_iter()
            .filter(|port_info| COMMON_DEV_PORTS.contains(&port_info.port))
            .map(|port_info| port_info.port)
            .collect(),
        Err(e) => {
            display_error(&format!("Failed to scan ports: {}", e));
            return Ok(());
        }
    };

    if occupied_ports.is_empty() {
        display_info("No processes found on common development ports");
        return Ok(());
    }

    display_info(&format!(
        "Found processes on {} common development ports: {}",
        occupied_ports.len(),
        occupied_ports
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    ));

    // Use the kill command to handle the actual killing
    kill::execute(occupied_ports, force).await
}
