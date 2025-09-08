using System;
using System.Globalization;

namespace MultiControllerApp.Controls;

/// <summary>
/// Numeric input control widget for precise numeric value entry
/// </summary>
public class NumericInputWidget : BaseControlWidget
{
    private readonly double _minValue;
    private readonly double _maxValue;
    private readonly double _stepValue;
    private readonly double _defaultValue;
    private readonly int _decimalPlaces;
    
    public NumericInputWidget(string id, string name, double minValue = double.MinValue, double maxValue = double.MaxValue, 
        double stepValue = 0.1, double defaultValue = 0.0, int decimalPlaces = 2)
        : base(id, name, ControlWidgetType.NumericInput)
    {
        if (minValue >= maxValue)
            throw new ArgumentException("MinValue must be less than MaxValue");
        
        if (stepValue <= 0)
            throw new ArgumentException("StepValue must be positive");
        
        if (defaultValue < minValue || defaultValue > maxValue)
            throw new ArgumentException("DefaultValue must be between MinValue and MaxValue");
        
        if (decimalPlaces < 0)
            throw new ArgumentException("DecimalPlaces must be non-negative");
        
        _minValue = minValue;
        _maxValue = maxValue;
        _stepValue = stepValue;
        _defaultValue = defaultValue;
        _decimalPlaces = decimalPlaces;
        
        // Set initial value
        Value = _defaultValue;
    }
    
    public override double? MinValue => _minValue;
    public override double? MaxValue => _maxValue;
    public override double? StepValue => _stepValue;
    
    /// <summary>
    /// Number of decimal places to display/validate
    /// </summary>
    public int DecimalPlaces => _decimalPlaces;
    
    public override ValidationResult ValidateValue(object? proposedValue)
    {
        if (proposedValue == null)
            return ValidationResult.Error("Numeric input value cannot be null");
        
        double numericValue;
        
        if (proposedValue is double doubleVal)
        {
            numericValue = doubleVal;
        }
        else if (proposedValue is int intVal)
        {
            numericValue = intVal;
        }
        else if (proposedValue is float floatVal)
        {
            numericValue = floatVal;
        }
        else if (proposedValue is decimal decimalVal)
        {
            numericValue = (double)decimalVal;
        }
        else if (proposedValue is string stringVal)
        {
            if (!double.TryParse(stringVal, NumberStyles.Float, CultureInfo.InvariantCulture, out numericValue))
                return ValidationResult.Error($"Cannot parse '{stringVal}' as a number");
        }
        else
        {
            return ValidationResult.Error($"Cannot convert '{proposedValue}' to a number");
        }
        
        // Check for special values
        if (double.IsNaN(numericValue))
            return ValidationResult.Error("Value cannot be NaN");
        
        if (double.IsInfinity(numericValue))
            return ValidationResult.Error("Value cannot be infinity");
        
        // Clamp to bounds
        var clampedValue = ClampValue(numericValue);
        
        // Round to specified decimal places
        var roundedValue = Math.Round(clampedValue, _decimalPlaces);
        
        return ValidationResult.Success(roundedValue);
    }
    
    protected override object? GetDefaultValue() => _defaultValue;
    
    /// <summary>
    /// Get the current value as a double
    /// </summary>
    public double DoubleValue => Convert.ToDouble(Value ?? _defaultValue);
    
    /// <summary>
    /// Get the current value formatted to the specified decimal places
    /// </summary>
    public string FormattedValue => DoubleValue.ToString($"F{_decimalPlaces}", CultureInfo.InvariantCulture);
    
    /// <summary>
    /// Increment the value by one step
    /// </summary>
    public void Increment()
    {
        var newValue = DoubleValue + _stepValue;
        if (newValue <= _maxValue)
        {
            Value = newValue;
        }
    }
    
    /// <summary>
    /// Decrement the value by one step
    /// </summary>
    public void Decrement()
    {
        var newValue = DoubleValue - _stepValue;
        if (newValue >= _minValue)
        {
            Value = newValue;
        }
    }
    
    /// <summary>
    /// Set value from string input with validation
    /// </summary>
    /// <param name="input">String representation of the number</param>
    /// <returns>True if the value was set successfully</returns>
    public bool SetFromString(string input)
    {
        var validationResult = ValidateValue(input);
        if (validationResult.IsValid)
        {
            Value = validationResult.ValidatedValue;
            return true;
        }
        return false;
    }
    
    /// <summary>
    /// Check if the current value is at the minimum bound
    /// </summary>
    public bool IsAtMinimum => Math.Abs(DoubleValue - _minValue) < double.Epsilon;
    
    /// <summary>
    /// Check if the current value is at the maximum bound
    /// </summary>
    public bool IsAtMaximum => Math.Abs(DoubleValue - _maxValue) < double.Epsilon;
    
    public override string ToString()
    {
        var range = _minValue == double.MinValue || _maxValue == double.MaxValue 
            ? "unbounded" 
            : $"[{_minValue.ToString($"F{_decimalPlaces}")}-{_maxValue.ToString($"F{_decimalPlaces}")}]";
        
        return $"NumericInput '{Name}' (ID: {Id}): {FormattedValue} {range} (Step: {_stepValue.ToString($"F{_decimalPlaces}")})";
    }
}