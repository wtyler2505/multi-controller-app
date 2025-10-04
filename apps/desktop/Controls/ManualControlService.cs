using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Controls;

/// <summary>
/// Service interface for managing manual control widgets and state
/// </summary>
public interface IManualControlService
{
    /// <summary>
    /// The current manual control state
    /// </summary>
    ManualControlState State { get; }
    
    /// <summary>
    /// Widget factory for creating new controls
    /// </summary>
    IControlWidgetFactory WidgetFactory { get; }
    
    /// <summary>
    /// Initialize the manual control service
    /// </summary>
    Task InitializeAsync();
    
    /// <summary>
    /// Create and register a new widget
    /// </summary>
    Task<IControlWidget> CreateWidgetAsync(WidgetCreationConfiguration config);
    
    /// <summary>
    /// Remove a widget from the system
    /// </summary>
    Task<bool> RemoveWidgetAsync(string widgetId);
    
    /// <summary>
    /// Get a widget by ID
    /// </summary>
    IControlWidget? GetWidget(string widgetId);
    
    /// <summary>
    /// Get all registered widgets
    /// </summary>
    IEnumerable<IControlWidget> GetAllWidgets();
    
    /// <summary>
    /// Set a widget value with validation
    /// </summary>
    Task<bool> SetWidgetValueAsync(string widgetId, object? value);
    
    /// <summary>
    /// Activate emergency stop for all controls
    /// </summary>
    Task ActivateEmergencyStopAsync();
    
    /// <summary>
    /// Deactivate emergency stop and restore controls
    /// </summary>
    Task<bool> DeactivateEmergencyStopAsync(string? confirmationCode = null);
    
    /// <summary>
    /// Reset all widgets to their default values
    /// </summary>
    Task ResetAllWidgetsAsync();
    
    /// <summary>
    /// Get a snapshot of all current values
    /// </summary>
    Dictionary<string, object?> GetCurrentSnapshot();
    
    /// <summary>
    /// Event fired when any widget value changes
    /// </summary>
    event EventHandler<WidgetValueChangedEventArgs>? WidgetValueChanged;
    
    /// <summary>
    /// Event fired when emergency stop state changes
    /// </summary>
    event EventHandler<EmergencyStopChangedEventArgs>? EmergencyStopChanged;
}

/// <summary>
/// Implementation of the manual control service
/// </summary>
public class ManualControlService : IManualControlService
{
    private readonly ILogger<ManualControlService> _logger;
    private readonly IControlWidgetFactory _widgetFactory;
    private readonly ManualControlState _state;
    private bool _isInitialized = false;
    
    public ManualControlService(ILogger<ManualControlService> logger, IControlWidgetFactory widgetFactory)
    {
        _logger = logger ?? throw new ArgumentNullException(nameof(logger));
        _widgetFactory = widgetFactory ?? throw new ArgumentNullException(nameof(widgetFactory));
        _state = new ManualControlState();
        
        // Subscribe to state events
        _state.WidgetValueChanged += OnWidgetValueChanged;
        _state.EmergencyStopChanged += OnEmergencyStopChanged;
    }
    
    public ManualControlState State => _state;
    public IControlWidgetFactory WidgetFactory => _widgetFactory;
    
    public event EventHandler<WidgetValueChangedEventArgs>? WidgetValueChanged;
    public event EventHandler<EmergencyStopChangedEventArgs>? EmergencyStopChanged;
    
    public async Task InitializeAsync()
    {
        if (_isInitialized)
        {
            _logger.LogWarning("Manual control service is already initialized");
            return;
        }
        
        _logger.LogInformation("Initializing manual control service...");
        
        try
        {
            // Create default emergency stop widget
            var emergencyStopConfig = new EmergencyStopConfiguration
            {
                RequireConfirmationToReset = true,
                ResetConfirmationCode = "RESET",
                MinimumActiveDuration = TimeSpan.FromSeconds(2),
                LogActivations = true
            };
            
            var emergencyStop = _widgetFactory.CreateEmergencyStop("emergency_stop", "Emergency Stop", emergencyStopConfig);
            _state.RegisterWidget(emergencyStop);
            
            // Create some example widgets for demonstration
            await CreateExampleWidgetsAsync();
            
            _isInitialized = true;
            _logger.LogInformation("Manual control service initialized with {WidgetCount} widgets", _state.WidgetCount);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to initialize manual control service");
            throw;
        }
    }
    
    public async Task<IControlWidget> CreateWidgetAsync(WidgetCreationConfiguration config)
    {
        if (!_isInitialized)
            throw new InvalidOperationException("Service must be initialized before creating widgets");
        
        _logger.LogInformation("Creating widget {WidgetId} of type {WidgetType}", config.Id, config.Type);
        
        try
        {
            var widget = _widgetFactory.CreateFromConfiguration(config);
            _state.RegisterWidget(widget);
            
            _logger.LogDebug("Widget {WidgetId} created and registered successfully", config.Id);
            return widget;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create widget {WidgetId}", config.Id);
            throw;
        }
    }
    
