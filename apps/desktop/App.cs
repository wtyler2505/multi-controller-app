using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using MultiControllerApp.Services;
using System;
using System.Threading.Tasks;

namespace MultiControllerApp;

/// <summary>
/// Main application class with performance monitoring integration
/// </summary>
public partial class App : Application
{
    private readonly IServiceProvider _serviceProvider;
    private Window? _window;
    private IPerformanceMonitorService? _performanceMonitor;
    private IMemoryOptimizationService? _memoryOptimizer;
    private IUIResponsivenessService? _uiResponsiveness;
    
    public App()
    {
        this.InitializeComponent();
        
        // Set up dependency injection
        var services = new ServiceCollection();
        ConfigureServices(services);
        _serviceProvider = services.BuildServiceProvider();
        
        // Configure logging
        var logger = _serviceProvider.GetService<ILogger<App>>();
        logger?.LogInformation("Multi-Controller App starting up...");
    }
    
    protected override async void OnLaunched(Microsoft.UI.Xaml.LaunchActivatedEventArgs args)
    {
        // Initialize services
        _performanceMonitor = _serviceProvider.GetService<IPerformanceMonitorService>();
        _memoryOptimizer = _serviceProvider.GetService<IMemoryOptimizationService>();
        _uiResponsiveness = _serviceProvider.GetService<IUIResponsivenessService>();
        
        // Start performance monitoring
        if (_performanceMonitor != null)
        {
            await _performanceMonitor.StartAsync();
        }
        
        // Create main window
        _window = new MainWindow(_serviceProvider);
        _window.Activate();
        
        // Validate startup performance
        Program.ValidateStartupPerformance();
        
        // Subscribe to performance events for logging
        if (_performanceMonitor != null)
        {
            _performanceMonitor.MetricsUpdated += OnPerformanceMetricsUpdated;
        }
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
            builder.AddDebug();
            builder.SetMinimumLevel(LogLevel.Information);
        });
        
        // Performance monitoring services
        services.AddSingleton<IPerformanceMonitorService, PerformanceMonitorService>();
        services.AddSingleton<IMemoryOptimizationService, MemoryOptimizationService>();
        services.AddSingleton<IUIResponsivenessService, UIResponsivenessService>();
        
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
    
    private void InitializeComponent()
    {
        // Required for XAML compilation
    }
}