//! Telemetry sample types and metadata
//! 
//! Defines the structure of telemetry samples with support for
//! various data types and rich metadata.

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

/// A telemetry sample with timestamp and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySample {
    /// Unix timestamp in milliseconds
    pub timestamp_ms: u64,
    /// The actual sample value
    pub value: SampleValue,
    /// Optional metadata
    pub metadata: Option<SampleMetadata>,
}

/// Possible telemetry sample values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SampleValue {
    /// 32-bit floating point (most common for analog values)
    Float32(f32),
    /// 64-bit floating point (high precision)
    Float64(f64),
    /// Signed 32-bit integer (digital values, counters)
    Int32(i32),
    /// Unsigned 32-bit integer
    UInt32(u32),
    /// Boolean value (digital I/O states)
    Bool(bool),
    /// String value (events, states)
    String(String),
    /// Binary data
    Bytes(Vec<u8>),
    /// Multiple values in one sample
    Vector(Vec<f32>),
}

/// Sample type enumeration for channel configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SampleType {
    Float32,
    Float64,
    Int32,
    UInt32,
    Bool,
    String,
    Bytes,
    Vector,
}

/// Metadata associated with a telemetry sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleMetadata {
    /// Source device or component
    pub source: Option<String>,
    /// Unit of measurement
    pub unit: Option<String>,
    /// Quality indicator (0.0 to 1.0)
    pub quality: Option<f32>,
    /// Additional tags
    pub tags: Vec<String>,
    /// Custom key-value pairs
    pub custom: std::collections::HashMap<String, String>,
}

impl TelemetrySample {
    /// Create a new telemetry sample with current timestamp
    pub fn new(value: SampleValue) -> Self {
        Self {
            timestamp_ms: Self::current_timestamp_ms(),
            value,
            metadata: None,
        }
    }
    
    /// Create a new sample with specific timestamp
    pub fn with_timestamp(value: SampleValue, timestamp_ms: u64) -> Self {
        Self {
            timestamp_ms,
            value,
            metadata: None,
        }
    }
    
    /// Create a new sample with metadata
    pub fn with_metadata(value: SampleValue, metadata: SampleMetadata) -> Self {
        Self {
            timestamp_ms: Self::current_timestamp_ms(),
            value,
            metadata: Some(metadata),
        }
    }
    
    /// Convenience constructor for f32 values
    pub fn new_f32(value: f32) -> Self {
        Self::new(SampleValue::Float32(value))
    }
    
    /// Convenience constructor for boolean values
    pub fn new_bool(value: bool) -> Self {
        Self::new(SampleValue::Bool(value))
    }
    
    /// Convenience constructor for integer values
    pub fn new_i32(value: i32) -> Self {
        Self::new(SampleValue::Int32(value))
    }
    
    /// Convenience constructor for string events
    pub fn new_event(event: String) -> Self {
        Self::new(SampleValue::String(event))
    }
    
    /// Get current timestamp in milliseconds
    fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
    
    /// Get the sample type
    pub fn sample_type(&self) -> SampleType {
        match self.value {
            SampleValue::Float32(_) => SampleType::Float32,
            SampleValue::Float64(_) => SampleType::Float64,
            SampleValue::Int32(_) => SampleType::Int32,
            SampleValue::UInt32(_) => SampleType::UInt32,
            SampleValue::Bool(_) => SampleType::Bool,
            SampleValue::String(_) => SampleType::String,
            SampleValue::Bytes(_) => SampleType::Bytes,
            SampleValue::Vector(_) => SampleType::Vector,
        }
    }
    
    /// Convert value to f32 if possible (for charting)
    pub fn as_f32(&self) -> Option<f32> {
        match &self.value {
            SampleValue::Float32(v) => Some(*v),
            SampleValue::Float64(v) => Some(*v as f32),
            SampleValue::Int32(v) => Some(*v as f32),
            SampleValue::UInt32(v) => Some(*v as f32),
            SampleValue::Bool(v) => Some(if *v { 1.0 } else { 0.0 }),
            _ => None,
        }
    }
    
    /// Get elapsed time since sample was taken (milliseconds)
    pub fn age_ms(&self) -> u64 {
        Self::current_timestamp_ms().saturating_sub(self.timestamp_ms)
    }
    
    /// Check if sample is older than specified milliseconds
    pub fn is_older_than(&self, ms: u64) -> bool {
        self.age_ms() > ms
    }
    
    /// Estimate memory size of this sample
    pub fn size_bytes(&self) -> usize {
        std::mem::size_of::<Self>() +
        match &self.value {
            SampleValue::String(s) => s.len(),
            SampleValue::Bytes(b) => b.len(),
            SampleValue::Vector(v) => v.len() * std::mem::size_of::<f32>(),
            _ => 0,
        }
    }
}

impl Default for SampleMetadata {
    fn default() -> Self {
        Self {
            source: None,
            unit: None,
            quality: None,
            tags: Vec::new(),
            custom: std::collections::HashMap::new(),
        }
    }
}

impl SampleMetadata {
    /// Create metadata with source
    pub fn with_source(source: String) -> Self {
        Self {
            source: Some(source),
            ..Default::default()
        }
    }
    
