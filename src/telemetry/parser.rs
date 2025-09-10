//! Multi-format telemetry data parsers for CSV, JSON, and binary formats
//! 
//! This module provides comprehensive parsing capabilities for various telemetry
//! data formats with robust error handling and schema validation.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::telemetry::{TelemetrySample, SampleValue, SampleMetadata, SampleType};

/// Supported telemetry data formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetryFormat {
    Csv,
    Json, 
    Binary,
}

/// Telemetry data frame containing parsed samples and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryFrame {
    pub timestamp: DateTime<Utc>,
    pub sequence_number: u64,
    pub device_id: String,
    pub data_points: Vec<DataPoint>,
    pub metadata: HashMap<String, String>,
}

/// Individual data point within a telemetry frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub channel: String,
    pub value: TelemetryValue,
    pub unit: Option<String>,
    pub quality: DataQuality,
}

/// Telemetry value types supported by the parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Bytes(Vec<u8>),
}

/// Data quality assessment for parsed values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataQuality {
    Good,
    Questionable,  // Within bounds but unusual
    Bad,           // Outside expected bounds
    Missing,       // No data received
}

/// Parsing errors that can occur during data parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParseError {
    InvalidFormat(String),
    ColumnCountMismatch { expected: usize, actual: usize },
    InvalidInteger(String),
    InvalidFloat(String),
    InvalidBoolean(String),
    InvalidTimestamp(String),
    JsonError(String),
    InsufficientData { required: usize, available: usize },
    UnsupportedVersion { expected: u8, actual: u8 },
    EmptyData,
}

/// Schema validation errors
#[derive(Debug, Clone)]
pub enum ValidationError {
    EmptyData,
    SchemaMismatch { expected_columns: usize, actual_columns: usize },
    InvalidJson(String),
    InsufficientData,
    UnsupportedVersion,
}

/// Trait for format-specific parsers
pub trait TelemetryParser: Send + Sync {
    fn parse(&self, data: &[u8]) -> Result<TelemetryFrame, ParseError>;
    fn format_name(&self) -> &'static str;
    fn validate_schema(&self, data: &[u8]) -> Result<(), ValidationError>;
}

/// Main telemetry parser that handles multiple formats
#[derive(Debug, Clone)]
pub struct MultiFormatTelemetryParser {
    csv_parser: CsvTelemetryParser,
    json_parser: JsonTelemetryParser,
    binary_parser: BinaryTelemetryParser,
}

impl MultiFormatTelemetryParser {
    /// Create a new multi-format parser with default configurations
    pub fn new() -> Self {
        Self {
            csv_parser: CsvTelemetryParser::new(),
            json_parser: JsonTelemetryParser::new(),
            binary_parser: BinaryTelemetryParser::new(),
        }
    }

    /// Create with custom CSV schema
    pub fn with_csv_schema(schema: CsvSchema) -> Self {
        Self {
            csv_parser: CsvTelemetryParser::with_schema(schema),
            json_parser: JsonTelemetryParser::new(),
            binary_parser: BinaryTelemetryParser::new(),
        }
    }

    /// Parse data using the specified format
    pub fn parse(&self, data: &[u8], format: TelemetryFormat) -> Result<TelemetryFrame, ParseError> {
        match format {
            TelemetryFormat::Csv => self.csv_parser.parse(data),
            TelemetryFormat::Json => self.json_parser.parse(data),
            TelemetryFormat::Binary => self.binary_parser.parse(data),
        }
    }

    /// Auto-detect format and parse data
    pub fn parse_auto(&self, data: &[u8]) -> Result<(TelemetryFormat, TelemetryFrame), ParseError> {
        // Try JSON first (most structured)
        if let Ok(frame) = self.json_parser.parse(data) {
            return Ok((TelemetryFormat::Json, frame));
        }

        // Try binary if data looks binary
        if self.is_likely_binary(data) {
            if let Ok(frame) = self.binary_parser.parse(data) {
                return Ok((TelemetryFormat::Binary, frame));
            }
        }

        // Try CSV last
        match self.csv_parser.parse(data) {
            Ok(frame) => Ok((TelemetryFormat::Csv, frame)),
            Err(e) => Err(e),
        }
    }

