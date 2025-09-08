using System;
using System.Threading;
using System.Threading.Tasks;
using System.Collections.Generic;

namespace MultiControllerApp.Services.Commands;

/// <summary>
/// Main interface for command processing and transmission
/// </summary>
public interface ICommandProcessor
{
    /// <summary>
    /// Queue a command for transmission with priority handling
    /// </summary>
    Task<string> QueueCommandAsync(DeviceCommand command, CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Queue multiple commands for batch transmission
    /// </summary>
    Task<IEnumerable<string>> QueueBatchAsync(IEnumerable<DeviceCommand> commands, CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Send emergency stop command with highest priority
    /// </summary>
    Task<CommandResult> EmergencyStopAsync(string deviceId, CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Send global stop command to all devices
    /// </summary>
    Task<CommandResult> GlobalStopAsync(CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Get command status by ID
    /// </summary>
    Task<DeviceCommand?> GetCommandStatusAsync(string commandId);
    
    /// <summary>
    /// Get command history for a device (limited by buffer size)
    /// </summary>
    Task<IEnumerable<DeviceCommand>> GetCommandHistoryAsync(string deviceId, int maxResults = 100);
    
    /// <summary>
    /// Get queue statistics
    /// </summary>
    Task<CommandQueueStatistics> GetQueueStatisticsAsync();
    
    /// <summary>
    /// Cancel a queued command
    /// </summary>
    Task<bool> CancelCommandAsync(string commandId);
    
    /// <summary>
    /// Clear all queued commands for a device
    /// </summary>
    Task<int> ClearDeviceQueueAsync(string deviceId);
    
    /// <summary>
    /// Event fired when command status changes
    /// </summary>
    event EventHandler<CommandStatusChangedEventArgs> CommandStatusChanged;
    
    /// <summary>
    /// Event fired when a command is completed (success or failure)
    /// </summary>
    event EventHandler<CommandCompletedEventArgs> CommandCompleted;
}

/// <summary>
/// Interface for command serialization to wire format
/// </summary>
public interface ICommandSerializer
{
    /// <summary>
    /// Serialize command to bytes for transmission
    /// </summary>
    Task<byte[]> SerializeAsync(DeviceCommand command, SerializationConfig config);
    
    /// <summary>
    /// Deserialize response bytes to object
    /// </summary>
    Task<object?> DeserializeResponseAsync(byte[] data, SerializationConfig config, Type expectedType);
    
    /// <summary>
    /// Get serialization configuration for a device type
    /// </summary>
    SerializationConfig GetSerializationConfig(string deviceType);
    
    /// <summary>
    /// Validate command parameters against device schema
    /// </summary>
    Task<ValidationResult> ValidateCommandAsync(DeviceCommand command, string deviceType);
}

/// <summary>
/// Interface for command transmission with retry logic
/// </summary>
public interface ICommandTransmitter : IDisposable
{
    /// <summary>
    /// Transmit command with acknowledgment and retry logic
    /// </summary>
    Task<CommandResult> TransmitAsync(DeviceCommand command, CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Transmit batch of commands
    /// </summary>
    Task<IEnumerable<CommandResult>> TransmitBatchAsync(IEnumerable<DeviceCommand> commands, CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Check if device is available for command transmission
    /// </summary>
    Task<bool> IsDeviceAvailableAsync(string deviceId);
    
    /// <summary>
    /// Get transmission statistics for a device
    /// </summary>
    Task<TransmissionStatistics> GetTransmissionStatisticsAsync(string deviceId);
}

/// <summary>
/// Interface for command history management
/// </summary>
public interface ICommandHistory : IDisposable
{
    /// <summary>
    /// Add command to history buffer
    /// </summary>
    Task AddAsync(DeviceCommand command);
    
    /// <summary>
    /// Get command history for device
    /// </summary>
    Task<IEnumerable<DeviceCommand>> GetHistoryAsync(string deviceId, int maxResults = 100);
    
    /// <summary>
    /// Get recent commands across all devices
    /// </summary>
    Task<IEnumerable<DeviceCommand>> GetRecentCommandsAsync(int maxResults = 50);
    
    /// <summary>
    /// Clear history for device
    /// </summary>
    Task ClearHistoryAsync(string deviceId);
    
    /// <summary>
    /// Get history statistics
    /// </summary>
    Task<HistoryStatistics> GetStatisticsAsync();
}

/// <summary>
/// Command queue statistics
/// </summary>
public class CommandQueueStatistics
{
    public int QueuedCommands { get; set; }
    public int ProcessingCommands { get; set; }
    public int CompletedCommands { get; set; }
    public int FailedCommands { get; set; }
    public TimeSpan AverageProcessingTime { get; set; }
    public DateTime LastProcessedAt { get; set; }
    public Dictionary<CommandPriority, int> CommandsByPriority { get; set; } = new();
    public Dictionary<string, int> CommandsByDevice { get; set; } = new();
}

/// <summary>
/// Transmission statistics for monitoring
/// </summary>
public class TransmissionStatistics
{
    public string DeviceId { get; set; } = string.Empty;
    public int TotalCommands { get; set; }
    public int SuccessfulCommands { get; set; }
    public int FailedCommands { get; set; }
    public int RetriedCommands { get; set; }
    public TimeSpan AverageLatency { get; set; }
    public DateTime LastTransmissionAt { get; set; }
    public double SuccessRate => TotalCommands > 0 ? (double)SuccessfulCommands / TotalCommands : 0;
}

/// <summary>
/// Command history statistics
/// </summary>
public class HistoryStatistics
{
    public int TotalCommands { get; set; }
    public int BufferSize { get; set; }
    public int BufferCapacity { get; set; }
    public DateTime OldestCommandAt { get; set; }
    public DateTime NewestCommandAt { get; set; }
    public Dictionary<CommandType, int> CommandsByType { get; set; } = new();
    public Dictionary<string, int> CommandsByDevice { get; set; } = new();
}

/// <summary>
/// Command validation result
/// </summary>
public class ValidationResult
{
    public bool IsValid { get; set; }
    public List<string> Errors { get; set; } = new();
    public List<string> Warnings { get; set; } = new();
}

/// <summary>
/// Event args for command status changes
/// </summary>
public class CommandStatusChangedEventArgs : EventArgs
{
    public DeviceCommand Command { get; set; } = null!;
    public CommandStatus PreviousStatus { get; set; }
    public CommandStatus NewStatus { get; set; }
    public DateTime Timestamp { get; set; } = DateTime.UtcNow;
}

/// <summary>
/// Event args for command completion
/// </summary>
public class CommandCompletedEventArgs : EventArgs
{
    public CommandResult Result { get; set; } = null!;
    public DateTime Timestamp { get; set; } = DateTime.UtcNow;
}