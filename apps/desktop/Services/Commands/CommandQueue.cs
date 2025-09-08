using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Channels;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Services.Commands;

/// <summary>
/// Priority-based command queue with async processing and batch support
/// </summary>
public class CommandQueue : IDisposable
{
    private readonly ILogger<CommandQueue> _logger;
    private readonly ConcurrentDictionary<string, DeviceCommand> _queuedCommands;
    private readonly ConcurrentDictionary<CommandPriority, Channel<string>> _priorityChannels;
    private readonly ConcurrentDictionary<string, CancellationTokenSource> _commandCancellationTokens;
    private readonly SemaphoreSlim _processingLock;
    private readonly CancellationTokenSource _shutdownToken;
    private readonly Timer _statsTimer;
    
    private CommandQueueStatistics _statistics;
    private volatile bool _disposed;
    private Task? _processingTask;

    public event EventHandler<CommandQueuedEventArgs>? CommandQueued;
    public event EventHandler<CommandDequeuedEventArgs>? CommandDequeued;
    public event EventHandler<QueueStatisticsUpdatedEventArgs>? StatisticsUpdated;

    public CommandQueue(ILogger<CommandQueue> logger)
    {
        _logger = logger;
        _queuedCommands = new ConcurrentDictionary<string, DeviceCommand>();
        _priorityChannels = new ConcurrentDictionary<CommandPriority, Channel<string>>();
        _commandCancellationTokens = new ConcurrentDictionary<string, CancellationTokenSource>();
        _processingLock = new SemaphoreSlim(1, 1);
        _shutdownToken = new CancellationTokenSource();
        _statistics = new CommandQueueStatistics();

        InitializePriorityChannels();
        
        // Update statistics every 5 seconds
        _statsTimer = new Timer(UpdateStatistics, null, TimeSpan.FromSeconds(5), TimeSpan.FromSeconds(5));
        
        _processingTask = ProcessQueueAsync(_shutdownToken.Token);
    }

