use crate::cli::OutputFormat;
use crate::output::{display_error, display_ports};
use crate::scanner::{parse_port_range, PortScanner, COMMON_DEV_PORTS};
use anyhow::Result;

pub async fn execute(
    range: Option<String>,
    filter: Option<String>,
    common: bool,
    format: OutputFormat,
) -> Result<()> {
    let mut scanner = PortScanner::new();

    match scanner.scan_all_ports() {
        Ok(mut ports) => {
            // Apply filters
            if common {
                ports.retain(|port_info| COMMON_DEV_PORTS.contains(&port_info.port));
            }

            if let Some(range_str) = range {
                match parse_port_range(&range_str) {
                    Ok((start, end)) => {
                        ports.retain(|port_info| port_info.port >= start && port_info.port <= end);
                    }
                    Err(e) => {
                        display_error(&format!("Invalid range: {e}"));
                        return Ok(());
                    }
                }
            }

            if let Some(filter_str) = filter {
                let filter_lower = filter_str.to_lowercase();
                ports.retain(|port_info| {
                    port_info
                        .process_name
                        .to_lowercase()
                        .contains(&filter_lower)
                        || port_info.command.to_lowercase().contains(&filter_lower)
                });
            }

            // Sort by port number
            ports.sort_by_key(|port_info| port_info.port);

            display_ports(&ports, &format);
        }
        Err(e) => {
            display_error(&format!("Failed to scan ports: {e}"));
        }
    }

    Ok(())
}
