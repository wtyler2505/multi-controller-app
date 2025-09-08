using System;
using System.Linq;
using System.Threading.Tasks;
using MultiControllerApp.Controls;
using Xunit;

namespace MultiControllerApp.Tests.Controls;

/// <summary>
/// Unit tests for manual control widgets
/// </summary>
public class ManualControlWidgetTests
{
    [Fact]
    public void SliderWidget_CreateAndValidate()
    {
        // Arrange & Act
        var slider = new SliderWidget("test_slider", "Test Slider", 0, 100, 5, 25);
        
        // Assert
        Assert.Equal("test_slider", slider.Id);
        Assert.Equal("Test Slider", slider.Name);
        Assert.Equal(ControlWidgetType.Slider, slider.Type);
        Assert.Equal(25.0, slider.DoubleValue);
        Assert.Equal(0, slider.MinValue);
        Assert.Equal(100, slider.MaxValue);
        Assert.Equal(5, slider.StepValue);
    }
    
    [Fact]
    public void SliderWidget_SetFromPercentage()
    {
        // Arrange
        var slider = new SliderWidget("test", "Test", 0, 100, 1, 0);
        
        // Act
        slider.SetFromPercentage(75);
        
        // Assert
        Assert.Equal(75.0, slider.DoubleValue);
        Assert.Equal(75.0, slider.PercentageValue);
    }
    
    [Fact]
    public void SliderWidget_ValueValidation()
    {
        // Arrange
        var slider = new SliderWidget("test", "Test", 0, 100, 5, 0);
        
        // Act & Assert - Valid value
        var result1 = slider.ValidateValue(50);
        Assert.True(result1.IsValid);
        Assert.Equal(50.0, result1.ValidatedValue);
        
        // Act & Assert - Value out of range gets clamped
        var result2 = slider.ValidateValue(150);
        Assert.True(result2.IsValid);
        Assert.Equal(100.0, result2.ValidatedValue);
        
        // Act & Assert - Invalid type
        var result3 = slider.ValidateValue("invalid");
        Assert.False(result3.IsValid);
    }
    
    [Fact]
    public void ToggleWidget_CreateAndToggle()
    {
        // Arrange
        var toggle = new ToggleWidget("test_toggle", "Test Toggle", false);
        
        // Assert initial state
        Assert.False(toggle.BooleanValue);
        Assert.Equal("OFF", toggle.StateText);
        
        // Act
        toggle.Toggle();
        
        // Assert
        Assert.True(toggle.BooleanValue);
        Assert.Equal("ON", toggle.StateText);
    }
    
    [Fact]
    public void ToggleWidget_ValueValidation()
    {
        // Arrange
        var toggle = new ToggleWidget("test", "Test");
        
        // Act & Assert - Boolean value
        var result1 = toggle.ValidateValue(true);
        Assert.True(result1.IsValid);
        Assert.Equal(true, result1.ValidatedValue);
        
        // Act & Assert - String value
        var result2 = toggle.ValidateValue("on");
        Assert.True(result2.IsValid);
        Assert.Equal(true, result2.ValidatedValue);
        
        // Act & Assert - Numeric value
        var result3 = toggle.ValidateValue(0);
        Assert.True(result3.IsValid);
        Assert.Equal(false, result3.ValidatedValue);
    }
    
    [Fact]
    public void NumericInputWidget_IncrementDecrement()
    {
        // Arrange
        var numeric = new NumericInputWidget("test", "Test", 0, 100, 5, 25, 0);
        
        // Act
        numeric.Increment();
        
        // Assert
        Assert.Equal(30.0, numeric.DoubleValue);
        
        // Act
        numeric.Decrement();
        numeric.Decrement();
        
        // Assert
        Assert.Equal(20.0, numeric.DoubleValue);
    }
    
    [Fact]
    public void DropdownWidget_SelectOptions()
    {
        // Arrange
        var options = new[] { "Option A", "Option B", "Option C" };
        var dropdown = new DropdownWidget("test", "Test", options, "Option A");
        
        // Assert initial state
        Assert.Equal("Option A", dropdown.StringValue);
        Assert.Equal(0, dropdown.SelectedIndex);
        
        // Act
        dropdown.SelectNext();
        
        // Assert
        Assert.Equal("Option B", dropdown.StringValue);
        Assert.Equal(1, dropdown.SelectedIndex);
        
        // Act
        dropdown.SetSelectedIndex(2);
        
        // Assert
        Assert.Equal("Option C", dropdown.StringValue);
        Assert.Equal(2, dropdown.SelectedIndex);
    }
    
