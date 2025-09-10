//! Telemetry data export functionality
//! 
//! Provides export capabilities for telemetry data in various formats
//! including JSON, CSV, and binary formats.

use crate::telemetry::{TelemetrySample, ChannelExportData};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::io::Write;

/// Supported export formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format (human-readable)
    Json,
    /// Pretty-printed JSON
    JsonPretty,
    /// CSV format (for spreadsheets)
    Csv,
    /// Binary format (compact)
    Binary,
    /// MessagePack format
    MessagePack,
}

/// Telemetry data exporter
pub struct TelemetryExporter {
    compression: bool,
}

impl TelemetryExporter {
    /// Create a new exporter
    pub fn new() -> Self {
        Self {
            compression: false,
        }
    }
    
    /// Enable compression for exports
    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.compression = enabled;
        self
    }
    
    /// Export a single channel's data
    pub fn export(&self, data: &ChannelExportData, format: ExportFormat) -> Result<Vec<u8>, String> {
        match format {
            ExportFormat::Json => self.export_json(data, false),
            ExportFormat::JsonPretty => self.export_json(data, true),
            ExportFormat::Csv => self.export_csv(data),
            ExportFormat::Binary => self.export_binary(data),
            ExportFormat::MessagePack => self.export_messagepack(data),
        }
    }
    
    /// Export multiple channels' data
    pub fn export_multiple(
        &self,
        channels: HashMap<String, ChannelExportData>,
        format: ExportFormat
    ) -> Result<Vec<u8>, String> {
        match format {
            ExportFormat::Json | ExportFormat::JsonPretty => {
                let pretty = format == ExportFormat::JsonPretty;
                self.export_multiple_json(channels, pretty)
            }
            ExportFormat::Csv => self.export_multiple_csv(channels),
            ExportFormat::Binary | ExportFormat::MessagePack => {
                // For binary formats, serialize the entire HashMap
                if format == ExportFormat::Binary {
                    bincode::serialize(&channels)
                        .map_err(|e| format!("Binary serialization failed: {}", e))
                } else {
                    rmp_serde::to_vec(&channels)
                        .map_err(|e| format!("MessagePack serialization failed: {}", e))
                }
            }
        }
    }
    
    /// Export to JSON format
    fn export_json(&self, data: &ChannelExportData, pretty: bool) -> Result<Vec<u8>, String> {
        let result = if pretty {
            serde_json::to_vec_pretty(data)
        } else {
            serde_json::to_vec(data)
        };
        
        result.map_err(|e| format!("JSON serialization failed: {}", e))
            .and_then(|data| self.maybe_compress(data))
    }
    
    /// Export multiple channels to JSON
    fn export_multiple_json(
        &self,
        channels: HashMap<String, ChannelExportData>,
        pretty: bool
    ) -> Result<Vec<u8>, String> {
        let result = if pretty {
            serde_json::to_vec_pretty(&channels)
        } else {
            serde_json::to_vec(&channels)
        };
        
        result.map_err(|e| format!("JSON serialization failed: {}", e))
            .and_then(|data| self.maybe_compress(data))
    }
    
    /// Export to CSV format
    fn export_csv(&self, data: &ChannelExportData) -> Result<Vec<u8>, String> {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        
        // Write header
        wtr.write_record(&[
            "timestamp_ms",
            "value",
            "source",
            "unit",
            "quality",
            "tags"
        ]).map_err(|e| format!("CSV write failed: {}", e))?;
        
        // Write samples
        for sample in &data.samples {
            let value_str = format_sample_value(&sample);
            let source = sample.metadata.as_ref()
                .and_then(|m| m.source.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("");
            let unit = sample.metadata.as_ref()
                .and_then(|m| m.unit.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("");
            let quality = sample.metadata.as_ref()
                .and_then(|m| m.quality)
                .map(|q| q.to_string())
                .unwrap_or_default();
            let tags = sample.metadata.as_ref()
                .map(|m| m.tags.join(";"))
                .unwrap_or_default();
            
            wtr.write_record(&[
                &sample.timestamp_ms.to_string(),
                &value_str,
                source,
                unit,
                &quality,
                &tags,
            ]).map_err(|e| format!("CSV write failed: {}", e))?;
        }
        
        let data = wtr.into_inner()
            .map_err(|e| format!("CSV finalization failed: {}", e))?;
        
        self.maybe_compress(data)
    }
    
    /// Export multiple channels to CSV (combined)
    fn export_multiple_csv(
        &self,
        channels: HashMap<String, ChannelExportData>
    ) -> Result<Vec<u8>, String> {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        
        // Write header with channel column
        wtr.write_record(&[
            "channel",
            "timestamp_ms",
            "value",
            "source",
            "unit",
            "quality",
            "tags"
        ]).map_err(|e| format!("CSV write failed: {}", e))?;
        
        // Write samples from all channels
        for (channel_name, data) in channels {
            for sample in &data.samples {
                let value_str = format_sample_value(&sample);
                let source = sample.metadata.as_ref()
                    .and_then(|m| m.source.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let unit = sample.metadata.as_ref()
                    .and_then(|m| m.unit.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let quality = sample.metadata.as_ref()
                    .and_then(|m| m.quality)
                    .map(|q| q.to_string())
                    .unwrap_or_default();
                let tags = sample.metadata.as_ref()
                    .map(|m| m.tags.join(";"))
                    .unwrap_or_default();
                
                wtr.write_record(&[
                    &channel_name,
                    &sample.timestamp_ms.to_string(),
                    &value_str,
                    source,
                    unit,
                    &quality,
                    &tags,
                ]).map_err(|e| format!("CSV write failed: {}", e))?;
            }
        }
        
        let data = wtr.into_inner()
            .map_err(|e| format!("CSV finalization failed: {}", e))?;
        
        self.maybe_compress(data)
    }
    
    /// Export to binary format
    fn export_binary(&self, data: &ChannelExportData) -> Result<Vec<u8>, String> {
        bincode::serialize(data)
            .map_err(|e| format!("Binary serialization failed: {}", e))
            .and_then(|data| self.maybe_compress(data))
    }
    
    /// Export to MessagePack format
    fn export_messagepack(&self, data: &ChannelExportData) -> Result<Vec<u8>, String> {
        rmp_serde::to_vec(data)
            .map_err(|e| format!("MessagePack serialization failed: {}", e))
            .and_then(|data| self.maybe_compress(data))
    }
    
    /// Optionally compress data
    fn maybe_compress(&self, data: Vec<u8>) -> Result<Vec<u8>, String> {
        if self.compression {
            use flate2::write::GzEncoder;
            use flate2::Compression;
            
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&data)
                .map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish()
                .map_err(|e| format!("Compression finalization failed: {}", e))
        } else {
            Ok(data)
        }
    }
}

/// Format sample value as string for CSV
fn format_sample_value(sample: &TelemetrySample) -> String {
    use crate::telemetry::SampleValue;
    
    match &sample.value {
        SampleValue::Float32(v) => v.to_string(),
        SampleValue::Float64(v) => v.to_string(),
        SampleValue::Int32(v) => v.to_string(),
        SampleValue::UInt32(v) => v.to_string(),
        SampleValue::Bool(v) => v.to_string(),
        SampleValue::String(v) => v.clone(),
        SampleValue::Bytes(v) => format!("bytes[{}]", v.len()),
        SampleValue::Vector(v) => format!("[{}]", 
            v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),
    }
}

/// Import telemetry data from various formats
pub struct TelemetryImporter;

impl TelemetryImporter {
    /// Import from JSON
    pub fn import_json(data: &[u8]) -> Result<ChannelExportData, String> {
        // Check if compressed
        let data = if data.starts_with(&[0x1f, 0x8b]) {
            // GZIP magic number
            use flate2::read::GzDecoder;
            use std::io::Read;
            
            let mut decoder = GzDecoder::new(data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)
                .map_err(|e| format!("Decompression failed: {}", e))?;
            decompressed
        } else {
            data.to_vec()
        };
        
        serde_json::from_slice(&data)
            .map_err(|e| format!("JSON deserialization failed: {}", e))
    }
    
    /// Import from binary format
    pub fn import_binary(data: &[u8]) -> Result<ChannelExportData, String> {
        // Check if compressed
        let data = if data.starts_with(&[0x1f, 0x8b]) {
            use flate2::read::GzDecoder;
            use std::io::Read;
            
            let mut decoder = GzDecoder::new(data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)
                .map_err(|e| format!("Decompression failed: {}", e))?;
            decompressed
        } else {
            data.to_vec()
        };
        
        bincode::deserialize(&data)
            .map_err(|e| format!("Binary deserialization failed: {}", e))
    }
    
    /// Import from MessagePack format
    pub fn import_messagepack(data: &[u8]) -> Result<ChannelExportData, String> {
        // Check if compressed
        let data = if data.starts_with(&[0x1f, 0x8b]) {
            use flate2::read::GzDecoder;
            use std::io::Read;
            
            let mut decoder = GzDecoder::new(data);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)
                .map_err(|e| format!("Decompression failed: {}", e))?;
            decompressed
        } else {
            data.to_vec()
        };
        
        rmp_serde::from_slice(&data)
            .map_err(|e| format!("MessagePack deserialization failed: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::{TelemetrySample, ChannelConfig, ChannelStats};
    use std::time::SystemTime;
    
    fn create_test_export_data() -> ChannelExportData {
        let samples = vec![
            TelemetrySample::new_f32(1.0),
            TelemetrySample::new_f32(2.0),
            TelemetrySample::new_f32(3.0),
        ];
        
        ChannelExportData {
            config: ChannelConfig::default(),
            samples,
            stats: ChannelStats::new("test".to_string()),
            exported_at: SystemTime::now(),
        }
    }
    
    #[test]
    fn test_json_export() {
        let exporter = TelemetryExporter::new();
        let data = create_test_export_data();
        
        let result = exporter.export(&data, ExportFormat::Json);
        assert!(result.is_ok());
        
        let json = result.unwrap();
        assert!(!json.is_empty());
        
        // Verify it's valid JSON
        let parsed: Result<ChannelExportData, _> = serde_json::from_slice(&json);
        assert!(parsed.is_ok());
    }
    
    #[test]
    fn test_csv_export() {
        let exporter = TelemetryExporter::new();
        let data = create_test_export_data();
        
        let result = exporter.export(&data, ExportFormat::Csv);
        assert!(result.is_ok());
        
        let csv = result.unwrap();
        let csv_str = String::from_utf8(csv).unwrap();
        
        // Check header
        assert!(csv_str.contains("timestamp_ms"));
        assert!(csv_str.contains("value"));
        
        // Check we have data rows
        let lines: Vec<_> = csv_str.lines().collect();
        assert!(lines.len() >= 4); // Header + 3 samples
    }
    
    #[test]
    fn test_binary_export_import() {
        let exporter = TelemetryExporter::new();
        let data = create_test_export_data();
        
        // Export
        let result = exporter.export(&data, ExportFormat::Binary);
        assert!(result.is_ok());
        
        // Import
        let imported = TelemetryImporter::import_binary(&result.unwrap());
        assert!(imported.is_ok());
        
        let imported_data = imported.unwrap();
        assert_eq!(imported_data.samples.len(), 3);
    }
    
    #[test]
    fn test_compression() {
        let exporter = TelemetryExporter::new().with_compression(true);
        let data = create_test_export_data();
        
        let compressed = exporter.export(&data, ExportFormat::Json).unwrap();
        
        // Check it's compressed (starts with GZIP magic)
        assert_eq!(&compressed[0..2], &[0x1f, 0x8b]);
        
        // Import should handle decompression
        let imported = TelemetryImporter::import_json(&compressed);
        assert!(imported.is_ok());
    }
}