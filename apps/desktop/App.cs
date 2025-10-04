using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using MultiControllerApp.Services;
using MultiControllerApp.Controls;
using System;
using System.Threading.Tasks;

namespace MultiControllerApp;

/// <summary>
/// Main application class with performance monitoring integration
/// </summary>
public class App
{
    private readonly IServiceProvider _serviceProvider;
    private IPerformanceMonitorService? _performanceMonitor;
    private IMemoryOptimizationService? _memoryOptimizer;
    private IUIResponsivenessService? _uiResponsiveness;
    private IManualControlService? _manualControlService;
    
    public App()
    {
        // Set up dependency injection
        var services = new ServiceCollection();
        ConfigureServices(services);
        _serviceProvider = services.BuildServiceProvider();
        
        // Configure logging
        var logger = _serviceProvider.GetService<ILogger<App>>();
        logger?.LogInformation("Multi-Controller App starting up...");
    }
    
    public async Task StartAsync()
    {
        // Initialize services
        _performanceMonitor = _serviceProvider.GetService<IPerformanceMonitorService>();
        _memoryOptimizer = _serviceProvider.GetService<IMemoryOptimizationService>();
        _uiResponsiveness = _serviceProvider.GetService<IUIResponsivenessService>();
        _manualControlService = _serviceProvider.GetService<IManualControlService>();
        
        // Start performance monitoring
        if (_performanceMonitor != null)
        {
            await _performanceMonitor.StartAsync();
        }
        
        // Initialize manual control system
        if (_manualControlService != null)
        {
            await _manualControlService.InitializeAsync();
        }
        
        // Subscribe to performance events for logging
        if (_performanceMonitor != null)
        {
            _performanceMonitor.MetricsUpdated += OnPerformanceMetricsUpdated;
        }
        
        var logger = _serviceProvider.GetService<ILogger<App>>();
        logger?.LogInformation("Application started successfully");
        
        Console.WriteLine("Multi-Controller App is running...");
        Console.WriteLine("Manual control widgets initialized and ready.");
        
        // Display current widget status
        await DisplayWidgetStatusAsync();
    }
    
    private async Task DisplayWidgetStatusAsync()
    {
        if (_manualControlService == null) return;
        
        Console.WriteLine("\n=== Manual Control Widgets ===");
        var widgets = _manualControlService.GetAllWidgets();
        foreach (var widget in widgets)
        {
            Console.WriteLine($"- {widget}");
        }
        
        Console.WriteLine($"\nEmergency Stop: {(_manualControlService.State.EmergencyStop ? "🛑 ACTIVE" : "✅ NORMAL")}");
        Console.WriteLine($"Total Widgets: {_manualControlService.State.WidgetCount}");
        Console.WriteLine($"Last Update: {_manualControlService.State.LastUpdateTime:HH:mm:ss}");
    }
    
    private void OnPerformanceMetricsUpdated(object? sender, PerformanceMetrics metrics)
    {
        var logger = _serviceProvider.GetService<ILogger<App>>();
        
        // Log performance violations
        if (!metrics.IsWithinBudget)
        {
            logger?.LogWarning("Performance budget violation: {Status}", metrics.BudgetStatus);
        }
        
        // Optimize memory if usage is high
        if (metrics.MemoryMB > 120 && _memoryOptimizer != null) // 80% of 150MB budget
        {
            Task.Run(() =>
            {
                _memoryOptimizer.OptimizeMemory();
                logger?.LogInformation("Memory optimization triggered at {MemoryMB}MB", metrics.MemoryMB);
            });
        }
    }
    
    private void ConfigureServices(IServiceCollection services)
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
        services.AddSingleton<IUIResponsivenessService, UIResponsivenessService>();
        
        // Manual control services
        services.AddSingleton<IManualControlService, ManualControlService>();
        services.AddSingleton<IControlWidgetFactory, ControlWidgetFactory>();
        
        // Configuration
        services.AddSingleton<IConfiguration>(provider =>
        {
            return new ConfigurationBuilder()
                .SetBasePath(AppContext.BaseDirectory)
                .AddJsonFile("appsettings.json", optional: true)
                .AddEnvironmentVariables()
                .Build();
        });
    }
}