    [Fact]
    public void EmergencyStopWidget_ActivateDeactivate()
    {
        // Arrange
        var config = new EmergencyStopConfiguration
        {
            RequireConfirmationToReset = true,
            ResetConfirmationCode = "RESET",
            MinimumActiveDuration = TimeSpan.FromMilliseconds(100)
        };
        var emergencyStop = new EmergencyStopWidget("test", "Test", config);
        
        // Assert initial state
        Assert.False(emergencyStop.IsActivated);
        
        // Act
        emergencyStop.Activate();
        
        // Assert
        Assert.True(emergencyStop.IsActivated);
        Assert.NotNull(emergencyStop.ActivatedTime);
        
        // Act - Try to deactivate without confirmation
        var result1 = emergencyStop.Deactivate();
        
        // Assert - Should fail without confirmation
        Assert.False(result1);
        Assert.True(emergencyStop.IsActivated);
        
        // Wait for minimum duration
        Task.Delay(150).Wait();
        
        // Act - Deactivate with confirmation
        var result2 = emergencyStop.Deactivate("RESET");
        
        // Assert
        Assert.True(result2);
        Assert.False(emergencyStop.IsActivated);
    }
    
    [Fact]
    public void ManualControlState_RegisterAndManageWidgets()
    {
        // Arrange
        var state = new ManualControlState();
        var slider = new SliderWidget("slider", "Slider", 0, 100, 1, 50);
        var toggle = new ToggleWidget("toggle", "Toggle", false);
        
        // Act
        state.RegisterWidget(slider);
        state.RegisterWidget(toggle);
        
        // Assert
        Assert.Equal(2, state.WidgetCount);
        Assert.Contains("slider", state.WidgetIds);
        Assert.Contains("toggle", state.WidgetIds);
        
        // Act
        var success = state.SetWidgetValue("slider", 75.0);
        
        // Assert
        Assert.True(success);
        Assert.Equal(75.0, state.GetWidgetValue("slider"));
    }
    
    [Fact]
    public void ManualControlState_EmergencyStopDisablesControls()
    {
        // Arrange
        var state = new ManualControlState();
        var slider = new SliderWidget("slider", "Slider", 0, 100, 1, 50);
        var emergencyStop = new EmergencyStopWidget("emergency", "Emergency Stop");
        
        state.RegisterWidget(slider);
        state.RegisterWidget(emergencyStop);
        
        // Act
        state.ActivateEmergencyStop();
        
        // Assert
        Assert.True(state.EmergencyStop);
        Assert.False(slider.IsEnabled);
        
        // Act - Try to set value during emergency stop
        var success = state.SetWidgetValue("slider", 75.0);
        
        // Assert - Should fail
        Assert.False(success);
        
        // Act
        state.DeactivateEmergencyStop();
        
        // Assert
        Assert.False(state.EmergencyStop);
        Assert.True(slider.IsEnabled);
    }
    
    [Fact]
    public void ControlWidgetFactory_CreateAllWidgetTypes()
    {
        // Arrange
        var factory = new ControlWidgetFactory();
        
        // Act & Assert
        var slider = factory.CreateSlider("slider", "Slider");
        Assert.Equal(ControlWidgetType.Slider, slider.Type);
        
        var toggle = factory.CreateToggle("toggle", "Toggle");
        Assert.Equal(ControlWidgetType.Toggle, toggle.Type);
        
        var numeric = factory.CreateNumericInput("numeric", "Numeric");
        Assert.Equal(ControlWidgetType.NumericInput, numeric.Type);
        
        var dropdown = factory.CreateDropdown("dropdown", "Dropdown", new[] { "A", "B" });
        Assert.Equal(ControlWidgetType.Dropdown, dropdown.Type);
        
        var emergency = factory.CreateEmergencyStop("emergency", "Emergency");
        Assert.Equal(ControlWidgetType.EmergencyStop, emergency.Type);
    }
    
    [Fact]
    public void WidgetBuilder_FluentAPI()
    {
        // Arrange
        var factory = new ControlWidgetFactory();
        var builder = new WidgetBuilder(factory);
        
        // Act
        var slider = builder.Slider("test", "Test")
            .WithRange(0, 50)
            .WithStep(2.5)
            .WithDefault(25)
            .Build();
        
        // Assert
        Assert.Equal(0, slider.MinValue);
        Assert.Equal(50, slider.MaxValue);
        Assert.Equal(2.5, slider.StepValue);
        Assert.Equal(25.0, ((SliderWidget)slider).DoubleValue);
    }
    
    [Fact]
    public void ManualControlState_ValueChangeEvents()
    {
        // Arrange
        var state = new ManualControlState();
        var slider = new SliderWidget("slider", "Slider", 0, 100, 1, 50);
        state.RegisterWidget(slider);
        
        bool eventFired = false;
        string? changedWidgetId = null;
        object? newValue = null;
        
        state.WidgetValueChanged += (sender, e) =>
        {
            eventFired = true;
            changedWidgetId = e.WidgetId;
            newValue = e.NewValue;
        };
        
        // Act
        slider.Value = 75.0;
        
        // Assert
        Assert.True(eventFired);
        Assert.Equal("slider", changedWidgetId);
        Assert.Equal(75.0, newValue);
    }
}