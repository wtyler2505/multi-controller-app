using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Services.Commands;

/// <summary>
/// Command transmitter with retry logic, exponential backoff, and acknowledgment handling
/// </summary>
public class CommandTransmitter : ICommandTransmitter
{
    private readonly ILogger<CommandTransmitter> _logger;
    private readonly ICommandSerializer _serializer;
    private readonly ConcurrentDictionary<string, IDeviceTransport> _deviceTransports;
    private readonly ConcurrentDictionary<string, TransmissionStatistics> _deviceStatistics;
    private readonly ConcurrentDictionary<string, SemaphoreSlim> _deviceLocks;
    private readonly Timer _statisticsTimer;

    // Retry configuration
    private readonly TimeSpan _baseDelay = TimeSpan.FromMilliseconds(100);
    private readonly TimeSpan _maxDelay = TimeSpan.FromSeconds(30);
    private readonly double _backoffMultiplier = 2.0;
    private readonly double _jitterFactor = 0.1;
    private readonly Random _random = new();

    public CommandTransmitter(
        ILogger<CommandTransmitter> logger,
        ICommandSerializer serializer)
    {
        _logger = logger;
        _serializer = serializer;
        _deviceTransports = new ConcurrentDictionary<string, IDeviceTransport>();
        _deviceStatistics = new ConcurrentDictionary<string, TransmissionStatistics>();
        _deviceLocks = new ConcurrentDictionary<string, SemaphoreSlim>();

        // Update statistics every 30 seconds
        _statisticsTimer = new Timer(UpdateStatistics, null, TimeSpan.FromSeconds(30), TimeSpan.FromSeconds(30));
    }

    public async Task<CommandResult> TransmitAsync(DeviceCommand command, CancellationToken cancellationToken = default)
    {
        var stopwatch = Stopwatch.StartNew();
        var deviceLock = _deviceLocks.GetOrAdd(command.DeviceId, _ => new SemaphoreSlim(1, 1));
        var stats = GetOrCreateStatistics(command.DeviceId);

        await deviceLock.WaitAsync(cancellationToken);
        try
        {
            _logger.LogDebug("Starting transmission of command {CommandId} ({Type}) to device {DeviceId}", 
                command.Id, command.Type, command.DeviceId);

            command.Status = CommandStatus.Transmitting;
            command.TransmittedAt = DateTime.UtcNow;

            var result = await TransmitWithRetryAsync(command, cancellationToken);
            
            stopwatch.Stop();
            result.ExecutionTime = stopwatch.Elapsed;

            // Update statistics
            UpdateTransmissionStatistics(stats, result);

            _logger.LogDebug("Completed transmission of command {CommandId} in {ElapsedMs}ms with result: {Success}", 
                command.Id, stopwatch.ElapsedMilliseconds, result.Success);

            return result;
        }
        finally
        {
            deviceLock.Release();
        }
    }

    public async Task<IEnumerable<CommandResult>> TransmitBatchAsync(IEnumerable<DeviceCommand> commands, CancellationToken cancellationToken = default)
    {
        var results = new List<CommandResult>();
        var groupedCommands = commands.GroupBy(cmd => cmd.DeviceId);

        foreach (var deviceGroup in groupedCommands)
        {
            var deviceCommands = deviceGroup.ToList();
            _logger.LogDebug("Transmitting batch of {Count} commands to device {DeviceId}", 
                deviceCommands.Count, deviceGroup.Key);

            // Check if device supports batch commands
            if (await SupportsBatchTransmissionAsync(deviceGroup.Key))
            {
                var batchCommand = DeviceCommand.CreateBatch(deviceGroup.Key, deviceCommands);
                var batchResult = await TransmitAsync(batchCommand, cancellationToken);
                
                // Convert batch result to individual results
                foreach (var cmd in deviceCommands)
                {
                    results.Add(new CommandResult
                    {
                        Command = cmd,
                        Success = batchResult.Success,
                        Response = batchResult.Response,
                        ErrorMessage = batchResult.ErrorMessage,
                        ExecutionTime = batchResult.ExecutionTime,
                        CompletedAt = batchResult.CompletedAt
                    });
                }
            }
            else
            {
                // Transmit commands sequentially
                foreach (var command in deviceCommands)
                {
                    var result = await TransmitAsync(command, cancellationToken);
                    results.Add(result);
                }
            }
        }

        return results;
    }

    public async Task<bool> IsDeviceAvailableAsync(string deviceId)
    {
        if (!_deviceTransports.TryGetValue(deviceId, out var transport))
        {
            return false;
        }

        try
        {
            return await transport.IsConnectedAsync();
        }
        catch (Exception ex)
        {
            _logger.LogWarning(ex, "Error checking availability of device {DeviceId}", deviceId);
            return false;
        }
    }

