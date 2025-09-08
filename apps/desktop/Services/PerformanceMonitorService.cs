using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Services;

/// <summary>
/// Monitors system performance metrics with rolling averages for stable readings
/// Ensures performance stays within defined budgets (CPU ≤ 2%, Memory ≤ 150MB)
/// </summary>
public class PerformanceMonitorService : IPerformanceMonitorService, IDisposable
{
    private readonly ILogger<PerformanceMonitorService> _logger;
    private readonly Timer _monitoringTimer;
    private readonly Process _currentProcess;
    private readonly PerformanceCounter _cpuCounter;
    private readonly object _lock = new();
    
    // Rolling averages for stable readings
    private readonly Queue<double> _cpuReadings = new();
    private readonly Queue<double> _memoryReadings = new();
    private const int MaxReadings = 10; // Rolling window size
    
    // Performance targets from project requirements
    private const double MaxIdleCpuPercent = 2.0;
    private const double MaxBaseMemoryMB = 150.0;
    
    // Monitoring state
    private volatile bool _isMonitoring;
    private DateTime _startTime;
    private double _maxCpuPercent;
    private double _maxMemoryMB;
    private int _budgetViolationCount;
    private bool _disposed;
    
    public PerformanceMonitorService(ILogger<PerformanceMonitorService> logger)
    {
        _logger = logger;
        _currentProcess = Process.GetCurrentProcess();
        
        // Initialize CPU performance counter for accurate Windows CPU monitoring
        try
        {
            _cpuCounter = new PerformanceCounter("Process", "% Processor Time", _currentProcess.ProcessName, true);
            _cpuCounter.NextValue(); // Initialize counter (first call returns 0)
        }
        catch (Exception ex)
        {
            _logger.LogWarning(ex, "Failed to initialize CPU performance counter, will use fallback method");
            _cpuCounter = null!;
        }
        
        _monitoringTimer = new Timer(UpdateMetrics, null, Timeout.Infinite, Timeout.Infinite);
        
        _logger.LogInformation("Performance monitor initialized with targets: CPU ≤ {MaxCpu}%, Memory ≤ {MaxMemory}MB", 
            MaxIdleCpuPercent, MaxBaseMemoryMB);
    }
    
    public double CpuUsagePercent { get; private set; }
    public double MemoryUsageMB { get; private set; }
    public double WorkingSetMB { get; private set; }
    public double GcMemoryMB { get; private set; }
    public bool IsWithinBudget { get; private set; } = true;
    public string BudgetStatus { get; private set; } = "Within budget";
    
    public event EventHandler<PerformanceMetrics>? MetricsUpdated;
    
    public async Task StartAsync(CancellationToken cancellationToken = default)
    {
        if (_isMonitoring)
        {
            _logger.LogWarning("Performance monitoring is already running");
            return;
        }
        
        _logger.LogInformation("Starting performance monitoring...");
        
        lock (_lock)
        {
            _isMonitoring = true;
            _startTime = DateTime.UtcNow;
            _maxCpuPercent = 0;
            _maxMemoryMB = 0;
            _budgetViolationCount = 0;
            _cpuReadings.Clear();
            _memoryReadings.Clear();
        }
        
        // Start monitoring with 1-second intervals for responsive feedback
        _monitoringTimer.Change(TimeSpan.Zero, TimeSpan.FromSeconds(1));
        
        _logger.LogInformation("Performance monitoring started");
        await Task.CompletedTask;
    }
    
    public async Task StopAsync(CancellationToken cancellationToken = default)
    {
        if (!_isMonitoring)
            return;
            
        _logger.LogInformation("Stopping performance monitoring...");
        
        _isMonitoring = false;
        _monitoringTimer.Change(Timeout.Infinite, Timeout.Infinite);
        
        var summary = GetSummary();
        _logger.LogInformation("Performance monitoring stopped. Summary: Avg CPU: {AvgCpu:F1}%, Max CPU: {MaxCpu:F1}%, " +
                             "Avg Memory: {AvgMem:F1}MB, Max Memory: {MaxMem:F1}MB, Budget violations: {Violations}",
            summary.AverageCpuPercent, summary.MaxCpuPercent, 
            summary.AverageMemoryMB, summary.MaxMemoryMB, summary.BudgetViolationCount);
            
        await Task.CompletedTask;
    }
    
    public PerformanceSummary GetSummary()
    {
        lock (_lock)
        {
            var duration = _isMonitoring ? DateTime.UtcNow - _startTime : TimeSpan.Zero;
            var avgCpu = _cpuReadings.Count > 0 ? _cpuReadings.Average() : 0;
            var avgMemory = _memoryReadings.Count > 0 ? _memoryReadings.Average() : 0;
            
            return new PerformanceSummary(
                duration,
                avgCpu,
                _maxCpuPercent,
                avgMemory,
                _maxMemoryMB,
                MemoryUsageMB,
                _budgetViolationCount,
                DateTime.UtcNow
            );
        }
    }
    
