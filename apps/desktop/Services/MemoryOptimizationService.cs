using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Services;

/// <summary>
/// Memory optimization service providing object pooling and lazy loading capabilities
/// </summary>
public interface IMemoryOptimizationService
{
    /// <summary>
    /// Get an object from the pool or create a new one
    /// </summary>
    T Get<T>() where T : class, new();
    
    /// <summary>
    /// Return an object to the pool for reuse
    /// </summary>
    void Return<T>(T item) where T : class;
    
    /// <summary>
    /// Get current pool statistics
    /// </summary>
    PoolStatistics GetPoolStatistics();
    
    /// <summary>
    /// Force garbage collection and cleanup unused objects
    /// </summary>
    void OptimizeMemory();
    
    /// <summary>
    /// Clear all pools and force cleanup
    /// </summary>
    void ClearPools();
}

/// <summary>
/// Pool statistics for monitoring memory optimization effectiveness
/// </summary>
public record PoolStatistics(
    int TotalPools,
    int TotalObjectsPooled,
    int TotalGets,
    int TotalReturns,
    double MemoryOptimizationRatio
);

/// <summary>
/// Memory optimization service implementation with object pooling
/// </summary>
public class MemoryOptimizationService : IMemoryOptimizationService, IDisposable
{
    private readonly ILogger<MemoryOptimizationService> _logger;
    private readonly ConcurrentDictionary<Type, ObjectPool> _pools = new();
    private readonly object _statsLock = new();
    
    private int _totalGets;
    private int _totalReturns;
    private bool _disposed;
    
    public MemoryOptimizationService(ILogger<MemoryOptimizationService> logger)
    {
        _logger = logger;
        _logger.LogInformation("Memory optimization service initialized");
    }
    
    public T Get<T>() where T : class, new()
    {
        if (_disposed)
            throw new ObjectDisposedException(nameof(MemoryOptimizationService));
            
        var type = typeof(T);
        var pool = _pools.GetOrAdd(type, _ => new ObjectPool<T>());
        
        var item = ((ObjectPool<T>)pool).Get();
        
        lock (_statsLock)
        {
            _totalGets++;
        }
        
        return item;
    }
    
    public void Return<T>(T item) where T : class
    {
        if (_disposed || item == null)
            return;
            
        var type = typeof(T);
        if (_pools.TryGetValue(type, out var pool))
        {
            ((ObjectPool<T>)pool).Return(item);
            
            lock (_statsLock)
            {
                _totalReturns++;
            }
        }
    }
    
    public PoolStatistics GetPoolStatistics()
    {
        lock (_statsLock)
        {
            var totalPooled = 0;
            foreach (var pool in _pools.Values)
            {
                totalPooled += pool.PoolSize;
            }
            
            var optimizationRatio = _totalGets > 0 ? (double)_totalReturns / _totalGets : 0.0;
            
            return new PoolStatistics(
                _pools.Count,
                totalPooled,
                _totalGets,
                _totalReturns,
                optimizationRatio
            );
        }
    }
    
    public void OptimizeMemory()
    {
        if (_disposed)
            return;
            
        _logger.LogDebug("Optimizing memory usage...");
        
        // Trim excess objects from pools
        foreach (var pool in _pools.Values)
        {
            pool.TrimExcess();
        }
        
        // Force garbage collection
        GC.Collect();
        GC.WaitForPendingFinalizers();
        GC.Collect();
        
        _logger.LogDebug("Memory optimization completed");
    }
    
    public void ClearPools()
    {
        if (_disposed)
            return;
            
        _logger.LogInformation("Clearing all object pools...");
        
        foreach (var pool in _pools.Values)
        {
            pool.Clear();
        }
        _pools.Clear();
        
        lock (_statsLock)
        {
            _totalGets = 0;
            _totalReturns = 0;
        }
        
        OptimizeMemory();
        
        _logger.LogInformation("All object pools cleared");
    }
    
    public void Dispose()
    {
        if (_disposed)
            return;
            
        _disposed = true;
        
        try
        {
            ClearPools();
        }
        catch (Exception ex)
        {
            _logger.LogWarning(ex, "Error disposing memory optimization service");
        }
        
        GC.SuppressFinalize(this);
    }
}

/// <summary>
/// Base class for object pools
/// </summary>
public abstract class ObjectPool
{
    public abstract int PoolSize { get; }
    public abstract void TrimExcess();
    public abstract void Clear();
}

/// <summary>
/// Generic object pool for reusing objects to reduce allocations
/// </summary>
public class ObjectPool<T> : ObjectPool where T : class, new()
{
    private readonly ConcurrentQueue<T> _pool = new();
    private readonly Func<T> _factory;
    private readonly Action<T>? _resetAction;
    private const int MaxPoolSize = 100; // Prevent unbounded growth
    
    public ObjectPool(Func<T>? factory = null, Action<T>? resetAction = null)
    {
        _factory = factory ?? (() => new T());
        _resetAction = resetAction;
    }
    
    public override int PoolSize => _pool.Count;
    
    public T Get()
    {
        if (_pool.TryDequeue(out var item))
        {
            return item;
        }
        
        return _factory();
    }
    
    public void Return(T item)
    {
        if (item == null || _pool.Count >= MaxPoolSize)
            return;
            
        // Reset object state if needed
        _resetAction?.Invoke(item);
        
        _pool.Enqueue(item);
    }
    
    public override void TrimExcess()
    {
        // Keep only half the current objects to free memory
        var targetSize = Math.Max(0, _pool.Count / 2);
        var toRemove = _pool.Count - targetSize;
        
        for (int i = 0; i < toRemove; i++)
        {
            _pool.TryDequeue(out _);
        }
    }
    
    public override void Clear()
    {
        while (_pool.TryDequeue(out _))
        {
            // Remove all objects
        }
    }
}