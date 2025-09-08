using System;
using System.Threading.Tasks;

namespace MultiControllerApp.Controls;

/// <summary>
/// Emergency stop control widget - visually prominent safety button
/// When activated, disables all other controls in the system
/// </summary>
public class EmergencyStopWidget : BaseControlWidget
{
    private readonly EmergencyStopConfiguration _config;
    private DateTime? _activatedTime;
    private TimeSpan _activeDuration;
    
    public EmergencyStopWidget(string id, string name = "Emergency Stop", EmergencyStopConfiguration? config = null)
        : base(id, name, ControlWidgetType.EmergencyStop)
    {
        _config = config ?? new EmergencyStopConfiguration();
        
        // Emergency stop always starts in the safe (not activated) state
        Value = false;
    }
    
    public override double? MinValue => null; // Not applicable for emergency stop
    public override double? MaxValue => null; // Not applicable for emergency stop
    public override double? StepValue => null; // Not applicable for emergency stop
    
    /// <summary>
    /// Whether the emergency stop is currently activated
    /// </summary>
    public bool IsActivated => Convert.ToBoolean(Value ?? false);
    
    /// <summary>
    /// When the emergency stop was last activated (null if never activated)
    /// </summary>
    public DateTime? ActivatedTime => _activatedTime;
    
    /// <summary>
    /// How long the emergency stop has been active (if currently activated)
    /// </summary>
    public TimeSpan ActiveDuration => IsActivated && _activatedTime.HasValue 
        ? DateTime.UtcNow - _activatedTime.Value 
        : _activeDuration;
    
    /// <summary>
    /// Configuration for the emergency stop behavior
    /// </summary>
    public EmergencyStopConfiguration Configuration => _config;
    
    /// <summary>
    /// Event fired when emergency stop is activated
    /// </summary>
    public event EventHandler<EmergencyStopActivatedEventArgs>? Activated;
    
    /// <summary>
    /// Event fired when emergency stop is deactivated/reset
    /// </summary>
    public event EventHandler<EmergencyStopDeactivatedEventArgs>? Deactivated;
    
    public override ValidationResult ValidateValue(object? proposedValue)
    {
        if (proposedValue == null)
            return ValidationResult.Success(false);
        
        // Handle various input types that can represent boolean values
        if (proposedValue is bool boolValue)
            return ValidationResult.Success(boolValue);
        
        if (proposedValue is string stringValue)
        {
            if (bool.TryParse(stringValue, out var parsedBool))
                return ValidationResult.Success(parsedBool);
            
            // Handle emergency stop specific commands
            var lowerString = stringValue.ToLowerInvariant();
            switch (lowerString)
            {
                case "activate":
                case "stop":
                case "emergency":
                case "halt":
                case "abort":
                case "true":
                case "1":
                    return ValidationResult.Success(true);
                
                case "deactivate":
                case "reset":
                case "release":
                case "continue":
                case "false":
                case "0":
                    return ValidationResult.Success(false);
                
                default:
                    return ValidationResult.Error($"Invalid emergency stop command: '{stringValue}'");
            }
        }
        
        if (proposedValue is int intValue)
            return ValidationResult.Success(intValue != 0);
        
        return ValidationResult.Error($"Cannot convert '{proposedValue}' to emergency stop state");
    }
    
    protected override object? GetDefaultValue() => false;
    
    public override object? Value
    {
        get => base.Value;
        set
        {
            var oldValue = Convert.ToBoolean(base.Value ?? false);
            base.Value = value;
            var newValue = Convert.ToBoolean(base.Value ?? false);
            
            // Handle state transitions
            if (oldValue != newValue)
            {
                if (newValue) // Activating emergency stop
                {
                    _activatedTime = DateTime.UtcNow;
                    _activeDuration = TimeSpan.Zero;
                    OnActivated();
                }
                else // Deactivating emergency stop
                {
                    if (_activatedTime.HasValue)
                    {
                        _activeDuration = DateTime.UtcNow - _activatedTime.Value;
                    }
                    _activatedTime = null;
                    OnDeactivated();
                }
            }
        }
    }
    
    /// <summary>
    /// Activate the emergency stop
    /// </summary>
    public void Activate()
    {
        Value = true;
    }
    