    public void Reset()
    {
        lock (_lock)
        {
            _cpuReadings.Clear();
            _memoryReadings.Clear();
            _maxCpuPercent = 0;
            _maxMemoryMB = 0;
            _budgetViolationCount = 0;
            _startTime = DateTime.UtcNow;
        }
        
        _logger.LogInformation("Performance monitoring metrics reset");
    }
    
    private void UpdateMetrics(object? state)
    {
        if (!_isMonitoring || _disposed)
            return;
            
        try
        {
            // Get CPU usage with fallback methods for Windows compatibility
            var cpuPercent = GetCpuUsage();
            
            // Get memory metrics
            _currentProcess.Refresh();
            var workingSetMB = _currentProcess.WorkingSet64 / (1024.0 * 1024.0);
            var gcMemoryMB = GC.GetTotalMemory(false) / (1024.0 * 1024.0);
            var totalMemoryMB = workingSetMB; // Use working set as primary memory metric
            
            lock (_lock)
            {
                // Update rolling averages for stable readings
                _cpuReadings.Enqueue(cpuPercent);
                if (_cpuReadings.Count > MaxReadings)
                    _cpuReadings.Dequeue();
                    
                _memoryReadings.Enqueue(totalMemoryMB);
                if (_memoryReadings.Count > MaxReadings)
                    _memoryReadings.Dequeue();
                
                // Calculate rolling averages
                CpuUsagePercent = _cpuReadings.Average();
                MemoryUsageMB = _memoryReadings.Average();
                WorkingSetMB = workingSetMB;
                GcMemoryMB = gcMemoryMB;
                
                // Track maximums
                _maxCpuPercent = Math.Max(_maxCpuPercent, CpuUsagePercent);
                _maxMemoryMB = Math.Max(_maxMemoryMB, MemoryUsageMB);
                
                // Check budget compliance
                var withinCpuBudget = CpuUsagePercent <= MaxIdleCpuPercent;
                var withinMemoryBudget = MemoryUsageMB <= MaxBaseMemoryMB;
                IsWithinBudget = withinCpuBudget && withinMemoryBudget;
                
                if (!IsWithinBudget)
                {
                    _budgetViolationCount++;
                    BudgetStatus = $"Budget exceeded: CPU {CpuUsagePercent:F1}%/{MaxIdleCpuPercent}%, Memory {MemoryUsageMB:F1}MB/{MaxBaseMemoryMB}MB";
                }
                else
                {
                    BudgetStatus = "Within budget";
                }
            }
            
            // Fire event for UI updates (on background thread to avoid blocking)
            var metrics = new PerformanceMetrics(
                DateTime.UtcNow,
                CpuUsagePercent,
                MemoryUsageMB,
                WorkingSetMB,
                GcMemoryMB,
                IsWithinBudget,
                BudgetStatus
            );
            
            MetricsUpdated?.Invoke(this, metrics);
            
            // Log budget violations
            if (!IsWithinBudget)
            {
                _logger.LogWarning("Performance budget violation: {Status}", BudgetStatus);
            }
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error updating performance metrics");
        }
    }
    
    private double GetCpuUsage()
    {
        try
        {
            // Primary method: Use Windows Performance Counter for accurate CPU monitoring
            if (_cpuCounter != null)
            {
                var cpuValue = _cpuCounter.NextValue();
                // Performance counter returns total CPU time, divide by core count for process CPU
                return cpuValue / Environment.ProcessorCount;
            }
        }
        catch (Exception ex)
        {
            _logger.LogDebug(ex, "Performance counter CPU reading failed, using fallback");
        }
        
        // Fallback method: Calculate CPU usage from process times
        try
        {
            _currentProcess.Refresh();
            var startTime = DateTime.UtcNow;
            var startCpuUsage = _currentProcess.TotalProcessorTime;
            
            // Small delay for measurement
            Thread.Sleep(100);
            
            _currentProcess.Refresh();
            var endTime = DateTime.UtcNow;
            var endCpuUsage = _currentProcess.TotalProcessorTime;
            
            var cpuUsedMs = (endCpuUsage - startCpuUsage).TotalMilliseconds;
            var totalMsPassed = (endTime - startTime).TotalMilliseconds;
            var cpuUsageTotal = cpuUsedMs / totalMsPassed;
            
            // Convert to percentage and account for multiple cores
            return (cpuUsageTotal * 100.0) / Environment.ProcessorCount;
        }
        catch (Exception ex)
        {
            _logger.LogDebug(ex, "Fallback CPU calculation failed");
            return 0.0; // Return 0 if all methods fail
        }
    }
    
    public void Dispose()
    {
        if (_disposed)
            return;
            
        _disposed = true;
        
        try
        {
            _monitoringTimer?.Dispose();
            _cpuCounter?.Dispose();
            _currentProcess?.Dispose();
        }
        catch (Exception ex)
        {
            _logger.LogWarning(ex, "Error disposing performance monitor resources");
        }
        
        GC.SuppressFinalize(this);
    }
}