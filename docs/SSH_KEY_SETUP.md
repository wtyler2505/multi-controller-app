# SSH Key Setup and Usage Guide

## Overview
The Multi-Controller App supports SSH key-based authentication for secure remote connections. This guide covers how to set up and use SSH keys with the transport layer.

## Features
- **Automatic key discovery** - Finds SSH keys in standard locations
- **Multiple key formats** - Supports Ed25519, RSA, ECDSA, and DSA keys
- **Password fallback** - Uses password authentication when no key is available
- **Key preference** - Automatically selects the best available key (Ed25519 > RSA > ECDSA > DSA)
- **Encrypted key support** - Handles encrypted private keys with passphrase

## SSH Key Generation

### Generate Ed25519 Key (Recommended)
```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
```

### Generate RSA Key
```bash
ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
```

## Key Locations

### Standard Locations (Auto-discovered)
- **Windows**: `%USERPROFILE%\.ssh\`
- **Linux/Mac**: `~/.ssh/`

### Supported Key Names
- `id_ed25519` (preferred)
- `id_rsa`
- `id_ecdsa`
- `id_dsa`

## Configuration

### Using Default Key Discovery
```rust
use multi_controller_app::transport::{TransportConfig, TransportType};
use multi_controller_app::transport::common::{TransportSettings, SshSettings};

let config = TransportConfig {
    transport_type: TransportType::Ssh,
    address: "192.168.1.100".to_string(),
    settings: TransportSettings::Ssh(SshSettings {
        username: "pi".to_string(),
        key_path: None,  // Auto-discover best key
        password: None,
        port: 22,
        ..Default::default()
    }),
    ..Default::default()
};
```

### Specifying a Custom Key Path
```rust
let config = TransportConfig {
    transport_type: TransportType::Ssh,
    address: "192.168.1.100".to_string(),
    settings: TransportSettings::Ssh(SshSettings {
        username: "pi".to_string(),
        key_path: Some("/path/to/custom/key".to_string()),
        password: None,
        port: 22,
        ..Default::default()
    }),
    ..Default::default()
};
```

### Using Password Authentication
```rust
let config = TransportConfig {
    transport_type: TransportType::Ssh,
    address: "192.168.1.100".to_string(),
    settings: TransportSettings::Ssh(SshSettings {
        username: "pi".to_string(),
        key_path: None,
        password: Some("secure_password".to_string()),
        port: 22,
        ..Default::default()
    }),
    ..Default::default()
};
```

### Encrypted Key with Passphrase
```rust
let config = TransportConfig {
    transport_type: TransportType::Ssh,
    address: "192.168.1.100".to_string(),
    settings: TransportSettings::Ssh(SshSettings {
        username: "pi".to_string(),
        key_path: Some("/home/user/.ssh/id_ed25519".to_string()),
        password: None,
        key_passphrase: Some("key_passphrase".to_string()),
        port: 22,
        ..Default::default()
    }),
    ..Default::default()
};
```

## Authentication Priority

1. **Specified SSH key** - If `key_path` is provided, use that key
2. **Auto-discovered key** - If no `key_path`, search for best available key
3. **Password authentication** - If no key found/available, use password
4. **Connection failure** - If no authentication method available

## Security Considerations

### Key Permissions (Linux/Mac)
SSH keys must have correct permissions:
```bash
chmod 600 ~/.ssh/id_*
chmod 644 ~/.ssh/id_*.pub
```

### Personal Project Settings
For personal projects, the following relaxed settings are default:
- `strict_host_key_checking: false` - Skip host key verification
- `known_hosts_path: None` - Don't maintain known_hosts file

For production use, consider enabling:
```rust
SshSettings {
    strict_host_key_checking: true,
    known_hosts_path: Some("/home/user/.ssh/known_hosts".to_string()),
    ..
}
```

## Troubleshooting

### Key Not Found
- Verify key exists in standard location or specified path
- Check file permissions (especially on Linux/Mac)
- Enable debug logging to see key discovery process

### Authentication Failed
- Verify username is correct
- Check if key is accepted on remote server (`~/.ssh/authorized_keys`)
- Try password authentication as fallback
- Check if key needs passphrase

### Connection Issues
- Verify SSH service is running on remote host
- Check firewall settings for port 22
- Test with standard SSH client first: `ssh user@host`

## Example Usage

```rust
use multi_controller_app::transport::ssh::SshTransport;

// Create SSH transport with auto-discovery
let mut transport = SshTransport::new(config)?;

// Connect (will use best available authentication)
transport.connect().await?;

// Send commands
transport.send(b"echo Hello\n").await?;

// Receive response
let response = transport.receive(Duration::from_secs(5)).await?;

// Disconnect
transport.disconnect().await?;
```

## Logging

Enable debug logging to see SSH key discovery and authentication details:
```rust
tracing_subscriber::fmt()
    .with_env_filter("multi_controller_app::transport::ssh=debug")
    .init();
```

This will show:
- Key discovery process
- Selected authentication method
- Connection attempts and results