# File Shredder

A secure file deletion utility written in Rust that permanently erases files and directories by overwriting them with random data before deletion.

## Overview

File Shredder is designed to securely delete sensitive files and directories by implementing a multi-pass overwrite approach that makes data recovery virtually impossible. Unlike standard file deletion which only removes file references from the file system, File Shredder overwrites the actual data multiple times with random patterns before removing the file.

## Features

- **Secure File Deletion**: Overwrites files with random data before deletion
- **Multi-pass Overwriting**: Configurable number of overwrite passes (default: 5)
- **Directory Support**: Recursively shreds all files in directories and subdirectories
- **Multi-threaded**: Parallel processing for faster operation on large files
- **Progress Tracking**: Shows real-time progress during the shredding process
- **Cross-platform**: Works on Windows, macOS, and Linux

## Installation

### Pre-built Binaries

Download the latest pre-built binary for your platform from the [Releases](https://github.com/rohandhamapurkar/file-shredder/releases) page.

Available platforms:
- Windows (x64)
- macOS (Intel and Apple Silicon)
- Linux (x64)

### From Source

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/rohandhamapurkar/file-shredder.git
   cd file-shredder
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/file-shredder`

### From Cargo

```bash
cargo install file-shredder
```

## Usage

### Basic Usage

To shred a file:
```bash
file-shredder /path/to/file
```

To shred a directory (including all subdirectories and files):
```bash
file-shredder /path/to/directory
```

### Advanced Options

Specify the number of overwrite passes (higher = more secure but slower):
```bash
file-shredder /path/to/file 7
```

## How It Works

File Shredder operates in three main steps:

1. **Analysis**: Scans the target file or recursively analyzes directory structure
2. **Overwriting**: For each file:
   - Divides the file into chunks for multi-threaded processing
   - Performs multiple passes, each time overwriting the entire file with random data
   - Uses cryptographically strong random number generation
3. **Deletion**: After overwriting, permanently removes the file or directory

For directories, files are processed first, then empty directories are removed in reverse order.

## Security Considerations

- **Default Security**: The default 5 passes is sufficient for most use cases
- **High Security**: For highly sensitive data, consider using 7-10 passes
- **Physical Media**: Different storage media may retain data differently:
  - SSDs with TRIM may require fewer passes
  - HDDs may benefit from more passes for maximum security

## Performance

Performance depends on file size, number of passes, and hardware:

- **Small Files**: Processing overhead dominates
- **Large Files**: I/O speed is the main bottleneck
- **Multi-threading**: Significantly improves performance on multi-core systems and large files

## Limitations

- Cannot securely delete system or locked files
- Cannot guarantee complete data destruction on some storage types (e.g., SSDs with wear-leveling)
- Read-only files must have permissions changed before shredding

## Development

### Prerequisites

- Rust 1.21 or newer
- Cargo package manager

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

### CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

- Automatically builds binaries for multiple platforms (Windows, macOS, Linux)
- Creates releases when new version tags are pushed
- Supports both x64 and ARM64 architectures (Apple Silicon)

To create a new release:
1. Tag the commit with a version (e.g., `git tag v1.0.0`)
2. Push the tag to GitHub (`git push origin v1.0.0`)
3. The workflow will automatically build binaries and create a release

## License

```
MIT License

Copyright (c) 2025 Rohan Dhamapurkar

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Warning

**USE AT YOUR OWN RISK**: Files deleted with this tool cannot be recovered. Always verify you're targeting the correct files before executing.