    /// <summary>
    /// Queue a command with priority handling
    /// </summary>
    public async Task<string> QueueAsync(DeviceCommand command, CancellationToken cancellationToken = default)
    {
        if (_disposed)
            throw new ObjectDisposedException(nameof(CommandQueue));

        try
        {
            command.Status = CommandStatus.Queued;
            command.QueuedAt = DateTime.UtcNow;

            // Store the command
            _queuedCommands[command.Id] = command;

            // Create cancellation token for this command
            var commandCts = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken, _shutdownToken.Token);
            _commandCancellationTokens[command.Id] = commandCts;

            // Queue in appropriate priority channel
            var channel = _priorityChannels[command.Priority];
            await channel.Writer.WriteAsync(command.Id, commandCts.Token);

            _logger.LogDebug("Queued command {CommandId} ({Type}) with {Priority} priority for device {DeviceId}", 
                command.Id, command.Type, command.Priority, command.DeviceId);

            CommandQueued?.Invoke(this, new CommandQueuedEventArgs { Command = command });
            
            // Update statistics immediately for real-time accuracy
            UpdateStatistics(null);

            return command.Id;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to queue command {CommandId} ({Type})", command.Id, command.Type);
            throw;
        }
    }

    /// <summary>
    /// Queue multiple commands as a batch
    /// </summary>
    public async Task<IEnumerable<string>> QueueBatchAsync(IEnumerable<DeviceCommand> commands, CancellationToken cancellationToken = default)
    {
        var commandIds = new List<string>();
        
        foreach (var command in commands)
        {
            var commandId = await QueueAsync(command, cancellationToken);
            commandIds.Add(commandId);
        }

        _logger.LogDebug("Queued batch of {Count} commands", commandIds.Count);
        return commandIds;
    }

    /// <summary>
    /// Dequeue the next command based on priority
    /// </summary>
    public async Task<DeviceCommand?> DequeueAsync(CancellationToken cancellationToken = default)
    {
        if (_disposed)
            return null;

        try
        {
            // Check channels in priority order (Emergency first, Low last)
            foreach (var priority in Enum.GetValues<CommandPriority>().OrderByDescending(p => (int)p))
            {
                var channel = _priorityChannels[priority];
                
                if (channel.Reader.TryRead(out var commandId))
                {
                    if (_queuedCommands.TryRemove(commandId, out var command))
                    {
                        _commandCancellationTokens.TryRemove(commandId, out var cts);
                        cts?.Dispose();

                        command.Status = CommandStatus.Transmitting;
                        
                        _logger.LogDebug("Dequeued command {CommandId} ({Type}) with {Priority} priority", 
                            command.Id, command.Type, command.Priority);

                        CommandDequeued?.Invoke(this, new CommandDequeuedEventArgs { Command = command });
                        
                        return command;
                    }
                }
            }

            // If no commands available immediately, wait for one with priority order
            using var linkedCts = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken, _shutdownToken.Token);
            
            var readTasks = _priorityChannels
                .OrderByDescending(kvp => (int)kvp.Key)
                .Select(kvp => ReadFromChannelAsync(kvp.Value, kvp.Key, linkedCts.Token))
                .ToArray();

            var completedTask = await Task.WhenAny(readTasks);
            var result = await completedTask;

            // Cancel other tasks
            linkedCts.Cancel();

            return result;
        }
        catch (OperationCanceledException)
        {
            return null;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error dequeuing command");
            return null;
        }
    }

    /// <summary>
    /// Cancel a queued command
    /// </summary>
    public async Task<bool> CancelCommandAsync(string commandId)
    {
        if (_queuedCommands.TryRemove(commandId, out var command))
        {
            command.Status = CommandStatus.Cancelled;
            
            if (_commandCancellationTokens.TryRemove(commandId, out var cts))
            {
                cts.Cancel();
                cts.Dispose();
            }

            _logger.LogDebug("Cancelled command {CommandId} ({Type})", command.Id, command.Type);
            return true;
        }

        return false;
    }

    /// <summary>
    /// Clear all queued commands for a device
    /// </summary>
    public async Task<int> ClearDeviceQueueAsync(string deviceId)
    {
        var cancelledCount = 0;
        var commandsToCancel = _queuedCommands.Values
            .Where(cmd => cmd.DeviceId == deviceId)
            .ToList();

        foreach (var command in commandsToCancel)
        {
            if (await CancelCommandAsync(command.Id))
            {
                cancelledCount++;
            }
        }

        _logger.LogDebug("Cancelled {Count} queued commands for device {DeviceId}", cancelledCount, deviceId);
        return cancelledCount;
    }

    /// <summary>
    /// Get current queue statistics
    /// </summary>
    public async Task<CommandQueueStatistics> GetStatisticsAsync()
    {
        await _processingLock.WaitAsync();
        try
        {
            return new CommandQueueStatistics
            {
                QueuedCommands = _statistics.QueuedCommands,
                ProcessingCommands = _statistics.ProcessingCommands,
                CompletedCommands = _statistics.CompletedCommands,
                FailedCommands = _statistics.FailedCommands,
                AverageProcessingTime = _statistics.AverageProcessingTime,
                LastProcessedAt = _statistics.LastProcessedAt,
                CommandsByPriority = new Dictionary<CommandPriority, int>(_statistics.CommandsByPriority),
                CommandsByDevice = new Dictionary<string, int>(_statistics.CommandsByDevice)
            };
        }
        finally
        {
            _processingLock.Release();
        }
    }

    /// <summary>
    /// Get count of queued commands
    /// </summary>
    public int Count => _queuedCommands.Count;

    /// <summary>
    /// Check if queue is empty
    /// </summary>
    public bool IsEmpty => _queuedCommands.IsEmpty;

    /// <summary>
    /// Get commands by device ID
    /// </summary>
    public IEnumerable<DeviceCommand> GetCommandsByDevice(string deviceId)
    {
        return _queuedCommands.Values.Where(cmd => cmd.DeviceId == deviceId);
    }

    /// <summary>
    /// Get commands by priority
    /// </summary>
    public IEnumerable<DeviceCommand> GetCommandsByPriority(CommandPriority priority)
    {
        return _queuedCommands.Values.Where(cmd => cmd.Priority == priority);
    }

    private void InitializePriorityChannels()
    {
        foreach (var priority in Enum.GetValues<CommandPriority>())
        {
            var options = new BoundedChannelOptions(1000) // Reasonable limit per priority
            {
                FullMode = BoundedChannelFullMode.Wait,
                SingleReader = true,
                SingleWriter = false
            };
            
            _priorityChannels[priority] = Channel.CreateBounded<string>(options);
        }
    }

    private async Task<DeviceCommand?> ReadFromChannelAsync(Channel<string> channel, CommandPriority priority, CancellationToken cancellationToken)
    {
        try
        {
            var commandId = await channel.Reader.ReadAsync(cancellationToken);
            
            if (_queuedCommands.TryRemove(commandId, out var command))
            {
                _commandCancellationTokens.TryRemove(commandId, out var cts);
                cts?.Dispose();

                command.Status = CommandStatus.Transmitting;
                
                _logger.LogDebug("Dequeued command {CommandId} ({Type}) with {Priority} priority", 
                    command.Id, command.Type, priority);

                CommandDequeued?.Invoke(this, new CommandDequeuedEventArgs { Command = command });
                
                return command;
            }
        }
        catch (OperationCanceledException)
        {
            // Expected when cancellation is requested
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error reading from {Priority} priority channel", priority);
        }

        return null;
    }

    private async Task ProcessQueueAsync(CancellationToken cancellationToken)
    {
        _logger.LogInformation("Command queue processing started");

        try
        {
            while (!cancellationToken.IsCancellationRequested)
            {
                // The actual processing is handled by the CommandProcessor
                // This task just keeps the queue alive and handles cleanup
                await Task.Delay(TimeSpan.FromSeconds(1), cancellationToken);
                
                // Clean up expired commands
                await CleanupExpiredCommandsAsync();
            }
        }
        catch (OperationCanceledException)
        {
            // Expected during shutdown
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error in command queue processing task");
        }
        finally
        {
            _logger.LogInformation("Command queue processing stopped");
        }
    }

    private async Task CleanupExpiredCommandsAsync()
    {
        var expiredCommands = _queuedCommands.Values
            .Where(cmd => DateTime.UtcNow - cmd.CreatedAt > TimeSpan.FromMinutes(10)) // 10 minute timeout
            .ToList();

        foreach (var command in expiredCommands)
        {
            await CancelCommandAsync(command.Id);
            _logger.LogWarning("Cancelled expired command {CommandId} ({Type}) for device {DeviceId}", 
                command.Id, command.Type, command.DeviceId);
        }
    }

    private void UpdateStatistics(object? state)
    {
        try
        {
            var newStats = new CommandQueueStatistics
            {
                QueuedCommands = _queuedCommands.Count,
                LastProcessedAt = DateTime.UtcNow
            };

            // Group by priority
            foreach (var priority in Enum.GetValues<CommandPriority>())
            {
                newStats.CommandsByPriority[priority] = _queuedCommands.Values.Count(cmd => cmd.Priority == priority);
            }

            // Group by device
            var deviceGroups = _queuedCommands.Values.GroupBy(cmd => cmd.DeviceId);
            foreach (var group in deviceGroups)
            {
                newStats.CommandsByDevice[group.Key] = group.Count();
            }

            _statistics = newStats;
            StatisticsUpdated?.Invoke(this, new QueueStatisticsUpdatedEventArgs { Statistics = newStats });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error updating queue statistics");
        }
    }

    public void Dispose()
    {
        if (_disposed)
            return;

        _disposed = true;

        _shutdownToken.Cancel();
        _statsTimer?.Dispose();

        // Cancel all queued commands
        foreach (var kvp in _commandCancellationTokens)
        {
            kvp.Value.Cancel();
            kvp.Value.Dispose();
        }

        // Close all channels
        foreach (var channel in _priorityChannels.Values)
        {
            channel.Writer.Complete();
        }

        try
        {
            _processingTask?.Wait(TimeSpan.FromSeconds(5));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error waiting for processing task to complete");
        }

        _processingLock.Dispose();
        _shutdownToken.Dispose();

        _logger.LogInformation("Command queue disposed");
    }
}

/// <summary>
/// Event args for command queued events
/// </summary>
public class CommandQueuedEventArgs : EventArgs
{
    public DeviceCommand Command { get; set; } = null!;
}

/// <summary>
/// Event args for command dequeued events
/// </summary>
public class CommandDequeuedEventArgs : EventArgs
{
    public DeviceCommand Command { get; set; } = null!;
}

/// <summary>
/// Event args for queue statistics updated events
/// </summary>
public class QueueStatisticsUpdatedEventArgs : EventArgs
{
    public CommandQueueStatistics Statistics { get; set; } = null!;
}