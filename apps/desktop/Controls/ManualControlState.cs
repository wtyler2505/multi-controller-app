using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Runtime.CompilerServices;
using System.Text.Json;
using System.Threading.Tasks;

namespace MultiControllerApp.Controls;

/// <summary>
/// Structure to store control states with real-time updates and validation
/// Implements INotifyPropertyChanged for reactive UI updates
/// </summary>
public class ManualControlState : INotifyPropertyChanged
{
    private readonly Dictionary<string, IControlWidget> _widgets = new();
    private readonly Dictionary<string, object?> _values = new();
    private readonly object _lockObject = new();
    private bool _emergencyStop;
    private DateTime _lastUpdateTime;
    
    /// <summary>
    /// Event fired when any property changes
    /// </summary>
    public event PropertyChangedEventHandler? PropertyChanged;
    
    /// <summary>
    /// Event fired when any widget value changes
    /// </summary>
    public event EventHandler<WidgetValueChangedEventArgs>? WidgetValueChanged;
    
    /// <summary>
    /// Event fired when emergency stop state changes
    /// </summary>
    public event EventHandler<EmergencyStopChangedEventArgs>? EmergencyStopChanged;
    
    /// <summary>
    /// Current emergency stop state - when true, all controls should be disabled
    /// </summary>
    public bool EmergencyStop
    {
        get => _emergencyStop;
        private set
        {
            if (_emergencyStop != value)
            {
                var oldValue = _emergencyStop;
                _emergencyStop = value;
                OnPropertyChanged();
                EmergencyStopChanged?.Invoke(this, new EmergencyStopChangedEventArgs(oldValue, value));
                
                // Disable all widgets when emergency stop is activated
                if (value)
                {
                    DisableAllWidgets();
                }
            }
        }
    }
    
    /// <summary>
    /// Last time any control value was updated
    /// </summary>
    public DateTime LastUpdateTime
    {
        get => _lastUpdateTime;
        private set
        {
            _lastUpdateTime = value;
            OnPropertyChanged();
        }
    }
    
    /// <summary>
    /// Number of registered widgets
    /// </summary>
    public int WidgetCount => _widgets.Count;
    
    /// <summary>
    /// Get all registered widget IDs
    /// </summary>
    public IEnumerable<string> WidgetIds => _widgets.Keys;
    
    /// <summary>
    /// Register a new control widget
    /// </summary>
    /// <param name="widget">Widget to register</param>
    public void RegisterWidget(IControlWidget widget)
    {
        if (widget == null) throw new ArgumentNullException(nameof(widget));
        
        lock (_lockObject)
        {
            if (_widgets.ContainsKey(widget.Id))
            {
                throw new ArgumentException($"Widget with ID '{widget.Id}' is already registered.");
            }
            
            _widgets[widget.Id] = widget;
            _values[widget.Id] = widget.Value;
            
            // Subscribe to widget events
            widget.ValueChanged += OnWidgetValueChanged;
            widget.StateChanged += OnWidgetStateChanged;
        }
    }
    
    /// <summary>
    /// Unregister a control widget
    /// </summary>
    /// <param name="widgetId">ID of widget to unregister</param>
    public bool UnregisterWidget(string widgetId)
    {
        lock (_lockObject)
        {
            if (_widgets.TryGetValue(widgetId, out var widget))
            {
                // Unsubscribe from events
                widget.ValueChanged -= OnWidgetValueChanged;
                widget.StateChanged -= OnWidgetStateChanged;
                
                _widgets.Remove(widgetId);
                _values.Remove(widgetId);
                return true;
            }
            return false;
        }
    }
    
    /// <summary>
    /// Get a widget by ID
    /// </summary>
    /// <param name="widgetId">Widget ID</param>
    /// <returns>Widget instance or null if not found</returns>
    public IControlWidget? GetWidget(string widgetId)
    {
        lock (_lockObject)
        {
            return _widgets.TryGetValue(widgetId, out var widget) ? widget : null;
        }
    }
    
    /// <summary>
    /// Get the current value of a widget
    /// </summary>
    /// <param name="widgetId">Widget ID</param>
    /// <returns>Current value or null if widget not found</returns>
    public object? GetWidgetValue(string widgetId)
    {
        lock (_lockObject)
        {
            return _values.TryGetValue(widgetId, out var value) ? value : null;
        }
    }
    
