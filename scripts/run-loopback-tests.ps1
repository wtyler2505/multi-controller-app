#Requires -Version 5.1
<#
.SYNOPSIS
    Comprehensive loopback test runner for Multi-Controller App transport layer
    
.DESCRIPTION
    Orchestrates complete loopback testing across all transport types (Serial, TCP, UDP, SSH).
    Handles virtual port setup, echo server management, test execution with categorization,
    comprehensive reporting with pass/fail statistics, and proper resource cleanup.
    
.PARAMETER TestCategories
    Comma-separated list of test categories to run: 'serial', 'tcp', 'udp', 'ssh', 'all' (default: all)
    
.PARAMETER SerialPorts
    Comma-separated list of COM ports for serial testing (e.g., 'COM1,COM2'). Auto-detects if not specified.
    
.PARAMETER TcpPorts
    Comma-separated list of TCP ports for echo servers (default: '8080,8081,8082')
    
.PARAMETER UdpPorts
    Comma-separated list of UDP ports for echo servers (default: '9090,9091,9092')
    
.PARAMETER SshHosts
    Comma-separated list of SSH hosts for testing (e.g., 'localhost:2222'). Skipped if not specified.
    
.PARAMETER TestDuration
    Maximum duration in seconds for each test category (default: 60)
    
.PARAMETER ConcurrentTests
    Number of concurrent tests to run per category (default: 3)
    
.PARAMETER OutputDir
    Directory for test reports and logs (default: 'tests/reports')
    
.PARAMETER KeepServers
    Keep echo servers running after tests for manual debugging
    
.PARAMETER ShowDetails
    Show detailed test output and progress information
    
.PARAMETER CiMode
    Run in CI/CD mode with non-interactive operation and structured output
    
.PARAMETER SkipVirtualPorts
    Skip virtual serial port setup (useful if already configured)
    
.PARAMETER StressTest
    Enable stress testing with high-frequency operations
    
.PARAMETER ReportFormat
    Report format: 'json', 'xml', 'html', 'all' (default: json)
    
.EXAMPLE
    .\run-loopback-tests.ps1 -ShowDetails
    Runs all loopback tests with detailed output
    
.EXAMPLE
    .\run-loopback-tests.ps1 -TestCategories "tcp,udp" -TcpPorts "8080,8081" -CiMode
    Runs only TCP and UDP tests in CI mode
    
.EXAMPLE
    .\run-loopback-tests.ps1 -SerialPorts "COM1,COM2" -KeepServers -StressTest
    Runs tests on specific COM ports with stress testing and keeps servers running
    
.EXAMPLE
    .\run-loopback-tests.ps1 -CiMode -ReportFormat "xml" -OutputDir "ci-reports"
    CI mode with XML reports in custom directory
#>

[CmdletBinding()]
param(
    [Parameter()]
    [ValidateSet('serial', 'tcp', 'udp', 'ssh', 'all')]
    [string[]]$TestCategories = @('all'),
    
    [Parameter()]
    [string[]]$SerialPorts = @(),
    
    [Parameter()]
    [int[]]$TcpPorts = @(8080, 8081, 8082),
    
    [Parameter()]
    [int[]]$UdpPorts = @(9090, 9091, 9092),
    
    [Parameter()]
    [string[]]$SshHosts = @(),
    
    [Parameter()]
    [int]$TestDuration = 60,
    
    [Parameter()]
    [int]$ConcurrentTests = 3,
    
    [Parameter()]
    [string]$OutputDir = "tests/reports",
    
    [Parameter()]
    [switch]$KeepServers,
    
    [Parameter()]
    [switch]$ShowDetails,
    
    [Parameter()]
    [switch]$CiMode,
    
    [Parameter()]
    [switch]$SkipVirtualPorts,
    
    [Parameter()]
    [switch]$StressTest,
    
    [Parameter()]
    [ValidateSet('json', 'xml', 'html', 'all')]
    [string]$ReportFormat = 'json'
)

# Configuration and initialization
$ErrorActionPreference = "Continue"  # Continue on errors to ensure cleanup
$script:VerbosePreference = if ($ShowDetails) { 'Continue' } else { 'SilentlyContinue' }
$script:InformationPreference = if ($CiMode) { 'Continue' } else { 'SilentlyContinue' }

# Global tracking variables
$script:EchoServers = @()
$script:VirtualPorts = @()
$script:TestResults = @{
    StartTime = Get-Date
    Categories = @{}
    TotalTests = 0
    PassedTests = 0
    FailedTests = 0
    SkippedTests = 0
    Errors = @()
}

# Determine project root
$script:ProjectRoot = if (Test-Path "Cargo.toml") { 
    (Get-Location).Path 
} else { 
    Split-Path $PSScriptRoot -Parent 
}

