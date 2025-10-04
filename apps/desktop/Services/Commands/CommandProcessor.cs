using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Services.Commands;

/// <summary>
/// Main command processor that orchestrates command queuing, transmission, and logging
/// </summary>
public class CommandProcessor : ICommandProcessor, IDisposable
{
    private readonly ILogger<CommandProcessor> _logger;
    private readonly CommandQueue _commandQueue;
    private readonly ICommandTransmitter _transmitter;
    private readonly ICommandHistory _history;
    private readonly ICommandSerializer _serializer;
    private readonly SemaphoreSlim _processingLock;
    private readonly CancellationTokenSource _cancellationTokenSource;
    private readonly Task _processingTask;
    private volatile bool _disposed;

    // Events
    public event EventHandler<CommandStatusChangedEventArgs>? CommandStatusChanged;
    public event EventHandler<CommandCompletedEventArgs>? CommandCompleted;

    public CommandProcessor(
        ILogger<CommandProcessor> logger,
        ICommandTransmitter transmitter,
        ICommandHistory history,
        ICommandSerializer serializer)
    {
        _logger = logger;
        _transmitter = transmitter;
        _history = history;
        _serializer = serializer;
        _processingLock = new SemaphoreSlim(1, 1);
        _cancellationTokenSource = new CancellationTokenSource();

        // Initialize command queue
        var queueLoggerFactory = LoggerFactory.Create(builder => builder.AddConsole());
        _commandQueue = new CommandQueue(queueLoggerFactory.CreateLogger<CommandQueue>());
        _commandQueue.CommandQueued += OnCommandQueued;
        _commandQueue.CommandDequeued += OnCommandDequeued;

        // Start background processing
        _processingTask = ProcessCommandsAsync(_cancellationTokenSource.Token);

        _logger.LogInformation("Command processor initialized and started");
    }

    public async Task<string> QueueCommandAsync(DeviceCommand command, CancellationToken cancellationToken = default)
    {
        if (_disposed)
            throw new ObjectDisposedException(nameof(CommandProcessor));

        try
        {
            _logger.LogDebug("Queuing command {CommandId} ({Type}) for device {DeviceId} with priority {Priority}",
                command.Id, command.Type, command.DeviceId, command.Priority);

            // Queue the command
            var commandId = await _commandQueue.QueueAsync(command, cancellationToken);

            // Add to history immediately for tracking
            await _history.AddAsync(command);

            // Notify status change
            OnCommandStatusChanged(command, CommandStatus.Pending, CommandStatus.Queued);

            return commandId;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to queue command {CommandId}", command.Id);
            throw;
        }
    }

    public async Task<IEnumerable<string>> QueueBatchAsync(IEnumerable<DeviceCommand> commands, CancellationToken cancellationToken = default)
    {
        if (_disposed)
            throw new ObjectDisposedException(nameof(CommandProcessor));

        var commandList = commands.ToList();
        _logger.LogDebug("Queuing batch of {Count} commands", commandList.Count);

        var commandIds = new List<string>();

        foreach (var command in commandList)
        {
            var commandId = await QueueCommandAsync(command, cancellationToken);
            commandIds.Add(commandId);
        }

        _logger.LogDebug("Successfully queued batch of {Count} commands", commandIds.Count);
        return commandIds;
    }

    public async Task<CommandResult> EmergencyStopAsync(string deviceId, CancellationToken cancellationToken = default)
    {
        if (_disposed)
            throw new ObjectDisposedException(nameof(CommandProcessor));

        _logger.LogWarning("Emergency stop requested for device {DeviceId}", deviceId);

        var emergencyCommand = DeviceCommand.CreateEmergencyStop(deviceId);
        
        try
        {
            // Clear existing queue for this device
            var clearedCount = await _commandQueue.ClearDeviceQueueAsync(deviceId);
            if (clearedCount > 0)
            {
                _logger.LogWarning("Cleared {Count} queued commands for device {DeviceId} due to emergency stop", 
                    clearedCount, deviceId);
            }

            // Transmit emergency stop immediately without queuing
            var result = await _transmitter.TransmitAsync(emergencyCommand, cancellationToken);
            
            // Add to history
            await _history.AddAsync(emergencyCommand);
            
            // Notify completion
            OnCommandCompleted(result);

            _logger.LogWarning("Emergency stop for device {DeviceId} completed with result: {Success}", 
                deviceId, result.Success);

            return result;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Emergency stop failed for device {DeviceId}", deviceId);
            
            var failedResult = new CommandResult
            {
                Command = emergencyCommand,
                Success = false,
                ErrorMessage = ex.Message,
                CompletedAt = DateTime.UtcNow
            };

            await _history.AddAsync(emergencyCommand);
            OnCommandCompleted(failedResult);
            
            return failedResult;
        }
    }

