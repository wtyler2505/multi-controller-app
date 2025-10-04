using System;

namespace MultiControllerApp.Controls;

/// <summary>
/// Toggle control widget for boolean on/off states
/// </summary>
public class ToggleWidget : BaseControlWidget
{
    private readonly bool _defaultValue;
    
    public ToggleWidget(string id, string name, bool defaultValue = false)
        : base(id, name, ControlWidgetType.Toggle)
    {
        _defaultValue = defaultValue;
        
        // Set initial value
        Value = _defaultValue;
    }
    
    public override double? MinValue => null; // Not applicable for toggle
    public override double? MaxValue => null; // Not applicable for toggle
    public override double? StepValue => null; // Not applicable for toggle
    
    public override ValidationResult ValidateValue(object? proposedValue)
    {
        if (proposedValue == null)
            return ValidationResult.Success(false); // Default to false if null
        
        // Handle various input types that can represent boolean values
        if (proposedValue is bool boolValue)
            return ValidationResult.Success(boolValue);
        
        if (proposedValue is string stringValue)
        {
            if (bool.TryParse(stringValue, out var parsedBool))
                return ValidationResult.Success(parsedBool);
            
            // Handle common string representations
            var lowerString = stringValue.ToLowerInvariant();
            switch (lowerString)
            {
                case "on":
                case "true":
                case "yes":
                case "1":
                case "enabled":
                case "active":
                    return ValidationResult.Success(true);
                
                case "off":
                case "false":
                case "no":
                case "0":
                case "disabled":
                case "inactive":
                    return ValidationResult.Success(false);
                
                default:
                    return ValidationResult.Error($"Cannot convert '{stringValue}' to boolean");
            }
        }
        
        if (proposedValue is int intValue)
            return ValidationResult.Success(intValue != 0);
        
        if (proposedValue is double doubleValue)
            return ValidationResult.Success(doubleValue != 0.0);
        
        return ValidationResult.Error($"Cannot convert '{proposedValue}' to boolean");
    }
    
    protected override object? GetDefaultValue() => _defaultValue;
    
    /// <summary>
    /// Get the current value as a boolean
    /// </summary>
    public bool BooleanValue => Convert.ToBoolean(Value ?? _defaultValue);
    
    /// <summary>
    /// Toggle the current state (true becomes false, false becomes true)
    /// </summary>
    public void Toggle()
    {
        Value = !BooleanValue;
    }
    
    /// <summary>
    /// Set the toggle to the ON state
    /// </summary>
    public void TurnOn()
    {
        Value = true;
    }
    
    /// <summary>
    /// Set the toggle to the OFF state
    /// </summary>
    public void TurnOff()
    {
        Value = false;
    }
    
    /// <summary>
    /// Get a human-readable representation of the current state
    /// </summary>
    public string StateText => BooleanValue ? "ON" : "OFF";
    
    /// <summary>
    /// Get an emoji representation of the current state
    /// </summary>
    public string StateEmoji => BooleanValue ? "🟢" : "🔴";
    
    public override string ToString()
    {
        return $"Toggle '{Name}' (ID: {Id}): {StateText} {StateEmoji}";
    }
}