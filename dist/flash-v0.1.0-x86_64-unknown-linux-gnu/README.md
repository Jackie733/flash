# Flash - File Compression & Upload Tool

A command-line tool for compressing files/folders and uploading them to remote servers via SFTP.

## Features

- 🗜️ **File & Folder Compression**: Compress individual files or entire directories into ZIP format
- 🚀 **SFTP Upload**: Secure file transfer to remote servers with progress bars
- 🔒 **Secure Input**: Safe password input without echo
- 📊 **Progress Tracking**: Visual progress bars for upload operations
- 🌐 **IP Validation**: Built-in validation for IPv4 and IPv6 addresses
- 🔍 **Comprehensive Logging**: Detailed logging with configurable levels

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

### Basic Usage

```bash
# Compress and upload a file
flash --path /path/to/file.txt --ip 192.168.1.100 --username user --password pass

# Compress and upload a directory
flash --path /path/to/directory --ip 192.168.1.100 --username user

# Interactive mode (prompts for missing information)
flash --path /path/to/file.txt
```

### Command Line Options

```
USAGE:
    flash [OPTIONS] --path <PATH>

OPTIONS:
    -p, --path <PATH>         Path to file or directory to compress and upload
        --ip <IP>            Server IP address (IPv4 or IPv6)
        --username <USERNAME> SSH username
        --password <PASSWORD> SSH password (will prompt if not provided)
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

### Example 1: Upload a File

```bash
flash --path ~/documents/report.pdf --ip 192.168.1.100 --username admin
# Will prompt for password securely
```

### Example 2: Upload a Directory

```bash
flash --path ~/projects/my-app --ip 10.0.0.5 --username deploy --password mypass
```

### Example 3: Interactive Mode

```bash
flash --path ~/backup.tar
# Will prompt for:
# - Server IP
# - Username
# - Password
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

```
flash/
├── src/
│   ├── main.rs      # Entry point and CLI handling
│   ├── lib.rs       # Library exports
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

### Debug Mode

Enable verbose logging:

```bash
export RUST_LOG=debug
flash --path /your/file
```