    public async Task<CommandResult> GlobalStopAsync(CancellationToken cancellationToken = default)
    {
        if (_disposed)
            throw new ObjectDisposedException(nameof(CommandProcessor));

        _logger.LogCritical("Global stop requested for all devices");

        var globalStopCommand = DeviceCommand.CreateGlobalStop();
        
        try
        {
            // Get all registered devices from transmitter
            var deviceStatistics = new Dictionary<string, TransmissionStatistics>();
            
            // For now, we'll need to implement a way to get all device IDs
            // This would typically come from a device manager or registry
            var deviceIds = await GetAllDeviceIdsAsync();

            var results = new List<CommandResult>();

            // Send emergency stop to all devices in parallel
            var stopTasks = deviceIds.Select(async deviceId =>
            {
                try
                {
                    return await EmergencyStopAsync(deviceId, cancellationToken);
                }
                catch (Exception ex)
                {
                    _logger.LogError(ex, "Failed to send emergency stop to device {DeviceId}", deviceId);
                    return new CommandResult
                    {
                        Command = DeviceCommand.CreateEmergencyStop(deviceId),
                        Success = false,
                        ErrorMessage = ex.Message,
                        CompletedAt = DateTime.UtcNow
                    };
                }
            });

            var individualResults = await Task.WhenAll(stopTasks);
            results.AddRange(individualResults);

            // Create summary result
            var successCount = results.Count(r => r.Success);
            var overallSuccess = successCount == results.Count;

            var globalResult = new CommandResult
            {
                Command = globalStopCommand,
                Success = overallSuccess,
                Response = new
                {
                    TotalDevices = results.Count,
                    SuccessfulStops = successCount,
                    FailedStops = results.Count - successCount,
                    Results = results.Select(r => new
                    {
                        r.Command.DeviceId,
                        r.Success,
                        r.ErrorMessage
                    })
                },
                ErrorMessage = overallSuccess ? null : $"Global stop partially failed: {successCount}/{results.Count} devices stopped successfully",
                CompletedAt = DateTime.UtcNow
            };

            await _history.AddAsync(globalStopCommand);
            OnCommandCompleted(globalResult);

            _logger.LogCritical("Global stop completed: {SuccessCount}/{TotalCount} devices stopped successfully", 
                successCount, results.Count);

            return globalResult;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Global stop failed");
            
            var failedResult = new CommandResult
            {
                Command = globalStopCommand,
                Success = false,
                ErrorMessage = ex.Message,
                CompletedAt = DateTime.UtcNow
            };

            await _history.AddAsync(globalStopCommand);
            OnCommandCompleted(failedResult);
            
            return failedResult;
        }
    }

    public async Task<DeviceCommand?> GetCommandStatusAsync(string commandId)
    {
        // Check if command is in queue
        var queuedCommands = _commandQueue.GetCommandsByDevice(""); // Need to search all devices
        var queuedCommand = queuedCommands.FirstOrDefault(cmd => cmd.Id == commandId);
        
        if (queuedCommand != null)
        {
            return queuedCommand;
        }

        // Check history for completed commands
        var recentCommands = await _history.GetRecentCommandsAsync(1000);
        return recentCommands.FirstOrDefault(cmd => cmd.Id == commandId);
    }

    public async Task<IEnumerable<DeviceCommand>> GetCommandHistoryAsync(string deviceId, int maxResults = 100)
    {
        return await _history.GetHistoryAsync(deviceId, maxResults);
    }

    public async Task<CommandQueueStatistics> GetQueueStatisticsAsync()
    {
        return await _commandQueue.GetStatisticsAsync();
    }

    public async Task<bool> CancelCommandAsync(string commandId)
    {
        _logger.LogDebug("Cancelling command {CommandId}", commandId);
        return await _commandQueue.CancelCommandAsync(commandId);
    }

    public async Task<int> ClearDeviceQueueAsync(string deviceId)
    {
        _logger.LogDebug("Clearing command queue for device {DeviceId}", deviceId);
        return await _commandQueue.ClearDeviceQueueAsync(deviceId);
    }

    /// <summary>
    /// Get comprehensive processor statistics
    /// </summary>
    public async Task<CommandProcessorStatistics> GetProcessorStatisticsAsync()
    {
        var queueStats = await GetQueueStatisticsAsync();
        var historyStats = await _history.GetStatisticsAsync();

        return new CommandProcessorStatistics
        {
            QueueStatistics = queueStats,
            HistoryStatistics = historyStats,
            ProcessorStartedAt = DateTime.UtcNow, // Would track actual start time
            TotalProcessedCommands = historyStats.TotalCommands,
            IsRunning = !_disposed
        };
    }

