using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Logging;
using Microsoft.UI.Xaml;
using System;
using System.Diagnostics;
using System.Threading.Tasks;

namespace MultiControllerApp;

/// <summary>
/// Main application entry point with performance monitoring
/// </summary>
public class Program
{
    private static readonly Stopwatch _startupTimer = new();
    
    [STAThread]
    public static void Main(string[] args)
    {
        _startupTimer.Start();
        
        try
        {
            // Initialize WinRT
            WinRT.ComWrappersSupport.InitializeComWrappers();
            
            // Start application
            Application.Start((p) =>
            {
                var context = new DispatcherQueueSynchronizationContext(
                    DispatcherQueueController.CreateOnCurrentThread().DispatcherQueue);
                SynchronizationContext.SetSynchronizationContext(context);
                
                _ = new App();
            });
        }
        catch (Exception ex)
        {
            Debug.WriteLine($"Fatal error during startup: {ex}");
            Environment.Exit(1);
        }
    }
    
    /// <summary>
    /// Check if startup time is within budget (< 2 seconds)
    /// </summary>
    public static void ValidateStartupPerformance()
    {
        _startupTimer.Stop();
        var startupMs = _startupTimer.ElapsedMilliseconds;
        
        if (startupMs > 2000)
        {
            Debug.WriteLine($"⚠️ Startup time ({startupMs}ms) exceeds 2s budget!");
        }
        else
        {
            Debug.WriteLine($"✅ Startup time: {startupMs}ms");
        }
    }
}