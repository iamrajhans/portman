use crate::config::{get_default_config_path, PortmanConfig};
use crate::output::{confirm_action, display_error, display_success, display_warning};
use anyhow::Result;

pub async fn execute(force: bool) -> Result<()> {
    let config_path = get_default_config_path();

    // Check if config file already exists
    if config_path.exists() && !force {
        display_warning(&format!(
            "Config file already exists: {path}",
            path = config_path.display()
        ));

        if !confirm_action("Overwrite existing config file?") {
            display_error("Initialization cancelled");
            return Ok(());
        }
    }

    match PortmanConfig::create_default_config(&config_path) {
        Ok(config) => {
            display_success(&format!(
                "Created config file: {path}",
                path = config_path.display()
            ));

            println!("\nGenerated configuration:");
            if let Some(project) = &config.project {
                println!("  Project: {project}");
            }
            println!("  Ports: {:?}", config.ports);
            if let Some(interval) = config.watch_interval {
                println!("  Watch interval: {interval}s");
            }

            println!("\nYou can now:");
            println!("  • Edit the config file to customize your ports");
            println!("  • Run 'portman watch' to monitor these ports");
            println!("  • Use 'portman list' to see current port usage");
        }
        Err(e) => {
            display_error(&format!("Failed to create config file: {e}"));
        }
    }

    Ok(())
}