    private async Task ProcessCommandsAsync(CancellationToken cancellationToken)
    {
        _logger.LogInformation("Command processing task started");

        try
        {
            while (!cancellationToken.IsCancellationRequested)
            {
                try
                {
                    // Dequeue and process next command
                    var command = await _commandQueue.DequeueAsync(cancellationToken);
                    
                    if (command != null)
                    {
                        // Process command in background to avoid blocking the queue
                        _ = Task.Run(async () => await ProcessSingleCommandAsync(command, cancellationToken), cancellationToken);
                    }
                }
                catch (OperationCanceledException)
                {
                    // Expected during shutdown
                    break;
                }
                catch (Exception ex)
                {
                    _logger.LogError(ex, "Error in command processing loop");
                    await Task.Delay(TimeSpan.FromSeconds(1), cancellationToken);
                }
            }
        }
        catch (OperationCanceledException)
        {
            // Expected during shutdown
        }
        finally
        {
            _logger.LogInformation("Command processing task stopped");
        }
    }

    private async Task ProcessSingleCommandAsync(DeviceCommand command, CancellationToken cancellationToken)
    {
        try
        {
            _logger.LogDebug("Processing command {CommandId} ({Type}) for device {DeviceId}",
                command.Id, command.Type, command.DeviceId);

            OnCommandStatusChanged(command, command.Status, CommandStatus.Transmitting);

            // Transmit the command
            var result = await _transmitter.TransmitAsync(command, cancellationToken);

            // Update command with result
            command.Status = result.Success ? CommandStatus.Acknowledged : CommandStatus.Failed;
            command.ActualResponse = result.Response;
            command.ErrorMessage = result.ErrorMessage;

            if (result.Success)
            {
                command.AcknowledgedAt = DateTime.UtcNow;
            }

            // Update in history
            await _history.AddAsync(command);

            // Notify completion
            OnCommandCompleted(result);

            _logger.LogDebug("Command {CommandId} processed with result: {Success} in {ElapsedMs}ms",
                command.Id, result.Success, result.ExecutionTime.TotalMilliseconds);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error processing command {CommandId}", command.Id);
            
            command.Status = CommandStatus.Failed;
            command.ErrorMessage = ex.Message;
            
            await _history.AddAsync(command);
            
            var failedResult = new CommandResult
            {
                Command = command,
                Success = false,
                ErrorMessage = ex.Message,
                CompletedAt = DateTime.UtcNow
            };
            
            OnCommandCompleted(failedResult);
        }
    }

    private async Task<List<string>> GetAllDeviceIdsAsync()
    {
        // This would typically be implemented by querying a device manager or registry
        // For now, return empty list - would need integration with device discovery system
        return new List<string>();
    }

    private void OnCommandQueued(object? sender, CommandQueuedEventArgs e)
    {
        _logger.LogTrace("Command {CommandId} queued", e.Command.Id);
    }

    private void OnCommandDequeued(object? sender, CommandDequeuedEventArgs e)
    {
        _logger.LogTrace("Command {CommandId} dequeued", e.Command.Id);
    }

    private void OnCommandStatusChanged(DeviceCommand command, CommandStatus previousStatus, CommandStatus newStatus)
    {
        command.Status = newStatus;
        
        var eventArgs = new CommandStatusChangedEventArgs
        {
            Command = command,
            PreviousStatus = previousStatus,
            NewStatus = newStatus
        };

        CommandStatusChanged?.Invoke(this, eventArgs);
    }

    private void OnCommandCompleted(CommandResult result)
    {
        var eventArgs = new CommandCompletedEventArgs
        {
            Result = result
        };

        CommandCompleted?.Invoke(this, eventArgs);
    }

    public void Dispose()
    {
        if (_disposed)
            return;

        _disposed = true;

        _logger.LogInformation("Disposing command processor");

        _cancellationTokenSource.Cancel();

        try
        {
            _processingTask?.Wait(TimeSpan.FromSeconds(10));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error waiting for processing task to complete");
        }

        _commandQueue?.Dispose();
        _transmitter?.Dispose();
        _history?.Dispose();
        _processingLock?.Dispose();
        _cancellationTokenSource?.Dispose();

        _logger.LogInformation("Command processor disposed");
    }
}

/// <summary>
/// Comprehensive statistics for the command processor
/// </summary>
public class CommandProcessorStatistics
{
    public CommandQueueStatistics QueueStatistics { get; set; } = null!;
    public HistoryStatistics HistoryStatistics { get; set; } = null!;
    public DateTime ProcessorStartedAt { get; set; }
    public int TotalProcessedCommands { get; set; }
    public bool IsRunning { get; set; }
}