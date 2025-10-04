using Microsoft.Extensions.Logging;
using Moq;
using MultiControllerApp.Services.Commands;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using Xunit;

namespace MultiControllerApp.Tests.Services.Commands;

public class CommandQueueTests : IDisposable
{
    private readonly Mock<ILogger<CommandQueue>> _mockLogger;
    private readonly CommandQueue _commandQueue;

    public CommandQueueTests()
    {
        _mockLogger = new Mock<ILogger<CommandQueue>>();
        _commandQueue = new CommandQueue(_mockLogger.Object);
    }

    [Fact]
    public async Task QueueAsync_ShouldQueueCommandSuccessfully()
    {
        // Arrange
        var command = new DeviceCommand
        {
            Type = CommandType.DigitalWrite,
            DeviceId = "test-device",
            Priority = CommandPriority.Normal
        };

        // Act
        var commandId = await _commandQueue.QueueAsync(command);

        // Assert
        Assert.Equal(command.Id, commandId);
        Assert.Equal(CommandStatus.Queued, command.Status);
        Assert.NotNull(command.QueuedAt);
        Assert.Equal(1, _commandQueue.Count);
    }

    [Fact]
    public async Task DequeueAsync_ShouldReturnHighestPriorityCommand()
    {
        // Arrange
        var lowPriorityCommand = new DeviceCommand
        {
            Type = CommandType.DigitalRead,
            DeviceId = "test-device",
            Priority = CommandPriority.Low
        };

        var highPriorityCommand = new DeviceCommand
        {
            Type = CommandType.EmergencyStop,
            DeviceId = "test-device",
            Priority = CommandPriority.Emergency
        };

        // Act
        await _commandQueue.QueueAsync(lowPriorityCommand);
        await _commandQueue.QueueAsync(highPriorityCommand);

        var dequeuedCommand = await _commandQueue.DequeueAsync();

        // Assert
        Assert.NotNull(dequeuedCommand);
        Assert.Equal(highPriorityCommand.Id, dequeuedCommand.Id);
        Assert.Equal(CommandStatus.Transmitting, dequeuedCommand.Status);
    }

    [Fact]
    public async Task CancelCommandAsync_ShouldCancelQueuedCommand()
    {
        // Arrange
        var command = new DeviceCommand
        {
            Type = CommandType.DigitalWrite,
            DeviceId = "test-device"
        };

        await _commandQueue.QueueAsync(command);

        // Act
        var cancelled = await _commandQueue.CancelCommandAsync(command.Id);

        // Assert
        Assert.True(cancelled);
        Assert.Equal(CommandStatus.Cancelled, command.Status);
        Assert.Equal(0, _commandQueue.Count);
    }

    [Fact]
    public async Task ClearDeviceQueueAsync_ShouldCancelAllCommandsForDevice()
    {
        // Arrange
        var device1Commands = new[]
        {
            new DeviceCommand { Type = CommandType.DigitalWrite, DeviceId = "device1" },
            new DeviceCommand { Type = CommandType.AnalogRead, DeviceId = "device1" }
        };

        var device2Commands = new[]
        {
            new DeviceCommand { Type = CommandType.SetRelay, DeviceId = "device2" }
        };

        foreach (var cmd in device1Commands.Concat(device2Commands))
        {
            await _commandQueue.QueueAsync(cmd);
        }

        // Act
        var cancelledCount = await _commandQueue.ClearDeviceQueueAsync("device1");

        // Assert
        Assert.Equal(2, cancelledCount);
        Assert.Equal(1, _commandQueue.Count); // Only device2 command remains
    }

    [Fact]
    public async Task GetStatisticsAsync_ShouldReturnCorrectStatistics()
    {
        // Arrange
        var commands = new[]
        {
            new DeviceCommand { Type = CommandType.DigitalWrite, DeviceId = "device1", Priority = CommandPriority.High },
            new DeviceCommand { Type = CommandType.AnalogRead, DeviceId = "device1", Priority = CommandPriority.Normal },
            new DeviceCommand { Type = CommandType.SetRelay, DeviceId = "device2", Priority = CommandPriority.High }
        };

        foreach (var cmd in commands)
        {
            await _commandQueue.QueueAsync(cmd);
        }

        // Act
        var statistics = await _commandQueue.GetStatisticsAsync();

        // Assert
        Assert.Equal(3, statistics.QueuedCommands);
        Assert.Equal(2, statistics.CommandsByPriority[CommandPriority.High]);
        Assert.Equal(1, statistics.CommandsByPriority[CommandPriority.Normal]);
        Assert.Equal(2, statistics.CommandsByDevice["device1"]);
        Assert.Equal(1, statistics.CommandsByDevice["device2"]);
    }

    public void Dispose()
    {
        _commandQueue?.Dispose();
        GC.SuppressFinalize(this);
    }
}

public class CommandSerializerTests
{
    private readonly Mock<ILogger<CommandSerializer>> _mockLogger;
    private readonly CommandSerializer _serializer;

