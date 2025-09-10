using System;
using System.Threading;
using System.Threading.Tasks;

namespace MultiControllerApp.Services;

/// <summary>
/// Interface for monitoring system performance metrics (CPU, memory, UI responsiveness)
/// </summary>
public interface IPerformanceMonitorService
{
    /// <summary>
    /// Current CPU usage percentage (0-100) with rolling average
    /// </summary>
    double CpuUsagePercent { get; }
    
    /// <summary>
    /// Current memory usage in MB
    /// </summary>
    double MemoryUsageMB { get; }
    
    /// <summary>
    /// Current working set memory in MB
    /// </summary>
    double WorkingSetMB { get; }
    
    /// <summary>
    /// Current GC memory in MB
    /// </summary>
    double GcMemoryMB { get; }
    
    /// <summary>
    /// Is the current performance within budget targets?
    /// </summary>
    bool IsWithinBudget { get; }
    
    /// <summary>
    /// Performance budget status message
    /// </summary>
    string BudgetStatus { get; }
    
    /// <summary>
    /// Event fired when performance metrics are updated
    /// </summary>
    event EventHandler<PerformanceMetrics>? MetricsUpdated;
    
    /// <summary>
    /// Start monitoring performance metrics
    /// </summary>
    Task StartAsync(CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Stop monitoring performance metrics
    /// </summary>
    Task StopAsync(CancellationToken cancellationToken = default);
    
    /// <summary>
    /// Get performance summary report
    /// </summary>
    PerformanceSummary GetSummary();
    
    /// <summary>
    /// Reset rolling averages and counters
    /// </summary>
    void Reset();
}

/// <summary>
/// Performance metrics snapshot
/// </summary>
public record PerformanceMetrics(
    DateTime Timestamp,
    double CpuPercent,
    double MemoryMB,
    double WorkingSetMB,
    double GcMemoryMB,
    bool IsWithinBudget,
    string BudgetStatus
);

/// <summary>
/// Performance summary over time
/// </summary>
public record PerformanceSummary(
    TimeSpan MonitoringDuration,
    double AverageCpuPercent,
    double MaxCpuPercent,
    double AverageMemoryMB,
    double MaxMemoryMB,
    double CurrentMemoryMB,
    int BudgetViolationCount,
    DateTime LastUpdate
);