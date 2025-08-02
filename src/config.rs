use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortmanConfig {
    pub project: Option<String>,
    pub ports: Vec<u16>,
    pub description: Option<String>,
    pub watch_interval: Option<u64>, // seconds
}

impl Default for PortmanConfig {
    fn default() -> Self {
        Self {
            project: None,
            ports: vec![3000, 3001, 5432, 6379], // Common defaults
            description: None,
            watch_interval: Some(5), // 5 seconds
        }
    }
}

impl PortmanConfig {
    /// Load configuration from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        let extension = path
            .as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("yaml");

        match extension {
            "yaml" | "yml" => {
                serde_yaml::from_str(&content).with_context(|| "Failed to parse YAML config file")
            }
            "toml" => toml::from_str(&content).with_context(|| "Failed to parse TOML config file"),
            _ => Err(anyhow::anyhow!(
                "Unsupported config file format: {}",
                extension
            )),
        }
    }

    /// Save configuration to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let extension = path
            .as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("yaml");

        let content = match extension {
            "yaml" | "yml" => {
                serde_yaml::to_string(self).with_context(|| "Failed to serialize config to YAML")?
            }
            "toml" => toml::to_string_pretty(self)
                .with_context(|| "Failed to serialize config to TOML")?,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported config file format: {}",
                    extension
                ))
            }
        };

        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Find config file in current directory or parent directories
    pub fn find_config_file() -> Option<PathBuf> {
        let possible_names = [
            ".portman.yaml",
            ".portman.yml",
            ".portman.toml",
            "portman.yaml",
            "portman.yml",
            "portman.toml",
        ];

        let mut current_dir = std::env::current_dir().ok()?;

        loop {
            for name in &possible_names {
                let config_path = current_dir.join(name);
                if config_path.exists() {
                    return Some(config_path);
                }
            }

            if !current_dir.pop() {
                break;
            }
        }

        None
    }

    /// Create a default config file with smart defaults based on current directory
    pub fn create_default_config<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut config = Self::default();

        // Try to detect project type and set smart defaults
        if let Ok(current_dir) = std::env::current_dir() {
            if let Some(project_name) = current_dir.file_name().and_then(|n| n.to_str()) {
                config.project = Some(project_name.to_string());
            }

            // Detect common project files and adjust port defaults
            let common_files = [
                ("package.json", vec![3000, 3001, 8080]), // Node.js
                ("Cargo.toml", vec![8000, 8080, 3000]),   // Rust
                ("requirements.txt", vec![8000, 5000]),   // Python
                ("pom.xml", vec![8080, 8081, 9090]),      // Java Maven
                ("build.gradle", vec![8080, 8081, 9090]), // Java Gradle
                ("docker-compose.yml", vec![3000, 5432, 6379, 8080]), // Docker
            ];

            for (file, ports) in &common_files {
                if current_dir.join(file).exists() {
                    config.ports.clone_from(ports);
                    break;
                }
            }
        }

        config.save(&path)?;
        Ok(config)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.ports.is_empty() {
            return Err(anyhow::anyhow!(
                "Configuration must specify at least one port"
            ));
        }

        for &port in &self.ports {
            if port == 0 {
                return Err(anyhow::anyhow!(
                    "Invalid port number: {} (port 0 is reserved)",
                    port
                ));
            }
        }

        if let Some(interval) = self.watch_interval {
            if interval == 0 {
                return Err(anyhow::anyhow!("Watch interval must be greater than 0"));
            }
        }

        Ok(())
    }
}

/// Get the default config file path for the current directory
pub fn get_default_config_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".portman.yaml")
}

/// Load config from default locations or create if not exists
pub fn load_or_create_config(config_path: Option<String>) -> Result<(PortmanConfig, PathBuf)> {
    let path = if let Some(custom_path) = config_path {
        PathBuf::from(custom_path)
    } else if let Some(found_path) = PortmanConfig::find_config_file() {
        found_path
    } else {
        get_default_config_path()
    };

    if path.exists() {
        let config = PortmanConfig::load(&path)?;
        config.validate()?;
        Ok((config, path))
    } else {
        let config = PortmanConfig::create_default_config(&path)?;
        Ok((config, path))
    }
}
