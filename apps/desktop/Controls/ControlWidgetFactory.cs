using System;
using System.Collections.Generic;

namespace MultiControllerApp.Controls;

/// <summary>
/// Interface for creating control widgets
/// </summary>
public interface IControlWidgetFactory
{
    /// <summary>
    /// Create a slider widget
    /// </summary>
    IControlWidget CreateSlider(string id, string name, double minValue = 0.0, double maxValue = 100.0, double stepValue = 1.0, double defaultValue = 0.0);
    
    /// <summary>
    /// Create a toggle widget
    /// </summary>
    IControlWidget CreateToggle(string id, string name, bool defaultValue = false);
    
    /// <summary>
    /// Create a numeric input widget
    /// </summary>
    IControlWidget CreateNumericInput(string id, string name, double minValue = double.MinValue, double maxValue = double.MaxValue, 
        double stepValue = 0.1, double defaultValue = 0.0, int decimalPlaces = 2);
    
    /// <summary>
    /// Create a dropdown widget
    /// </summary>
    IControlWidget CreateDropdown(string id, string name, IEnumerable<string> options, string? defaultValue = null);
    
    /// <summary>
    /// Create an emergency stop widget
    /// </summary>
    IControlWidget CreateEmergencyStop(string id, string name = "Emergency Stop", EmergencyStopConfiguration? config = null);
    
    /// <summary>
    /// Create a widget from configuration
    /// </summary>
    IControlWidget CreateFromConfiguration(WidgetCreationConfiguration config);
    
    /// <summary>
    /// Get a list of all supported widget types
    /// </summary>
    IEnumerable<ControlWidgetType> GetSupportedTypes();
}

/// <summary>
/// Factory implementation for creating control widgets
/// </summary>
public class ControlWidgetFactory : IControlWidgetFactory
{
    public IControlWidget CreateSlider(string id, string name, double minValue = 0.0, double maxValue = 100.0, double stepValue = 1.0, double defaultValue = 0.0)
    {
        return new SliderWidget(id, name, minValue, maxValue, stepValue, defaultValue);
    }
    
    public IControlWidget CreateToggle(string id, string name, bool defaultValue = false)
    {
        return new ToggleWidget(id, name, defaultValue);
    }
    
    public IControlWidget CreateNumericInput(string id, string name, double minValue = double.MinValue, double maxValue = double.MaxValue, 
        double stepValue = 0.1, double defaultValue = 0.0, int decimalPlaces = 2)
    {
        return new NumericInputWidget(id, name, minValue, maxValue, stepValue, defaultValue, decimalPlaces);
    }
    
    public IControlWidget CreateDropdown(string id, string name, IEnumerable<string> options, string? defaultValue = null)
    {
        return new DropdownWidget(id, name, options, defaultValue);
    }
    
    public IControlWidget CreateEmergencyStop(string id, string name = "Emergency Stop", EmergencyStopConfiguration? config = null)
    {
        return new EmergencyStopWidget(id, name, config);
    }
    
    public IControlWidget CreateFromConfiguration(WidgetCreationConfiguration config)
    {
        return config.Type switch
        {
            ControlWidgetType.Slider => CreateSlider(
                config.Id, 
                config.Name, 
                config.MinValue ?? 0.0, 
                config.MaxValue ?? 100.0, 
                config.StepValue ?? 1.0, 
                config.DefaultValue as double? ?? 0.0),
                
            ControlWidgetType.Toggle => CreateToggle(
                config.Id, 
                config.Name, 
                config.DefaultValue as bool? ?? false),
                
            ControlWidgetType.NumericInput => CreateNumericInput(
                config.Id, 
                config.Name, 
                config.MinValue ?? double.MinValue, 
                config.MaxValue ?? double.MaxValue, 
                config.StepValue ?? 0.1, 
                config.DefaultValue as double? ?? 0.0, 
                config.DecimalPlaces ?? 2),
                
            ControlWidgetType.Dropdown => CreateDropdown(
                config.Id, 
                config.Name, 
                config.Options ?? new[] { "Option 1", "Option 2" }, 
                config.DefaultValue as string),
                
            ControlWidgetType.EmergencyStop => CreateEmergencyStop(
                config.Id, 
                config.Name, 
                config.EmergencyStopConfig),
                
            _ => throw new ArgumentException($"Unsupported widget type: {config.Type}")
        };
    }
    
    public IEnumerable<ControlWidgetType> GetSupportedTypes()
    {
        return new[]
        {
            ControlWidgetType.Slider,
            ControlWidgetType.Toggle,
            ControlWidgetType.NumericInput,
            ControlWidgetType.Dropdown,
            ControlWidgetType.EmergencyStop
        };
    }
}

/// <summary>
/// Configuration for creating widgets
/// </summary>
public class WidgetCreationConfiguration
{
    public required string Id { get; set; }
    public required string Name { get; set; }
    public required ControlWidgetType Type { get; set; }
    public double? MinValue { get; set; }
    public double? MaxValue { get; set; }
    public double? StepValue { get; set; }
    public object? DefaultValue { get; set; }
    public int? DecimalPlaces { get; set; }
    public string[]? Options { get; set; }
    public EmergencyStopConfiguration? EmergencyStopConfig { get; set; }
}

/// <summary>
/// Builder pattern for easy widget creation
/// </summary>
public class WidgetBuilder
{
    private readonly IControlWidgetFactory _factory;
    
    public WidgetBuilder(IControlWidgetFactory factory)
    {
        _factory = factory ?? throw new ArgumentNullException(nameof(factory));
    }
    
    /// <summary>
    /// Start building a slider widget
    /// </summary>
    public SliderBuilder Slider(string id, string name) => new(_factory, id, name);
    