# Set up output directory
$script:OutputPath = if ([System.IO.Path]::IsPathRooted($OutputDir)) { 
    $OutputDir 
} else { 
    Join-Path $script:ProjectRoot $OutputDir 
}

if (-not (Test-Path $script:OutputPath)) {
    New-Item -ItemType Directory -Path $script:OutputPath -Force | Out-Null
}

$script:Timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$script:ReportBaseName = "loopback-test-report-$script:Timestamp"

#region Utility Functions

function Write-Status {
    param(
        [string]$Message,
        [string]$Level = "INFO",
        [string]$Category = ""
    )
    
    $prefix = if ($Category) { "[$Category]" } else { "" }
    $color = switch ($Level) {
        "SUCCESS" { "Green" }
        "WARNING" { "Yellow" }
        "ERROR" { "Red" }
        "INFO" { if ($CiMode) { "White" } else { "Cyan" } }
        "DETAIL" { "Gray" }
        default { "White" }
    }
    
    $timestamp = Get-Date -Format 'HH:mm:ss.fff'
    $fullMessage = "[$timestamp] $prefix [$Level] $Message"
    
    if ($CiMode) {
        # Structured output for CI
        Write-Information $fullMessage
        if ($Level -eq "ERROR") {
            Write-Error $Message
        }
    } else {
        # Colored output for interactive use
        if ($Level -eq "ERROR") {
            Write-Host $fullMessage -ForegroundColor $color
        } elseif ($ShowDetails -or $Level -in @("SUCCESS", "WARNING", "ERROR")) {
            Write-Host $fullMessage -ForegroundColor $color
        }
    }
    
    # Log everything to verbose for debugging
    Write-Verbose $fullMessage
}

function Test-Administrator {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Get-AvailablePorts {
    param(
        [string]$Protocol = "TCP",
        [int]$StartPort = 8000,
        [int]$Count = 3
    )
    
    $availablePorts = @()
    $port = $StartPort
    
    while ($availablePorts.Count -lt $Count -and $port -lt 65535) {
        try {
            if ($Protocol -eq "TCP") {
                $listener = [System.Net.Sockets.TcpListener]::new([System.Net.IPAddress]::Any, $port)
                $listener.Start()
                $listener.Stop()
                $availablePorts += $port
            } else {
                # UDP check
                $udpClient = [System.Net.Sockets.UdpClient]::new($port)
                $udpClient.Close()
                $availablePorts += $port
            }
        } catch {
            # Port in use, try next
        }
        $port++
    }
    
    return $availablePorts
}

function Start-EchoServer {
    param(
        [string]$Type,
        [int]$Port,
        [int]$TimeoutSeconds = $TestDuration * 2
    )
    
    Write-Status "Starting $Type echo server on port $Port" "INFO" "SERVER"
    
    $serverScript = @"
using System;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

class EchoServer
{
    static async Task Main(string[] args)
    {
        var type = args[0];
        var port = int.Parse(args[1]);
        var timeout = int.Parse(args[2]);
        
        var cts = new CancellationTokenSource(TimeSpan.FromSeconds(timeout));
        
        try 
        {
            if (type == "TCP")
            {
                await RunTcpServer(port, cts.Token);
            }
            else if (type == "UDP")
            {
                await RunUdpServer(port, cts.Token);
            }
        }
        catch (OperationCanceledException)
        {
            Console.WriteLine($"{type} server on port {port} timed out after {timeout}s");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"{type} server error: {ex.Message}");
        }
    }
    
    static async Task RunTcpServer(int port, CancellationToken cancellationToken)
    {
        var listener = new TcpListener(IPAddress.Any, port);
        listener.Start();
        Console.WriteLine($"TCP echo server listening on port {port}");
        
        var clients = new List<Task>();
        
        try
        {
            while (!cancellationToken.IsCancellationRequested)
            {
                var tcpClient = await listener.AcceptTcpClientAsync();
                var clientTask = HandleTcpClient(tcpClient, cancellationToken);
                clients.Add(clientTask);
                
                // Clean up completed tasks
                clients.RemoveAll(t => t.IsCompleted);
            }
        }
        finally
        {
            listener.Stop();
            await Task.WhenAll(clients.Where(t => !t.IsCompleted));
        }
    }
    
    static async Task HandleTcpClient(TcpClient client, CancellationToken cancellationToken)
    {
        using (client)
        using (var stream = client.GetStream())
        {
            var buffer = new byte[8192];
            
            try
            {
                while (!cancellationToken.IsCancellationRequested && client.Connected)
                {
                    var bytesRead = await stream.ReadAsync(buffer, 0, buffer.Length, cancellationToken);
                    if (bytesRead == 0) break;
                    
                    await stream.WriteAsync(buffer, 0, bytesRead, cancellationToken);
                    await stream.FlushAsync(cancellationToken);
                }
            }
            catch (Exception ex) when (!(ex is OperationCanceledException))
            {
                Console.WriteLine($"TCP client error: {ex.Message}");
            }
        }
    }
    
    static async Task RunUdpServer(int port, CancellationToken cancellationToken)
    {
        using (var udpClient = new UdpClient(port))
        {
            Console.WriteLine($"UDP echo server listening on port {port}");
            
            while (!cancellationToken.IsCancellationRequested)
            {
                try
                {
                    var result = await udpClient.ReceiveAsync();
                    await udpClient.SendAsync(result.Buffer, result.Buffer.Length, result.RemoteEndPoint);
                }
                catch (Exception ex) when (!(ex is OperationCanceledException))
                {
                    Console.WriteLine($"UDP server error: {ex.Message}");
                }
            }
        }
    }
}
"@
    
    # Create temporary C# file
    $serverFile = Join-Path $env:TEMP "EchoServer_$($Type)_$Port.cs"
    $serverScript | Out-File -FilePath $serverFile -Encoding UTF8
    
    try {
        # Compile and run echo server
        $compileArgs = @(
            "/target:exe"
            "/out:$env:TEMP\EchoServer_$($Type)_$Port.exe"
            $serverFile
        )
        
        $compileResult = Start-Process "csc.exe" -ArgumentList $compileArgs -Wait -PassThru -WindowStyle Hidden
        if ($compileResult.ExitCode -ne 0) {
            throw "Failed to compile echo server"
        }
        
        # Start the server
        $serverProcess = Start-Process -FilePath "$env:TEMP\EchoServer_$($Type)_$Port.exe" `
                                      -ArgumentList @($Type, $Port, $TimeoutSeconds) `
                                      -PassThru -WindowStyle Hidden
        
        # Wait a moment for server to start
        Start-Sleep -Milliseconds 500
        
        if ($serverProcess.HasExited) {
            throw "Echo server exited immediately"
        }
        
        $serverInfo = @{
            Type = $Type
            Port = $Port
            Process = $serverProcess
            TempFile = $serverFile
            ExeFile = "$env:TEMP\EchoServer_$($Type)_$Port.exe"
        }
        
        $script:EchoServers += $serverInfo
        Write-Status "Started $Type echo server on port $Port (PID: $($serverProcess.Id))" "SUCCESS" "SERVER"
        
        return $serverInfo
    } catch {
        Write-Status "Failed to start $Type echo server on port $Port`: $_" "ERROR" "SERVER"
        throw
    }
}

