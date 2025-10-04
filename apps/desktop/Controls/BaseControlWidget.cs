using System;
using System.Threading.Tasks;

namespace MultiControllerApp.Controls;

/// <summary>
/// Base implementation of IControlWidget providing common functionality
/// </summary>
public abstract class BaseControlWidget : IControlWidget
{
    private object? _value;
    private bool _isEnabled = true;
    
    protected BaseControlWidget(string id, string name, ControlWidgetType type)
    {
        Id = id ?? throw new ArgumentNullException(nameof(id));
        Name = name ?? throw new ArgumentNullException(nameof(name));
        Type = type;
    }
    
    public string Id { get; }
    public string Name { get; }
    public ControlWidgetType Type { get; }
    
    public virtual object? Value
    {
        get => _value;
        set
        {
            var oldValue = _value;
            var validationResult = ValidateValue(value);
            
            if (validationResult.IsValid)
            {
                _value = validationResult.ValidatedValue;
                OnValueChanged(oldValue, _value);
            }
            else
            {
                throw new ArgumentException(validationResult.ErrorMessage ?? "Invalid value");
            }
        }
    }
    
    public virtual bool IsEnabled
    {
        get => _isEnabled;
        set
        {
            if (_isEnabled != value)
            {
                _isEnabled = value;
                OnStateChanged(value);
            }
        }
    }
    
    public abstract double? MinValue { get; }
    public abstract double? MaxValue { get; }
    public abstract double? StepValue { get; }
    
    public event EventHandler<ControlValueChangedEventArgs>? ValueChanged;
    public event EventHandler<ControlStateChangedEventArgs>? StateChanged;
    
    public abstract ValidationResult ValidateValue(object? proposedValue);
    
    public virtual async Task ResetAsync()
    {
        await Task.CompletedTask;
        Value = GetDefaultValue();
    }
    
    public virtual async Task UpdateConfigurationAsync(WidgetConfiguration config)
    {
        await Task.CompletedTask;
        
        IsEnabled = config.IsEnabled;
        
        if (config.DefaultValue != null)
        {
            Value = config.DefaultValue;
        }
    }
    
    protected virtual object? GetDefaultValue() => null;
    
    protected virtual void OnValueChanged(object? oldValue, object? newValue)
    {
        ValueChanged?.Invoke(this, new ControlValueChangedEventArgs(oldValue, newValue));
    }
    
    protected virtual void OnStateChanged(bool isEnabled)
    {
        StateChanged?.Invoke(this, new ControlStateChangedEventArgs(isEnabled));
    }
    
    /// <summary>
    /// Clamp a numeric value to the widget's min/max bounds
    /// </summary>
    protected double ClampValue(double value)
    {
        if (MinValue.HasValue && value < MinValue.Value)
            return MinValue.Value;
        
        if (MaxValue.HasValue && value > MaxValue.Value)
            return MaxValue.Value;
        
        return value;
    }
    
    /// <summary>
    /// Snap a value to the nearest step increment
    /// </summary>
    protected double SnapToStep(double value)
    {
        if (!StepValue.HasValue || StepValue.Value == 0)
            return value;
        
        var steps = Math.Round(value / StepValue.Value);
        return steps * StepValue.Value;
    }
}