    public async Task<TransmissionStatistics> GetTransmissionStatisticsAsync(string deviceId)
    {
        var stats = GetOrCreateStatistics(deviceId);
        
        return new TransmissionStatistics
        {
            DeviceId = stats.DeviceId,
            TotalCommands = stats.TotalCommands,
            SuccessfulCommands = stats.SuccessfulCommands,
            FailedCommands = stats.FailedCommands,
            RetriedCommands = stats.RetriedCommands,
            AverageLatency = stats.AverageLatency,
            LastTransmissionAt = stats.LastTransmissionAt
        };
    }

    /// <summary>
    /// Register a transport for a device
    /// </summary>
    public void RegisterDeviceTransport(string deviceId, IDeviceTransport transport)
    {
        _deviceTransports[deviceId] = transport;
        _logger.LogDebug("Registered transport for device {DeviceId}", deviceId);
    }

    /// <summary>
    /// Unregister a transport for a device
    /// </summary>
    public void UnregisterDeviceTransport(string deviceId)
    {
        _deviceTransports.TryRemove(deviceId, out _);
        _deviceLocks.TryRemove(deviceId, out var semaphore);
        semaphore?.Dispose();
        _logger.LogDebug("Unregistered transport for device {DeviceId}", deviceId);
    }

    private async Task<CommandResult> TransmitWithRetryAsync(DeviceCommand command, CancellationToken cancellationToken)
    {
        var attempts = 0;
        var maxAttempts = command.MaxRetries + 1;
        Exception? lastException = null;

        while (attempts < maxAttempts && !cancellationToken.IsCancellationRequested)
        {
            try
            {
                attempts++;
                command.RetryCount = attempts - 1;

                if (attempts > 1)
                {
                    command.Status = CommandStatus.Retrying;
                    var delay = CalculateRetryDelay(attempts - 1);
                    
                    _logger.LogDebug("Retrying command {CommandId} (attempt {Attempt}/{MaxAttempts}) after {DelayMs}ms delay", 
                        command.Id, attempts, maxAttempts, delay.TotalMilliseconds);
                    
                    await Task.Delay(delay, cancellationToken);
                }

                var result = await AttemptTransmissionAsync(command, cancellationToken);
                
                if (result.Success)
                {
                    command.Status = CommandStatus.Acknowledged;
                    command.AcknowledgedAt = DateTime.UtcNow;
                    command.ActualResponse = result.Response;
                    return result;
                }
                else
                {
                    lastException = new InvalidOperationException(result.ErrorMessage);
                    command.ErrorMessage = result.ErrorMessage;
                }
            }
            catch (OperationCanceledException)
            {
                command.Status = CommandStatus.Cancelled;
                return new CommandResult
                {
                    Command = command,
                    Success = false,
                    ErrorMessage = "Command transmission was cancelled"
                };
            }
            catch (Exception ex)
            {
                lastException = ex;
                command.ErrorMessage = ex.Message;
                
                _logger.LogWarning(ex, "Transmission attempt {Attempt} failed for command {CommandId}", 
                    attempts, command.Id);
            }
        }

        // All attempts failed
        command.Status = CommandStatus.Failed;
        var finalResult = new CommandResult
        {
            Command = command,
            Success = false,
            ErrorMessage = lastException?.Message ?? "Command transmission failed after all retry attempts"
        };

        _logger.LogError("Command {CommandId} failed after {Attempts} attempts: {Error}", 
            command.Id, attempts, finalResult.ErrorMessage);

        return finalResult;
    }

    private async Task<CommandResult> AttemptTransmissionAsync(DeviceCommand command, CancellationToken cancellationToken)
    {
        if (!_deviceTransports.TryGetValue(command.DeviceId, out var transport))
        {
            return new CommandResult
            {
                Command = command,
                Success = false,
                ErrorMessage = $"No transport registered for device {command.DeviceId}"
            };
        }

        try
        {
            // Check if device is available
            if (!await transport.IsConnectedAsync())
            {
                return new CommandResult
                {
                    Command = command,
                    Success = false,
                    ErrorMessage = $"Device {command.DeviceId} is not connected"
                };
            }

            // Get serialization config for device
            var deviceType = await transport.GetDeviceTypeAsync();
            var serializationConfig = _serializer.GetSerializationConfig(deviceType);

            // Validate command
            var validation = await _serializer.ValidateCommandAsync(command, deviceType);
            if (!validation.IsValid)
            {
                return new CommandResult
                {
                    Command = command,
                    Success = false,
                    ErrorMessage = $"Command validation failed: {string.Join(", ", validation.Errors)}"
                };
            }

            // Serialize command
            var serializedData = await _serializer.SerializeAsync(command, serializationConfig);

            // Transmit with timeout
            using var timeoutCts = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
            timeoutCts.CancelAfter(command.Timeout);

            var response = await transport.SendAndReceiveAsync(serializedData, timeoutCts.Token);

            // Deserialize response if expected
            object? deserializedResponse = null;
            if (command.ExpectedResponse != null && response?.Length > 0)
            {
                var expectedType = command.ExpectedResponse.GetType();
                deserializedResponse = await _serializer.DeserializeResponseAsync(response, serializationConfig, expectedType);
            }

            return new CommandResult
            {
                Command = command,
                Success = true,
                Response = deserializedResponse ?? (response?.Length > 0 ? response : null)
            };
        }
        catch (OperationCanceledException) when (cancellationToken.IsCancellationRequested)
        {
            throw; // Re-throw cancellation
        }
        catch (OperationCanceledException)
        {
            return new CommandResult
            {
                Command = command,
                Success = false,
                ErrorMessage = $"Command timed out after {command.Timeout.TotalSeconds} seconds"
            };
        }
        catch (Exception ex)
        {
            return new CommandResult
            {
                Command = command,
                Success = false,
                ErrorMessage = ex.Message
            };
        }
    }