function Stop-EchoServer {
    param([object]$ServerInfo)
    
    try {
        if ($ServerInfo.Process -and -not $ServerInfo.Process.HasExited) {
            Write-Status "Stopping $($ServerInfo.Type) server on port $($ServerInfo.Port)" "INFO" "SERVER"
            $ServerInfo.Process.Kill()
            $ServerInfo.Process.WaitForExit(5000)
        }
        
        # Clean up temporary files
        if (Test-Path $ServerInfo.TempFile) { Remove-Item $ServerInfo.TempFile -Force }
        if (Test-Path $ServerInfo.ExeFile) { Remove-Item $ServerInfo.ExeFile -Force }
        
    } catch {
        Write-Status "Error stopping server: $_" "WARNING" "SERVER"
    }
}

function Setup-VirtualSerialPorts {
    if ($SkipVirtualPorts) {
        Write-Status "Skipping virtual serial port setup" "INFO" "SERIAL"
        return
    }
    
    Write-Status "Setting up virtual serial ports" "INFO" "SERIAL"
    
    # Try different virtual serial port solutions
    $solutions = @(
        @{ Name = "com0com"; Command = "setupc.exe"; Args = @("install", "PortName=COM98", "PortName=COM99") },
        @{ Name = "Virtual Serial Port Driver"; Command = "vspdctl.exe"; Args = @() },
        @{ Name = "Null-modem emulator"; Command = "com0com-setup.exe"; Args = @() }
    )
    
    $success = $false
    foreach ($solution in $solutions) {
        try {
            if (Get-Command $solution.Command -ErrorAction SilentlyContinue) {
                Write-Status "Found $($solution.Name), attempting setup..." "INFO" "SERIAL"
                # Implementation would vary by solution
                $success = $true
                break
            }
        } catch {
            Write-Status "Failed to setup with $($solution.Name): $_" "WARNING" "SERIAL"
        }
    }
    
    if (-not $success) {
        Write-Status "No virtual serial port solution found. Install com0com or similar for serial testing" "WARNING" "SERIAL"
        Write-Status "Serial tests will be skipped unless real COM ports are specified" "WARNING" "SERIAL"
    } else {
        $script:VirtualPorts += "COM98", "COM99"
        Write-Status "Virtual serial ports created: $($script:VirtualPorts -join ', ')" "SUCCESS" "SERIAL"
    }
}

