use crate::cli::OutputFormat;
use crate::scanner::PortInfo;
use colored::*;
use serde_json::json;
use tabled::{
    settings::{object::Columns, Alignment, Modify, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct PortTableRow {
    #[tabled(rename = "Port")]
    port: String,
    #[tabled(rename = "PID")]
    pid: String,
    #[tabled(rename = "Process")]
    process: String,
    #[tabled(rename = "Command")]
    command: String,
    #[tabled(rename = "Duration")]
    duration: String,
    #[tabled(rename = "Memory")]
    memory: String,
}

pub fn display_ports(ports: &[PortInfo], format: &OutputFormat) {
    match format {
        OutputFormat::Table => display_table(ports),
        OutputFormat::Json => display_json(ports),
        OutputFormat::Csv => display_csv(ports),
    }
}

fn display_table(ports: &[PortInfo]) {
    if ports.is_empty() {
        println!("{}", "No occupied ports found.".yellow());
        return;
    }

    // Calculate dynamic column widths based on content
    let process_width = ports
        .iter()
        .map(|p| p.process_name.len())
        .max()
        .unwrap_or(7)
        .clamp(7, 20);
    let command_width = 45; // Increased width for better readability

    let rows: Vec<PortTableRow> = ports
        .iter()
        .map(|port_info| PortTableRow {
            port: port_info.port.to_string(),
            pid: port_info.pid.to_string(),
            process: truncate_string(&port_info.process_name, process_width),
            command: truncate_command(&port_info.command, command_width),
            duration: format_duration(port_info.start_time),
            memory: format_memory(port_info.memory_usage),
        })
        .collect();

    let mut table = Table::new(rows);
    table
        .with(Style::modern())
        .with(Modify::new(Columns::single(0)).with(Alignment::right()))
        .with(Modify::new(Columns::single(1)).with(Alignment::right()))
        .with(Modify::new(Columns::single(2)).with(Alignment::left()))
        .with(Modify::new(Columns::single(3)).with(Alignment::left()))
        .with(Modify::new(Columns::single(4)).with(Alignment::center()))
        .with(Modify::new(Columns::single(5)).with(Alignment::right()));

    println!("{}", table);

    // Add colored summary line
    println!(
        "\n{} {} ports found",
        "📊".bold(),
        ports.len().to_string().cyan().bold()
    );
}

fn display_json(ports: &[PortInfo]) {
    let json_data = ports
        .iter()
        .map(|port_info| {
            json!({
                "port": port_info.port,
                "pid": port_info.pid,
                "process_name": port_info.process_name,
                "command": port_info.command,
                "start_time": port_info.start_time,
                "memory_usage": port_info.memory_usage
            })
        })
        .collect::<Vec<_>>();

    println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
}

fn display_csv(ports: &[PortInfo]) {
    println!("Port,PID,Process,Command,StartTime,MemoryUsage");
    for port_info in ports {
        println!(
            "{},{},{},\"{}\",{},{}",
            port_info.port,
            port_info.pid,
            port_info.process_name,
            port_info.command.replace('"', "\"\""), // Escape quotes in CSV
            port_info.start_time,
            port_info.memory_usage
        );
    }
}

#[allow(dead_code)]
fn format_port_simple(port: u16) -> String {
    port.to_string().cyan().bold().to_string()
}

#[allow(dead_code)]
fn format_process_simple(process_name: &str, max_length: usize) -> String {
    let truncated = truncate_string(process_name, max_length);

    // Color-code common development processes
    match process_name.to_lowercase().as_str() {
        name if name.contains("node") => truncated.green().to_string(),
        name if name.contains("python") => truncated.yellow().to_string(),
        name if name.contains("java") => truncated.red().to_string(),
        name if name.contains("nginx") => truncated.purple().to_string(),
        name if name.contains("postgres") => truncated.blue().to_string(),
        name if name.contains("redis") => truncated.bright_red().to_string(),
        name if name.contains("docker") => truncated.bright_blue().to_string(),
        _ => truncated,
    }
}

// Keep old functions for potential future use
#[allow(dead_code)]
fn format_port(port: u16) -> String {
    format!("{}", port.to_string().cyan().bold())
}

#[allow(dead_code)]
fn format_pid(pid: u32) -> String {
    format!("{}", pid.to_string().blue())
}

#[allow(dead_code)]
fn format_process(process_name: &str) -> String {
    // Color-code common development processes
    match process_name.to_lowercase().as_str() {
        name if name.contains("node") => name.green().to_string(),
        name if name.contains("python") => name.yellow().to_string(),
        name if name.contains("java") => name.red().to_string(),
        name if name.contains("nginx") => name.purple().to_string(),
        name if name.contains("postgres") => name.blue().to_string(),
        name if name.contains("redis") => name.bright_red().to_string(),
        name if name.contains("docker") => name.bright_blue().to_string(),
        _ => process_name.to_string(),
    }
}

fn truncate_command(command: &str, max_length: usize) -> String {
    if command.len() <= max_length {
        command.to_string()
    } else {
        format!("{}...", &command[..max_length.saturating_sub(3)])
    }
}

fn truncate_string(input: &str, max_length: usize) -> String {
    if input.len() <= max_length {
        input.to_string()
    } else {
        format!("{}...", &input[..max_length.saturating_sub(3)])
    }
}

fn format_duration(start_time: u64) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let runtime_seconds = now.saturating_sub(start_time);
    crate::process::format_duration(runtime_seconds)
}

fn format_memory(bytes: u64) -> String {
    crate::process::format_memory(bytes)
}

pub fn display_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn display_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message.red());
}

pub fn display_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message.yellow());
}

pub fn display_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

pub fn confirm_action(message: &str) -> bool {
    print!("{} [y/N]: ", message);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
