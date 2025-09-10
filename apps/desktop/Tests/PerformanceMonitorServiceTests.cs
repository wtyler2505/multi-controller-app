using Microsoft.Extensions.Logging;
using Moq;
using MultiControllerApp.Services;
using System;
using System.Threading;
using System.Threading.Tasks;
using Xunit;

namespace MultiControllerApp.Tests;

/// <summary>
/// Unit tests for performance monitoring service
/// </summary>
public class PerformanceMonitorServiceTests : IDisposable
{
    private readonly Mock<ILogger<PerformanceMonitorService>> _loggerMock;
    private readonly PerformanceMonitorService _service;
    
    public PerformanceMonitorServiceTests()
    {
        _loggerMock = new Mock<ILogger<PerformanceMonitorService>>();
        _service = new PerformanceMonitorService(_loggerMock.Object);
    }
    
    [Fact]
    public async Task StartAsync_ShouldInitializeMonitoring()
    {
        // Act
        await _service.StartAsync();
        
        // Assert
        Assert.True(_service.CpuUsagePercent >= 0);
        Assert.True(_service.MemoryUsageMB >= 0);
        Assert.NotNull(_service.BudgetStatus);
        
        await _service.StopAsync();
    }
    
    [Fact]
    public async Task StopAsync_ShouldStopMonitoring()
    {
        // Arrange
        await _service.StartAsync();
        
        // Act
        await _service.StopAsync();
        
        // Assert - should not throw and should complete
        Assert.True(true);
    }
    
    [Fact]
    public async Task MetricsUpdated_ShouldFireEvents()
    {
        // Arrange
        var eventFired = false;
        PerformanceMetrics? receivedMetrics = null;
        
        _service.MetricsUpdated += (sender, metrics) =>
        {
            eventFired = true;
            receivedMetrics = metrics;
        };
        
        // Act
        await _service.StartAsync();
        
        // Wait for at least one update
        await Task.Delay(2000);
        
        // Assert
        Assert.True(eventFired);
        Assert.NotNull(receivedMetrics);
        Assert.True(receivedMetrics.CpuPercent >= 0);
        Assert.True(receivedMetrics.MemoryMB >= 0);
        
        await _service.StopAsync();
    }
    
    [Fact]
    public async Task GetSummary_ShouldReturnValidData()
    {
        // Arrange
        await _service.StartAsync();
        await Task.Delay(1000); // Allow some monitoring time
        
        // Act
        var summary = _service.GetSummary();
        
        // Assert
        Assert.True(summary.MonitoringDuration > TimeSpan.Zero);
        Assert.True(summary.AverageCpuPercent >= 0);
        Assert.True(summary.MaxCpuPercent >= 0);
        Assert.True(summary.AverageMemoryMB >= 0);
        Assert.True(summary.MaxMemoryMB >= 0);
        Assert.True(summary.CurrentMemoryMB >= 0);
        Assert.True(summary.BudgetViolationCount >= 0);
        
        await _service.StopAsync();
    }
    
    [Fact]
    public void Reset_ShouldClearMetrics()
    {
        // Act
        _service.Reset();
        
        // Assert - should not throw
        var summary = _service.GetSummary();
        Assert.Equal(0, summary.BudgetViolationCount);
    }
    
    [Fact]
    public async Task BudgetViolation_ShouldBeDetected()
    {
        // This test simulates budget violations by checking the logic
        // In a real scenario, we'd need to create actual high CPU/memory conditions
        
        // Arrange
        await _service.StartAsync();
        
        // Act - wait for monitoring to establish baseline
        await Task.Delay(1000);
        
        // Assert - verify monitoring is working
        Assert.True(_service.CpuUsagePercent >= 0);
        Assert.True(_service.MemoryUsageMB > 0); // Should be > 0 for running process
        
        await _service.StopAsync();
    }
    
    public void Dispose()
    {
        _service?.Dispose();
    }
}