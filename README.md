# 🔧 Portman

[![CI](https://github.com/yourusername/portman/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/portman/actions/workflows/ci.yml)
[![Release](https://github.com/yourusername/portman/actions/workflows/release.yml/badge.svg)](https://github.com/yourusername/portman/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful CLI tool for managing ports and processes on your system. Solve the common "port already in use" problem with clear visibility and easy management of occupied ports.

![Portman Demo](https://github.com/yourusername/portman/raw/main/demo.gif)

## ✨ Features

- 🔍 **List occupied ports** with detailed process information
- ✅ **Check port availability** for scripts and automation  
- ⚡ **Kill processes** on specific ports with safety checks
- 🎯 **Filter by process name** or port ranges
- 📊 **Multiple output formats** (table, JSON, CSV)
- 🚀 **Cross-platform** support (Linux, macOS, Windows)
- 🎨 **Beautiful terminal output** with colors and proper alignment
- ⚙️ **Configuration files** for project-specific port monitoring
- 🛡️ **Safety features** to protect system-critical processes

## 🚀 Quick Start

### Installation

#### Quick Install (Linux/macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/yourusername/portman/main/install.sh | sh
```

#### Manual Installation
1. Download the latest release for your platform from [GitHub Releases](https://github.com/yourusername/portman/releases)
2. Extract the archive
3. Move the binary to your PATH (e.g., `/usr/local/bin/portman`)
4. Make it executable: `chmod +x portman`

#### Build from Source
```bash
git clone https://github.com/yourusername/portman.git
cd portman
cargo build --release
sudo cp target/release/portman /usr/local/bin/
```

### Usage Examples

```bash
# List all occupied ports
portman list

# Check if port is available
portman check 3000

# Check multiple ports or ranges
portman check 3000 3001 8080-8090

# Kill process on specific port
portman kill 3000

# Kill multiple processes (with confirmation)
portman kill 3000 3001 8080

# Force kill without confirmation
portman kill 3000 --force

# List only common development ports
portman list --common

# Filter by process name
portman list --filter node

# Output as JSON for scripting
portman list --format json

# Kill all processes on common dev ports
portman free --common

# Initialize project configuration
portman init

# Monitor ports from config file
portman watch
```

## 📋 Command Reference

### `portman list`
Display all occupied ports with process information.

**Options:**
- `--range START-END` - Show only ports in specified range (e.g., `3000-9000`)
- `--filter PROCESS` - Filter by process name (e.g., `node`, `java`)
- `--common` - Show only common development ports
- `--format FORMAT` - Output format: `table` (default), `json`, `csv`

**Example:**
```bash
┌──────┬───────┬──────────────────────┬───────────────────────────────────────────────┬──────────┬─────────┐
│ Port │   PID │ Process              │ Command                                       │ Duration │  Memory │
├──────┼───────┼──────────────────────┼───────────────────────────────────────────────┼──────────┼─────────┤
│ 3000 │ 55606 │ node                 │ node server.js                               │ 34m 19s  │ 58.0 MB │
├──────┼───────┼──────────────────────┼───────────────────────────────────────────────┼──────────┼─────────┤
│ 5432 │  1234 │ postgres             │ postgres -D /usr/local/var/postgres          │   2d 5h  │ 125.3MB │
└──────┴───────┴──────────────────────┴───────────────────────────────────────────────┴──────────┴─────────┘

📊 2 ports found
```

### `portman check`
Check if ports are available. Useful for scripts and automation.

**Exit codes:**
- `0` - All ports are available
- `1` - Some ports are occupied

**Examples:**
```bash
# Single port
portman check 3000 && echo "Port is free"

# Multiple ports
portman check 3000 3001 5432

# Port range
portman check 8000-8010
```

### `portman kill`
Terminate processes using specified ports.

**Options:**
- `--force, -f` - Kill without confirmation prompt

**Examples:**
```bash
# Kill with confirmation
portman kill 3000

# Force kill without prompt
portman kill 3000 --force

# Kill multiple processes
portman kill 3000 3001 8080
```

### `portman free`
Kill processes on common development ports.

**Options:**
- `--common` - Target common development ports only
- `--force, -f` - Kill without confirmation

**Example:**
```bash
portman free --common --force
```

### `portman init`
Create a `.portman.yaml` configuration file in the current directory.

**Options:**
- `--force` - Overwrite existing config file

### `portman watch`
Monitor ports defined in configuration file and alert when they become unavailable.

**Options:**
- `--config, -c` - Specify config file path

## ⚙️ Configuration

Create a `.portman.yaml` file in your project directory:

```yaml
project: my-awesome-app
ports:
  - 3000  # frontend
  - 3001  # backend API  
  - 5432  # postgres
  - 6379  # redis
watch_interval: 5  # seconds
```

Then run `portman watch` to monitor these ports.

## 🎯 Common Development Ports

Portman recognizes these common development ports for the `--common` flag:

- **3000-3003** - React, Next.js, Node.js development servers
- **4000, 4200** - Angular, development servers
- **5000, 5001, 5173** - Flask, Vite
- **8000, 8001, 8080, 8081, 8888** - Django, Java, Jupyter
- **9000, 9001, 9090** - Various development servers

## 🛡️ Safety Features

- **System Process Protection** - Won't kill critical system processes
- **Confirmation Prompts** - Asks before killing processes (unless `--force`)
- **Clear Process Information** - Shows exactly what will be killed
- **Graceful Termination** - Attempts SIGTERM before SIGKILL on Unix systems

## 🔧 Development

### Prerequisites
- Rust 1.70+ 
- Cargo

### Building
```bash
git clone https://github.com/yourusername/portman.git
cd portman
cargo build
```

### Testing
```bash
cargo test
cargo clippy
cargo fmt
```

### Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📖 Platform Support

- ✅ **Linux** - Full support using `netstat` and `/proc`
- ✅ **macOS** - Full support using `lsof` 
- ✅ **Windows** - Full support using `netstat`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- CLI parsing with [clap](https://github.com/clap-rs/clap)
- Table formatting with [tabled](https://github.com/zhiburt/tabled)
- Cross-platform system info with [sysinfo](https://github.com/GuillaumeGomez/sysinfo)

---

<div align="center">

**⭐ Star this repo if you find it useful! ⭐**

[Report Bug](https://github.com/yourusername/portman/issues) · [Request Feature](https://github.com/yourusername/portman/issues) · [Documentation](https://github.com/yourusername/portman/wiki)

</div>