    /// Convert TelemetryFrame to TelemetrySample vector for storage
    pub fn frame_to_samples(&self, frame: TelemetryFrame) -> Vec<TelemetrySample> {
        let timestamp_ms = frame.timestamp.timestamp_millis() as u64;
        
        frame.data_points
            .into_iter()
            .map(|point| {
                let value = match point.value {
                    TelemetryValue::Integer(i) => SampleValue::Int32(i as i32),
                    TelemetryValue::Float(f) => SampleValue::Float64(f),
                    TelemetryValue::Boolean(b) => SampleValue::Bool(b),
                    TelemetryValue::String(s) => SampleValue::String(s),
                    TelemetryValue::Bytes(b) => SampleValue::Bytes(b),
                };

                let metadata = SampleMetadata {
                    source: Some(frame.device_id.clone()),
                    unit: point.unit,
                    quality: Some(match point.quality {
                        DataQuality::Good => 1.0,
                        DataQuality::Questionable => 0.5,
                        DataQuality::Bad => 0.1,
                        DataQuality::Missing => 0.0,
                    }),
                    tags: vec![point.channel],
                    custom: frame.metadata.clone(),
                };

                TelemetrySample::with_timestamp(value, timestamp_ms)
                    .with_metadata(metadata)
            })
            .collect()
    }

    fn is_likely_binary(&self, data: &[u8]) -> bool {
        if data.len() < 4 {
            return false;
        }

        // Check for high ratio of non-printable characters
        let non_printable = data
            .iter()
            .take(100) // Check first 100 bytes
            .filter(|&&b| b < 32 && b != 9 && b != 10 && b != 13)
            .count();

        non_printable > data.len().min(100) / 4
    }
}

/// CSV format parser
#[derive(Debug, Clone)]
pub struct CsvTelemetryParser {
    schema: CsvSchema,
    delimiter: char,
    has_header: bool,
}

/// CSV parsing schema definition
#[derive(Debug, Clone)]
pub struct CsvSchema {
    pub columns: Vec<ColumnDefinition>,
    pub timestamp_column: Option<usize>,
    pub sequence_column: Option<usize>,
    pub device_id_column: Option<usize>,
}

/// Column definition for CSV parsing
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: TelemetryValueType,
    pub unit: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

/// Data type enumeration for CSV columns
#[derive(Debug, Clone)]
pub enum TelemetryValueType {
    Integer,
    Float,
    Boolean,
    String,
}

impl CsvTelemetryParser {
    /// Create parser with default schema
    pub fn new() -> Self {
        Self::with_schema(CsvSchema::default())
    }

    /// Create parser with custom schema
    pub fn with_schema(schema: CsvSchema) -> Self {
        Self {
            schema,
            delimiter: ',',
            has_header: true,
        }
    }

    /// Set delimiter character
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Set whether CSV has header row
    pub fn with_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }
}

