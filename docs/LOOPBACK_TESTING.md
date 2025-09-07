# Hardware-in-the-Loop Testing Guide

This guide provides comprehensive instructions for setting up and running hardware-in-the-loop (HIL) testing for the Multi-Controller App transport layer. HIL testing validates real-world communication scenarios without requiring actual hardware devices.

## Table of Contents

1. [Overview](#overview)
2. [Virtual Serial Ports](#virtual-serial-ports)
3. [TCP/UDP Echo Servers](#tcpudp-echo-servers)
4. [SSH Test Server](#ssh-test-server)
5. [Hardware Loopback Testing](#hardware-loopback-testing)
6. [Running Test Suites](#running-test-suites)
7. [CI/CD Integration](#cicd-integration)
8. [Troubleshooting](#troubleshooting)

## Overview

The Multi-Controller App uses loopback testing to validate transport layer functionality across different communication protocols:

- **Serial**: RS-232/485 communication with device drivers
- **TCP**: Network-based device communication
- **UDP**: Broadcast and multicast device discovery
- **SSH**: Secure remote device management

All tests are located in `tests/loopback/` and use common utilities from `tests/loopback/common.rs`.

## Virtual Serial Ports

Virtual serial ports create software-based serial connections for testing without physical hardware.

### Windows Setup

#### Using com0com (Recommended)

1. **Download and Install com0com:**
   ```powershell
   # Download from: https://sourceforge.net/projects/com0com/
   # Install with Administrator privileges
   ```

2. **Create Virtual Port Pair:**
   ```powershell
   # Open Command Prompt as Administrator
   setupc install PortName=COM10 PortName=COM11
   ```

3. **Verify Installation:**
   ```powershell
   setupc list
   # Should show: COM10 <-> COM11
   ```

4. **Configure Test:**
   ```rust
   // In test configuration
   let config = SerialLoopbackConfig {
       port: "COM10".to_string(),  // Write port
       baud_rate: 115200,
       use_hardware_loopback: false,
   };
   ```

#### Using Virtual Serial Port Driver

1. **Install VSPD:**
   ```powershell
   # Download from Eltima Software
   # Create pair: COM99 <-> COM100
   ```

2. **Test Configuration:**
   ```toml
   # test-config.toml
   [serial]
   primary_port = "COM99"
   secondary_port = "COM100"
   baud_rate = 115200
   ```

### Linux Setup

#### Using socat

1. **Install socat:**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install socat
   
   # RHEL/CentOS
   sudo yum install socat
   ```

2. **Create Virtual Serial Port Pair:**
   ```bash
   # Create bidirectional pipe
   socat -d -d pty,raw,echo=0 pty,raw,echo=0
   # Output: PTY is /dev/pts/2, PTY is /dev/pts/3
   ```

3. **Create Persistent Links:**
   ```bash
   # Create symbolic links for consistent naming
   sudo ln -s /dev/pts/2 /dev/ttyVIRTUAL0
   sudo ln -s /dev/pts/3 /dev/ttyVIRTUAL1
   ```

4. **Test Configuration:**
   ```rust
   let config = SerialLoopbackConfig {
       port: "/dev/ttyVIRTUAL0".to_string(),
       baud_rate: 115200,
       use_hardware_loopback: false,
   };
   ```

#### Using tty0tty

1. **Install tty0tty:**
   ```bash
   git clone https://github.com/freemed/tty0tty.git
   cd tty0tty/module
   make
   sudo cp tty0tty.ko /lib/modules/$(uname -r)/kernel/drivers/misc/
   sudo depmod
   ```

2. **Load Module:**
   ```bash
   sudo modprobe tty0tty
   # Creates /dev/tnt0 <-> /dev/tnt1, /dev/tnt2 <-> /dev/tnt3, etc.
   ```

### macOS Setup

#### Using socat

1. **Install socat:**
   ```bash
   brew install socat
   ```

2. **Create Virtual Ports:**
   ```bash
   # Terminal 1
   socat -d -d pty,raw,echo=0 pty,raw,echo=0
   
   # Terminal 2 - Create persistent links
   ln -s /dev/ttys002 /dev/cu.virtual0
   ln -s /dev/ttys003 /dev/cu.virtual1
   ```

3. **Test Configuration:**
   ```rust
   let config = SerialLoopbackConfig {
       port: "/dev/cu.virtual0".to_string(),
       baud_rate: 115200,
       use_hardware_loopback: false,
   };
   ```

### Running Serial Loopback Tests

```bash
# Run all serial loopback tests
cargo test --test loopback -- serial_loopback --ignored

# Run specific serial test
cargo test --test loopback test_serial_basic_loopback --ignored

# Run with debug output
RUST_LOG=debug cargo test --test loopback test_serial_framing --ignored
```

## TCP/UDP Echo Servers

Echo servers reflect received data back to the sender, enabling round-trip communication testing.

### Python Echo Server (Cross-Platform)

Create `scripts/echo_server.py`:

```python
#!/usr/bin/env python3
import socket
import threading
import argparse
import sys
import time

class EchoServer:
    def __init__(self, host='127.0.0.1', tcp_port=8080, udp_port=8081):
        self.host = host
        self.tcp_port = tcp_port
        self.udp_port = udp_port
        self.running = False
        
    def start(self):
        self.running = True
        
        # Start TCP server
        tcp_thread = threading.Thread(target=self._tcp_server)
        tcp_thread.daemon = True
        tcp_thread.start()
        
        # Start UDP server  
        udp_thread = threading.Thread(target=self._udp_server)
        udp_thread.daemon = True
        udp_thread.start()
        
        print(f"Echo servers started:")
        print(f"  TCP: {self.host}:{self.tcp_port}")
        print(f"  UDP: {self.host}:{self.udp_port}")
        
        return tcp_thread, udp_thread
        
    def _tcp_server(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
            server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            server_socket.bind((self.host, self.tcp_port))
            server_socket.listen(5)
            server_socket.settimeout(1.0)
            
            while self.running:
                try:
                    client_socket, addr = server_socket.accept()
                    client_thread = threading.Thread(
                        target=self._handle_tcp_client, 
                        args=(client_socket, addr)
                    )
                    client_thread.daemon = True
                    client_thread.start()
                except socket.timeout:
                    continue
                    
    def _handle_tcp_client(self, client_socket, addr):
        print(f"TCP client connected: {addr}")
        try:
            with client_socket:
                while self.running:
                    data = client_socket.recv(1024)
                    if not data:
                        break
                    # Echo data back
                    client_socket.sendall(data)
                    print(f"TCP echoed {len(data)} bytes to {addr}")
        except Exception as e:
            print(f"TCP client {addr} error: {e}")
        finally:
            print(f"TCP client disconnected: {addr}")
            
    def _udp_server(self):
        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as server_socket:
            server_socket.bind((self.host, self.udp_port))
            server_socket.settimeout(1.0)
            
            while self.running:
                try:
                    data, addr = server_socket.recvfrom(1024)
                    # Echo data back
                    server_socket.sendto(data, addr)
                    print(f"UDP echoed {len(data)} bytes to {addr}")
                except socket.timeout:
                    continue
                except Exception as e:
                    print(f"UDP server error: {e}")
                    
    def stop(self):
        self.running = False

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="TCP/UDP Echo Server for testing")
    parser.add_argument("--host", default="127.0.0.1", help="Server host")
    parser.add_argument("--tcp-port", type=int, default=8080, help="TCP port")
    parser.add_argument("--udp-port", type=int, default=8081, help="UDP port")
    
    args = parser.parse_args()
    
    server = EchoServer(args.host, args.tcp_port, args.udp_port)
    tcp_thread, udp_thread = server.start()
    
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\nShutting down echo servers...")
        server.stop()
        sys.exit(0)
```

### PowerShell Echo Server (Windows)

Create `scripts/echo_server.ps1`:

```powershell
param(
    [string]$Host = "127.0.0.1",
    [int]$TcpPort = 8080,
    [int]$UdpPort = 8081
)

# TCP Echo Server
$TcpJob = Start-Job -ScriptBlock {
    param($Host, $Port)
    
    $listener = New-Object System.Net.Sockets.TcpListener([System.Net.IPAddress]::Parse($Host), $Port)
    $listener.Start()
    Write-Host "TCP Echo Server listening on ${Host}:${Port}"
    
    while ($true) {
        try {
            $client = $listener.AcceptTcpClient()
            $stream = $client.GetStream()
            
            $buffer = New-Object byte[] 1024
            while ($client.Connected) {
                $bytesRead = $stream.Read($buffer, 0, $buffer.Length)
                if ($bytesRead -eq 0) { break }
                
                # Echo data back
                $stream.Write($buffer, 0, $bytesRead)
                $stream.Flush()
                
                Write-Host "TCP echoed $bytesRead bytes"
            }
            
            $client.Close()
        }
        catch {
            Write-Warning "TCP server error: $_"
        }
    }
} -ArgumentList $Host, $TcpPort

# UDP Echo Server
$UdpJob = Start-Job -ScriptBlock {
    param($Host, $Port)
    
    $udpClient = New-Object System.Net.Sockets.UdpClient($Port)
    $endpoint = New-Object System.Net.IPEndPoint([System.Net.IPAddress]::Parse($Host), $Port)
    
    Write-Host "UDP Echo Server listening on ${Host}:${Port}"
    
    while ($true) {
        try {
            $result = $udpClient.Receive([ref]$endpoint)
            # Echo data back
            $udpClient.Send($result, $result.Length, $endpoint)
            Write-Host "UDP echoed $($result.Length) bytes to $endpoint"
        }
        catch {
            Write-Warning "UDP server error: $_"
        }
    }
} -ArgumentList $Host, $UdpPort

Write-Host "Echo servers started. Press Ctrl+C to stop."
Write-Host "TCP: ${Host}:${TcpPort}"
Write-Host "UDP: ${Host}:${UdpPort}"

# Wait for user interrupt
try {
    while ($true) {
        Start-Sleep -Seconds 1
    }
}
finally {
    Write-Host "Stopping echo servers..."
    Stop-Job $TcpJob, $UdpJob
    Remove-Job $TcpJob, $UdpJob
}
```

### Using netcat (Linux/macOS)

```bash
# TCP Echo Server
nc -l -p 8080 -c 'while read line; do echo "$line"; done'

# UDP Echo Server (in separate terminal)
nc -u -l -p 8081 -c 'while read line; do echo "$line"; done'
```

### Running Network Tests

```bash
# Start echo servers first
python scripts/echo_server.py --tcp-port 8080 --udp-port 8081

# Run TCP tests
cargo test --test loopback test_tcp_basic_loopback --ignored

# Run UDP tests  
cargo test --test loopback test_udp_basic_loopback --ignored

# Run all network tests
cargo test --test loopback -- "tcp\|udp" --ignored
```

## SSH Test Server

SSH testing requires a local SSH server for secure communication validation.

### Windows SSH Server Setup

#### Using OpenSSH Server

1. **Install OpenSSH Server:**
   ```powershell
   # Check if installed
   Get-WindowsCapability -Online | Where-Object Name -like 'OpenSSH*'
   
   # Install if needed
   Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
   ```

2. **Start SSH Service:**
   ```powershell
   # Start service
   Start-Service sshd
   
   # Set to start automatically
   Set-Service -Name sshd -StartupType 'Automatic'
   ```

3. **Configure SSH:**
   ```powershell
   # Edit C:\ProgramData\ssh\sshd_config
   # Add/modify:
   # Port 2222
   # PasswordAuthentication yes
   # PubkeyAuthentication yes
   
   # Restart service
   Restart-Service sshd
   ```

4. **Create Test User:**
   ```powershell
   # Create test user
   net user test_ssh test_password /add
   net localgroup "Remote Desktop Users" test_ssh /add
   ```

### Linux SSH Server Setup

1. **Install SSH Server:**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install openssh-server
   
   # RHEL/CentOS
   sudo yum install openssh-server
   ```

2. **Configure SSH:**
   ```bash
   # Edit /etc/ssh/sshd_config
   sudo nano /etc/ssh/sshd_config
   
   # Add test configuration:
   # Port 2222
   # PasswordAuthentication yes
   # PermitRootLogin no
   ```

3. **Start SSH Service:**
   ```bash
   sudo systemctl start ssh
   sudo systemctl enable ssh
   ```

4. **Create Test User:**
   ```bash
   sudo useradd -m test_ssh
   echo "test_ssh:test_password" | sudo chpasswd
   ```

### SSH Key Setup for Testing

```bash
# Generate test key pair
ssh-keygen -t rsa -b 2048 -f ~/.ssh/test_key -N ""

# Copy public key to test user
ssh-copy-id -i ~/.ssh/test_key.pub test_ssh@localhost -p 2222

# Test connection
ssh -i ~/.ssh/test_key test_ssh@localhost -p 2222 "echo 'SSH connection successful'"
```

### SSH Loopback Test Configuration

```rust
// In tests/loopback/ssh_loopback.rs
struct SshLoopbackConfig {
    host: String,
    port: u16,
    username: String,
    key_path: String,
}

impl Default for SshLoopbackConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 2222,
            username: "test_ssh".to_string(),
            key_path: "~/.ssh/test_key".to_string(),
        }
    }
}
```

### Running SSH Tests

```bash
# Ensure SSH server is running
sudo systemctl status ssh  # Linux
Get-Service sshd           # Windows

# Run SSH loopback tests
cargo test --test loopback test_ssh_basic_loopback --ignored

# Test with key authentication
SSH_KEY_PATH=~/.ssh/test_key cargo test --test loopback test_ssh_key_auth --ignored
```

## Hardware Loopback Testing

Physical loopback testing validates actual hardware communication using wired connections.

### Serial Loopback Wiring

#### RS-232 Loopback

**DB-9 Connector Wiring:**
```
Pin 2 (TX) <-> Pin 3 (RX)
Pin 7 (RTS) <-> Pin 8 (CTS)  
Pin 4 (DTR) <-> Pin 6 (DSR) <-> Pin 1 (DCD)
Pin 5 (GND) <-> Pin 5 (GND)
```

**USB-to-Serial Adapter Loopback:**
```
TX <-> RX
RTS <-> CTS
DTR <-> DSR
GND <-> GND
```

#### RS-485 Loopback

**Terminal Block Wiring:**
```
A+ <-> A+
B- <-> B-
GND <-> GND
```

### Ethernet Loopback

**Physical Loopback:**
- Use crossover cable or loopback plug
- Connect TX+ to RX+, TX- to RX-

**Software Loopback:**
```bash
# Use localhost interface
ifconfig lo up  # Linux
```

### Testing with Real Hardware

#### Arduino Test Setup

1. **Arduino Sketch for Testing:**
```arduino
// File: arduino/test_sketches/loopback_test/loopback_test.ino
void setup() {
    Serial.begin(115200);
    while (!Serial) {
        delay(10);
    }
    Serial.println("Arduino Loopback Test Ready");
    Serial.println("Multi-Controller:Arduino:UNO:v1.0");
}

void loop() {
    if (Serial.available()) {
        String message = Serial.readStringUntil('\n');
        message.trim();
        
        // Echo with prefix
        Serial.print("ECHO: ");
        Serial.println(message);
    }
    
    delay(10);
}
```

2. **Upload and Test:**
```bash
# Upload sketch to Arduino
arduino-cli compile --fqbn arduino:avr:uno arduino/test_sketches/loopback_test
arduino-cli upload -p COM3 --fqbn arduino:avr:uno arduino/test_sketches/loopback_test

# Run hardware test
cargo run --bin hardware_test -- --port COM3 --baud 115200
```

### Hardware Test Configuration

```toml
# test-config.toml
[hardware]
enable_real_hardware_tests = true

[hardware.serial]
ports = ["COM3", "/dev/ttyUSB0", "/dev/cu.usbmodem14101"]
baud_rates = [9600, 115200]
test_timeout_ms = 5000

[hardware.tcp] 
test_hosts = ["192.168.1.100:8080"]

[hardware.udp]
broadcast_addresses = ["192.168.1.255:8081"]
```

### Running Hardware Tests

```bash
# Enable hardware testing feature
cargo test --features hardware-tests --test loopback --ignored

# Test specific hardware
cargo run --bin hardware_test -- --config test-config.toml

# Interactive hardware test
cargo run --bin interactive_reconnect
```

## Running Test Suites

The project includes multiple test categories with different execution patterns.

### Individual Test Execution

```bash
# Run single loopback test
cargo test test_serial_basic_loopback --ignored

# Run with output
cargo test test_tcp_reconnection --ignored -- --nocapture

# Run with environment variables
RUST_LOG=debug LOG_LEVEL=trace cargo test test_udp_broadcast --ignored
```

### Test Categories

#### 1. Unit Tests (Fast)
```bash
# Core transport logic tests
cargo test --lib

# Specific module tests
cargo test transport::serial::tests
```

#### 2. Integration Tests (Medium)
```bash
# Component interaction tests
cargo test --test integration

# Device manager tests
cargo test --test integration test_device_lifecycle
```

#### 3. Loopback Tests (Slow, Requires Setup)
```bash
# All loopback tests
cargo test --test loopback --ignored

# Specific transport loopback
cargo test --test loopback serial_loopback --ignored
cargo test --test loopback tcp_loopback --ignored
cargo test --test loopback udp_loopback --ignored
cargo test --test loopback ssh_loopback --ignored
```

#### 4. Hardware Tests (Very Slow, Requires Hardware)
```bash
# Real hardware validation
cargo test --features hardware-tests --ignored

# Arduino-specific tests
cargo test --test arduino_driver_tests --ignored
```

### Test Suite Scripts

Create `scripts/run_tests.ps1` (Windows):

```powershell
param(
    [string]$Suite = "all",
    [switch]$IgnoreHardware = $false,
    [switch]$Verbose = $false
)

$VerboseFlag = if ($Verbose) { "--nocapture" } else { "" }

switch ($Suite.ToLower()) {
    "unit" {
        Write-Host "Running unit tests..."
        cargo test --lib $VerboseFlag
    }
    "integration" {  
        Write-Host "Running integration tests..."
        cargo test --test integration $VerboseFlag
    }
    "loopback" {
        Write-Host "Running loopback tests (requires setup)..."
        cargo test --test loopback --ignored $VerboseFlag
    }
    "hardware" {
        if ($IgnoreHardware) {
            Write-Host "Hardware tests skipped"
        } else {
            Write-Host "Running hardware tests..."
            cargo test --features hardware-tests --ignored $VerboseFlag
        }
    }
    "all" {
        Write-Host "Running complete test suite..."
        cargo test --lib $VerboseFlag
        cargo test --test integration $VerboseFlag
        cargo test --test loopback --ignored $VerboseFlag
        
        if (-not $IgnoreHardware) {
            cargo test --features hardware-tests --ignored $VerboseFlag
        }
    }
    default {
        Write-Host "Unknown test suite: $Suite"
        Write-Host "Valid options: unit, integration, loopback, hardware, all"
        exit 1
    }
}
```

Create `scripts/run_tests.sh` (Linux/macOS):

```bash
#!/bin/bash

SUITE="all"
IGNORE_HARDWARE=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --suite)
            SUITE="$2"
            shift 2
            ;;
        --ignore-hardware)
            IGNORE_HARDWARE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        *)
            echo "Unknown option $1"
            exit 1
            ;;
    esac
done

VERBOSE_FLAG=""
if [ "$VERBOSE" = true ]; then
    VERBOSE_FLAG="-- --nocapture"
fi

case $SUITE in
    unit)
        echo "Running unit tests..."
        cargo test --lib $VERBOSE_FLAG
        ;;
    integration)
        echo "Running integration tests..."
        cargo test --test integration $VERBOSE_FLAG
        ;;
    loopback)
        echo "Running loopback tests (requires setup)..."
        cargo test --test loopback --ignored $VERBOSE_FLAG
        ;;
    hardware)
        if [ "$IGNORE_HARDWARE" = true ]; then
            echo "Hardware tests skipped"
        else
            echo "Running hardware tests..."
            cargo test --features hardware-tests --ignored $VERBOSE_FLAG
        fi
        ;;
    all)
        echo "Running complete test suite..."
        cargo test --lib $VERBOSE_FLAG
        cargo test --test integration $VERBOSE_FLAG
        cargo test --test loopback --ignored $VERBOSE_FLAG
        
        if [ "$IGNORE_HARDWARE" = false ]; then
            cargo test --features hardware-tests --ignored $VERBOSE_FLAG
        fi
        ;;
    *)
        echo "Unknown test suite: $SUITE"
        echo "Valid options: unit, integration, loopback, hardware, all"
        exit 1
        ;;
esac
```

### Parallel Test Execution

```bash
# Run tests in parallel (default)
cargo test --jobs 4

# Run serial tests sequentially
cargo test --test loopback -- --test-threads=1

# Mixed parallel execution
cargo test --lib --jobs 4  # Fast unit tests in parallel
cargo test --test loopback --ignored -- --test-threads=1  # Slow tests sequential
```

## CI/CD Integration

Integration with continuous integration systems requires virtual device setup and headless test execution.

### GitHub Actions Configuration

Update `.github/workflows/ci.yml`:

```yaml
name: CI - Full Test Suite

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
        
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy
        
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Setup Virtual Devices (Ubuntu)
      if: matrix.os == 'ubuntu-latest'
      run: |
        # Install socat for virtual serial ports
        sudo apt-get update
        sudo apt-get install -y socat netcat-openbsd openssh-server
        
        # Create virtual serial ports
        socat -d -d pty,raw,echo=0 pty,raw,echo=0 &
        sleep 2
        
        # Setup SSH server
        sudo systemctl start ssh
        echo "test_ssh:test_password" | sudo chpasswd
        
    - name: Setup Virtual Devices (Windows) 
      if: matrix.os == 'windows-latest'
      run: |
        # Install com0com virtual serial ports
        $url = "https://sourceforge.net/projects/com0com/files/com0com/3.0.0.0/com0com-3.0.0.0-i386-and-x64-signed.zip"
        # Note: In practice, use a stable package manager or pre-built image
        
        # Enable SSH server
        Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
        Start-Service sshd
        Set-Service -Name sshd -StartupType 'Automatic'
        
    - name: Setup Virtual Devices (macOS)
      if: matrix.os == 'macos-latest' 
      run: |
        # Install socat
        brew install socat
        
        # Create virtual serial ports
        socat -d -d pty,raw,echo=0 pty,raw,echo=0 &
        sleep 2
        
        # Enable SSH
        sudo systemsetup -setremotelogin on
        
    - name: Start Echo Servers
      run: |
        # Start TCP/UDP echo servers in background
        python scripts/echo_server.py --tcp-port 8080 --udp-port 8081 &
        sleep 2
        
    - name: Run Unit Tests
      run: cargo test --lib
      
    - name: Run Integration Tests
      run: cargo test --test integration
      
    - name: Run Loopback Tests
      run: |
        # Run with retries for stability
        for i in {1..3}; do
          if cargo test --test loopback --ignored -- --test-threads=1; then
            break
          elif [ $i -eq 3 ]; then
            exit 1
          fi
          sleep 5
        done
        
    - name: Run Hardware Tests (No Real Hardware)
      run: |
        # These tests should use mocks/simulators in CI
        cargo test --features hardware-tests --test arduino_driver_tests
        
    - name: Upload Test Results
      if: failure()
      uses: actions/upload-artifact@v4
      with:
        name: test-results-${{ matrix.os }}-${{ matrix.rust }}
        path: |
          target/debug/test-*.log
          test-output.txt
        retention-days: 7
```

### Docker Test Environment

Create `Dockerfile.test`:

```dockerfile
FROM rust:1.70

# Install system dependencies
RUN apt-get update && apt-get install -y \
    socat \
    netcat-openbsd \
    openssh-server \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Configure SSH server
RUN mkdir /var/run/sshd
RUN echo 'root:testpassword' | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
EXPOSE 22

# Setup test user
RUN useradd -m test_ssh && echo "test_ssh:test_password" | chpasswd

WORKDIR /app
COPY . .

# Start services and run tests
CMD ["/bin/bash", "-c", "service ssh start && python3 scripts/echo_server.py & cargo test --ignored -- --test-threads=1"]
```

### Jenkins Pipeline

Create `Jenkinsfile`:

```groovy
pipeline {
    agent any
    
    stages {
        stage('Setup') {
            parallel {
                stage('Virtual Serial Ports') {
                    steps {
                        sh '''
                            pkill socat || true
                            socat -d -d pty,raw,echo=0 pty,raw,echo=0 &
                            sleep 2
                        '''
                    }
                }
                stage('Echo Servers') {
                    steps {
                        sh '''
                            pkill -f echo_server.py || true
                            python3 scripts/echo_server.py &
                            sleep 2
                        '''
                    }
                }
            }
        }
        
        stage('Test') {
            parallel {
                stage('Unit Tests') {
                    steps {
                        sh 'cargo test --lib'
                    }
                }
                stage('Integration Tests') {
                    steps {
                        sh 'cargo test --test integration'
                    }
                }
                stage('Loopback Tests') {
                    steps {
                        sh 'cargo test --test loopback --ignored -- --test-threads=1'
                    }
                }
            }
        }
    }
    
    post {
        always {
            sh '''
                pkill socat || true
                pkill -f echo_server.py || true
            '''
        }
        failure {
            archiveArtifacts artifacts: 'target/debug/test-*.log', allowEmptyArchive: true
        }
    }
}
```

### Local Test Automation

Create `scripts/ci_test.ps1`:

```powershell
# Local CI simulation script
param(
    [switch]$SetupOnly = $false,
    [switch]$SkipSetup = $false
)

if (-not $SkipSetup) {
    Write-Host "Setting up test environment..."
    
    # Start echo servers
    Start-Job -ScriptBlock {
        python scripts/echo_server.py --tcp-port 8080 --udp-port 8081
    }
    
    # Wait for services
    Start-Sleep -Seconds 3
    
    # Test connectivity
    Test-NetConnection -ComputerName localhost -Port 8080 -WarningAction SilentlyContinue
    Test-NetConnection -ComputerName localhost -Port 8081 -WarningAction SilentlyContinue
}

if ($SetupOnly) {
    Write-Host "Setup complete. Echo servers running in background."
    exit 0
}

# Run test suite
Write-Host "Running test suite..."

try {
    # Unit tests (fast)
    cargo test --lib
    if ($LASTEXITCODE -ne 0) { throw "Unit tests failed" }
    
    # Integration tests
    cargo test --test integration  
    if ($LASTEXITCODE -ne 0) { throw "Integration tests failed" }
    
    # Loopback tests (requires setup)
    cargo test --test loopback --ignored -- --test-threads=1
    if ($LASTEXITCODE -ne 0) { throw "Loopback tests failed" }
    
    Write-Host "All tests passed!" -ForegroundColor Green
}
catch {
    Write-Host "Test failure: $_" -ForegroundColor Red
    exit 1
}
finally {
    # Cleanup
    Get-Job | Stop-Job
    Get-Job | Remove-Job
}
```

## Troubleshooting

### Common Issues and Solutions

#### 1. Virtual Serial Port Issues

**Problem**: Virtual port not appearing or access denied

```bash
# Linux: Check permissions
ls -la /dev/tty*
sudo chmod 666 /dev/ttyUSB0

# Windows: Check com0com installation
setupc list

# macOS: Check device ownership
ls -la /dev/cu.*
sudo chown $USER /dev/cu.usbserial-*
```

**Problem**: Port already in use

```bash
# Find process using port
lsof -t -i:8080  # Linux/macOS
netstat -ano | findstr :8080  # Windows

# Kill process
kill $(lsof -t -i:8080)  # Linux/macOS  
taskkill /PID <pid> /F   # Windows
```

#### 2. Test Timeouts

**Problem**: Tests timing out due to slow hardware

```rust
// Increase timeout in test configuration
let config = LoopbackConfig {
    timeout: Duration::from_secs(10), // Increased from 5
    retry_count: 5,                   // More retries
    ..Default::default()
};
```

**Problem**: Race conditions in parallel tests

```bash
# Force sequential execution
cargo test --test loopback -- --test-threads=1

# Or run individual tests
cargo test test_serial_basic_loopback
```

#### 3. Network Test Failures

**Problem**: Echo server not responding

```bash
# Test server connectivity
telnet localhost 8080  # TCP
nc -u localhost 8081   # UDP

# Check if ports are bound
netstat -tuln | grep 8080
ss -tuln | grep 8080
```

**Problem**: Firewall blocking connections

```bash
# Linux: Allow test ports
sudo ufw allow 8080
sudo ufw allow 8081

# Windows: Add firewall rule  
netsh advfirewall firewall add rule name="Test Ports" dir=in action=allow protocol=TCP localport=8080,8081
```

#### 4. SSH Test Issues

**Problem**: SSH server not started

```bash
# Linux
sudo systemctl status ssh
sudo systemctl start ssh

# Windows
Get-Service sshd
Start-Service sshd
```

**Problem**: Authentication failures

```bash
# Check SSH configuration
sudo sshd -T | grep -i password
sudo sshd -T | grep -i pubkey

# Test SSH connection manually
ssh -v test_ssh@localhost -p 2222
```

#### 5. Hardware Test Problems

**Problem**: Arduino not responding

```bash
# Check serial port connection
ls -la /dev/ttyUSB* # Linux
mode # Windows

# Reset Arduino connection
# Unplug and reconnect USB cable

# Verify baud rate match
arduino-cli board list
```

**Problem**: Data corruption in loopback

```rust
// Add data integrity verification
let checksum = data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
// Send checksum with data
```

#### 6. CI/CD Integration Issues

**Problem**: Tests pass locally but fail in CI

```yaml
# Add debugging to CI pipeline
- name: Debug Test Environment
  run: |
    netstat -tuln
    ps aux | grep echo_server
    ls -la /dev/tty*
```

**Problem**: Flaky tests in CI

```bash
# Add retry logic
for i in {1..3}; do
  if cargo test --test loopback --ignored; then
    break
  elif [ $i -eq 3 ]; then
    exit 1
  fi
  sleep 5
done
```

### Debug Configuration

Create `test-debug.toml`:

```toml
[debug]
enable_logging = true
log_level = "debug"
capture_io = true
save_test_data = true

[debug.serial]
log_bytes = true
hex_dump = true
timing_analysis = true

[debug.network] 
packet_capture = true
connection_tracing = true

[debug.ssh]
verbose_logging = true
save_session_logs = true
```

### Test Data Collection

```rust
// Add to test utilities
#[cfg(test)]
pub fn save_test_artifacts(test_name: &str, data: &[u8]) {
    use std::fs;
    let filename = format!("test-output/{}-{}.bin", test_name, chrono::Utc::now().timestamp());
    fs::create_dir_all("test-output").ok();
    fs::write(&filename, data).ok();
    println!("Test data saved to {}", filename);
}
```

### Performance Monitoring

```bash
# Monitor test performance
time cargo test --test loopback --ignored

# Profile memory usage
valgrind --tool=memcheck cargo test --test loopback test_serial_basic_loopback

# Analyze test bottlenecks
cargo test --test loopback -- --report-time
```

---

This guide provides comprehensive coverage of hardware-in-the-loop testing setup and execution. For specific issues not covered here, check the project's issue tracker or create detailed bug reports with environment information and full error logs.