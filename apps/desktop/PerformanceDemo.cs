using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using MultiControllerApp.Services;
using System;
using System.Threading;
using System.Threading.Tasks;

namespace MultiControllerApp.Demo;

/// <summary>
/// Console demonstration of performance monitoring features
/// This demonstrates the core functionality without WinUI dependencies
/// </summary>
public class PerformanceDemo
{
    public static async Task RunDemoAsync()
    {
        Console.WriteLine("Multi-Controller App - Performance Monitoring Demo");
        Console.WriteLine("==================================================");
        
        // Set up dependency injection
        var services = new ServiceCollection();
        ConfigureServices(services);
        var serviceProvider = services.BuildServiceProvider();
        
        // Get services
        var performanceMonitor = serviceProvider.GetRequiredService<IPerformanceMonitorService>();
        var memoryOptimizer = serviceProvider.GetRequiredService<IMemoryOptimizationService>();
        var logger = serviceProvider.GetRequiredService<ILogger<PerformanceDemo>>();
        
        // Subscribe to performance events
        performanceMonitor.MetricsUpdated += (sender, metrics) =>
        {
            var status = metrics.IsWithinBudget ? "✅" : "⚠️";
            Console.WriteLine($"{status} CPU: {metrics.CpuPercent:F1}% | Memory: {metrics.MemoryMB:F1}MB | {metrics.BudgetStatus}");
        };
        
        // Start monitoring
        logger.LogInformation("Starting performance monitoring...");
        await performanceMonitor.StartAsync();
        
        Console.WriteLine("\nMonitoring performance for 30 seconds...");
        Console.WriteLine("Press 'o' to optimize memory, 'w' to simulate workload, 'q' to quit");
        
        var cts = new CancellationTokenSource();
        
        // Monitor for 30 seconds or until user input
        var monitoringTask = Task.Delay(30000, cts.Token);
        var inputTask = Task.Run(async () =>
        {
            while (!cts.Token.IsCancellationRequested)
            {
                var key = Console.ReadKey(true);
                switch (key.KeyChar)
                {
                    case 'o':
                        Console.WriteLine("🔧 Optimizing memory...");
                        memoryOptimizer.OptimizeMemory();
                        var stats = memoryOptimizer.GetPoolStatistics();
                        Console.WriteLine($"   Pool stats: {stats.TotalPools} pools, {stats.TotalObjectsPooled} objects pooled");
                        break;
                        
                    case 'w':
                        Console.WriteLine("⚡ Simulating CPU workload...");
                        _ = Task.Run(() => SimulateCpuWork());
                        break;
                        
                    case 'q':
                        Console.WriteLine("👋 Stopping monitoring...");
                        cts.Cancel();
                        return;
                }
            }
        });
        
        // Wait for completion
        await Task.WhenAny(monitoringTask, inputTask);
        cts.Cancel();
        
        // Stop monitoring and show summary
        await performanceMonitor.StopAsync();
        
        var summary = performanceMonitor.GetSummary();
        Console.WriteLine("\n📊 Performance Summary:");
        Console.WriteLine($"   Duration: {summary.MonitoringDuration:mm\\:ss}");
        Console.WriteLine($"   Avg CPU: {summary.AverageCpuPercent:F1}%");
        Console.WriteLine($"   Max CPU: {summary.MaxCpuPercent:F1}%");
        Console.WriteLine($"   Avg Memory: {summary.AverageMemoryMB:F1}MB");
        Console.WriteLine($"   Max Memory: {summary.MaxMemoryMB:F1}MB");
        Console.WriteLine($"   Budget violations: {summary.BudgetViolationCount}");
        
        Console.WriteLine("\nDemo completed successfully! ✨");
    }
    
    private static void ConfigureServices(IServiceCollection services)
    {
        // Logging
        services.AddLogging(builder =>
        {
            builder.AddConsole();
            builder.SetMinimumLevel(LogLevel.Information);
        });
        
        // Performance monitoring services
        services.AddSingleton<IPerformanceMonitorService, PerformanceMonitorService>();
        services.AddSingleton<IMemoryOptimizationService, MemoryOptimizationService>();
    }
    
    private static void SimulateCpuWork()
    {
        // Simulate CPU-intensive work
        var random = new Random();
        for (int i = 0; i < 1000000; i++)
        {
            _ = Math.Sqrt(random.NextDouble() * 12345.67);
            if (i % 10000 == 0)
            {
                Thread.Sleep(1); // Small pause to see effect
            }
        }
    }
}