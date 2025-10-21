# Wifite3

Modern WiFi penetration testing tool with Python + Rust integration.

## ⚠️ DISCLAIMER

**This tool is for educational and authorized penetration testing purposes only.**

- **Legal Use Only**: Only use on networks you own or have explicit written permission to test
- **Ethical Guidelines**: Follow responsible disclosure practices and respect privacy
- **No Malicious Use**: Do not use for unauthorized access, data theft, or any illegal activities
- **Compliance**: Ensure compliance with local laws and regulations regarding network security testing
- **Responsibility**: Users are solely responsible for their actions and any legal consequences

**By using this tool, you agree to use it ethically and legally.**

## Features

- **Network Scanning**: Discover WiFi networks with encryption details
- **PMKID Capture**: Extract PMKID hashes from EAPOL frames
- **Rust Performance**: High-performance packet processing with PyO3 bindings
- **Rich CLI**: Beautiful console output with progress tracking

## Installation

```bash
# Install with uv
uv sync

# Or install in development mode
uv pip install -e .
```

## Usage

```bash
# Scan for networks
wifite3 --scan

# Capture PMKID hashes
wifite3 --pmkid --duration 60

# Specify interface
wifite3 -i wlan0 --scan
```

## Development

```bash
# Run tests
make test

# Format code
make fmt

# Lint code
make lint
```

## Requirements

- Python 3.12+
- Rust toolchain
- Network interface with monitor mode support