impl TelemetryParser for CsvTelemetryParser {
    fn parse(&self, data: &[u8]) -> Result<TelemetryFrame, ParseError> {
        let text = String::from_utf8_lossy(data);
        let mut lines = text.lines();
        
        // Skip header if present
        if self.has_header {
            lines.next();
        }
        
        let mut data_points = Vec::new();
        let mut timestamp = Utc::now();
        let mut sequence_number = 0u64;
        let mut device_id = "unknown".to_string();
        
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            
            let fields: Vec<&str> = line.split(self.delimiter).collect();
            
            if fields.len() != self.schema.columns.len() {
                return Err(ParseError::ColumnCountMismatch {
                    expected: self.schema.columns.len(),
                    actual: fields.len(),
                });
            }
            
            // Extract timestamp if defined
            if let Some(ts_col) = self.schema.timestamp_column {
                if let Some(ts_str) = fields.get(ts_col) {
                    timestamp = self.parse_timestamp(ts_str)?;
                }
            }
            
            // Extract sequence number if defined
            if let Some(seq_col) = self.schema.sequence_column {
                if let Some(seq_str) = fields.get(seq_col) {
                    sequence_number = seq_str.parse()
                        .map_err(|_| ParseError::InvalidInteger(seq_str.to_string()))?;
                }
            }

            // Extract device ID if defined
            if let Some(dev_col) = self.schema.device_id_column {
                if let Some(dev_str) = fields.get(dev_col) {
                    device_id = dev_str.to_string();
                }
            }
            
            // Parse data columns
            for (i, (field, column_def)) in fields.iter().zip(&self.schema.columns).enumerate() {
                // Skip metadata columns
                if Some(i) == self.schema.timestamp_column 
                    || Some(i) == self.schema.sequence_column
                    || Some(i) == self.schema.device_id_column {
                    continue;
                }
                
                let value = self.parse_field_value(field, &column_def.data_type)?;
                let quality = self.assess_data_quality(&value, column_def);
                
                data_points.push(DataPoint {
                    channel: column_def.name.clone(),
                    value,
                    unit: column_def.unit.clone(),
                    quality,
                });
            }
            
            break; // Only process first data line for now
        }
        
        Ok(TelemetryFrame {
            timestamp,
            sequence_number,
            device_id,
            data_points,
            metadata: HashMap::new(),
        })
    }
    
    fn format_name(&self) -> &'static str {
        "CSV"
    }
    
    fn validate_schema(&self, data: &[u8]) -> Result<(), ValidationError> {
        let text = String::from_utf8_lossy(data);
        let first_line = text.lines().next()
            .ok_or(ValidationError::EmptyData)?;
        
        let field_count = first_line.split(self.delimiter).count();
        if field_count != self.schema.columns.len() {
            return Err(ValidationError::SchemaMismatch {
                expected_columns: self.schema.columns.len(),
                actual_columns: field_count,
            });
        }
        
        Ok(())
    }
}

impl CsvTelemetryParser {
    fn parse_field_value(&self, field: &str, value_type: &TelemetryValueType) -> Result<TelemetryValue, ParseError> {
        match value_type {
            TelemetryValueType::Integer => {
                field.parse::<i64>()
                    .map(TelemetryValue::Integer)
                    .map_err(|_| ParseError::InvalidInteger(field.to_string()))
            }
            TelemetryValueType::Float => {
                field.parse::<f64>()
                    .map(TelemetryValue::Float)
                    .map_err(|_| ParseError::InvalidFloat(field.to_string()))
            }
            TelemetryValueType::Boolean => {
                match field.to_lowercase().as_str() {
                    "true" | "1" | "yes" | "on" => Ok(TelemetryValue::Boolean(true)),
                    "false" | "0" | "no" | "off" => Ok(TelemetryValue::Boolean(false)),
                    _ => Err(ParseError::InvalidBoolean(field.to_string())),
                }
            }
            TelemetryValueType::String => Ok(TelemetryValue::String(field.to_string())),
        }
    }
    
    fn assess_data_quality(&self, value: &TelemetryValue, column_def: &ColumnDefinition) -> DataQuality {
        match value {
            TelemetryValue::Float(f) => {
                if f.is_nan() || f.is_infinite() {
                    return DataQuality::Bad;
                }
                
                if let (Some(min), Some(max)) = (column_def.min_value, column_def.max_value) {
                    if *f < min || *f > max {
                        return DataQuality::Bad;
                    }
                    
                    // Check for values near bounds (might be questionable)
                    let range = max - min;
                    let tolerance = range * 0.05; // 5% tolerance
                    if (*f < min + tolerance) || (*f > max - tolerance) {
                        return DataQuality::Questionable;
                    }
                }
                
                DataQuality::Good
            }
            TelemetryValue::Integer(i) => {
                if let (Some(min), Some(max)) = (column_def.min_value, column_def.max_value) {
                    let val = *i as f64;
                    if val < min || val > max {
                        return DataQuality::Bad;
                    }
                }
                DataQuality::Good
            }
            _ => DataQuality::Good,
        }
    }
    