    /// Create metadata with unit
    pub fn with_unit(unit: String) -> Self {
        Self {
            unit: Some(unit),
            ..Default::default()
        }
    }
    
    /// Builder-style method to add source
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }
    
    /// Builder-style method to add unit
    pub fn unit(mut self, unit: String) -> Self {
        self.unit = Some(unit);
        self
    }
    
    /// Builder-style method to add quality
    pub fn quality(mut self, quality: f32) -> Self {
        self.quality = Some(quality.clamp(0.0, 1.0));
        self
    }
    
    /// Builder-style method to add tag
    pub fn tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    /// Builder-style method to add custom field
    pub fn custom_field(mut self, key: String, value: String) -> Self {
        self.custom.insert(key, value);
        self
    }
}

/// Statistics for a collection of samples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleStatistics {
    pub count: usize,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub mean: Option<f32>,
    pub std_dev: Option<f32>,
    pub first_timestamp_ms: Option<u64>,
    pub last_timestamp_ms: Option<u64>,
}

impl SampleStatistics {
    /// Calculate statistics from a slice of samples
    pub fn from_samples(samples: &[TelemetrySample]) -> Self {
        if samples.is_empty() {
            return Self {
                count: 0,
                min: None,
                max: None,
                mean: None,
                std_dev: None,
                first_timestamp_ms: None,
                last_timestamp_ms: None,
            };
        }
        
        let mut values: Vec<f32> = samples
            .iter()
            .filter_map(|s| s.as_f32())
            .collect();
        
        if values.is_empty() {
            return Self {
                count: samples.len(),
                min: None,
                max: None,
                mean: None,
                std_dev: None,
                first_timestamp_ms: samples.first().map(|s| s.timestamp_ms),
                last_timestamp_ms: samples.last().map(|s| s.timestamp_ms),
            };
        }
        
        let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let sum: f32 = values.iter().sum();
        let mean = sum / values.len() as f32;
        
        let variance = values.iter()
            .map(|v| {
                let diff = v - mean;
                diff * diff
            })
            .sum::<f32>() / values.len() as f32;
        
        let std_dev = variance.sqrt();
        
        Self {
            count: samples.len(),
            min: Some(min),
            max: Some(max),
            mean: Some(mean),
            std_dev: Some(std_dev),
            first_timestamp_ms: samples.first().map(|s| s.timestamp_ms),
            last_timestamp_ms: samples.last().map(|s| s.timestamp_ms),
        }
    }
    
    /// Get the time span covered by samples (milliseconds)
    pub fn time_span_ms(&self) -> Option<u64> {
        match (self.first_timestamp_ms, self.last_timestamp_ms) {
            (Some(first), Some(last)) => Some(last.saturating_sub(first)),
            _ => None,
        }
    }
    
    /// Get the sample rate (Hz) if time span is available
    pub fn sample_rate_hz(&self) -> Option<f32> {
        if self.count <= 1 {
            return None;
        }
        
        self.time_span_ms().and_then(|span_ms| {
            if span_ms > 0 {
                Some((self.count as f32 - 1.0) * 1000.0 / span_ms as f32)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sample_creation() {
        let sample = TelemetrySample::new_f32(42.0);
        assert!(matches!(sample.value, SampleValue::Float32(42.0)));
        assert!(sample.timestamp_ms > 0);
        assert!(sample.metadata.is_none());
    }
    
    #[test]
    fn test_sample_with_metadata() {
        let metadata = SampleMetadata::default()
            .source("Arduino".to_string())
            .unit("V".to_string())
            .quality(0.95)
            .tag("analog".to_string());
        
        let sample = TelemetrySample::with_metadata(
            SampleValue::Float32(3.3),
            metadata.clone()
        );
        
        assert!(sample.metadata.is_some());
        let meta = sample.metadata.unwrap();
        assert_eq!(meta.source, Some("Arduino".to_string()));
        assert_eq!(meta.unit, Some("V".to_string()));
        assert_eq!(meta.quality, Some(0.95));
        assert_eq!(meta.tags.len(), 1);
    }
    
    #[test]
    fn test_value_conversion() {
        assert_eq!(TelemetrySample::new_f32(42.0).as_f32(), Some(42.0));
        assert_eq!(TelemetrySample::new_i32(42).as_f32(), Some(42.0));
        assert_eq!(TelemetrySample::new_bool(true).as_f32(), Some(1.0));
        assert_eq!(TelemetrySample::new_bool(false).as_f32(), Some(0.0));
        assert_eq!(TelemetrySample::new_event("test".to_string()).as_f32(), None);
    }
    
    #[test]
    fn test_statistics() {
        let samples = vec![
            TelemetrySample::new_f32(1.0),
            TelemetrySample::new_f32(2.0),
            TelemetrySample::new_f32(3.0),
            TelemetrySample::new_f32(4.0),
            TelemetrySample::new_f32(5.0),
        ];
        
        let stats = SampleStatistics::from_samples(&samples);
        
        assert_eq!(stats.count, 5);
        assert_eq!(stats.min, Some(1.0));
        assert_eq!(stats.max, Some(5.0));
        assert_eq!(stats.mean, Some(3.0));
        assert!(stats.std_dev.is_some());
        assert!(stats.first_timestamp_ms.is_some());
        assert!(stats.last_timestamp_ms.is_some());
    }
}