    public CommandSerializerTests()
    {
        _mockLogger = new Mock<ILogger<CommandSerializer>>();
        _serializer = new CommandSerializer(_mockLogger.Object);
    }

    [Fact]
    public async Task SerializeAsync_JsonFormat_ShouldProduceValidJson()
    {
        // Arrange
        var command = new DeviceCommand
        {
            Type = CommandType.DigitalWrite,
            DeviceId = "arduino-001",
            Parameters = new Dictionary<string, object>
            {
                ["pin"] = 13,
                ["value"] = true
            }
        };

        var config = new SerializationConfig
        {
            Format = "json",
            Encoding = "utf8"
        };

        // Act
        var serializedData = await _serializer.SerializeAsync(command, config);
        var jsonString = System.Text.Encoding.UTF8.GetString(serializedData);

        // Assert
        Assert.Contains("\"type\":\"DigitalWrite\"", jsonString);
        Assert.Contains("\"pin\":13", jsonString);
        Assert.Contains("\"value\":true", jsonString);
    }

    [Fact]
    public async Task SerializeAsync_ArduinoFormat_ShouldProduceCorrectFormat()
    {
        // Arrange
        var command = new DeviceCommand
        {
            Type = CommandType.DigitalWrite,
            Parameters = new Dictionary<string, object>
            {
                ["pin"] = 13,
                ["value"] = true
            }
        };

        var config = _serializer.GetSerializationConfig("arduino");

        // Act
        var serializedData = await _serializer.SerializeAsync(command, config);
        var commandString = System.Text.Encoding.ASCII.GetString(serializedData);

        // Assert
        Assert.StartsWith("$", commandString);
        Assert.EndsWith("\r\n", commandString);
        Assert.Contains("DigitalWrite", commandString);
        Assert.Contains("pin=13", commandString);
        Assert.Contains("value=True", commandString);
    }

    [Fact]
    public async Task ValidateCommandAsync_ValidArduinoCommand_ShouldPass()
    {
        // Arrange
        var command = new DeviceCommand
        {
            Type = CommandType.DigitalWrite,
            DeviceId = "arduino-001",
            Parameters = new Dictionary<string, object>
            {
                ["pin"] = 13,
                ["value"] = true
            }
        };

        // Act
        var result = await _serializer.ValidateCommandAsync(command, "arduino");

        // Assert
        Assert.True(result.IsValid);
        Assert.Empty(result.Errors);
    }

    [Fact]
    public async Task ValidateCommandAsync_MissingRequiredParameter_ShouldFail()
    {
        // Arrange
        var command = new DeviceCommand
        {
            Type = CommandType.DigitalWrite,
            DeviceId = "arduino-001",
            Parameters = new Dictionary<string, object>
            {
                ["pin"] = 13
                // Missing "value" parameter
            }
        };

        // Act
        var result = await _serializer.ValidateCommandAsync(command, "arduino");

        // Assert
        Assert.False(result.IsValid);
        Assert.Contains("requires 'pin' and 'value' parameters", result.Errors.First());
    }

    [Fact]
    public async Task ValidateCommandAsync_UnsafePWMFrequency_ShouldWarn()
    {
        // Arrange
        var command = new DeviceCommand
        {
            Type = CommandType.SetPWMFrequency,
            DeviceId = "arduino-001",
            Parameters = new Dictionary<string, object>
            {
                ["frequency"] = 100000 // Very high frequency
            }
        };

        // Act
        var result = await _serializer.ValidateCommandAsync(command, "arduino");

        // Assert
        Assert.True(result.IsValid);
        Assert.Contains("PWM frequency 100000Hz is very high", result.Warnings.First());
    }

    [Fact]
    public async Task GetSerializationConfig_KnownDevice_ShouldReturnCorrectConfig()
    {
        // Act
        var arduinoConfig = _serializer.GetSerializationConfig("arduino");
        var esp32Config = _serializer.GetSerializationConfig("esp32");
        var riorandConfig = _serializer.GetSerializationConfig("riorand");

        // Assert
        Assert.Equal("arduino", arduinoConfig.Format);
        Assert.True(arduinoConfig.IncludeChecksum);
        Assert.Equal("crc8", arduinoConfig.ChecksumAlgorithm);

        Assert.Equal("json", esp32Config.Format);
        Assert.True(esp32Config.IncludeChecksum);
        Assert.Equal("crc32", esp32Config.ChecksumAlgorithm);

        Assert.Equal("binary", riorandConfig.Format);
        Assert.Equal("xor", riorandConfig.ChecksumAlgorithm);
    }
}

public class CommandHistoryTests : IDisposable
{
    private readonly Mock<ILogger<CommandHistory>> _mockLogger;
    private readonly CommandHistory _history;

    public CommandHistoryTests()
    {
        _mockLogger = new Mock<ILogger<CommandHistory>>();
        _history = new CommandHistory(_mockLogger.Object, globalCapacity: 100, deviceCapacity: 50);
    }