    fn parse_timestamp(&self, ts_str: &str) -> Result<DateTime<Utc>, ParseError> {
        // Try multiple timestamp formats
        let formats = [
            "%Y-%m-%d %H:%M:%S%.f",
            "%Y-%m-%dT%H:%M:%S%.fZ",
            "%Y-%m-%d %H:%M:%S",
            "%s", // Unix timestamp
        ];
        
        for format in &formats {
            if let Ok(dt) = DateTime::parse_from_str(ts_str, format) {
                return Ok(dt.with_timezone(&Utc));
            }
        }
        
        // Try parsing as Unix timestamp
        if let Ok(timestamp) = ts_str.parse::<i64>() {
            if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
                return Ok(dt);
            }
        }
        
        Err(ParseError::InvalidTimestamp(ts_str.to_string()))
    }
}

/// JSON format parser
#[derive(Debug, Clone)]
pub struct JsonTelemetryParser;

impl JsonTelemetryParser {
    pub fn new() -> Self {
        Self
    }
}

impl TelemetryParser for JsonTelemetryParser {
    fn parse(&self, data: &[u8]) -> Result<TelemetryFrame, ParseError> {
        serde_json::from_slice(data)
            .map_err(|e| ParseError::JsonError(e.to_string()))
    }
    
    fn format_name(&self) -> &'static str {
        "JSON"
    }
    
    fn validate_schema(&self, data: &[u8]) -> Result<(), ValidationError> {
        // Basic JSON validation
        let _: serde_json::Value = serde_json::from_slice(data)
            .map_err(|e| ValidationError::InvalidJson(e.to_string()))?;
        Ok(())
    }
}

/// Binary format parser (custom protocol)
#[derive(Debug, Clone)]
pub struct BinaryTelemetryParser {
    header_size: usize,
    frame_version: u8,
}

impl BinaryTelemetryParser {
    pub fn new() -> Self {
        Self {
            header_size: 19, // Fixed header size for this implementation
            frame_version: 1,
        }
    }

    pub fn with_version(mut self, version: u8) -> Self {
        self.frame_version = version;
        self
    }
}

impl TelemetryParser for BinaryTelemetryParser {
    fn parse(&self, data: &[u8]) -> Result<TelemetryFrame, ParseError> {
        if data.len() < self.header_size {
            return Err(ParseError::InsufficientData {
                required: self.header_size,
                available: data.len(),
            });
        }
        
        // Parse binary header (example format)
        let version = data[0];
        if version != self.frame_version {
            return Err(ParseError::UnsupportedVersion {
                expected: self.frame_version,
                actual: version,
            });
        }
        
        let sequence_number = u64::from_le_bytes([
            data[1], data[2], data[3], data[4],
            data[5], data[6], data[7], data[8],
        ]);
        
        let timestamp_ms = u64::from_le_bytes([
            data[9], data[10], data[11], data[12],
            data[13], data[14], data[15], data[16],
        ]);
        
        let timestamp = DateTime::from_timestamp_millis(timestamp_ms as i64)
            .ok_or_else(|| ParseError::InvalidTimestamp(timestamp_ms.to_string()))?;
        
        let data_point_count = u16::from_le_bytes([data[17], data[18]]) as usize;
        
        // Parse data points (simplified)
        let mut data_points = Vec::with_capacity(data_point_count);
        let mut offset = self.header_size;
        
        for _ in 0..data_point_count {
            if offset + 12 > data.len() {
                break;
            }
            
            let channel_id = u16::from_le_bytes([data[offset], data[offset + 1]]);
            let value = f64::from_le_bytes([
                data[offset + 2], data[offset + 3], data[offset + 4], data[offset + 5],
                data[offset + 6], data[offset + 7], data[offset + 8], data[offset + 9],
            ]);
            let quality_byte = data[offset + 10];
            
            let quality = match quality_byte {
                0 => DataQuality::Good,
                1 => DataQuality::Questionable,
                2 => DataQuality::Bad,
                _ => DataQuality::Missing,
            };
            
            data_points.push(DataPoint {
                channel: format!("channel_{}", channel_id),
                value: TelemetryValue::Float(value),
                unit: None,
                quality,
            });
            
            offset += 11;
        }
        
        Ok(TelemetryFrame {
            timestamp,
            sequence_number,
            device_id: "binary_device".to_string(),
            data_points,
            metadata: HashMap::new(),
        })
    }
    
