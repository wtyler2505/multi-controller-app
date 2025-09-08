using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Services.Commands;

/// <summary>
/// Command history implementation using circular buffer for efficient memory usage
/// </summary>
public class CommandHistory : ICommandHistory
{
    private readonly ILogger<CommandHistory> _logger;
    private readonly CircularBuffer<DeviceCommand> _globalHistory;
    private readonly ConcurrentDictionary<string, CircularBuffer<DeviceCommand>> _deviceHistories;
    private readonly ReaderWriterLockSlim _lock;
    private readonly Timer _cleanupTimer;
    
    // Configuration
    private readonly int _globalCapacity;
    private readonly int _deviceCapacity;
    private readonly TimeSpan _retentionPeriod;

    public CommandHistory(
        ILogger<CommandHistory> logger,
        int globalCapacity = 10000,
        int deviceCapacity = 1000,
        TimeSpan? retentionPeriod = null)
    {
        _logger = logger;
        _globalCapacity = globalCapacity;
        _deviceCapacity = deviceCapacity;
        _retentionPeriod = retentionPeriod ?? TimeSpan.FromHours(24);
        
        _globalHistory = new CircularBuffer<DeviceCommand>(_globalCapacity);
        _deviceHistories = new ConcurrentDictionary<string, CircularBuffer<DeviceCommand>>();
        _lock = new ReaderWriterLockSlim();
        
        // Clean up old entries every hour
        _cleanupTimer = new Timer(CleanupExpiredEntries, null, TimeSpan.FromHours(1), TimeSpan.FromHours(1));
        
        _logger.LogDebug("Command history initialized with global capacity {GlobalCapacity}, device capacity {DeviceCapacity}, retention {RetentionHours}h",
            _globalCapacity, _deviceCapacity, _retentionPeriod.TotalHours);
    }

    public async Task AddAsync(DeviceCommand command)
    {
        try
        {
            _lock.EnterWriteLock();
            
            // Add to global history
            _globalHistory.Add(command);
            
            // Add to device-specific history
            var deviceHistory = _deviceHistories.GetOrAdd(command.DeviceId, 
                _ => new CircularBuffer<DeviceCommand>(_deviceCapacity));
            deviceHistory.Add(command);
            
            _logger.LogTrace("Added command {CommandId} to history for device {DeviceId}", 
                command.Id, command.DeviceId);
        }
        finally
        {
            _lock.ExitWriteLock();
        }
    }