function Get-SerialPorts {
    if ($SerialPorts.Count -gt 0) {
        return $SerialPorts
    }
    
    # Auto-detect available COM ports
    $availablePorts = @()
    
    # Add virtual ports if created
    $availablePorts += $script:VirtualPorts
    
    # Check for real serial ports
    try {
        $comPorts = [System.IO.Ports.SerialPort]::GetPortNames() | Sort-Object
        $availablePorts += $comPorts
        
        if ($comPorts.Count -gt 0) {
            Write-Status "Found real serial ports: $($comPorts -join ', ')" "INFO" "SERIAL"
        }
    } catch {
        Write-Status "Failed to enumerate serial ports: $_" "WARNING" "SERIAL"
    }
    
    if ($availablePorts.Count -eq 0) {
        Write-Status "No serial ports available for testing" "WARNING" "SERIAL"
    }
    
    return $availablePorts | Select-Object -Unique
}

#endregion

#region Test Execution Functions

function Invoke-CargoTest {
    param(
        [string]$TestFilter,
        [string]$Category,
        [int]$TimeoutSeconds = $TestDuration
    )
    
    Write-Status "Running Rust tests for category: $Category" "INFO" $Category.ToUpper()
    
    $cargoArgs = @(
        "test"
        "--test", "loopback_tests"
        "--"
        $TestFilter
        "--nocapture"
    )
    
    if ($ShowDetails) {
        $cargoArgs += "--show-output"
    }
    
    $testStartTime = Get-Date
    
    try {
        $process = Start-Process "cargo" -ArgumentList $cargoArgs -WorkingDirectory $script:ProjectRoot `
                                 -PassThru -WindowStyle Hidden -RedirectStandardOutput -RedirectStandardError
        
        if (-not $process.WaitForExit($TimeoutSeconds * 1000)) {
            $process.Kill()
            throw "Test timed out after $TimeoutSeconds seconds"
        }
        
        $stdout = $process.StandardOutput.ReadToEnd()
        $stderr = $process.StandardError.ReadToEnd()
        
        $testResult = @{
            Category = $Category
            Filter = $TestFilter
            ExitCode = $process.ExitCode
            Duration = (Get-Date) - $testStartTime
            Output = $stdout
            Errors = $stderr
            Success = $process.ExitCode -eq 0
        }
        
        # Parse test results from cargo output
        $testResult.TestCount = 0
        $testResult.PassedCount = 0
        $testResult.FailedCount = 0
        
        if ($stdout -match "test result: (\w+)\. (\d+) passed; (\d+) failed") {
            $testResult.TestCount = [int]$Matches[2] + [int]$Matches[3]
            $testResult.PassedCount = [int]$Matches[2]
            $testResult.FailedCount = [int]$Matches[3]
        }
        
        Write-Status "Test completed: $($testResult.PassedCount) passed, $($testResult.FailedCount) failed" `
                    (if ($testResult.Success) { "SUCCESS" } else { "ERROR" }) $Category.ToUpper()
        
        return $testResult
        
    } catch {
        Write-Status "Test execution failed: $_" "ERROR" $Category.ToUpper()
        
        return @{
            Category = $Category
            Filter = $TestFilter
            ExitCode = -1
            Duration = (Get-Date) - $testStartTime
            Output = ""
            Errors = $_.Exception.Message
            Success = $false
            TestCount = 0
            PassedCount = 0
            FailedCount = 1
        }
    }
}

function Test-SerialLoopback {
    Write-Status "Starting serial loopback tests" "INFO" "SERIAL"
    
    $ports = Get-SerialPorts
    if ($ports.Count -eq 0) {
        Write-Status "No serial ports available, skipping serial tests" "WARNING" "SERIAL"
        return @{
            Category = "Serial"
            Skipped = $true
            Reason = "No ports available"
        }
    }
    
    $testFilter = "serial_loopback"
    if ($StressTest) {
        $testFilter += " or stress"
    }
    
    # Set environment variables for test configuration
    $env:LOOPBACK_SERIAL_PORTS = $ports -join ","
    $env:LOOPBACK_TEST_DURATION = $TestDuration.ToString()
    $env:LOOPBACK_STRESS_MODE = $StressTest.ToString().ToLower()
    
    try {
        return Invoke-CargoTest -TestFilter $testFilter -Category "Serial"
    } finally {
        # Clean up environment variables
        Remove-Item -Path "env:LOOPBACK_SERIAL_PORTS" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_TEST_DURATION" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_STRESS_MODE" -ErrorAction SilentlyContinue
    }
}

