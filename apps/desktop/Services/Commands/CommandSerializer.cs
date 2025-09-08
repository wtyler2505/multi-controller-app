using System;
using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using System.Security.Cryptography;
using Microsoft.Extensions.Logging;

namespace MultiControllerApp.Services.Commands;

/// <summary>
/// Command serializer that converts commands to wire format for device transmission
/// </summary>
public class CommandSerializer : ICommandSerializer
{
    private readonly ILogger<CommandSerializer> _logger;
    private readonly Dictionary<string, SerializationConfig> _deviceConfigs;
    private readonly JsonSerializerOptions _jsonOptions;

    public CommandSerializer(ILogger<CommandSerializer> logger)
    {
        _logger = logger;
        _deviceConfigs = new Dictionary<string, SerializationConfig>();
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            WriteIndented = false
        };
        
        InitializeDefaultConfigs();
    }

    public async Task<byte[]> SerializeAsync(DeviceCommand command, SerializationConfig config)
    {
        try
        {
            var payload = CreateCommandPayload(command);
            byte[] data;

            switch (config.Format.ToLowerInvariant())
            {
                case "json":
                    data = SerializeToJson(payload, config);
                    break;
                case "binary":
                    data = SerializeToBinary(payload, config);
                    break;
                case "arduino":
                    data = SerializeToArduino(payload, config);
                    break;
                case "custom":
                    data = SerializeCustomFormat(payload, config);
                    break;
                default:
                    throw new NotSupportedException($"Serialization format '{config.Format}' is not supported");
            }

            if (config.IncludeChecksum)
            {
                data = AddChecksum(data, config.ChecksumAlgorithm ?? "crc32");
            }

            _logger.LogDebug("Serialized command {CommandId} ({Type}) to {ByteCount} bytes using {Format} format", 
                command.Id, command.Type, data.Length, config.Format);

            return data;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to serialize command {CommandId} ({Type})", command.Id, command.Type);
            throw;
        }
    }

    public async Task<object?> DeserializeResponseAsync(byte[] data, SerializationConfig config, Type expectedType)
    {
        try
        {
            if (config.IncludeChecksum)
            {
                data = ValidateAndRemoveChecksum(data, config.ChecksumAlgorithm ?? "crc32");
            }

            object? result = config.Format.ToLowerInvariant() switch
            {
                "json" => DeserializeFromJson(data, config, expectedType),
                "binary" => DeserializeFromBinary(data, config, expectedType),
                "arduino" => DeserializeFromArduino(data, config, expectedType),
                "custom" => DeserializeCustomFormat(data, config, expectedType),
                _ => throw new NotSupportedException($"Deserialization format '{config.Format}' is not supported")
            };

            _logger.LogDebug("Deserialized {ByteCount} bytes to {Type} using {Format} format", 
                data.Length, expectedType.Name, config.Format);

            return result;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to deserialize response data ({ByteCount} bytes) using {Format} format", 
                data.Length, config.Format);
            throw;
        }
    }

    public SerializationConfig GetSerializationConfig(string deviceType)
    {
        return _deviceConfigs.TryGetValue(deviceType.ToLowerInvariant(), out var config) 
            ? config 
            : _deviceConfigs["default"];
    }

    public async Task<ValidationResult> ValidateCommandAsync(DeviceCommand command, string deviceType)
    {
        var result = new ValidationResult { IsValid = true };

        try
        {
            // Basic validation
            if (string.IsNullOrWhiteSpace(command.DeviceId))
            {
                result.Errors.Add("DeviceId is required");
                result.IsValid = false;
            }

            if (command.Type == CommandType.Custom && string.IsNullOrWhiteSpace(command.Endpoint))
            {
                result.Errors.Add("Endpoint is required for custom commands");
                result.IsValid = false;
            }

            // Device-specific validation
            await ValidateDeviceSpecificParameters(command, deviceType, result);

            // Parameter validation
            ValidateParameters(command, result);

            // Safety validation
            ValidateSafetyConstraints(command, result);

            _logger.LogDebug("Command validation {Result} for {CommandId} ({Type})", 
                result.IsValid ? "passed" : "failed", command.Id, command.Type);

            return result;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error validating command {CommandId} ({Type})", command.Id, command.Type);
            result.IsValid = false;
            result.Errors.Add($"Validation error: {ex.Message}");
            return result;
        }
    }

    private void InitializeDefaultConfigs()
    {
        // Default JSON configuration
        _deviceConfigs["default"] = new SerializationConfig
        {
            Format = "json",
            Encoding = "utf8",
            IncludeChecksum = false
        };

        // Arduino-specific configuration
        _deviceConfigs["arduino"] = new SerializationConfig
        {
            Format = "arduino",
            Encoding = "ascii",
            IncludeChecksum = true,
            ChecksumAlgorithm = "crc8",
            FormatOptions = new Dictionary<string, object>
            {
                ["commandPrefix"] = "$",
                ["commandSuffix"] = "\r\n",
                ["fieldSeparator"] = ",",
                ["escapeChar"] = "\\"
            }
        };

        // ESP32/ESP8266 configuration
        _deviceConfigs["esp32"] = new SerializationConfig
        {
            Format = "json",
            Encoding = "utf8",
            IncludeChecksum = true,
            ChecksumAlgorithm = "crc32"
        };

        _deviceConfigs["esp8266"] = _deviceConfigs["esp32"];

        // RioRand relay configuration
        _deviceConfigs["riorand"] = new SerializationConfig
        {
            Format = "binary",
            Encoding = "binary",
            IncludeChecksum = true,
            ChecksumAlgorithm = "xor",
            FormatOptions = new Dictionary<string, object>
            {
                ["commandBytes"] = 4,
                ["addressByte"] = 0,
                ["commandByte"] = 1,
                ["dataByte"] = 2,
                ["checksumByte"] = 3
            }
        };

        // Raspberry Pi configuration (SSH/JSON)
        _deviceConfigs["raspberry-pi"] = new SerializationConfig
        {
            Format = "json",
            Encoding = "utf8",
            IncludeChecksum = false,
            FormatOptions = new Dictionary<string, object>
            {
                ["wrapInShell"] = true,
                ["responseFormat"] = "json"
            }
        };
    }

    private object CreateCommandPayload(DeviceCommand command)
    {
        return new
        {
            id = command.Id,
            type = command.Type.ToString(),
            endpoint = command.Endpoint,
            parameters = command.Parameters,
            timestamp = command.CreatedAt.ToString("O"),
            priority = command.Priority.ToString()
        };
    }

    private byte[] SerializeToJson(object payload, SerializationConfig config)
    {
        var json = JsonSerializer.Serialize(payload, _jsonOptions);
        var encoding = GetEncoding(config.Encoding);
        return encoding.GetBytes(json);
    }

    private byte[] SerializeToBinary(object payload, SerializationConfig config)
    {
        // Simple binary format for devices like RioRand relays
        var commandData = JsonSerializer.Deserialize<Dictionary<string, object>>(
            JsonSerializer.Serialize(payload));

        var buffer = new List<byte>();

        // Add command type byte
        buffer.Add((byte)(int)Enum.Parse<CommandType>(commandData["type"].ToString()!));

        // Add parameters based on device requirements
        if (commandData.ContainsKey("parameters"))
        {
            var parameters = JsonSerializer.Deserialize<Dictionary<string, object>>(
                commandData["parameters"].ToString()!);
            
            foreach (var param in parameters)
            {
                buffer.AddRange(SerializeParameter(param.Key, param.Value));
            }
        }

        return buffer.ToArray();
    }

    private byte[] SerializeToArduino(object payload, SerializationConfig config)
    {
        var commandData = JsonSerializer.Deserialize<Dictionary<string, object>>(
            JsonSerializer.Serialize(payload));

        var options = config.FormatOptions;
        var prefix = options.GetValueOrDefault("commandPrefix", "$").ToString();
        var suffix = options.GetValueOrDefault("commandSuffix", "\r\n").ToString();
        var separator = options.GetValueOrDefault("fieldSeparator", ",").ToString();

        var commandString = new StringBuilder(prefix);
        commandString.Append(commandData["type"]);

        if (commandData.ContainsKey("parameters"))
        {
            var parameters = JsonSerializer.Deserialize<Dictionary<string, object>>(
                commandData["parameters"].ToString()!);

            foreach (var param in parameters)
            {
                commandString.Append(separator);
                commandString.Append($"{param.Key}={param.Value}");
            }
        }

        commandString.Append(suffix);
        
        var encoding = GetEncoding(config.Encoding);
        return encoding.GetBytes(commandString.ToString());
    }

    private byte[] SerializeCustomFormat(object payload, SerializationConfig config)
    {
        // Extensible custom format - can be implemented for specific devices
        // For now, fallback to JSON
        return SerializeToJson(payload, config);
    }

    private object? DeserializeFromJson(byte[] data, SerializationConfig config, Type expectedType)
    {
        var encoding = GetEncoding(config.Encoding);
        var json = encoding.GetString(data);
        return JsonSerializer.Deserialize(json, expectedType, _jsonOptions);
    }

    private object? DeserializeFromBinary(byte[] data, SerializationConfig config, Type expectedType)
    {
        // Simple binary deserialization - device specific
        if (expectedType == typeof(bool))
        {
            return data.Length > 0 && data[0] != 0;
        }
        if (expectedType == typeof(int))
        {
            return data.Length >= 4 ? BitConverter.ToInt32(data, 0) : 0;
        }
        if (expectedType == typeof(string))
        {
            var encoding = GetEncoding(config.Encoding);
            return encoding.GetString(data);
        }

        return null;
    }

    private object? DeserializeFromArduino(byte[] data, SerializationConfig config, Type expectedType)
    {
        var encoding = GetEncoding(config.Encoding);
        var response = encoding.GetString(data).Trim();
        
        // Parse Arduino response format (typically simple values or key=value pairs)
        if (expectedType == typeof(int) && int.TryParse(response, out var intValue))
        {
            return intValue;
        }
        if (expectedType == typeof(bool))
        {
            return response.Equals("1") || response.Equals("true", StringComparison.OrdinalIgnoreCase);
        }
        
        return response;
    }

    private object? DeserializeCustomFormat(byte[] data, SerializationConfig config, Type expectedType)
    {
        // Fallback to JSON for custom formats
        return DeserializeFromJson(data, config, expectedType);
    }

    private byte[] SerializeParameter(string name, object value)
    {
        return value switch
        {
            bool b => new[] { (byte)(b ? 1 : 0) },
            int i => BitConverter.GetBytes(i),
            float f => BitConverter.GetBytes(f),
            string s => Encoding.UTF8.GetBytes(s),
            _ => Encoding.UTF8.GetBytes(value.ToString() ?? "")
        };
    }

    private Encoding GetEncoding(string encodingName) => encodingName.ToLowerInvariant() switch
    {
        "utf8" => Encoding.UTF8,
        "ascii" => Encoding.ASCII,
        "utf16" => Encoding.Unicode,
        "binary" => Encoding.ASCII, // For binary data
        _ => Encoding.UTF8
    };

    private byte[] AddChecksum(byte[] data, string algorithm)
    {
        var checksum = algorithm.ToLowerInvariant() switch
        {
            "crc8" => CalculateCrc8(data),
            "crc32" => BitConverter.GetBytes(CalculateCrc32(data)),
            "xor" => new[] { CalculateXorChecksum(data) },
            "md5" => MD5.HashData(data),
            _ => new[] { CalculateXorChecksum(data) }
        };

        var result = new byte[data.Length + checksum.Length];
        Array.Copy(data, result, data.Length);
        Array.Copy(checksum, 0, result, data.Length, checksum.Length);
        return result;
    }

    private byte[] ValidateAndRemoveChecksum(byte[] data, string algorithm)
    {
        var checksumLength = algorithm.ToLowerInvariant() switch
        {
            "crc8" => 1,
            "crc32" => 4,
            "xor" => 1,
            "md5" => 16,
            _ => 1
        };

        if (data.Length < checksumLength)
        {
            throw new InvalidOperationException("Data too short to contain checksum");
        }

        var payload = new byte[data.Length - checksumLength];
        Array.Copy(data, payload, payload.Length);

        var expectedChecksum = new byte[checksumLength];
        Array.Copy(data, payload.Length, expectedChecksum, 0, checksumLength);

        var calculatedChecksum = algorithm.ToLowerInvariant() switch
        {
            "crc8" => CalculateCrc8(payload),
            "crc32" => BitConverter.GetBytes(CalculateCrc32(payload)),
            "xor" => new[] { CalculateXorChecksum(payload) },
            "md5" => MD5.HashData(payload),
            _ => new[] { CalculateXorChecksum(payload) }
        };

        if (!expectedChecksum.AsSpan().SequenceEqual(calculatedChecksum.AsSpan()))
        {
            throw new InvalidOperationException("Checksum validation failed");
        }

        return payload;
    }

    private byte[] CalculateCrc8(byte[] data)
    {
        byte crc = 0;
        foreach (var b in data)
        {
            crc ^= b;
            for (int i = 0; i < 8; i++)
            {
                if ((crc & 0x80) != 0)
                    crc = (byte)((crc << 1) ^ 0x07);
                else
                    crc <<= 1;
            }
        }
        return new[] { crc };
    }

    private uint CalculateCrc32(byte[] data)
    {
        uint crc = 0xFFFFFFFF;
        foreach (var b in data)
        {
            crc ^= b;
            for (int i = 0; i < 8; i++)
            {
                if ((crc & 1) != 0)
                    crc = (crc >> 1) ^ 0xEDB88320;
                else
                    crc >>= 1;
            }
        }
        return ~crc;
    }

    private byte CalculateXorChecksum(byte[] data)
    {
        byte checksum = 0;
        foreach (var b in data)
        {
            checksum ^= b;
        }
        return checksum;
    }

    private async Task ValidateDeviceSpecificParameters(DeviceCommand command, string deviceType, ValidationResult result)
    {
        // Device-specific parameter validation logic
        switch (deviceType.ToLowerInvariant())
        {
            case "arduino":
                ValidateArduinoCommand(command, result);
                break;
            case "esp32":
            case "esp8266":
                ValidateEspCommand(command, result);
                break;
            case "riorand":
                ValidateRioRandCommand(command, result);
                break;
            case "raspberry-pi":
                ValidateRaspberryPiCommand(command, result);
                break;
        }
    }

    private void ValidateArduinoCommand(DeviceCommand command, ValidationResult result)
    {
        switch (command.Type)
        {
            case CommandType.DigitalWrite:
                if (!command.Parameters.ContainsKey("pin") || !command.Parameters.ContainsKey("value"))
                {
                    result.Errors.Add("DigitalWrite requires 'pin' and 'value' parameters");
                    result.IsValid = false;
                }
                break;
            case CommandType.AnalogRead:
                if (!command.Parameters.ContainsKey("pin"))
                {
                    result.Errors.Add("AnalogRead requires 'pin' parameter");
                    result.IsValid = false;
                }
                break;
        }
    }

    private void ValidateEspCommand(DeviceCommand command, ValidationResult result)
    {
        // ESP32/ESP8266 validation logic
        ValidateArduinoCommand(command, result); // ESPs support Arduino-compatible commands
    }

    private void ValidateRioRandCommand(DeviceCommand command, ValidationResult result)
    {
        switch (command.Type)
        {
            case CommandType.SetRelay:
                if (!command.Parameters.ContainsKey("relay") || !command.Parameters.ContainsKey("state"))
                {
                    result.Errors.Add("SetRelay requires 'relay' and 'state' parameters");
                    result.IsValid = false;
                }
                break;
        }
    }

    private void ValidateRaspberryPiCommand(DeviceCommand command, ValidationResult result)
    {
        // Raspberry Pi GPIO validation
        if (command.Type == CommandType.DigitalWrite)
        {
            if (command.Parameters.TryGetValue("pin", out var pinObj) && pinObj is int pin)
            {
                if (pin < 2 || pin > 27)
                {
                    result.Warnings.Add($"GPIO pin {pin} may not be safe to use");
                }
            }
        }
    }

    private void ValidateParameters(DeviceCommand command, ValidationResult result)
    {
        foreach (var param in command.Parameters)
        {
            if (param.Value == null)
            {
                result.Warnings.Add($"Parameter '{param.Key}' is null");
            }
        }
    }

    private void ValidateSafetyConstraints(DeviceCommand command, ValidationResult result)
    {
        // PWM frequency limits
        if (command.Type == CommandType.SetPWMFrequency)
        {
            if (command.Parameters.TryGetValue("frequency", out var freqObj) && freqObj is int freq)
            {
                if (freq > 50000)
                {
                    result.Warnings.Add($"PWM frequency {freq}Hz is very high - ensure hardware can handle it");
                }
                if (freq < 1)
                {
                    result.Errors.Add("PWM frequency must be positive");
                    result.IsValid = false;
                }
            }
        }

        // Motor speed limits
        if (command.Type == CommandType.SetMotorSpeed)
        {
            if (command.Parameters.TryGetValue("speed", out var speedObj) && speedObj is int speed)
            {
                if (speed > 255 || speed < -255)
                {
                    result.Errors.Add("Motor speed must be between -255 and 255");
                    result.IsValid = false;
                }
            }
        }
    }
}