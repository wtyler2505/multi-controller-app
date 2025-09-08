using System;

namespace MultiControllerApp.Controls;

/// <summary>
/// Slider control widget for continuous numeric value input
/// </summary>
public class SliderWidget : BaseControlWidget
{
    private readonly double _minValue;
    private readonly double _maxValue;
    private readonly double _stepValue;
    private readonly double _defaultValue;
    
    public SliderWidget(string id, string name, double minValue = 0.0, double maxValue = 100.0, double stepValue = 1.0, double defaultValue = 0.0)
        : base(id, name, ControlWidgetType.Slider)
    {
        if (minValue >= maxValue)
            throw new ArgumentException("MinValue must be less than MaxValue");
        
        if (stepValue <= 0)
            throw new ArgumentException("StepValue must be positive");
        
        if (defaultValue < minValue || defaultValue > maxValue)
            throw new ArgumentException("DefaultValue must be between MinValue and MaxValue");
        
        _minValue = minValue;
        _maxValue = maxValue;
        _stepValue = stepValue;
        _defaultValue = defaultValue;
        
        // Set initial value
        Value = _defaultValue;
    }
    
    public override double? MinValue => _minValue;
    public override double? MaxValue => _maxValue;
    public override double? StepValue => _stepValue;
    
    public override ValidationResult ValidateValue(object? proposedValue)
    {
        if (proposedValue == null)
            return ValidationResult.Error("Slider value cannot be null");
        
        if (!double.TryParse(proposedValue.ToString(), out var numericValue))
            return ValidationResult.Error("Slider value must be numeric");
        
        // Clamp to bounds and snap to step
        var clampedValue = ClampValue(numericValue);
        var snappedValue = SnapToStep(clampedValue);
        
        return ValidationResult.Success(snappedValue);
    }
    
    protected override object? GetDefaultValue() => _defaultValue;
    
    /// <summary>
    /// Get the current value as a double
    /// </summary>
    public double DoubleValue => Convert.ToDouble(Value ?? _defaultValue);
    
    /// <summary>
    /// Get the current value as a percentage (0-100) of the slider range
    /// </summary>
    public double PercentageValue
    {
        get
        {
            var range = _maxValue - _minValue;
            if (range == 0) return 0;
            
            var normalizedValue = (DoubleValue - _minValue) / range;
            return normalizedValue * 100.0;
        }
    }
    
    /// <summary>
    /// Set value from percentage (0-100)
    /// </summary>
    /// <param name="percentage">Percentage value (0-100)</param>
    public void SetFromPercentage(double percentage)
    {
        if (percentage < 0) percentage = 0;
        if (percentage > 100) percentage = 100;
        
        var range = _maxValue - _minValue;
        var actualValue = _minValue + (percentage / 100.0 * range);
        Value = actualValue;
    }
    
    /// <summary>
    /// Increment the slider value by one step
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
    /// Decrement the slider value by one step
    /// </summary>
    public void Decrement()
    {
        var newValue = DoubleValue - _stepValue;
        if (newValue >= _minValue)
        {
            Value = newValue;
        }
    }
    
    public override string ToString()
    {
        return $"Slider '{Name}' (ID: {Id}): {DoubleValue:F2} [{_minValue}-{_maxValue}] (Step: {_stepValue})";
    }
}