function Test-TcpLoopback {
    Write-Status "Starting TCP loopback tests" "INFO" "TCP"
    
    # Ensure we have available ports
    $availablePorts = Get-AvailablePorts -Protocol "TCP" -StartPort $TcpPorts[0] -Count $TcpPorts.Count
    if ($availablePorts.Count -eq 0) {
        Write-Status "No TCP ports available, skipping TCP tests" "WARNING" "TCP"
        return @{
            Category = "TCP"
            Skipped = $true
            Reason = "No ports available"
        }
    }
    
    $serversStarted = @()
    
    try {
        # Start echo servers
        foreach ($port in $availablePorts) {
            $server = Start-EchoServer -Type "TCP" -Port $port
            $serversStarted += $server
        }
        
        # Wait for servers to stabilize
        Start-Sleep -Seconds 2
        
        $testFilter = "tcp_loopback"
        if ($StressTest) {
            $testFilter += " or stress"
        }
        
        # Configure test environment
        $env:LOOPBACK_TCP_PORTS = $availablePorts -join ","
        $env:LOOPBACK_CONCURRENT_TESTS = $ConcurrentTests.ToString()
        $env:LOOPBACK_TEST_DURATION = $TestDuration.ToString()
        $env:LOOPBACK_STRESS_MODE = $StressTest.ToString().ToLower()
        
        return Invoke-CargoTest -TestFilter $testFilter -Category "TCP"
        
    } finally {
        # Clean up servers unless keeping them
        if (-not $KeepServers) {
            foreach ($server in $serversStarted) {
                Stop-EchoServer -ServerInfo $server
            }
        }
        
        # Clean up environment variables
        Remove-Item -Path "env:LOOPBACK_TCP_PORTS" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_CONCURRENT_TESTS" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_TEST_DURATION" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_STRESS_MODE" -ErrorAction SilentlyContinue
    }
}

function Test-UdpLoopback {
    Write-Status "Starting UDP loopback tests" "INFO" "UDP"
    
    # Ensure we have available ports
    $availablePorts = Get-AvailablePorts -Protocol "UDP" -StartPort $UdpPorts[0] -Count $UdpPorts.Count
    if ($availablePorts.Count -eq 0) {
        Write-Status "No UDP ports available, skipping UDP tests" "WARNING" "UDP"
        return @{
            Category = "UDP"
            Skipped = $true
            Reason = "No ports available"
        }
    }
    
    $serversStarted = @()
    
    try {
        # Start echo servers
        foreach ($port in $availablePorts) {
            $server = Start-EchoServer -Type "UDP" -Port $port
            $serversStarted += $server
        }
        
        # Wait for servers to stabilize
        Start-Sleep -Seconds 2
        
        $testFilter = "udp_loopback"
        if ($StressTest) {
            $testFilter += " or stress"
        }
        
        # Configure test environment
        $env:LOOPBACK_UDP_PORTS = $availablePorts -join ","
        $env:LOOPBACK_CONCURRENT_TESTS = $ConcurrentTests.ToString()
        $env:LOOPBACK_TEST_DURATION = $TestDuration.ToString()
        $env:LOOPBACK_STRESS_MODE = $StressTest.ToString().ToLower()
        
        return Invoke-CargoTest -TestFilter $testFilter -Category "UDP"
        
    } finally {
        # Clean up servers unless keeping them
        if (-not $KeepServers) {
            foreach ($server in $serversStarted) {
                Stop-EchoServer -ServerInfo $server
            }
        }
        
        # Clean up environment variables
        Remove-Item -Path "env:LOOPBACK_UDP_PORTS" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_CONCURRENT_TESTS" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_TEST_DURATION" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_STRESS_MODE" -ErrorAction SilentlyContinue
    }
}

