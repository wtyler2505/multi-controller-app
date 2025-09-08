using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Text;
using MultiControllerApp.Services;
using System;
using System.ComponentModel;
using System.Threading.Tasks;

namespace MultiControllerApp;

/// <summary>
/// Main application window with performance monitoring UI
/// </summary>
public sealed partial class MainWindow : Window, INotifyPropertyChanged
{
    private readonly IServiceProvider _serviceProvider;
    private readonly ILogger<MainWindow> _logger;
    private readonly IPerformanceMonitorService _performanceMonitor;
    private readonly IUIResponsivenessService _uiResponsiveness;
    
    private string _cpuUsage = "0.0%";
    private string _memoryUsage = "0.0 MB";
    private string _budgetStatus = "Within budget";
    private bool _isWithinBudget = true;
    
    public MainWindow(IServiceProvider serviceProvider)
    {
        _serviceProvider = serviceProvider;
        _logger = serviceProvider.GetRequiredService<ILogger<MainWindow>>();
        _performanceMonitor = serviceProvider.GetRequiredService<IPerformanceMonitorService>();
        _uiResponsiveness = serviceProvider.GetRequiredService<IUIResponsivenessService>();
        
        this.InitializeComponent();
        this.Title = "Multi-Controller App - Performance Optimized";
        
        // Subscribe to performance updates
        _performanceMonitor.MetricsUpdated += OnPerformanceMetricsUpdated;
        
        _logger.LogInformation("Main window initialized");
    }
    
    public string CpuUsage
    {
        get => _cpuUsage;
        set
        {
            _cpuUsage = value;
            OnPropertyChanged(nameof(CpuUsage));
        }
    }
    
    public string MemoryUsage
    {
        get => _memoryUsage;
        set
        {
            _memoryUsage = value;
            OnPropertyChanged(nameof(MemoryUsage));
        }
    }
    
    public string BudgetStatus
    {
        get => _budgetStatus;
        set
        {
            _budgetStatus = value;
            OnPropertyChanged(nameof(BudgetStatus));
        }
    }
    
    public bool IsWithinBudget
    {
        get => _isWithinBudget;
        set
        {
            _isWithinBudget = value;
            OnPropertyChanged(nameof(IsWithinBudget));
        }
    }
    
    public event PropertyChangedEventHandler? PropertyChanged;
    
    private async void OnPerformanceMetricsUpdated(object? sender, PerformanceMetrics metrics)
    {
        // Update UI on UI thread to ensure responsiveness
        await _uiResponsiveness.ExecuteOnUIThreadAsync(() =>
        {
            CpuUsage = $"{metrics.CpuPercent:F1}%";
            MemoryUsage = $"{metrics.MemoryMB:F1} MB";
            BudgetStatus = metrics.BudgetStatus;
            IsWithinBudget = metrics.IsWithinBudget;
        });
    }
    
    private async void OnOptimizeMemoryClick(object sender, RoutedEventArgs e)
    {
        var memoryOptimizer = _serviceProvider.GetService<IMemoryOptimizationService>();
        if (memoryOptimizer != null)
        {
            // Execute on background thread to avoid blocking UI
            await _uiResponsiveness.ExecuteAsync(async () =>
            {
                memoryOptimizer.OptimizeMemory();
                _logger.LogInformation("Manual memory optimization completed");
                await Task.Delay(100); // Small delay to see the effect
            });
        }
    }
    
    private async void OnSimulateWorkloadClick(object sender, RoutedEventArgs e)
    {
        // Simulate CPU-intensive work with yielding to maintain UI responsiveness
        await _uiResponsiveness.ExecuteWithYieldingAsync(async (cancellationToken) =>
        {
            _logger.LogInformation("Starting simulated workload...");
            
            // Simulate CPU work
            for (int i = 0; i < 1000 && !cancellationToken.IsCancellationRequested; i++)
            {
                // Do some CPU work
                var result = Math.Sqrt(i * 12345.67);
                
                // Yield every 10 iterations to maintain responsiveness
                if (i % 10 == 0)
                {
                    await Task.Yield();
                }
            }
            
            _logger.LogInformation("Simulated workload completed");
        });
    }
    