    private TimeSpan CalculateRetryDelay(int retryAttempt)
    {
        // Exponential backoff with jitter
        var exponentialDelay = TimeSpan.FromMilliseconds(_baseDelay.TotalMilliseconds * Math.Pow(_backoffMultiplier, retryAttempt));
        
        // Cap at maximum delay
        if (exponentialDelay > _maxDelay)
        {
            exponentialDelay = _maxDelay;
        }

        // Add jitter to avoid thundering herd
        var jitterMs = exponentialDelay.TotalMilliseconds * _jitterFactor * (_random.NextDouble() * 2 - 1);
        var finalDelay = exponentialDelay.Add(TimeSpan.FromMilliseconds(jitterMs));

        // Ensure minimum delay
        if (finalDelay < _baseDelay)
        {
            finalDelay = _baseDelay;
        }

        return finalDelay;
    }

    private async Task<bool> SupportsBatchTransmissionAsync(string deviceId)
    {
        if (!_deviceTransports.TryGetValue(deviceId, out var transport))
        {
            return false;
        }

        try
        {
            return await transport.SupportsBatchCommandsAsync();
        }
        catch (Exception ex)
        {
            _logger.LogWarning(ex, "Error checking batch support for device {DeviceId}", deviceId);
            return false;
        }
    }

    private TransmissionStatistics GetOrCreateStatistics(string deviceId)
    {
        return _deviceStatistics.GetOrAdd(deviceId, id => new TransmissionStatistics
        {
            DeviceId = id,
            TotalCommands = 0,
            SuccessfulCommands = 0,
            FailedCommands = 0,
            RetriedCommands = 0,
            AverageLatency = TimeSpan.Zero,
            LastTransmissionAt = DateTime.MinValue
        });
    }

    private void UpdateTransmissionStatistics(TransmissionStatistics stats, CommandResult result)
    {
        stats.TotalCommands++;
        stats.LastTransmissionAt = DateTime.UtcNow;

        if (result.Success)
        {
            stats.SuccessfulCommands++;
        }
        else
        {
            stats.FailedCommands++;
        }

        if (result.Command.RetryCount > 0)
        {
            stats.RetriedCommands++;
        }

        // Update average latency (simple moving average)
        var totalLatency = stats.AverageLatency.TotalMilliseconds * (stats.TotalCommands - 1) + result.ExecutionTime.TotalMilliseconds;
        stats.AverageLatency = TimeSpan.FromMilliseconds(totalLatency / stats.TotalCommands);
    }

    private void UpdateStatistics(object? state)
    {
        try
        {
            var totalStats = new
            {
                TotalDevices = _deviceStatistics.Count,
                TotalCommands = _deviceStatistics.Values.Sum(s => s.TotalCommands),
                SuccessfulCommands = _deviceStatistics.Values.Sum(s => s.SuccessfulCommands),
                FailedCommands = _deviceStatistics.Values.Sum(s => s.FailedCommands),
                AverageSuccessRate = _deviceStatistics.Values.Average(s => s.SuccessRate)
            };

            _logger.LogDebug("Transmission statistics: {TotalDevices} devices, {TotalCommands} commands, {SuccessRate:P2} success rate",
                totalStats.TotalDevices, totalStats.TotalCommands, totalStats.AverageSuccessRate);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error updating transmission statistics");
        }
    }

    public void Dispose()
    {
        _statisticsTimer?.Dispose();
        
        foreach (var semaphore in _deviceLocks.Values)
        {
            semaphore.Dispose();
        }
        
        _deviceLocks.Clear();
        _deviceTransports.Clear();
        _deviceStatistics.Clear();
    }
}

/// <summary>
/// Interface for device transport abstraction
/// </summary>
public interface IDeviceTransport
{
    Task<bool> IsConnectedAsync();
    Task<string> GetDeviceTypeAsync();
    Task<byte[]?> SendAndReceiveAsync(byte[] data, CancellationToken cancellationToken);
    Task<bool> SupportsBatchCommandsAsync();
}