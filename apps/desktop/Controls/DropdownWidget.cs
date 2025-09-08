using System;
using System.Collections.Generic;
using System.Linq;

namespace MultiControllerApp.Controls;

/// <summary>
/// Dropdown control widget for selecting from a predefined list of options
/// </summary>
public class DropdownWidget : BaseControlWidget
{
    private readonly List<string> _options;
    private readonly string _defaultValue;
    
    public DropdownWidget(string id, string name, IEnumerable<string> options, string? defaultValue = null)
        : base(id, name, ControlWidgetType.Dropdown)
    {
        if (options == null)
            throw new ArgumentNullException(nameof(options));
        
        _options = options.ToList();
        
        if (_options.Count == 0)
            throw new ArgumentException("Dropdown must have at least one option");
        
        // Remove duplicates while preserving order
        _options = _options.Distinct().ToList();
        
        _defaultValue = defaultValue ?? _options[0];
        
        if (!_options.Contains(_defaultValue))
            throw new ArgumentException($"Default value '{_defaultValue}' is not in the options list");
        
        // Set initial value
        Value = _defaultValue;
    }
    
    public override double? MinValue => null; // Not applicable for dropdown
    public override double? MaxValue => null; // Not applicable for dropdown
    public override double? StepValue => null; // Not applicable for dropdown
    
    /// <summary>
    /// Get all available options
    /// </summary>
    public IReadOnlyList<string> Options => _options.AsReadOnly();
    
    /// <summary>
    /// Get the number of available options
    /// </summary>
    public int OptionCount => _options.Count;
    
    public override ValidationResult ValidateValue(object? proposedValue)
    {
        if (proposedValue == null)
            return ValidationResult.Error("Dropdown value cannot be null");
        
        var stringValue = proposedValue.ToString();
        
        if (string.IsNullOrEmpty(stringValue))
            return ValidationResult.Error("Dropdown value cannot be empty");
        
        // Check if the value exists in the options (case-sensitive)
        if (_options.Contains(stringValue))
            return ValidationResult.Success(stringValue);
        
        // Try case-insensitive match
        var caseInsensitiveMatch = _options.FirstOrDefault(option => 
            string.Equals(option, stringValue, StringComparison.OrdinalIgnoreCase));
        
        if (caseInsensitiveMatch != null)
            return ValidationResult.Success(caseInsensitiveMatch);
        
        return ValidationResult.Error($"'{stringValue}' is not a valid option. Available options: {string.Join(", ", _options)}");
    }
    
    protected override object? GetDefaultValue() => _defaultValue;
    
    /// <summary>
    /// Get the current value as a string
    /// </summary>
    public string StringValue => Value?.ToString() ?? _defaultValue;
    
    /// <summary>
    /// Get the index of the currently selected option
    /// </summary>
    public int SelectedIndex => _options.IndexOf(StringValue);
    
    /// <summary>
    /// Set the selected option by index
    /// </summary>
    /// <param name="index">Zero-based index of the option to select</param>
    /// <returns>True if the index was valid and the value was set</returns>
    public bool SetSelectedIndex(int index)
    {
        if (index < 0 || index >= _options.Count)
            return false;
        
        Value = _options[index];
        return true;
    }
    
    /// <summary>
    /// Select the next option in the list (wraps around to first option)
    /// </summary>
    public void SelectNext()
    {
        var currentIndex = SelectedIndex;
        var nextIndex = (currentIndex + 1) % _options.Count;
        SetSelectedIndex(nextIndex);
    }
    
    /// <summary>
    /// Select the previous option in the list (wraps around to last option)
    /// </summary>
    public void SelectPrevious()
    {
        var currentIndex = SelectedIndex;
        var previousIndex = currentIndex == 0 ? _options.Count - 1 : currentIndex - 1;
        SetSelectedIndex(previousIndex);
    }
    
    /// <summary>
    /// Check if a specific option is available
    /// </summary>
    /// <param name="option">Option to check</param>
    /// <param name="ignoreCase">Whether to ignore case when comparing</param>
    /// <returns>True if the option is available</returns>
    public bool HasOption(string option, bool ignoreCase = false)
    {
        if (ignoreCase)
            return _options.Any(o => string.Equals(o, option, StringComparison.OrdinalIgnoreCase));
        
        return _options.Contains(option);
    }
    
    /// <summary>
    /// Add a new option to the dropdown
    /// </summary>
    /// <param name="option">Option to add</param>
    /// <returns>True if the option was added (not already present)</returns>
    public bool AddOption(string option)
    {
        if (string.IsNullOrEmpty(option) || _options.Contains(option))
            return false;
        
        _options.Add(option);
        return true;
    }
    
    /// <summary>
    /// Remove an option from the dropdown
    /// </summary>
    /// <param name="option">Option to remove</param>
    /// <returns>True if the option was removed</returns>
    public bool RemoveOption(string option)
    {
        if (_options.Count <= 1) // Must have at least one option
            return false;
        
        bool removed = _options.Remove(option);
        
        // If the removed option was selected, select the default or first option
        if (removed && StringValue == option)
        {
            Value = _options.Contains(_defaultValue) ? _defaultValue : _options[0];
        }
        
        return removed;
    }
    
    /// <summary>
    /// Clear all options and set new ones
    /// </summary>
    /// <param name="newOptions">New list of options</param>
    /// <param name="newDefaultValue">New default value (optional)</param>
    public void SetOptions(IEnumerable<string> newOptions, string? newDefaultValue = null)
    {
        var optionsList = newOptions?.ToList() ?? throw new ArgumentNullException(nameof(newOptions));
        
        if (optionsList.Count == 0)
            throw new ArgumentException("Must provide at least one option");
        
        // Remove duplicates while preserving order
        optionsList = optionsList.Distinct().ToList();
        
        var defaultValue = newDefaultValue ?? optionsList[0];
        
        if (!optionsList.Contains(defaultValue))
            throw new ArgumentException($"Default value '{defaultValue}' is not in the new options list");
        
        _options.Clear();
        _options.AddRange(optionsList);
        
        Value = defaultValue;
    }
    
    public override string ToString()
    {
        return $"Dropdown '{Name}' (ID: {Id}): '{StringValue}' (Options: {_options.Count})";
    }
}