    private void OnPropertyChanged(string propertyName)
    {
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
    }
    
    private void InitializeComponent()
    {
        // Create UI programmatically since we don't have XAML files set up
        var rootGrid = new Grid();
        rootGrid.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Auto) });
        rootGrid.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Star) });
        rootGrid.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Auto) });
        
        // Header
        var headerText = new TextBlock
        {
            Text = "Multi-Controller App - Performance Monitor",
            FontSize = 20,
            FontWeight = FontWeights.Bold,
            Margin = new Thickness(20),
            HorizontalAlignment = HorizontalAlignment.Center
        };
        Grid.SetRow(headerText, 0);
        rootGrid.Children.Add(headerText);
        
        // Performance info panel
        var perfPanel = new StackPanel
        {
            Margin = new Thickness(20),
            Spacing = 10
        };
        Grid.SetRow(perfPanel, 1);
        
        // CPU Usage
        var cpuPanel = new StackPanel { Orientation = Orientation.Horizontal };
        cpuPanel.Children.Add(new TextBlock { Text = "CPU Usage: ", FontWeight = FontWeights.SemiBold });
        var cpuText = new TextBlock();
        cpuText.SetBinding(TextBlock.TextProperty, new Microsoft.UI.Xaml.Data.Binding
        {
            Source = this,
            Path = new PropertyPath(nameof(CpuUsage))
        });
        cpuPanel.Children.Add(cpuText);
        perfPanel.Children.Add(cpuPanel);
        
        // Memory Usage
        var memPanel = new StackPanel { Orientation = Orientation.Horizontal };
        memPanel.Children.Add(new TextBlock { Text = "Memory Usage: ", FontWeight = FontWeights.SemiBold });
        var memText = new TextBlock();
        memText.SetBinding(TextBlock.TextProperty, new Microsoft.UI.Xaml.Data.Binding
        {
            Source = this,
            Path = new PropertyPath(nameof(MemoryUsage))
        });
        memPanel.Children.Add(memText);
        perfPanel.Children.Add(memPanel);
        
        // Budget Status
        var statusPanel = new StackPanel { Orientation = Orientation.Horizontal };
        statusPanel.Children.Add(new TextBlock { Text = "Status: ", FontWeight = FontWeights.SemiBold });
        var statusText = new TextBlock();
        statusText.SetBinding(TextBlock.TextProperty, new Microsoft.UI.Xaml.Data.Binding
        {
            Source = this,
            Path = new PropertyPath(nameof(BudgetStatus))
        });
        statusPanel.Children.Add(statusText);
        perfPanel.Children.Add(statusPanel);
        
        rootGrid.Children.Add(perfPanel);
        
        // Buttons
        var buttonPanel = new StackPanel
        {
            Orientation = Orientation.Horizontal,
            Margin = new Thickness(20),
            Spacing = 10,
            HorizontalAlignment = HorizontalAlignment.Center
        };
        Grid.SetRow(buttonPanel, 2);
        
        var optimizeButton = new Button
        {
            Content = "Optimize Memory",
            Padding = new Thickness(15, 8, 15, 8)
        };
        optimizeButton.Click += OnOptimizeMemoryClick;
        buttonPanel.Children.Add(optimizeButton);
        
        var workloadButton = new Button
        {
            Content = "Simulate Workload",
            Padding = new Thickness(15, 8, 15, 8)
        };
        workloadButton.Click += OnSimulateWorkloadClick;
        buttonPanel.Children.Add(workloadButton);
        
        rootGrid.Children.Add(buttonPanel);
        
        this.Content = rootGrid;
    }
}