    [Fact]
    public async Task AddAsync_ShouldAddCommandToHistory()
    {
        // Arrange
        var command = new DeviceCommand
        {
            Type = CommandType.DigitalWrite,
            DeviceId = "test-device"
        };

        // Act
        await _history.AddAsync(command);
        var deviceHistory = await _history.GetHistoryAsync("test-device");

        // Assert
        Assert.Single(deviceHistory);
        Assert.Equal(command.Id, deviceHistory.First().Id);
    }

    [Fact]
    public async Task GetHistoryAsync_ShouldReturnCommandsInDescendingOrder()
    {
        // Arrange
        var commands = new[]
        {
            new DeviceCommand { Type = CommandType.DigitalWrite, DeviceId = "test-device", CreatedAt = DateTime.UtcNow.AddMinutes(-2) },
            new DeviceCommand { Type = CommandType.AnalogRead, DeviceId = "test-device", CreatedAt = DateTime.UtcNow.AddMinutes(-1) },
            new DeviceCommand { Type = CommandType.SetRelay, DeviceId = "test-device", CreatedAt = DateTime.UtcNow }
        };

        foreach (var cmd in commands)
        {
            await _history.AddAsync(cmd);
        }

        // Act
        var history = await _history.GetHistoryAsync("test-device");

        // Assert
        var historyList = history.ToList();
        Assert.Equal(3, historyList.Count);
        Assert.True(historyList[0].CreatedAt >= historyList[1].CreatedAt);
        Assert.True(historyList[1].CreatedAt >= historyList[2].CreatedAt);
    }

    [Fact]
    public async Task GetStatisticsAsync_ShouldReturnCorrectStatistics()
    {
        // Arrange
        var commands = new[]
        {
            new DeviceCommand { Type = CommandType.DigitalWrite, DeviceId = "device1" },
            new DeviceCommand { Type = CommandType.DigitalWrite, DeviceId = "device2" },
            new DeviceCommand { Type = CommandType.AnalogRead, DeviceId = "device1" }
        };

        foreach (var cmd in commands)
        {
            await _history.AddAsync(cmd);
        }

        // Act
        var statistics = await _history.GetStatisticsAsync();

        // Assert
        Assert.Equal(3, statistics.TotalCommands);
        Assert.Equal(2, statistics.CommandsByType[CommandType.DigitalWrite]);
        Assert.Equal(1, statistics.CommandsByType[CommandType.AnalogRead]);
        Assert.Equal(2, statistics.CommandsByDevice["device1"]);
        Assert.Equal(1, statistics.CommandsByDevice["device2"]);
    }

    [Fact]
    public async Task ClearHistoryAsync_ShouldRemoveDeviceHistory()
    {
        // Arrange
        var command1 = new DeviceCommand { Type = CommandType.DigitalWrite, DeviceId = "device1" };
        var command2 = new DeviceCommand { Type = CommandType.AnalogRead, DeviceId = "device2" };

        await _history.AddAsync(command1);
        await _history.AddAsync(command2);

        // Act
        await _history.ClearHistoryAsync("device1");

        // Assert
        var device1History = await _history.GetHistoryAsync("device1");
        var device2History = await _history.GetHistoryAsync("device2");

        Assert.Empty(device1History);
        Assert.Single(device2History);
    }

    public void Dispose()
    {
        _history?.Dispose();
        GC.SuppressFinalize(this);
    }
}

public class CircularBufferTests
{
    [Fact]
    public void Add_ShouldAddItemsUpToCapacity()
    {
        // Arrange
        var buffer = new CircularBuffer<int>(3);

        // Act
        buffer.Add(1);
        buffer.Add(2);
        buffer.Add(3);

        // Assert
        Assert.Equal(3, buffer.Count);
        Assert.True(buffer.IsFull);
        Assert.Equal(new[] { 1, 2, 3 }, buffer.GetItems());
    }

    [Fact]
    public void Add_WhenFull_ShouldOverwriteOldestItem()
    {
        // Arrange
        var buffer = new CircularBuffer<int>(3);
        buffer.Add(1);
        buffer.Add(2);
        buffer.Add(3);

        // Act
        buffer.Add(4); // Should overwrite 1

        // Assert
        Assert.Equal(3, buffer.Count);
        Assert.True(buffer.IsFull);
        Assert.Equal(new[] { 2, 3, 4 }, buffer.GetItems());
    }

    [Fact]
    public void GetLatest_ShouldReturnMostRecentlyAddedItem()
    {
        // Arrange
        var buffer = new CircularBuffer<string>(5);
        buffer.Add("first");
        buffer.Add("second");
        buffer.Add("latest");

        // Act
        var latest = buffer.GetLatest();

        // Assert
        Assert.Equal("latest", latest);
    }

    [Fact]
    public void Clear_ShouldRemoveAllItems()
    {
        // Arrange
        var buffer = new CircularBuffer<int>(3);
        buffer.Add(1);
        buffer.Add(2);
        buffer.Add(3);

        // Act
        buffer.Clear();

        // Assert
        Assert.Equal(0, buffer.Count);
        Assert.True(buffer.IsEmpty);
        Assert.False(buffer.IsFull);
        Assert.Empty(buffer.GetItems());
    }
}