    /// <summary>
    /// Deactivate/reset the emergency stop
    /// Only allowed if confirmation is not required or has been provided
    /// </summary>
    /// <param name="confirmationCode">Required confirmation code if configured</param>
    /// <returns>True if deactivation was successful</returns>
    public bool Deactivate(string? confirmationCode = null)
    {
        if (!IsActivated)
            return true; // Already deactivated
        
        if (_config.RequireConfirmationToReset)
        {
            if (string.IsNullOrEmpty(confirmationCode) || confirmationCode != _config.ResetConfirmationCode)
            {
                return false; // Invalid or missing confirmation
            }
        }
        
        if (_config.MinimumActiveDuration.HasValue && ActiveDuration < _config.MinimumActiveDuration.Value)
        {
            return false; // Must remain active for minimum duration
        }
        
        Value = false;
        return true;
    }
    
    /// <summary>
    /// Toggle the emergency stop state
    /// Note: Deactivation may require confirmation depending on configuration
    /// </summary>
    /// <param name="confirmationCode">Required confirmation code for deactivation if configured</param>
    /// <returns>True if toggle was successful</returns>
    public bool Toggle(string? confirmationCode = null)
    {
        if (IsActivated)
        {
            return Deactivate(confirmationCode);
        }
        else
        {
            Activate();
            return true;
        }
    }
    
    /// <summary>
    /// Get the current state as a prominent display string
    /// </summary>
    public string StateDisplay => IsActivated ? "🛑 EMERGENCY STOP ACTIVE" : "✅ NORMAL OPERATION";
    
    /// <summary>
    /// Get the current state as a color indicator
    /// </summary>
    public string ColorIndicator => IsActivated ? "🔴" : "🟢";
    
    /// <summary>
    /// Check if the emergency stop can be deactivated with current settings
    /// </summary>
    /// <param name="confirmationCode">Confirmation code to validate</param>
    /// <returns>True if deactivation would be allowed</returns>
    public bool CanDeactivate(string? confirmationCode = null)
    {
        if (!IsActivated)
            return true;
        
        if (_config.RequireConfirmationToReset && confirmationCode != _config.ResetConfirmationCode)
            return false;
        
        if (_config.MinimumActiveDuration.HasValue && ActiveDuration < _config.MinimumActiveDuration.Value)
            return false;
        
        return true;
    }
    
    public override async Task ResetAsync()
    {
        // Emergency stop reset requires special handling
        if (IsActivated && _config.RequireConfirmationToReset)
        {
            // Cannot auto-reset if confirmation is required
            return;
        }
        
        await Task.CompletedTask;
        Value = false; // Reset to safe state
    }
    
    private void OnActivated()
    {
        Activated?.Invoke(this, new EmergencyStopActivatedEventArgs(DateTime.UtcNow));
    }
    
    private void OnDeactivated()
    {
        Deactivated?.Invoke(this, new EmergencyStopDeactivatedEventArgs(DateTime.UtcNow, _activeDuration));
    }
    
    public override string ToString()
    {
        var duration = IsActivated ? $" (Active for: {ActiveDuration:hh\\:mm\\:ss})" : "";
        return $"EmergencyStop '{Name}' (ID: {Id}): {StateDisplay}{duration}";
    }
}

/// <summary>
/// Configuration options for emergency stop behavior
/// </summary>
public class EmergencyStopConfiguration
{
    /// <summary>
    /// Whether confirmation is required to reset the emergency stop
    /// </summary>
    public bool RequireConfirmationToReset { get; set; } = true;
    
    /// <summary>
    /// Confirmation code required to reset (if RequireConfirmationToReset is true)
    /// </summary>
    public string ResetConfirmationCode { get; set; } = "RESET";
    
    /// <summary>
    /// Minimum time the emergency stop must remain active before it can be reset
    /// </summary>
    public TimeSpan? MinimumActiveDuration { get; set; } = TimeSpan.FromSeconds(2);
    
    /// <summary>
    /// Whether the emergency stop should automatically log activation events
    /// </summary>
    public bool LogActivations { get; set; } = true;
}

/// <summary>
/// Event arguments for emergency stop activation
/// </summary>
public class EmergencyStopActivatedEventArgs : EventArgs
{
    public DateTime ActivatedTime { get; }
    
    public EmergencyStopActivatedEventArgs(DateTime activatedTime)
    {
        ActivatedTime = activatedTime;
    }
}

/// <summary>
/// Event arguments for emergency stop deactivation
/// </summary>
public class EmergencyStopDeactivatedEventArgs : EventArgs
{
    public DateTime DeactivatedTime { get; }
    public TimeSpan ActiveDuration { get; }
    
    public EmergencyStopDeactivatedEventArgs(DateTime deactivatedTime, TimeSpan activeDuration)
    {
        DeactivatedTime = deactivatedTime;
        ActiveDuration = activeDuration;
    }
}