    /// <summary>
    /// Start building a toggle widget
    /// </summary>
    public ToggleBuilder Toggle(string id, string name) => new(_factory, id, name);
    
    /// <summary>
    /// Start building a numeric input widget
    /// </summary>
    public NumericInputBuilder NumericInput(string id, string name) => new(_factory, id, name);
    
    /// <summary>
    /// Start building a dropdown widget
    /// </summary>
    public DropdownBuilder Dropdown(string id, string name) => new(_factory, id, name);
    
    /// <summary>
    /// Start building an emergency stop widget
    /// </summary>
    public EmergencyStopBuilder EmergencyStop(string id, string name = "Emergency Stop") => new(_factory, id, name);
}

/// <summary>
/// Builder for slider widgets
/// </summary>
public class SliderBuilder
{
    private readonly IControlWidgetFactory _factory;
    private readonly string _id;
    private readonly string _name;
    private double _minValue = 0.0;
    private double _maxValue = 100.0;
    private double _stepValue = 1.0;
    private double _defaultValue = 0.0;
    
    internal SliderBuilder(IControlWidgetFactory factory, string id, string name)
    {
        _factory = factory;
        _id = id;
        _name = name;
    }
    
    public SliderBuilder WithRange(double min, double max)
    {
        _minValue = min;
        _maxValue = max;
        return this;
    }
    
    public SliderBuilder WithStep(double step)
    {
        _stepValue = step;
        return this;
    }
    
    public SliderBuilder WithDefault(double defaultValue)
    {
        _defaultValue = defaultValue;
        return this;
    }
    
    public IControlWidget Build() => _factory.CreateSlider(_id, _name, _minValue, _maxValue, _stepValue, _defaultValue);
}

/// <summary>
/// Builder for toggle widgets
/// </summary>
public class ToggleBuilder
{
    private readonly IControlWidgetFactory _factory;
    private readonly string _id;
    private readonly string _name;
    private bool _defaultValue = false;
    
    internal ToggleBuilder(IControlWidgetFactory factory, string id, string name)
    {
        _factory = factory;
        _id = id;
        _name = name;
    }
    
    public ToggleBuilder WithDefault(bool defaultValue)
    {
        _defaultValue = defaultValue;
        return this;
    }
    
    public IControlWidget Build() => _factory.CreateToggle(_id, _name, _defaultValue);
}

/// <summary>
/// Builder for numeric input widgets
/// </summary>
public class NumericInputBuilder
{
    private readonly IControlWidgetFactory _factory;
    private readonly string _id;
    private readonly string _name;
    private double _minValue = double.MinValue;
    private double _maxValue = double.MaxValue;
    private double _stepValue = 0.1;
    private double _defaultValue = 0.0;
    private int _decimalPlaces = 2;
    
    internal NumericInputBuilder(IControlWidgetFactory factory, string id, string name)
    {
        _factory = factory;
        _id = id;
        _name = name;
    }
    
    public NumericInputBuilder WithRange(double min, double max)
    {
        _minValue = min;
        _maxValue = max;
        return this;
    }
    
    public NumericInputBuilder WithStep(double step)
    {
        _stepValue = step;
        return this;
    }
    
    public NumericInputBuilder WithDefault(double defaultValue)
    {
        _defaultValue = defaultValue;
        return this;
    }
    
    public NumericInputBuilder WithDecimalPlaces(int decimalPlaces)
    {
        _decimalPlaces = decimalPlaces;
        return this;
    }
    
    public IControlWidget Build() => _factory.CreateNumericInput(_id, _name, _minValue, _maxValue, _stepValue, _defaultValue, _decimalPlaces);
}

/// <summary>
/// Builder for dropdown widgets
/// </summary>
public class DropdownBuilder
{
    private readonly IControlWidgetFactory _factory;
    private readonly string _id;
    private readonly string _name;
    private List<string> _options = new();
    private string? _defaultValue;
    
    internal DropdownBuilder(IControlWidgetFactory factory, string id, string name)
    {
        _factory = factory;
        _id = id;
        _name = name;
    }
    
    public DropdownBuilder WithOptions(params string[] options)
    {
        _options.AddRange(options);
        return this;
    }
    
    public DropdownBuilder WithOptions(IEnumerable<string> options)
    {
        _options.AddRange(options);
        return this;
    }
    
    public DropdownBuilder WithDefault(string defaultValue)
    {
        _defaultValue = defaultValue;
        return this;
    }
    
    public IControlWidget Build() => _factory.CreateDropdown(_id, _name, _options, _defaultValue);
}

/// <summary>
/// Builder for emergency stop widgets
/// </summary>
public class EmergencyStopBuilder
{
    private readonly IControlWidgetFactory _factory;
    private readonly string _id;
    private readonly string _name;
    private EmergencyStopConfiguration? _config;
    
    internal EmergencyStopBuilder(IControlWidgetFactory factory, string id, string name)
    {
        _factory = factory;
        _id = id;
        _name = name;
    }
    
    public EmergencyStopBuilder WithConfiguration(EmergencyStopConfiguration config)
    {
        _config = config;
        return this;
    }
    
    public EmergencyStopBuilder RequireConfirmation(string confirmationCode = "RESET")
    {
        _config ??= new EmergencyStopConfiguration();
        _config.RequireConfirmationToReset = true;
        _config.ResetConfirmationCode = confirmationCode;
        return this;
    }
    
    public EmergencyStopBuilder WithMinimumDuration(TimeSpan duration)
    {
        _config ??= new EmergencyStopConfiguration();
        _config.MinimumActiveDuration = duration;
        return this;
    }
    
    public IControlWidget Build() => _factory.CreateEmergencyStop(_id, _name, _config);
}