    /// <summary>
    /// Set the value of a widget with validation
    /// </summary>
    /// <param name="widgetId">Widget ID</param>
    /// <param name="value">New value</param>
    /// <returns>True if value was set successfully</returns>
    public bool SetWidgetValue(string widgetId, object? value)
    {
        lock (_lockObject)
        {
            if (!_widgets.TryGetValue(widgetId, out var widget))
                return false;
            
            // Don't allow changes during emergency stop (except for emergency stop widget itself)
            if (_emergencyStop && widget.Type != ControlWidgetType.EmergencyStop)
                return false;
            
            var validationResult = widget.ValidateValue(value);
            if (validationResult.IsValid)
            {
                widget.Value = validationResult.ValidatedValue;
                return true;
            }
            
            return false;
        }
    }
    
    /// <summary>
    /// Activate emergency stop - disables all controls except emergency stop itself
    /// </summary>
    public void ActivateEmergencyStop()
    {
        EmergencyStop = true;
    }
    
    /// <summary>
    /// Deactivate emergency stop - re-enables all controls
    /// </summary>
    public void DeactivateEmergencyStop()
    {
        EmergencyStop = false;
        
        // Re-enable all widgets
        lock (_lockObject)
        {
            foreach (var widget in _widgets.Values)
            {
                if (widget.Type != ControlWidgetType.EmergencyStop)
                {
                    widget.IsEnabled = true;
                }
            }
        }
    }
    
    /// <summary>
    /// Reset all widgets to their default values
    /// </summary>
    public async Task ResetAllAsync()
    {
        var tasks = new List<Task>();
        
        lock (_lockObject)
        {
            foreach (var widget in _widgets.Values)
            {
                tasks.Add(widget.ResetAsync());
            }
        }
        
        await Task.WhenAll(tasks);
        LastUpdateTime = DateTime.UtcNow;
    }
    
    /// <summary>
    /// Get a snapshot of all current widget values
    /// </summary>
    /// <returns>Dictionary of widget ID to current value</returns>
    public Dictionary<string, object?> GetSnapshot()
    {
        lock (_lockObject)
        {
            return new Dictionary<string, object?>(_values);
        }
    }
    
    /// <summary>
    /// Serialize current state to JSON
    /// </summary>
    /// <returns>JSON representation of current state</returns>
    public string ToJson()
    {
        var snapshot = GetSnapshot();
        var stateData = new
        {
            EmergencyStop = _emergencyStop,
            LastUpdateTime = _lastUpdateTime,
            Values = snapshot
        };
        
        return JsonSerializer.Serialize(stateData, new JsonSerializerOptions { WriteIndented = true });
    }
    
    private void DisableAllWidgets()
    {
        lock (_lockObject)
        {
            foreach (var widget in _widgets.Values)
            {
                if (widget.Type != ControlWidgetType.EmergencyStop)
                {
                    widget.IsEnabled = false;
                }
            }
        }
    }
    
    private void OnWidgetValueChanged(object? sender, ControlValueChangedEventArgs e)
    {
        if (sender is IControlWidget widget)
        {
            lock (_lockObject)
            {
                _values[widget.Id] = e.NewValue;
            }
            
            LastUpdateTime = DateTime.UtcNow;
            WidgetValueChanged?.Invoke(this, new WidgetValueChangedEventArgs(widget.Id, e.OldValue, e.NewValue));
        }
    }
    
    private void OnWidgetStateChanged(object? sender, ControlStateChangedEventArgs e)
    {
        // Handle widget state changes if needed
        LastUpdateTime = DateTime.UtcNow;
    }
    
    protected virtual void OnPropertyChanged([CallerMemberName] string? propertyName = null)
    {
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
    }
}

/// <summary>
/// Event arguments for widget value changes in the manual control state
/// </summary>
public class WidgetValueChangedEventArgs : EventArgs
{
    public string WidgetId { get; }
    public object? OldValue { get; }
    public object? NewValue { get; }
    public DateTime Timestamp { get; }
    
    public WidgetValueChangedEventArgs(string widgetId, object? oldValue, object? newValue)
    {
        WidgetId = widgetId;
        OldValue = oldValue;
        NewValue = newValue;
        Timestamp = DateTime.UtcNow;
    }
}

/// <summary>
/// Event arguments for emergency stop state changes
/// </summary>
public class EmergencyStopChangedEventArgs : EventArgs
{
    public bool OldState { get; }
    public bool NewState { get; }
    public DateTime Timestamp { get; }
    
    public EmergencyStopChangedEventArgs(bool oldState, bool newState)
    {
        OldState = oldState;
        NewState = newState;
        Timestamp = DateTime.UtcNow;
    }
}