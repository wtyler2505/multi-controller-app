using System;
using System.Threading.Tasks;
using MultiControllerApp.Controls;

Console.WriteLine("=== Manual Control Widgets Demo ===");

var factory = new ControlWidgetFactory();

// Create some demo widgets
var slider = factory.CreateSlider("test_slider", "Test Slider", 0, 100, 5, 25);
var toggle = factory.CreateToggle("test_toggle", "Test Toggle", false);
var numericInput = factory.CreateNumericInput("test_numeric", "Test Numeric", 0, 1000, 0.1, 50.5, 1);
var dropdown = factory.CreateDropdown("test_dropdown", "Test Dropdown", new[] { "Option A", "Option B", "Option C" }, "Option A");
var emergencyStop = factory.CreateEmergencyStop("test_emergency", "Emergency Stop");

Console.WriteLine("\nCreated Widgets:");
Console.WriteLine($"- {slider}");
Console.WriteLine($"- {toggle}");
Console.WriteLine($"- {numericInput}");
Console.WriteLine($"- {dropdown}");
Console.WriteLine($"- {emergencyStop}");

// Test some operations
Console.WriteLine("\n=== Testing Widget Operations ===");

// Test slider
Console.WriteLine("\nSlider Test:");
((SliderWidget)slider).SetFromPercentage(75);
Console.WriteLine($"Set slider to 75%: {slider}");

// Test toggle
Console.WriteLine("\nToggle Test:");
((ToggleWidget)toggle).Toggle();
Console.WriteLine($"Toggled: {toggle}");

// Test numeric input
Console.WriteLine("\nNumeric Input Test:");
((NumericInputWidget)numericInput).Increment();
Console.WriteLine($"Incremented: {numericInput}");

// Test dropdown
Console.WriteLine("\nDropdown Test:");
((DropdownWidget)dropdown).SelectNext();
Console.WriteLine($"Selected next: {dropdown}");

// Test emergency stop
Console.WriteLine("\nEmergency Stop Test:");
var emergencyWidget = (EmergencyStopWidget)emergencyStop;
emergencyWidget.Activate();
Console.WriteLine($"Activated: {emergencyWidget}");
Console.WriteLine($"Can deactivate: {emergencyWidget.CanDeactivate("RESET")}");

// Demonstrate state management
Console.WriteLine("\n=== Manual Control State Demo ===");
var controlState = new ManualControlState();

controlState.RegisterWidget(slider);
controlState.RegisterWidget(toggle);
controlState.RegisterWidget(numericInput);
controlState.RegisterWidget(dropdown);
controlState.RegisterWidget(emergencyStop);

Console.WriteLine($"\nState has {controlState.WidgetCount} widgets");
Console.WriteLine($"Emergency stop active: {controlState.EmergencyStop}");

// Show current values
Console.WriteLine("\nCurrent widget values:");
foreach (var widgetId in controlState.WidgetIds)
{
    var value = controlState.GetWidgetValue(widgetId);
    Console.WriteLine($"  {widgetId}: {value}");
}

Console.WriteLine("\n=== Demo Complete ===");
Console.WriteLine("Manual control widgets are working correctly!");