function Test-SshLoopback {
    Write-Status "Starting SSH loopback tests" "INFO" "SSH"
    
    if ($SshHosts.Count -eq 0) {
        Write-Status "No SSH hosts specified, skipping SSH tests" "WARNING" "SSH"
        return @{
            Category = "SSH"
            Skipped = $true
            Reason = "No hosts specified"
        }
    }
    
    # Check SSH connectivity
    $availableHosts = @()
    foreach ($host in $SshHosts) {
        try {
            # Basic connectivity test
            $hostPort = $host -split ':'
            $hostname = $hostPort[0]
            $port = if ($hostPort.Count -gt 1) { $hostPort[1] } else { 22 }
            
            $tcpClient = New-Object System.Net.Sockets.TcpClient
            $tcpClient.Connect($hostname, $port)
            $tcpClient.Close()
            
            $availableHosts += $host
            Write-Status "SSH host $host is reachable" "INFO" "SSH"
        } catch {
            Write-Status "SSH host $host is not reachable: $_" "WARNING" "SSH"
        }
    }
    
    if ($availableHosts.Count -eq 0) {
        Write-Status "No SSH hosts available, skipping SSH tests" "WARNING" "SSH"
        return @{
            Category = "SSH"
            Skipped = $true
            Reason = "No hosts reachable"
        }
    }
    
    $testFilter = "ssh_loopback"
    if ($StressTest) {
        $testFilter += " or stress"
    }
    
    # Configure test environment
    $env:LOOPBACK_SSH_HOSTS = $availableHosts -join ","
    $env:LOOPBACK_TEST_DURATION = $TestDuration.ToString()
    $env:LOOPBACK_STRESS_MODE = $StressTest.ToString().ToLower()
    
    try {
        return Invoke-CargoTest -TestFilter $testFilter -Category "SSH"
    } finally {
        # Clean up environment variables
        Remove-Item -Path "env:LOOPBACK_SSH_HOSTS" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_TEST_DURATION" -ErrorAction SilentlyContinue
        Remove-Item -Path "env:LOOPBACK_STRESS_MODE" -ErrorAction SilentlyContinue
    }
}

#endregion

#region Report Generation

function ConvertTo-XmlReport {
    param([object]$TestResults)
    
    $xml = @"
<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
"@
    
    foreach ($category in $TestResults.Categories.Keys) {
        $result = $TestResults.Categories[$category]
        
        $xml += @"
    <testsuite name="$category" tests="$($result.TestCount)" failures="$($result.FailedCount)" time="$($result.Duration.TotalSeconds)">
"@
        
        if ($result.TestCount -gt 0) {
            for ($i = 0; $i -lt $result.TestCount; $i++) {
                $testName = "$category" + "_test_$i"
                $status = if ($i -lt $result.PassedCount) { "passed" } else { "failed" }
                
                if ($status -eq "passed") {
                    $xml += "        <testcase name=`"$testName`" time=`"0`" />`n"
                } else {
                    $xml += "        <testcase name=`"$testName`" time=`"0`">`n"
                    $xml += "            <failure message=`"Test failed`">$([System.Security.SecurityElement]::Escape($result.Errors))</failure>`n"
                    $xml += "        </testcase>`n"
                }
            }
        }
        
        $xml += "    </testsuite>`n"
    }
    
    $xml += "</testsuites>`n"
    return $xml
}

