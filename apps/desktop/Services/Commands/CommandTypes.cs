using System;
using System.Collections.Generic;
using System.Linq;

namespace MultiControllerApp.Services.Commands;

/// <summary>
/// Enum for all supported command types across different device types
/// </summary>
public enum CommandType
{
    // Digital I/O Commands
    DigitalWrite,
    DigitalRead,
    
    // Analog I/O Commands  
    AnalogWrite,
    AnalogRead,
    
    // PWM Commands
    SetPWM,
    SetPWMFrequency,
    
    // Motor Control Commands
    SetMotorSpeed,
    StopMotor,
    SetMotorDirection,
    
    // Relay Commands (RioRand)
    SetRelay,
    ToggleRelay,
    SetAllRelays,
    
    // Servo Commands
    SetServoPosition,
    SetServoSpeed,
    
    // I2C Commands
    I2CWrite,
    I2CRead,
    I2CScan,
    
    // SPI Commands
    SPIWrite,
    SPIRead,
    SPITransfer,
    
    // System Commands
    Reset,
    GetStatus,
    GetVersion,
    SetConfiguration,
    
    // Custom Commands (for extensibility)
    Custom,
    
    // Batch Commands
    Batch,
    
    // Emergency Commands
    EmergencyStop,
    GlobalStop
}

/// <summary>
/// Priority levels for command queue processing
/// </summary>
public enum CommandPriority
{
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
    Emergency = 4
}

/// <summary>
/// Command execution status
/// </summary>
public enum CommandStatus
{
    Pending,
    Queued,
    Transmitting,
    Transmitted,
    Acknowledged,
    Failed,
    Timeout,
    Cancelled,
    Retrying
}

/// <summary>
/// Represents a device command with parameters and metadata
/// </summary>
public class DeviceCommand
{
    public string Id { get; set; } = Guid.NewGuid().ToString("N")[..8];
    public CommandType Type { get; set; }
    public CommandPriority Priority { get; set; } = CommandPriority.Normal;
    public CommandStatus Status { get; set; } = CommandStatus.Pending;
    
    public string DeviceId { get; set; } = string.Empty;
    public string Endpoint { get; set; } = string.Empty;
    public Dictionary<string, object> Parameters { get; set; } = new();
    
    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;
    public DateTime? QueuedAt { get; set; }
    public DateTime? TransmittedAt { get; set; }
    public DateTime? AcknowledgedAt { get; set; }
    
    public int RetryCount { get; set; } = 0;
    public int MaxRetries { get; set; } = 3;
    public TimeSpan Timeout { get; set; } = TimeSpan.FromSeconds(5);
    
    public object? ExpectedResponse { get; set; }
    public object? ActualResponse { get; set; }
    public string? ErrorMessage { get; set; }
    
    public Dictionary<string, object> Metadata { get; set; } = new();
    
    /// <summary>
    /// Creates a batch command containing multiple sub-commands
    /// </summary>
    public static DeviceCommand CreateBatch(string deviceId, IEnumerable<DeviceCommand> commands, CommandPriority priority = CommandPriority.Normal)
    {
        return new DeviceCommand
        {
            Type = CommandType.Batch,
            DeviceId = deviceId,
            Priority = priority,
            Parameters = new Dictionary<string, object>
            {
                ["commands"] = commands.ToList()
            }
        };
    }
    
    /// <summary>
    /// Creates an emergency stop command with highest priority
    /// </summary>
    public static DeviceCommand CreateEmergencyStop(string deviceId)
    {
        return new DeviceCommand
        {
            Type = CommandType.EmergencyStop,
            DeviceId = deviceId,
            Priority = CommandPriority.Emergency,
            Timeout = TimeSpan.FromSeconds(1),
            MaxRetries = 0
        };
    }
    
    /// <summary>
    /// Creates a global stop command for all devices
    /// </summary>
    public static DeviceCommand CreateGlobalStop()
    {
        return new DeviceCommand
        {
            Type = CommandType.GlobalStop,
            DeviceId = "*",
            Priority = CommandPriority.Emergency,
            Timeout = TimeSpan.FromSeconds(1),
            MaxRetries = 0
        };
    }
}

/// <summary>
/// Result of command transmission including acknowledgment data
/// </summary>
public class CommandResult
{
    public DeviceCommand Command { get; set; } = null!;
    public bool Success { get; set; }
    public object? Response { get; set; }
    public string? ErrorMessage { get; set; }
    public TimeSpan ExecutionTime { get; set; }
    public DateTime CompletedAt { get; set; } = DateTime.UtcNow;
}

/// <summary>
/// Configuration for command serialization formats
/// </summary>
public class SerializationConfig
{
    public string Format { get; set; } = "json"; // json, binary, custom
    public Dictionary<string, object> FormatOptions { get; set; } = new();
    public string Encoding { get; set; } = "utf8";
    public bool IncludeChecksum { get; set; } = false;
    public string? ChecksumAlgorithm { get; set; }
}