    public async Task<bool> RemoveWidgetAsync(string widgetId)
    {
        if (!_isInitialized)
            throw new InvalidOperationException("Service must be initialized before removing widgets");
        
        _logger.LogInformation("Removing widget {WidgetId}", widgetId);
        
        try
        {
            var result = _state.UnregisterWidget(widgetId);
            if (result)
            {
                _logger.LogDebug("Widget {WidgetId} removed successfully", widgetId);
            }
            else
            {
                _logger.LogWarning("Widget {WidgetId} not found for removal", widgetId);
            }
            return result;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to remove widget {WidgetId}", widgetId);
            return false;
        }
    }
    
    public IControlWidget? GetWidget(string widgetId)
    {
        return _state.GetWidget(widgetId);
    }
    
    public IEnumerable<IControlWidget> GetAllWidgets()
    {
        var widgets = new List<IControlWidget>();
        foreach (var widgetId in _state.WidgetIds)
        {
            var widget = _state.GetWidget(widgetId);
            if (widget != null)
            {
                widgets.Add(widget);
            }
        }
        return widgets;
    }
    
    public async Task<bool> SetWidgetValueAsync(string widgetId, object? value)
    {
        if (!_isInitialized)
            throw new InvalidOperationException("Service must be initialized before setting widget values");
        
        _logger.LogDebug("Setting widget {WidgetId} value to {Value}", widgetId, value);
        
        try
        {
            var result = _state.SetWidgetValue(widgetId, value);
            if (!result)
            {
                _logger.LogWarning("Failed to set widget {WidgetId} value to {Value}", widgetId, value);
            }
            return result;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error setting widget {WidgetId} value", widgetId);
            return false;
        }
    }
    
    public async Task ActivateEmergencyStopAsync()
    {
        _logger.LogWarning("Emergency stop activated!");
        _state.ActivateEmergencyStop();
    }
    
    public async Task<bool> DeactivateEmergencyStopAsync(string? confirmationCode = null)
    {
        _logger.LogInformation("Attempting to deactivate emergency stop with confirmation: {HasConfirmation}", !string.IsNullOrEmpty(confirmationCode));
        
        try
        {
            var emergencyStopWidget = GetWidget("emergency_stop") as EmergencyStopWidget;
            if (emergencyStopWidget != null)
            {
                var result = emergencyStopWidget.Deactivate(confirmationCode);
                if (result)
                {
                    _logger.LogInformation("Emergency stop deactivated successfully");
                }
                else
                {
                    _logger.LogWarning("Emergency stop deactivation failed - invalid confirmation or timing constraints");
                }
                return result;
            }
            else
            {
                // Fallback to state-level deactivation
                _state.DeactivateEmergencyStop();
                _logger.LogInformation("Emergency stop deactivated via state fallback");
                return true;
            }
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error deactivating emergency stop");
            return false;
        }
    }
    
    public async Task ResetAllWidgetsAsync()
    {
        _logger.LogInformation("Resetting all widgets to default values");
        
        try
        {
            await _state.ResetAllAsync();
            _logger.LogDebug("All widgets reset successfully");
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error resetting widgets");
            throw;
        }
    }
    
    public Dictionary<string, object?> GetCurrentSnapshot()
    {
        return _state.GetSnapshot();
    }
    
    private async Task CreateExampleWidgetsAsync()
    {
        try
        {
            // Motor speed slider
            await CreateWidgetAsync(new WidgetCreationConfiguration
            {
                Id = "motor_speed",
                Name = "Motor Speed",
                Type = ControlWidgetType.Slider,
                MinValue = 0,
                MaxValue = 100,
                StepValue = 5,
                DefaultValue = 0.0
            });
            
            // Power toggle
            await CreateWidgetAsync(new WidgetCreationConfiguration
            {
                Id = "power_enable",
                Name = "Power Enable",
                Type = ControlWidgetType.Toggle,
                DefaultValue = false
            });
            
            // Temperature setpoint
            await CreateWidgetAsync(new WidgetCreationConfiguration
            {
                Id = "temperature_setpoint",
                Name = "Temperature Setpoint",
                Type = ControlWidgetType.NumericInput,
                MinValue = 0,
                MaxValue = 200,
                StepValue = 0.5,
                DefaultValue = 25.0,
                DecimalPlaces = 1
            });
            
            // Operation mode dropdown
            await CreateWidgetAsync(new WidgetCreationConfiguration
            {
                Id = "operation_mode",
                Name = "Operation Mode",
                Type = ControlWidgetType.Dropdown,
                Options = new[] { "Manual", "Automatic", "Semi-Auto", "Calibration" },
                DefaultValue = "Manual"
            });
            
            _logger.LogDebug("Example widgets created successfully");
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to create example widgets");
            throw;
        }
    }
    
    private void OnWidgetValueChanged(object? sender, WidgetValueChangedEventArgs e)
    {
        _logger.LogDebug("Widget {WidgetId} value changed from {OldValue} to {NewValue}", 
            e.WidgetId, e.OldValue, e.NewValue);
        
        WidgetValueChanged?.Invoke(this, e);
    }
    
    private void OnEmergencyStopChanged(object? sender, EmergencyStopChangedEventArgs e)
    {
        _logger.LogWarning("Emergency stop state changed from {OldState} to {NewState}", 
            e.OldState, e.NewState);
        
        EmergencyStopChanged?.Invoke(this, e);
    }
}