using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using Microsoft.UI.Dispatching;

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
    /// Execute work on the UI thread (dispatcher)
    /// </summary>
    Task ExecuteOnUIThreadAsync(Action work);
    
    /// <summary>
    /// Execute work on the UI thread and return a result
    /// </summary>
    Task<T> ExecuteOnUIThreadAsync<T>(Func<T> work);
    
    /// <summary>
    /// Check if we're currently on the UI thread
    /// </summary>
    bool IsUIThread { get; }
    
    /// <summary>
    /// Current UI thread workload (pending operations)
    /// </summary>
    int UIThreadWorkload { get; }
    
    /// <summary>
    /// Execute CPU-intensive work with yielding to maintain responsiveness
    /// </summary>
    Task ExecuteWithYieldingAsync(Func<CancellationToken, Task> work, CancellationToken cancellationToken = default);
}

/// <summary>
/// UI responsiveness service implementation
/// </summary>
public class UIResponsivenessService : IUIResponsivenessService
{
    private readonly ILogger<UIResponsivenessService> _logger;
    private readonly DispatcherQueue _dispatcherQueue;
    private readonly SemaphoreSlim _backgroundWorkSemaphore;
    private readonly object _workloadLock = new();
    
    private int _uiThreadWorkload;
    private static readonly int MaxConcurrentBackgroundTasks = Environment.ProcessorCount * 2;
    private const int YieldThresholdMs = 50; // Yield every 50ms for long operations
    
    public UIResponsivenessService(ILogger<UIResponsivenessService> logger, DispatcherQueue? dispatcherQueue = null)
    {
        _logger = logger;
        _dispatcherQueue = dispatcherQueue ?? DispatcherQueue.GetForCurrentThread();
        _backgroundWorkSemaphore = new SemaphoreSlim(MaxConcurrentBackgroundTasks, MaxConcurrentBackgroundTasks);
        
        _logger.LogInformation("UI responsiveness service initialized with {MaxTasks} max concurrent background tasks", 
            MaxConcurrentBackgroundTasks);
    }
    
    public bool IsUIThread => _dispatcherQueue.HasThreadAccess;
    
    public int UIThreadWorkload
    {
        get
        {
            lock (_workloadLock)
            {
                return _uiThreadWorkload;
            }
        }
    }
    
    public async Task ExecuteAsync(Func<Task> work, CancellationToken cancellationToken = default)
    {
        await _backgroundWorkSemaphore.WaitAsync(cancellationToken);
        
        try
        {
            await Task.Run(async () =>
            {
                await work();
            }, cancellationToken);
        }
        finally
        {
            _backgroundWorkSemaphore.Release();
        }
    }
    
    public async Task<T> ExecuteAsync<T>(Func<Task<T>> work, CancellationToken cancellationToken = default)
    {
        await _backgroundWorkSemaphore.WaitAsync(cancellationToken);
        
        try
        {
            return await Task.Run(async () =>
            {
                return await work();
            }, cancellationToken);
        }
        finally
        {
            _backgroundWorkSemaphore.Release();
        }
    }
    
    public async Task ExecuteOnUIThreadAsync(Action work)
    {
        if (IsUIThread)
        {
            work();
            return;
        }
        
        var tcs = new TaskCompletionSource<bool>();
        
        IncrementUIWorkload();
        
        try
        {
            var enqueued = _dispatcherQueue.TryEnqueue(() =>
            {
                try
                {
                    work();
                    tcs.SetResult(true);
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
                finally
                {
                    DecrementUIWorkload();
                }
            });
            
            if (!enqueued)
            {
                DecrementUIWorkload();
                throw new InvalidOperationException("Failed to enqueue work on UI thread");
            }
            
            await tcs.Task;
        }
        catch
        {
            DecrementUIWorkload();
            throw;
        }
    }
    
    public async Task<T> ExecuteOnUIThreadAsync<T>(Func<T> work)
    {
        if (IsUIThread)
        {
            return work();
        }
        
        var tcs = new TaskCompletionSource<T>();
        
        IncrementUIWorkload();
        
        try
        {
            var enqueued = _dispatcherQueue.TryEnqueue(() =>
            {
                try
                {
                    var result = work();
                    tcs.SetResult(result);
                }
                catch (Exception ex)
                {
                    tcs.SetException(ex);
                }
                finally
                {
                    DecrementUIWorkload();
                }
            });
            
            if (!enqueued)
            {
                DecrementUIWorkload();
                throw new InvalidOperationException("Failed to enqueue work on UI thread");
            }
            
            return await tcs.Task;
        }
        catch
        {
            DecrementUIWorkload();
            throw;
        }
    }
    
    public async Task ExecuteWithYieldingAsync(Func<CancellationToken, Task> work, CancellationToken cancellationToken = default)
    {
        await _backgroundWorkSemaphore.WaitAsync(cancellationToken);
        
        try
        {
            await Task.Run(async () =>
            {
                var yieldingCancellationSource = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
                var yieldingToken = yieldingCancellationSource.Token;
                
                // Create a timer to periodically yield
                using var yieldTimer = new Timer(_ =>
                {
                    if (!yieldingToken.IsCancellationRequested)
                    {
                        Task.Yield(); // Yield control to allow other tasks to run
                    }
                }, null, TimeSpan.FromMilliseconds(YieldThresholdMs), TimeSpan.FromMilliseconds(YieldThresholdMs));
                
                await work(yieldingToken);
                
            }, cancellationToken);
        }
        finally
        {
            _backgroundWorkSemaphore.Release();
        }
    }
    
    private void IncrementUIWorkload()
    {
        lock (_workloadLock)
        {
            _uiThreadWorkload++;
            
            if (_uiThreadWorkload > 10) // Warn if UI thread is getting overloaded
            {
                _logger.LogWarning("UI thread workload is high: {Workload} pending operations", _uiThreadWorkload);
            }
        }
    }
    
    private void DecrementUIWorkload()
    {
        lock (_workloadLock)
        {
            _uiThreadWorkload = Math.Max(0, _uiThreadWorkload - 1);
        }
    }
}

/// <summary>
/// Extension methods for UI responsiveness
/// </summary>
public static class UIResponsivenessExtensions
{
    /// <summary>
    /// Execute long-running enumerable operations with periodic yielding
    /// </summary>
    public static async Task ProcessWithYieldingAsync<T>(
        this IUIResponsivenessService service,
        IEnumerable<T> items,
        Func<T, Task> processor,
        int batchSize = 10,
        CancellationToken cancellationToken = default)
    {
        await service.ExecuteWithYieldingAsync(async (token) =>
        {
            var batch = new List<T>(batchSize);
            
            foreach (var item in items)
            {
                token.ThrowIfCancellationRequested();
                
                batch.Add(item);
                
                if (batch.Count >= batchSize)
                {
                    await ProcessBatch(batch, processor, token);
                    batch.Clear();
                    
                    // Yield after each batch
                    await Task.Yield();
                }
            }
            
            // Process remaining items
            if (batch.Count > 0)
            {
                await ProcessBatch(batch, processor, token);
            }
            
        }, cancellationToken);
    }
    
    private static async Task ProcessBatch<T>(IList<T> batch, Func<T, Task> processor, CancellationToken cancellationToken)
    {
        var tasks = new Task[batch.Count];
        for (int i = 0; i < batch.Count; i++)
        {
            var item = batch[i];
            tasks[i] = processor(item);
        }
        
        await Task.WhenAll(tasks);
    }
}