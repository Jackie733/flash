# Flash - File Compression & Upload Tool

A command-line tool for compressing files/folders and uploading them to remote servers via SFTP.

## Features

- 🗜️ **File & Folder Compression**: Compress individual files or entire directories into ZIP format
- 🚀 **SFTP Upload**: Secure file transfer to remote servers with progress bars
- ⚙️ **Configuration File Support**: Save multiple server configurations for easy reuse
- 🔒 **Secure Input**: Safe password input without echo
- 📊 **Progress Tracking**: Visual progress bars for upload operations
- 🌐 **IP Validation**: Built-in validation for IPv4 and IPv6 addresses
- 🔍 **Comprehensive Logging**: Detailed logging with configurable levels
- 🎯 **Interactive Server Selection**: Choose from configured servers or input manually

## Installation

### Option 1: Install from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Jackie733/flash.git
cd flash

# Build for multiple platforms
./build.sh

# Install locally
./install.sh
```

### Option 2: Build Manually

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/Jackie733/flash.git
cd flash
cargo build --release

# The binary will be available at target/release/flash
```

### Option 3: Direct Cargo Install

```bash
cargo install --path .
```

## Usage

### Configuration Setup (Recommended)

First, initialize a configuration file to save your server settings:

```bash
# Create configuration file
flash --init-config
```

This creates a config file at `~/.config/flash/config.toml` (or `./flash.toml` in current directory) with example servers:

```toml
# Flash configuration file
[servers.home]
name = "Home Server"
ip = "192.168.1.100"
username = "admin"
port = 22
remote_path = "/home/admin"

[servers.work]
name = "Work Server"
ip = "10.0.0.5"
username = "deploy"
port = 2222
remote_path = "/opt/uploads"

[default]
server = "home"  # Default server to use
```

### Basic Usage

```bash
# Using configured servers
flash --path /path/to/file.txt --server work
flash --path /path/to/file.txt --server home

# Interactive server selection (will show available servers)
flash --path /path/to/file.txt

# Manual input (traditional way, still supported)
flash --path /path/to/file.txt --ip 192.168.1.100 --username user --password pass

# Compress and upload a directory
flash --path /path/to/directory --server work
```

### Command Line Options

```text
USAGE:
    flash [OPTIONS]

OPTIONS:
    -p, --path <PATH>         Path to file or directory to compress and upload
        --ip <IP>            Server IP address (IPv4 or IPv6)
        --username <USERNAME> SSH username
        --password <PASSWORD> SSH password (will prompt if not provided)
        --server <SERVER>     Use a configured server from config file
        --init-config         Create example configuration file
    -h, --help               Print help information
    -V, --version            Print version information
```

### Environment Variables

```bash
# Enable debug logging
export RUST_LOG=debug
flash --path /path/to/file.txt
```

## Examples

### Example 1: First Time Setup

```bash
# Initialize configuration
flash --init-config

# Edit the config file to add your servers
# File location: ~/.config/flash/config.toml

# Use configured server
flash --path ~/documents/report.pdf --server home
```

### Example 2: Using Different Servers

```bash
# Use work server
flash --path ~/projects/my-app --server work

# Use home server with interactive confirmation
flash --path ~/backup.tar
```

### Example 3: Traditional Manual Input

```bash
# Manual server input (no config needed)
flash --path ~/documents/report.pdf --ip 192.168.1.100 --username admin
# Will prompt for password securely

# Full manual specification
flash --path ~/projects/my-app --ip 10.0.0.5 --username deploy --password mypass
```

### Example 4: Interactive Mode

````bash
flash --path ~/backup.tar
# Will show:
# 1. Available configured servers to choose from
# 2. Option to input server details manually
# 3. Secure password prompt
```## Configuration

### Configuration File Locations

Flash looks for configuration files in the following order:
1. `./flash.toml` (current directory)
2. `~/.config/flash/config.toml` (user config directory)

### Configuration Format

```toml
# Multiple server configurations
[servers.server_name]
name = "Display Name"
ip = "server.ip.address"
username = "your_username"
port = 22                    # Optional, defaults to 22
remote_path = "/upload/path" # Optional, defaults to /home/username

# Default server selection
[default]
server = "server_name"  # Optional, which server to use by default
````

### Configuration Management

```bash
# Create initial config file with examples
flash --init-config

# Use a specific configured server
flash --path file.txt --server production

# List available options (interactive mode)
flash --path file.txt
```

## Supported Platforms

- **Linux**: x86_64
- **macOS**: x86_64 (Intel) and aarch64 (Apple Silicon)
- **Windows**: x86_64

## Requirements

- SSH/SFTP server on the target machine
- Network connectivity to the remote server
- Sufficient disk space for temporary ZIP files

## Development

### Running Tests

```bash
cargo test
```

### Building for Specific Platform

```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-gnu
```

### Project Structure

```text
flash/
├── src/
│   ├── main.rs      # Entry point and CLI handling
│   ├── lib.rs       # Library exports
│   ├── config.rs    # Configuration file handling
│   ├── compress.rs  # Compression functionality
│   ├── upload.rs    # SFTP upload functionality
│   └── input.rs     # User input utilities
├── tests/           # Integration tests
├── build.sh         # Cross-platform build script
└── install.sh       # Installation script
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite: `cargo test`
6. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Troubleshooting

### Common Issues

**Connection refused**: Ensure the SSH/SFTP server is running on the target machine.

**Permission denied**: Check that the username and password are correct and the user has write permissions.

**Invalid IP address**: Use the format `192.168.1.100` for IPv4 or `::1` for IPv6.

**File not found**: Ensure the path exists and is accessible.

**Server not found**: Check that the server name exists in your config file using `flash --init-config` to create/view the config.

**Config file not found**: Run `flash --init-config` to create an example configuration file, then edit it with your server details.

### Debug Mode

Enable verbose logging:

```bash
export RUST_LOG=debug
flash --path /your/file
```
