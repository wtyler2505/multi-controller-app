using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Services;

/// <summary>
/// Service for ensuring UI responsiveness by managing thread operations and workload distribution
/// </summary>
public interface IUIResponsivenessService
{
    /// <summary>
    /// Execute work on a background thread to avoid blocking UI
    /// </summary>
    Task ExecuteAsync(Func<Task> work, CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Execute work on a background thread and return a result
    /// </summary>
    Task<T> ExecuteAsync<T>(Func<Task<T>> work, CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Execute work on the main thread (simulated UI thread for console apps)
    /// </summary>
    Task ExecuteOnUIThreadAsync(Action work);
    
    /// <summary>
    /// Execute async work on the main thread
    /// </summary>
    Task ExecuteOnUIThreadAsync(Func<Task> work);
    
    /// <summary>
    /// Execute work with yielding to maintain responsiveness
    /// </summary>
    Task ExecuteWithYieldingAsync(Func<CancellationToken, Task> work, CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Process a collection with yielding between items
    /// </summary>
    Task ProcessWithYieldingAsync<T>(IEnumerable<T> items, Func<T, Task> processor, int yieldEvery = 10, CancellationToken cancellationToken = default);
}

/// <summary>
/// Implementation of UI responsiveness service for console/non-UI applications
/// </summary>
public class UIResponsivenessService : IUIResponsivenessService
{
    private readonly ILogger<UIResponsivenessService> _logger;
    private readonly SemaphoreSlim _uiThreadSemaphore;
    private int _currentUIWorkload = 0;
    
    public UIResponsivenessService(ILogger<UIResponsivenessService> logger)
    {
        _logger = logger ?? throw new ArgumentNullException(nameof(logger));
        _uiThreadSemaphore = new SemaphoreSlim(1, 1); // Simulate single UI thread
    }
    
    public async Task ExecuteAsync(Func<Task> work, CancellationToken cancellationToken = default)
    {
        if (work == null) throw new ArgumentNullException(nameof(work));
        
        _logger.LogDebug("Executing background work");
        
        try
        {
            await Task.Run(work, cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error executing background work");
            throw;
        }
    }
    
    public async Task<T> ExecuteAsync<T>(Func<Task<T>> work, CancellationToken cancellationToken = default)
    {
        if (work == null) throw new ArgumentNullException(nameof(work));
        
        _logger.LogDebug("Executing background work with result");
        
        try
        {
            return await Task.Run(work, cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error executing background work with result");
            throw;
        }
    }
    
    public async Task ExecuteOnUIThreadAsync(Action work)
    {
        if (work == null) throw new ArgumentNullException(nameof(work));
        
        await _uiThreadSemaphore.WaitAsync();
        try
        {
            IncrementUIWorkload();
            _logger.LogDebug("Executing work on UI thread");
            
            // Execute on main thread (simulated)
            work();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error executing work on UI thread");
            throw;
        }
        finally
        {
            DecrementUIWorkload();
            _uiThreadSemaphore.Release();
        }
    }
    
    public async Task ExecuteOnUIThreadAsync(Func<Task> work)
    {
        if (work == null) throw new ArgumentNullException(nameof(work));
        
        await _uiThreadSemaphore.WaitAsync();
        try
        {
            IncrementUIWorkload();
            _logger.LogDebug("Executing async work on UI thread");
            
            // Execute on main thread (simulated)
            await work();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error executing async work on UI thread");
            throw;
        }
        finally
        {
            DecrementUIWorkload();
            _uiThreadSemaphore.Release();
        }
    }
    
    public async Task ExecuteWithYieldingAsync(Func<CancellationToken, Task> work, CancellationToken cancellationToken = default)
    {
        if (work == null) throw new ArgumentNullException(nameof(work));
        
        _logger.LogDebug("Executing work with yielding for responsiveness");
        
        try
        {
            await work(cancellationToken);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error executing work with yielding");
            throw;
        }
    }
    
    public async Task ProcessWithYieldingAsync<T>(IEnumerable<T> items, Func<T, Task> processor, int yieldEvery = 10, CancellationToken cancellationToken = default)
    {
        if (items == null) throw new ArgumentNullException(nameof(items));
        if (processor == null) throw new ArgumentNullException(nameof(processor));
        
        _logger.LogDebug("Processing items with yielding every {YieldEvery} items", yieldEvery);
        
        int count = 0;
        foreach (var item in items)
        {
            cancellationToken.ThrowIfCancellationRequested();
            
            await processor(item);
            count++;
            
            // Yield control periodically to maintain responsiveness
            if (count % yieldEvery == 0)
            {
                await Task.Yield();
            }
        }
        
        _logger.LogDebug("Processed {Count} items with yielding", count);
    }
    
    private void IncrementUIWorkload()
    {
        Interlocked.Increment(ref _currentUIWorkload);
        _logger.LogTrace("UI workload incremented to {Workload}", _currentUIWorkload);
    }
    
    private void DecrementUIWorkload()
    {
        Interlocked.Decrement(ref _currentUIWorkload);
        _logger.LogTrace("UI workload decremented to {Workload}", _currentUIWorkload);
    }
}