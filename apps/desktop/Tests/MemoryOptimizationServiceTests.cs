using Microsoft.Extensions.Logging;
using Moq;
using MultiControllerApp.Services;
using System;
using Xunit;

namespace MultiControllerApp.Tests;

/// <summary>
/// Unit tests for memory optimization service
/// </summary>
public class MemoryOptimizationServiceTests : IDisposable
{
    private readonly Mock<ILogger<MemoryOptimizationService>> _loggerMock;
    private readonly MemoryOptimizationService _service;
    
    public MemoryOptimizationServiceTests()
    {
        _loggerMock = new Mock<ILogger<MemoryOptimizationService>>();
        _service = new MemoryOptimizationService(_loggerMock.Object);
    }
    
    [Fact]
    public void Get_ShouldReturnNewInstance()
    {
        // Act
        var item = _service.Get<TestObject>();
        
        // Assert
        Assert.NotNull(item);
        Assert.IsType<TestObject>(item);
    }
    
    [Fact]
    public void Get_AfterReturn_ShouldReuseInstance()
    {
        // Arrange
        var item1 = _service.Get<TestObject>();
        item1.Value = 42;
        
        // Act
        _service.Return(item1);
        var item2 = _service.Get<TestObject>();
        
        // Assert
        Assert.Same(item1, item2);
        Assert.Equal(42, item2.Value);
    }
    
    [Fact]
    public void GetPoolStatistics_ShouldReturnValidData()
    {
        // Arrange
        var item1 = _service.Get<TestObject>();
        var item2 = _service.Get<TestObject>();
        _service.Return(item1);
        
        // Act
        var stats = _service.GetPoolStatistics();
        
        // Assert
        Assert.True(stats.TotalPools >= 0);
        Assert.True(stats.TotalObjectsPooled >= 0);
        Assert.True(stats.TotalGets >= 2);
        Assert.True(stats.TotalReturns >= 1);
        Assert.True(stats.MemoryOptimizationRatio >= 0);
    }
    
    [Fact]
    public void OptimizeMemory_ShouldNotThrow()
    {
        // Arrange
        var item = _service.Get<TestObject>();
        _service.Return(item);
        
        // Act & Assert
        _service.OptimizeMemory();
    }
    
    [Fact]
    public void ClearPools_ShouldResetStatistics()
    {
        // Arrange
        var item = _service.Get<TestObject>();
        _service.Return(item);
        
        // Act
        _service.ClearPools();
        
        // Assert
        var stats = _service.GetPoolStatistics();
        Assert.Equal(0, stats.TotalPools);
        Assert.Equal(0, stats.TotalObjectsPooled);
        Assert.Equal(0, stats.TotalGets);
        Assert.Equal(0, stats.TotalReturns);
    }
    
    [Fact]
    public void Return_WithNullItem_ShouldNotThrow()
    {
        // Act & Assert
        _service.Return<TestObject>(null);
    }
    
    [Fact]
    public void MultipleTypes_ShouldUseSeparatePools()
    {
        // Arrange & Act
        var obj1 = _service.Get<TestObject>();
        var obj2 = _service.Get<AnotherTestObject>();
        _service.Return(obj1);
        _service.Return(obj2);
        
        // Assert
        var stats = _service.GetPoolStatistics();
        Assert.True(stats.TotalPools >= 2);
    }
    
    public void Dispose()
    {
        _service?.Dispose();
    }
    
    private class TestObject
    {
        public int Value { get; set; }
    }
    
    private class AnotherTestObject
    {
        public string Data { get; set; } = "";
    }
}