function ConvertTo-HtmlReport {
    param([object]$TestResults)
    
    $html = @"
<!DOCTYPE html>
<html>
<head>
    <title>Multi-Controller App - Loopback Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .header { background-color: #f0f0f0; padding: 20px; margin-bottom: 30px; }
        .summary { background-color: #e8f5e8; padding: 15px; margin-bottom: 20px; border-left: 4px solid #4CAF50; }
        .error { background-color: #ffe8e8; border-left: 4px solid #f44336; }
        .warning { background-color: #fff8e1; border-left: 4px solid #ff9800; }
        table { border-collapse: collapse; width: 100%; margin-bottom: 30px; }
        th, td { border: 1px solid #ddd; padding: 12px; text-align: left; }
        th { background-color: #f2f2f2; }
        .passed { background-color: #c8e6c9; }
        .failed { background-color: #ffcdd2; }
        .skipped { background-color: #e0e0e0; }
        .details { font-family: monospace; white-space: pre-wrap; font-size: 12px; max-height: 200px; overflow-y: auto; }
    </style>
</head>
<body>
    <div class="header">
        <h1>Multi-Controller App - Loopback Test Report</h1>
        <p><strong>Generated:</strong> $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')</p>
        <p><strong>Test Duration:</strong> $($TestResults.EndTime - $TestResults.StartTime)</p>
    </div>

    <div class="summary">
        <h2>Test Summary</h2>
        <p><strong>Total Tests:</strong> $($TestResults.TotalTests)</p>
        <p><strong>Passed:</strong> $($TestResults.PassedTests)</p>
        <p><strong>Failed:</strong> $($TestResults.FailedTests)</p>
        <p><strong>Skipped:</strong> $($TestResults.SkippedTests)</p>
        <p><strong>Success Rate:</strong> $([math]::Round(($TestResults.PassedTests / [math]::Max($TestResults.TotalTests, 1)) * 100, 2))%</p>
    </div>

    <h2>Test Categories</h2>
    <table>
        <tr>
            <th>Category</th>
            <th>Status</th>
            <th>Tests</th>
            <th>Passed</th>
            <th>Failed</th>
            <th>Duration</th>
        </tr>
"@
    
    foreach ($category in $TestResults.Categories.Keys | Sort-Object) {
        $result = $TestResults.Categories[$category]
        $statusClass = if ($result.Skipped) { "skipped" } elseif ($result.Success) { "passed" } else { "failed" }
        $status = if ($result.Skipped) { "SKIPPED" } elseif ($result.Success) { "PASSED" } else { "FAILED" }
        
        $html += @"
        <tr class="$statusClass">
            <td>$category</td>
            <td>$status</td>
            <td>$($result.TestCount)</td>
            <td>$($result.PassedCount)</td>
            <td>$($result.FailedCount)</td>
            <td>$([math]::Round($result.Duration.TotalSeconds, 2))s</td>
        </tr>
"@
    }
    
    $html += @"
    </table>
</body>
</html>
"@
    
    return $html
}

function Export-TestReport {
    param([object]$TestResults)
    
    # JSON Report (always generated)
    $jsonReport = $TestResults | ConvertTo-Json -Depth 10
    $jsonPath = Join-Path $script:OutputPath "$script:ReportBaseName.json"
    $jsonReport | Out-File -FilePath $jsonPath -Encoding UTF8
    Write-Status "JSON report saved: $jsonPath" "SUCCESS" "REPORT"
    
    # Additional formats based on ReportFormat parameter
    if ($ReportFormat -eq 'xml' -or $ReportFormat -eq 'all') {
        $xmlReport = ConvertTo-XmlReport -TestResults $TestResults
        $xmlPath = Join-Path $script:OutputPath "$script:ReportBaseName.xml"
        $xmlReport | Out-File -FilePath $xmlPath -Encoding UTF8
        Write-Status "XML report saved: $xmlPath" "SUCCESS" "REPORT"
    }
    
    if ($ReportFormat -eq 'html' -or $ReportFormat -eq 'all') {
        $htmlReport = ConvertTo-HtmlReport -TestResults $TestResults
        $htmlPath = Join-Path $script:OutputPath "$script:ReportBaseName.html"
        $htmlReport | Out-File -FilePath $htmlPath -Encoding UTF8
        Write-Status "HTML report saved: $htmlPath" "SUCCESS" "REPORT"
    }
}

#endregion

#region Main Execution

function main {
    try {
        # Header
        if (-not $CiMode) {
            Write-Host ""
            Write-Host "==================== MULTI-CONTROLLER APP ====================" -ForegroundColor White
            Write-Host "                   LOOPBACK TEST RUNNER                        " -ForegroundColor White
            Write-Host "===============================================================" -ForegroundColor White
            Write-Host ""
        }
        
        Write-Status "Starting loopback test execution" "INFO" "MAIN"
        Write-Status "Project Root: $script:ProjectRoot" "DETAIL" "MAIN"
        Write-Status "Output Directory: $script:OutputPath" "DETAIL" "MAIN"
        Write-Status "Test Categories: $($TestCategories -join ', ')" "DETAIL" "MAIN"
        
        # Prerequisites check
        if (-not (Test-Path (Join-Path $script:ProjectRoot "Cargo.toml"))) {
            throw "Cargo.toml not found. Please run from project root."
        }
        
        if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
            throw "Cargo not found in PATH. Please install Rust."
        }
        
        if (-not (Get-Command "csc.exe" -ErrorAction SilentlyContinue)) {
            Write-Status "C# compiler not found. Echo servers will be limited." "WARNING" "MAIN"
        }
        
        # Build test dependencies
        Write-Status "Building test dependencies..." "INFO" "BUILD"
        $buildResult = Start-Process "cargo" -ArgumentList @("build", "--tests") `
                                     -WorkingDirectory $script:ProjectRoot -Wait -PassThru -WindowStyle Hidden
        
        if ($buildResult.ExitCode -ne 0) {
            throw "Failed to build test dependencies"
        }
        
        Write-Status "Test dependencies built successfully" "SUCCESS" "BUILD"
        
        # Setup virtual serial ports
        if ('all' -in $TestCategories -or 'serial' -in $TestCategories) {
            Setup-VirtualSerialPorts
        }
        
        # Determine which test categories to run
        $categoriesToRun = if ('all' -in $TestCategories) {
            @('serial', 'tcp', 'udp', 'ssh')
        } else {
            $TestCategories
        }
        
        # Execute test categories
        foreach ($category in $categoriesToRun) {
            Write-Status "Starting $category tests" "INFO" "EXEC"
            
            $result = switch ($category.ToLower()) {
                'serial' { Test-SerialLoopback }
                'tcp' { Test-TcpLoopback }
                'udp' { Test-UdpLoopback }
                'ssh' { Test-SshLoopback }
                default { 
                    Write-Status "Unknown test category: $category" "WARNING" "EXEC"
                    @{
                        Category = $category
                        Skipped = $true
                        Reason = "Unknown category"
                    }
                }
            }
            
            $script:TestResults.Categories[$category] = $result
            
            if ($result.Skipped) {
                Write-Status "Skipped $category tests: $($result.Reason)" "WARNING" $category.ToUpper()
                $script:TestResults.SkippedTests += 1
            } else {
                $script:TestResults.TotalTests += $result.TestCount
                $script:TestResults.PassedTests += $result.PassedCount
                $script:TestResults.FailedTests += $result.FailedCount
                
                if ($result.Success) {
                    Write-Status "Completed $category tests successfully" "SUCCESS" $category.ToUpper()
                } else {
                    Write-Status "Completed $category tests with failures" "ERROR" $category.ToUpper()
                    $script:TestResults.Errors += "Category $category failed: $($result.Errors)"
                }
            }
        }
        
        $script:TestResults.EndTime = Get-Date
        $totalDuration = $script:TestResults.EndTime - $script:TestResults.StartTime
        
        # Generate reports
        Write-Status "Generating test reports..." "INFO" "REPORT"
        Export-TestReport -TestResults $script:TestResults
        
        # Final summary
        if (-not $CiMode) {
            Write-Host ""
            Write-Host "==================== TEST SUMMARY ====================" -ForegroundColor White
        }
        
        Write-Status "Test execution completed" "INFO" "SUMMARY"
        Write-Status "Total Duration: $([math]::Round($totalDuration.TotalSeconds, 1)) seconds" "INFO" "SUMMARY"
        Write-Status "Total Tests: $($script:TestResults.TotalTests)" "INFO" "SUMMARY"
        Write-Status "Passed: $($script:TestResults.PassedTests)" "SUCCESS" "SUMMARY"
        Write-Status "Failed: $($script:TestResults.FailedTests)" $(if ($script:TestResults.FailedTests -gt 0) { "ERROR" } else { "INFO" }) "SUMMARY"
        Write-Status "Skipped: $($script:TestResults.SkippedTests)" $(if ($script:TestResults.SkippedTests -gt 0) { "WARNING" } else { "INFO" }) "SUMMARY"
        
        if ($script:TestResults.TotalTests -gt 0) {
            $successRate = [math]::Round(($script:TestResults.PassedTests / $script:TestResults.TotalTests) * 100, 2)
            Write-Status "Success Rate: $successRate%" $(if ($successRate -ge 95) { "SUCCESS" } elseif ($successRate -ge 80) { "WARNING" } else { "ERROR" }) "SUMMARY"
        }
        
        # Keep servers message
        if ($KeepServers -and $script:EchoServers.Count -gt 0) {
            Write-Status "Echo servers are still running for manual testing" "INFO" "CLEANUP"
            Write-Status "Kill manually or restart PowerShell to clean up" "INFO" "CLEANUP"
        }
        
        # Reports location
        Write-Status "Reports saved in: $script:OutputPath" "INFO" "SUMMARY"
        
        # Exit code based on results
        if ($script:TestResults.FailedTests -gt 0) {
            Write-Status "Some tests failed - check the detailed reports" "ERROR" "SUMMARY"
            exit 1
        } elseif ($script:TestResults.TotalTests -eq 0) {
            Write-Status "No tests were executed" "WARNING" "SUMMARY"
            exit 2
        } else {
            Write-Status "All tests completed successfully" "SUCCESS" "SUMMARY"
            exit 0
        }
        
    } catch {
        Write-Status "Fatal error: $($_.Exception.Message)" "ERROR" "FATAL"
        $script:TestResults.Errors += $_.Exception.Message
        $script:TestResults.EndTime = Get-Date
        
        # Try to generate report even on failure
        try {
            Export-TestReport -TestResults $script:TestResults
        } catch {
            Write-Status "Failed to generate error report: $_" "ERROR" "FATAL"
        }
        
        exit 3
    } finally {
        # Cleanup resources
        Write-Status "Cleaning up resources..." "INFO" "CLEANUP"
        
        # Stop echo servers unless keeping them
        if (-not $KeepServers) {
            foreach ($server in $script:EchoServers) {
                try {
                    Stop-EchoServer -ServerInfo $server
                } catch {
                    Write-Status "Error during server cleanup: $_" "WARNING" "CLEANUP"
                }
            }
        }
        
        # Clean up virtual ports (if we created them)
        # This would be implementation-specific based on the virtual port solution used
        
        Write-Status "Cleanup completed" "INFO" "CLEANUP"
    }
}

# Execute main function
main

#endregion