    public async Task<IEnumerable<DeviceCommand>> GetHistoryAsync(string deviceId, int maxResults = 100)
    {
        try
        {
            _lock.EnterReadLock();
            
            if (!_deviceHistories.TryGetValue(deviceId, out var deviceHistory))
            {
                return Enumerable.Empty<DeviceCommand>();
            }

            var commands = deviceHistory.GetItems()
                .Where(cmd => DateTime.UtcNow - cmd.CreatedAt <= _retentionPeriod)
                .OrderByDescending(cmd => cmd.CreatedAt)
                .Take(maxResults)
                .ToList();

            _logger.LogDebug("Retrieved {Count} history entries for device {DeviceId}", 
                commands.Count, deviceId);

            return commands;
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    public async Task<IEnumerable<DeviceCommand>> GetRecentCommandsAsync(int maxResults = 50)
    {
        try
        {
            _lock.EnterReadLock();
            
            var commands = _globalHistory.GetItems()
                .Where(cmd => DateTime.UtcNow - cmd.CreatedAt <= _retentionPeriod)
                .OrderByDescending(cmd => cmd.CreatedAt)
                .Take(maxResults)
                .ToList();

            _logger.LogDebug("Retrieved {Count} recent commands from global history", commands.Count);

            return commands;
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    public async Task ClearHistoryAsync(string deviceId)
    {
        try
        {
            _lock.EnterWriteLock();
            
            if (_deviceHistories.TryRemove(deviceId, out var deviceHistory))
            {
                var clearedCount = deviceHistory.Count;
                _logger.LogDebug("Cleared {Count} history entries for device {DeviceId}", 
                    clearedCount, deviceId);
            }
        }
        finally
        {
            _lock.ExitWriteLock();
        }
    }

    public async Task<HistoryStatistics> GetStatisticsAsync()
    {
        try
        {
            _lock.EnterReadLock();
            
            var allCommands = _globalHistory.GetItems().ToList();
            var validCommands = allCommands
                .Where(cmd => DateTime.UtcNow - cmd.CreatedAt <= _retentionPeriod)
                .ToList();

            var statistics = new HistoryStatistics
            {
                TotalCommands = validCommands.Count,
                BufferSize = _globalHistory.Count,
                BufferCapacity = _globalCapacity,
                OldestCommandAt = validCommands.Any() ? validCommands.Min(cmd => cmd.CreatedAt) : DateTime.MinValue,
                NewestCommandAt = validCommands.Any() ? validCommands.Max(cmd => cmd.CreatedAt) : DateTime.MinValue
            };

            // Group by command type
            foreach (var typeGroup in validCommands.GroupBy(cmd => cmd.Type))
            {
                statistics.CommandsByType[typeGroup.Key] = typeGroup.Count();
            }

            // Group by device
            foreach (var deviceGroup in validCommands.GroupBy(cmd => cmd.DeviceId))
            {
                statistics.CommandsByDevice[deviceGroup.Key] = deviceGroup.Count();
            }

            _logger.LogDebug("Generated history statistics: {TotalCommands} commands across {DeviceCount} devices",
                statistics.TotalCommands, statistics.CommandsByDevice.Count);

            return statistics;
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Get command history filtered by command type
    /// </summary>
    public async Task<IEnumerable<DeviceCommand>> GetHistoryByTypeAsync(CommandType commandType, int maxResults = 100)
    {
        try
        {
            _lock.EnterReadLock();
            
            var commands = _globalHistory.GetItems()
                .Where(cmd => cmd.Type == commandType && DateTime.UtcNow - cmd.CreatedAt <= _retentionPeriod)
                .OrderByDescending(cmd => cmd.CreatedAt)
                .Take(maxResults)
                .ToList();

            _logger.LogDebug("Retrieved {Count} history entries for command type {CommandType}", 
                commands.Count, commandType);

            return commands;
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Get command history filtered by status
    /// </summary>
    public async Task<IEnumerable<DeviceCommand>> GetHistoryByStatusAsync(CommandStatus status, int maxResults = 100)
    {
        try
        {
            _lock.EnterReadLock();
            
            var commands = _globalHistory.GetItems()
                .Where(cmd => cmd.Status == status && DateTime.UtcNow - cmd.CreatedAt <= _retentionPeriod)
                .OrderByDescending(cmd => cmd.CreatedAt)
                .Take(maxResults)
                .ToList();

            _logger.LogDebug("Retrieved {Count} history entries with status {Status}", 
                commands.Count, status);

            return commands;
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Get command history within a time range
    /// </summary>
    public async Task<IEnumerable<DeviceCommand>> GetHistoryByTimeRangeAsync(DateTime startTime, DateTime endTime, int maxResults = 1000)
    {
        try
        {
            _lock.EnterReadLock();
            
            var commands = _globalHistory.GetItems()
                .Where(cmd => cmd.CreatedAt >= startTime && cmd.CreatedAt <= endTime)
                .OrderByDescending(cmd => cmd.CreatedAt)
                .Take(maxResults)
                .ToList();

            _logger.LogDebug("Retrieved {Count} history entries between {StartTime} and {EndTime}", 
                commands.Count, startTime, endTime);

            return commands;
        }
        finally
        {
            _lock.ExitReadLock();
        }
    }

    /// <summary>
    /// Export command history to JSON for debugging/analysis
    /// </summary>
    public async Task<string> ExportHistoryToJsonAsync(string? deviceId = null, int maxResults = 1000)
    {
        var commands = deviceId != null 
            ? await GetHistoryAsync(deviceId, maxResults)
            : await GetRecentCommandsAsync(maxResults);

        var exportData = new
        {
            ExportedAt = DateTime.UtcNow,
            DeviceId = deviceId,
            CommandCount = commands.Count(),
            Commands = commands.Select(cmd => new
            {
                cmd.Id,
                cmd.Type,
                cmd.DeviceId,
                cmd.Status,
                cmd.Priority,
                cmd.CreatedAt,
                cmd.QueuedAt,
                cmd.TransmittedAt,
                cmd.AcknowledgedAt,
                cmd.RetryCount,
                cmd.Parameters,
                cmd.ErrorMessage,
                ExecutionTimeMs = cmd.AcknowledgedAt.HasValue && cmd.TransmittedAt.HasValue 
                    ? (cmd.AcknowledgedAt.Value - cmd.TransmittedAt.Value).TotalMilliseconds 
                    : (double?)null
            })
        };

        return System.Text.Json.JsonSerializer.Serialize(exportData, new System.Text.Json.JsonSerializerOptions
        {
            WriteIndented = true,
            PropertyNamingPolicy = System.Text.Json.JsonNamingPolicy.CamelCase
        });
    }

    private void CleanupExpiredEntries(object? state)
    {
        try
        {
            _lock.EnterWriteLock();
            
            var cutoffTime = DateTime.UtcNow - _retentionPeriod;
            var removedCount = 0;

            // Note: Circular buffer automatically removes old entries, but we track statistics
            var expiredCommands = _globalHistory.GetItems()
                .Where(cmd => cmd.CreatedAt < cutoffTime)
                .Count();

            if (expiredCommands > 0)
            {
                _logger.LogDebug("Cleanup found {ExpiredCount} expired commands older than {CutoffTime}", 
                    expiredCommands, cutoffTime);
            }

            // Clean up empty device histories
            var emptyDeviceHistories = _deviceHistories
                .Where(kvp => kvp.Value.Count == 0)
                .Select(kvp => kvp.Key)
                .ToList();

            foreach (var deviceId in emptyDeviceHistories)
            {
                _deviceHistories.TryRemove(deviceId, out _);
                removedCount++;
            }

            if (removedCount > 0)
            {
                _logger.LogDebug("Removed {Count} empty device histories during cleanup", removedCount);
            }
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error during command history cleanup");
        }
        finally
        {
            _lock.ExitWriteLock();
        }
    }

    public void Dispose()
    {
        _cleanupTimer?.Dispose();
        _lock?.Dispose();
        _logger.LogDebug("Command history disposed");
    }
}

/// <summary>
/// Generic circular buffer implementation for efficient memory usage
/// </summary>
public class CircularBuffer<T>
{
    private readonly T[] _buffer;
    private readonly object _lock = new();
    private int _head;
    private int _tail;
    private int _count;

    public CircularBuffer(int capacity)
    {
        if (capacity <= 0)
            throw new ArgumentException("Capacity must be positive", nameof(capacity));

        _buffer = new T[capacity];
        Capacity = capacity;
    }

    public int Capacity { get; }
    public int Count 
    { 
        get 
        { 
            lock (_lock) 
            { 
                return _count; 
            } 
        } 
    }

    public bool IsFull 
    { 
        get 
        { 
            lock (_lock) 
            { 
                return _count == Capacity; 
            } 
        } 
    }

    public bool IsEmpty 
    { 
        get 
        { 
            lock (_lock) 
            { 
                return _count == 0; 
            } 
        } 
    }

    public void Add(T item)
    {
        lock (_lock)
        {
            _buffer[_head] = item;
            _head = (_head + 1) % Capacity;

            if (_count < Capacity)
            {
                _count++;
            }
            else
            {
                // Buffer is full, move tail to maintain circular behavior
                _tail = (_tail + 1) % Capacity;
            }
        }
    }

    public IEnumerable<T> GetItems()
    {
        lock (_lock)
        {
            var items = new T[_count];
            
            for (int i = 0; i < _count; i++)
            {
                var index = (_tail + i) % Capacity;
                items[i] = _buffer[index];
            }

            return items;
        }
    }

    public T? GetLatest()
    {
        lock (_lock)
        {
            if (_count == 0)
                return default(T);

            var latestIndex = (_head - 1 + Capacity) % Capacity;
            return _buffer[latestIndex];
        }
    }

    public void Clear()
    {
        lock (_lock)
        {
            Array.Clear(_buffer, 0, _buffer.Length);
            _head = 0;
            _tail = 0;
            _count = 0;
        }
    }
}