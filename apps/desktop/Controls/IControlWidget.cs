using System;
using System.Threading.Tasks;

namespace MultiControllerApp.Controls;

/// <summary>
/// Interface for manual control widgets providing extensibility
/// Equivalent to the ControlWidget trait mentioned in the task requirements
/// </summary>
public interface IControlWidget
{
    /// <summary>
    /// Unique identifier for this widget
    /// </summary>
    string Id { get; }
    
    /// <summary>
    /// Display name for the widget
    /// </summary>
    string Name { get; }
    
    /// <summary>
    /// Type of control widget (Slider, Toggle, NumericInput, Dropdown, EmergencyStop)
    /// </summary>
    ControlWidgetType Type { get; }
    
    /// <summary>
    /// Current value of the widget
    /// </summary>
    object? Value { get; set; }
    
    /// <summary>
    /// Whether the widget is currently enabled for user interaction
    /// </summary>
    bool IsEnabled { get; set; }
    
    /// <summary>
    /// Minimum allowed value (for numeric widgets)
    /// </summary>
    double? MinValue { get; }
    
    /// <summary>
    /// Maximum allowed value (for numeric widgets)
    /// </summary>
    double? MaxValue { get; }
    
    /// <summary>
    /// Step/increment value (for numeric widgets)
    /// </summary>
    double? StepValue { get; }
    
    /// <summary>
    /// Event fired when the widget value changes
    /// </summary>
    event EventHandler<ControlValueChangedEventArgs>? ValueChanged;
    
    /// <summary>
    /// Event fired when the widget state changes (enabled/disabled)
    /// </summary>
    event EventHandler<ControlStateChangedEventArgs>? StateChanged;
    
    /// <summary>
    /// Validate that a proposed value is within acceptable bounds
    /// </summary>
    /// <param name="proposedValue">Value to validate</param>
    /// <returns>Validation result with clamped value if needed</returns>
    ValidationResult ValidateValue(object? proposedValue);
    
    /// <summary>
    /// Reset the widget to its default state and value
    /// </summary>
    Task ResetAsync();
    
    /// <summary>
    /// Update the widget's configuration
    /// </summary>
    /// <param name="config">New configuration parameters</param>
    Task UpdateConfigurationAsync(WidgetConfiguration config);
}

/// <summary>
/// Types of control widgets available
/// </summary>
public enum ControlWidgetType
{
    Slider,
    Toggle,
    NumericInput,
    Dropdown,
    EmergencyStop,
    Button
}

/// <summary>
/// Event arguments for widget value changes
/// </summary>
public class ControlValueChangedEventArgs : EventArgs
{
    public object? OldValue { get; }
    public object? NewValue { get; }
    public DateTime Timestamp { get; }
    
    public ControlValueChangedEventArgs(object? oldValue, object? newValue)
    {
        OldValue = oldValue;
        NewValue = newValue;
        Timestamp = DateTime.UtcNow;
    }
}

/// <summary>
/// Event arguments for widget state changes
/// </summary>
public class ControlStateChangedEventArgs : EventArgs
{
    public bool IsEnabled { get; }
    public DateTime Timestamp { get; }
    
    public ControlStateChangedEventArgs(bool isEnabled)
    {
        IsEnabled = isEnabled;
        Timestamp = DateTime.UtcNow;
    }
}

/// <summary>
/// Result of value validation
/// </summary>
public class ValidationResult
{
    public bool IsValid { get; }
    public object? ValidatedValue { get; }
    public string? ErrorMessage { get; }
    
    public ValidationResult(bool isValid, object? validatedValue, string? errorMessage = null)
    {
        IsValid = isValid;
        ValidatedValue = validatedValue;
        ErrorMessage = errorMessage;
    }
    
    public static ValidationResult Success(object? value) => new(true, value);
    public static ValidationResult Error(string message) => new(false, null, message);
}

/// <summary>
/// Configuration parameters for widgets
/// </summary>
public class WidgetConfiguration
{
    public double? MinValue { get; set; }
    public double? MaxValue { get; set; }
    public double? StepValue { get; set; }
    public string[]? Options { get; set; } // For dropdown widgets
    public bool IsEnabled { get; set; } = true;
    public object? DefaultValue { get; set; }
}