    fn format_name(&self) -> &'static str {
        "Binary"
    }
    
    fn validate_schema(&self, data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < self.header_size {
            return Err(ValidationError::InsufficientData);
        }
        
        if data[0] != self.frame_version {
            return Err(ValidationError::UnsupportedVersion);
        }
        
        Ok(())
    }
}

impl Default for CsvSchema {
    fn default() -> Self {
        Self {
            columns: vec![
                ColumnDefinition {
                    name: "timestamp".to_string(),
                    data_type: TelemetryValueType::String,
                    unit: None,
                    min_value: None,
                    max_value: None,
                },
                ColumnDefinition {
                    name: "value".to_string(),
                    data_type: TelemetryValueType::Float,
                    unit: Some("V".to_string()),
                    min_value: Some(-10.0),
                    max_value: Some(10.0),
                },
            ],
            timestamp_column: Some(0),
            sequence_column: None,
            device_id_column: None,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::ColumnCountMismatch { expected, actual } => {
                write!(f, "Column count mismatch: expected {}, got {}", expected, actual)
            }
            ParseError::InvalidInteger(val) => write!(f, "Invalid integer: {}", val),
            ParseError::InvalidFloat(val) => write!(f, "Invalid float: {}", val),
            ParseError::InvalidBoolean(val) => write!(f, "Invalid boolean: {}", val),
            ParseError::InvalidTimestamp(val) => write!(f, "Invalid timestamp: {}", val),
            ParseError::JsonError(msg) => write!(f, "JSON error: {}", msg),
            ParseError::InsufficientData { required, available } => {
                write!(f, "Insufficient data: need {} bytes, got {}", required, available)
            }
            ParseError::UnsupportedVersion { expected, actual } => {
                write!(f, "Unsupported version: expected {}, got {}", expected, actual)
            }
            ParseError::EmptyData => write!(f, "Empty data"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_parsing() {
        let csv_data = b"timestamp,voltage,current\n2024-01-01 12:00:00,3.3,0.5\n2024-01-01 12:00:01,3.2,0.6";
        
        let schema = CsvSchema {
            columns: vec![
                ColumnDefinition {
                    name: "timestamp".to_string(),
                    data_type: TelemetryValueType::String,
                    unit: None,
                    min_value: None,
                    max_value: None,
                },
                ColumnDefinition {
                    name: "voltage".to_string(),
                    data_type: TelemetryValueType::Float,
                    unit: Some("V".to_string()),
                    min_value: Some(0.0),
                    max_value: Some(5.0),
                },
                ColumnDefinition {
                    name: "current".to_string(),
                    data_type: TelemetryValueType::Float,
                    unit: Some("A".to_string()),
                    min_value: Some(0.0),
                    max_value: Some(2.0),
                },
            ],
            timestamp_column: Some(0),
            sequence_column: None,
            device_id_column: None,
        };
        
        let parser = CsvTelemetryParser::with_schema(schema);
        let result = parser.parse(csv_data);
        
        assert!(result.is_ok());
        let frame = result.unwrap();
        assert_eq!(frame.data_points.len(), 2);
        assert_eq!(frame.data_points[0].channel, "voltage");
        assert_eq!(frame.data_points[1].channel, "current");
    }

    #[test]
    fn test_json_parsing() {
        let json_data = r#"{
            "timestamp": "2024-01-01T12:00:00Z",
            "sequence_number": 1,
            "device_id": "arduino_001",
            "data_points": [
                {
                    "channel": "voltage",
                    "value": {"Float": 3.3},
                    "unit": "V",
                    "quality": "Good"
                }
            ],
            "metadata": {}
        }"#.as_bytes();
        
        let parser = JsonTelemetryParser::new();
        let result = parser.parse(json_data);
        
        assert!(result.is_ok());
        let frame = result.unwrap();
        assert_eq!(frame.device_id, "arduino_001");
        assert_eq!(frame.data_points.len(), 1);
    }

    #[test]
    fn test_binary_parsing() {
        // Construct a binary frame manually
        let mut binary_data = Vec::new();
        
        // Header: version (1 byte)
        binary_data.push(1u8);
        
        // Sequence number (8 bytes)
        binary_data.extend_from_slice(&42u64.to_le_bytes());
        
        // Timestamp (8 bytes)
        binary_data.extend_from_slice(&1640995200000u64.to_le_bytes()); // 2022-01-01 00:00:00
        
        // Data point count (2 bytes)
        binary_data.extend_from_slice(&1u16.to_le_bytes());
        
        // Data point: channel_id (2 bytes) + value (8 bytes) + quality (1 byte)
        binary_data.extend_from_slice(&0u16.to_le_bytes());  // channel 0
        binary_data.extend_from_slice(&3.3f64.to_le_bytes()); // value
        binary_data.push(0u8); // good quality
        
        let parser = BinaryTelemetryParser::new();
        let result = parser.parse(&binary_data);
        
        assert!(result.is_ok());
        let frame = result.unwrap();
        assert_eq!(frame.sequence_number, 42);
        assert_eq!(frame.data_points.len(), 1);
        assert_eq!(frame.data_points[0].channel, "channel_0");
    }

    #[test]
    fn test_auto_format_detection() {
        let parser = MultiFormatTelemetryParser::new();
        
        // Test JSON detection
        let json_data = r#"{"timestamp": "2024-01-01T12:00:00Z", "sequence_number": 1, "device_id": "test", "data_points": [], "metadata": {}}"#.as_bytes();
        let result = parser.parse_auto(json_data);
        assert!(result.is_ok());
        let (format, _) = result.unwrap();
        assert_eq!(format, TelemetryFormat::Json);
    }

    #[test]
    fn test_data_quality_assessment() {
        let schema = CsvSchema {
            columns: vec![
                ColumnDefinition {
                    name: "value".to_string(),
                    data_type: TelemetryValueType::Float,
                    unit: Some("V".to_string()),
                    min_value: Some(0.0),
                    max_value: Some(5.0),
                },
            ],
            timestamp_column: None,
            sequence_column: None,
            device_id_column: None,
        };
        
        let parser = CsvTelemetryParser::with_schema(schema.clone());
        
        // Test good quality
        let good_value = TelemetryValue::Float(2.5);
        let quality = parser.assess_data_quality(&good_value, &schema.columns[0]);
        assert_eq!(quality, DataQuality::Good);
        
        // Test bad quality (out of range)
        let bad_value = TelemetryValue::Float(10.0);
        let quality = parser.assess_data_quality(&bad_value, &schema.columns[0]);
        assert_eq!(quality, DataQuality::Bad);
        
        // Test questionable quality (near bounds)
        let questionable_value = TelemetryValue::Float(0.1);
        let quality = parser.assess_data_quality(&questionable_value, &schema.columns[0]);
        assert_eq!(quality, DataQuality::Questionable);
    }
}