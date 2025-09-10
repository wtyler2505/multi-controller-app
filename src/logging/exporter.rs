// Log export functionality with multiple format support

use super::buffer::{LogBuffer, LogEntry, LogLevel};
use serde_json;
use std::io::{self, Write};
use chrono::{DateTime, Utc};

/// Log export formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogFormat {
    /// Plain text format
    Text,
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// HTML format for viewing in browser
    Html,
}

/// Log exporter
pub struct LogExporter {
    format: LogFormat,
}

impl LogExporter {
    /// Create a new exporter with specified format
    pub fn new(format: LogFormat) -> Self {
        Self { format }
    }
    
    /// Export a single log buffer
    pub fn export(&self, buffer: &LogBuffer) -> Result<Vec<u8>, io::Error> {
        match self.format {
            LogFormat::Text => self.export_text(buffer),
            LogFormat::Json => self.export_json(buffer),
            LogFormat::Csv => self.export_csv(buffer),
            LogFormat::Html => self.export_html(buffer),
        }
    }
    
    /// Export multiple buffers with labels
    pub fn export_multiple(&self, buffers: &[(&str, &LogBuffer)]) -> Result<Vec<u8>, io::Error> {
        match self.format {
            LogFormat::Text => self.export_multiple_text(buffers),
            LogFormat::Json => self.export_multiple_json(buffers),
            LogFormat::Csv => self.export_multiple_csv(buffers),
            LogFormat::Html => self.export_multiple_html(buffers),
        }
    }
    
    /// Export as plain text
    fn export_text(&self, buffer: &LogBuffer) -> Result<Vec<u8>, io::Error> {
        let mut output = Vec::new();
        
        // Write header
        writeln!(&mut output, "=== LOG EXPORT ===")?;
        writeln!(&mut output, "Total Entries: {}", buffer.len())?;
        writeln!(&mut output, "Total Logged: {}", buffer.total_logged())?;
        writeln!(&mut output, "Rolled Out: {}", buffer.rolled_out())?;
        writeln!(&mut output, "Memory Usage: {} bytes", buffer.memory_usage())?;
        writeln!(&mut output, "==================\n")?;
        
        // Write entries
        for entry in buffer.entries() {
            writeln!(&mut output, "{}", entry.format_line())?;
        }
        
        Ok(output)
    }
    
    /// Export multiple buffers as text
    fn export_multiple_text(&self, buffers: &[(&str, &LogBuffer)]) -> Result<Vec<u8>, io::Error> {
        let mut output = Vec::new();
        
        for (name, buffer) in buffers {
            writeln!(&mut output, "\n=== {} LOGS ===", name.to_uppercase())?;
            let buffer_export = self.export_text(buffer)?;
            output.write_all(&buffer_export)?;
        }
        
        Ok(output)
    }
    
    /// Export as JSON
    fn export_json(&self, buffer: &LogBuffer) -> Result<Vec<u8>, io::Error> {
        let export_data = serde_json::json!({
            "metadata": {
                "exported_at": Utc::now().to_rfc3339(),
                "total_entries": buffer.len(),
                "total_logged": buffer.total_logged(),
                "rolled_out": buffer.rolled_out(),
                "memory_usage": buffer.memory_usage(),
            },
            "entries": buffer.entries().iter().collect::<Vec<_>>(),
        });
        
        serde_json::to_vec_pretty(&export_data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
    
    /// Export multiple buffers as JSON
    fn export_multiple_json(&self, buffers: &[(&str, &LogBuffer)]) -> Result<Vec<u8>, io::Error> {
        let mut all_buffers = serde_json::Map::new();
        
        for (name, buffer) in buffers {
            let buffer_data = serde_json::json!({
                "metadata": {
                    "total_entries": buffer.len(),
                    "total_logged": buffer.total_logged(),
                    "rolled_out": buffer.rolled_out(),
                    "memory_usage": buffer.memory_usage(),
                },
                "entries": buffer.entries().iter().collect::<Vec<_>>(),
            });
            all_buffers.insert(name.to_string(), buffer_data);
        }
        
        let export_data = serde_json::json!({
            "exported_at": Utc::now().to_rfc3339(),
            "buffers": all_buffers,
        });
        
        serde_json::to_vec_pretty(&export_data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
    
    /// Export as CSV
    fn export_csv(&self, buffer: &LogBuffer) -> Result<Vec<u8>, io::Error> {
        let mut output = Vec::new();
        
        // Write CSV header
        writeln!(&mut output, "Timestamp,Level,Source,Message,Thread,DataSize")?;
        
        // Write entries
        for entry in buffer.entries() {
            let timestamp = DateTime::<Utc>::from_timestamp_millis(entry.timestamp as i64)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "Invalid".to_string());
            
            let data_size = entry.data.as_ref().map_or(0, |d| d.len());
            
            // Escape CSV fields
            let message = entry.message.replace('"', "\"\"");
            let source = entry.source.replace('"', "\"\"");
            
            writeln!(
                &mut output,
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",{}",
                timestamp,
                entry.level.as_str(),
                source,
                message,
                entry.thread_id,
                data_size
            )?;
        }
        
        Ok(output)
    }
    
    /// Export multiple buffers as CSV
    fn export_multiple_csv(&self, buffers: &[(&str, &LogBuffer)]) -> Result<Vec<u8>, io::Error> {
        let mut output = Vec::new();
        
        // Write CSV header with buffer column
        writeln!(&mut output, "Buffer,Timestamp,Level,Source,Message,Thread,DataSize")?;
        
        for (name, buffer) in buffers {
            for entry in buffer.entries() {
                let timestamp = DateTime::<Utc>::from_timestamp_millis(entry.timestamp as i64)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_else(|| "Invalid".to_string());
                
                let data_size = entry.data.as_ref().map_or(0, |d| d.len());
                
                // Escape CSV fields
                let message = entry.message.replace('"', "\"\"");
                let source = entry.source.replace('"', "\"\"");
                
                writeln!(
                    &mut output,
                    "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",{}",
                    name,
                    timestamp,
                    entry.level.as_str(),
                    source,
                    message,
                    entry.thread_id,
                    data_size
                )?;
            }
        }
        
        Ok(output)
    }
    
    /// Export as HTML
    fn export_html(&self, buffer: &LogBuffer) -> Result<Vec<u8>, io::Error> {
        let mut output = Vec::new();
        
        // Write HTML header
        writeln!(&mut output, "<!DOCTYPE html>")?;
        writeln!(&mut output, "<html><head>")?;
        writeln!(&mut output, "<title>Log Export</title>")?;
        writeln!(&mut output, "<style>")?;
        writeln!(&mut output, "{}", HTML_STYLES)?;
        writeln!(&mut output, "</style></head><body>")?;
        
        // Write metadata
        writeln!(&mut output, "<div class='metadata'>")?;
        writeln!(&mut output, "<h1>Log Export</h1>")?;
        writeln!(&mut output, "<p>Exported: {}</p>", Utc::now().to_rfc3339())?;
        writeln!(&mut output, "<p>Total Entries: {}</p>", buffer.len())?;
        writeln!(&mut output, "<p>Total Logged: {}</p>", buffer.total_logged())?;
        writeln!(&mut output, "<p>Rolled Out: {}</p>", buffer.rolled_out())?;
        writeln!(&mut output, "</div>")?;
        
        // Write log table
        writeln!(&mut output, "<table>")?;
        writeln!(&mut output, "<thead><tr>")?;
        writeln!(&mut output, "<th>Timestamp</th>")?;
        writeln!(&mut output, "<th>Level</th>")?;
        writeln!(&mut output, "<th>Source</th>")?;
        writeln!(&mut output, "<th>Message</th>")?;
        writeln!(&mut output, "<th>Data</th>")?;
        writeln!(&mut output, "</tr></thead><tbody>")?;
        
        for entry in buffer.entries() {
            let level_class = match entry.level {
                LogLevel::Trace | LogLevel::Debug => "debug",
                LogLevel::Info => "info",
                LogLevel::Warning => "warning",
                LogLevel::Error => "error",
                LogLevel::Fatal => "fatal",
            };
            
            let timestamp = DateTime::<Utc>::from_timestamp_millis(entry.timestamp as i64)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string())
                .unwrap_or_else(|| "Invalid".to_string());
            
            writeln!(&mut output, "<tr class='{}'>", level_class)?;
            writeln!(&mut output, "<td>{}</td>", timestamp)?;
            writeln!(&mut output, "<td>{}</td>", entry.level.as_str())?;
            writeln!(&mut output, "<td>{}</td>", html_escape(&entry.source))?;
            writeln!(&mut output, "<td>{}</td>", html_escape(&entry.message))?;
            writeln!(&mut output, "<td>{}</td>", 
                entry.data.as_ref().map_or("".to_string(), |d| format!("{}B", d.len()))
            )?;
            writeln!(&mut output, "</tr>")?;
        }
        
        writeln!(&mut output, "</tbody></table>")?;
        writeln!(&mut output, "</body></html>")?;
        
        Ok(output)
    }
    
    /// Export multiple buffers as HTML
    fn export_multiple_html(&self, buffers: &[(&str, &LogBuffer)]) -> Result<Vec<u8>, io::Error> {
        let mut output = Vec::new();
        
        // Write HTML header
        writeln!(&mut output, "<!DOCTYPE html>")?;
        writeln!(&mut output, "<html><head>")?;
        writeln!(&mut output, "<title>Multi-Buffer Log Export</title>")?;
        writeln!(&mut output, "<style>")?;
        writeln!(&mut output, "{}", HTML_STYLES)?;
        writeln!(&mut output, "</style></head><body>")?;
        
        writeln!(&mut output, "<h1>Multi-Buffer Log Export</h1>")?;
        writeln!(&mut output, "<p>Exported: {}</p>", Utc::now().to_rfc3339())?;
        
        for (name, buffer) in buffers {
            writeln!(&mut output, "<div class='buffer-section'>")?;
            writeln!(&mut output, "<h2>{}</h2>", name.to_uppercase())?;
            
            // Write metadata
            writeln!(&mut output, "<div class='metadata'>")?;
            writeln!(&mut output, "<p>Entries: {} | Total: {} | Rolled Out: {}</p>",
                buffer.len(), buffer.total_logged(), buffer.rolled_out())?;
            writeln!(&mut output, "</div>")?;
            
            // Write log table
            writeln!(&mut output, "<table>")?;
            writeln!(&mut output, "<thead><tr>")?;
            writeln!(&mut output, "<th>Timestamp</th>")?;
            writeln!(&mut output, "<th>Level</th>")?;
            writeln!(&mut output, "<th>Source</th>")?;
            writeln!(&mut output, "<th>Message</th>")?;
            writeln!(&mut output, "</tr></thead><tbody>")?;
            
            for entry in buffer.entries() {
                let level_class = match entry.level {
                    LogLevel::Trace | LogLevel::Debug => "debug",
                    LogLevel::Info => "info",
                    LogLevel::Warning => "warning",
                    LogLevel::Error => "error",
                    LogLevel::Fatal => "fatal",
                };
                
                let timestamp = DateTime::<Utc>::from_timestamp_millis(entry.timestamp as i64)
                    .map(|dt| dt.format("%H:%M:%S%.3f").to_string())
                    .unwrap_or_else(|| "Invalid".to_string());
                
                writeln!(&mut output, "<tr class='{}'>", level_class)?;
                writeln!(&mut output, "<td>{}</td>", timestamp)?;
                writeln!(&mut output, "<td>{}</td>", entry.level.as_str())?;
                writeln!(&mut output, "<td>{}</td>", html_escape(&entry.source))?;
                writeln!(&mut output, "<td>{}</td>", html_escape(&entry.message))?;
                writeln!(&mut output, "</tr>")?;
            }
            
            writeln!(&mut output, "</tbody></table>")?;
            writeln!(&mut output, "</div>")?;
        }
        
        writeln!(&mut output, "</body></html>")?;
        
        Ok(output)
    }
}

/// HTML escape string
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// CSS styles for HTML export
const HTML_STYLES: &str = r#"
body { 
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
    margin: 20px;
    background: #f5f5f5;
}
h1 { color: #333; }
h2 { 
    color: #555; 
    border-bottom: 2px solid #ddd;
    padding-bottom: 5px;
}
.metadata { 
    background: white;
    padding: 10px;
    border-radius: 5px;
    margin-bottom: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}
.buffer-section {
    background: white;
    padding: 20px;
    border-radius: 5px;
    margin-bottom: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}
table { 
    width: 100%; 
    border-collapse: collapse;
    background: white;
}
th { 
    background: #4CAF50; 
    color: white;
    padding: 10px;
    text-align: left;
    position: sticky;
    top: 0;
}
td { 
    padding: 8px;
    border-bottom: 1px solid #ddd;
}
tr:hover { background: #f9f9f9; }
.debug { color: #888; }
.info { color: #333; }
.warning { color: #ff9800; background: #fff3e0; }
.error { color: #f44336; background: #ffebee; }
.fatal { color: white; background: #d32f2f; font-weight: bold; }
"#;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_export() {
        let mut buffer = LogBuffer::new(10);
        buffer.log(LogLevel::Info, "Test", "Test message".to_string(), None);
        
        let exporter = LogExporter::new(LogFormat::Text);
        let result = exporter.export(&buffer).unwrap();
        let text = String::from_utf8(result).unwrap();
        
        assert!(text.contains("Test message"));
        assert!(text.contains("INFO"));
    }
    
    #[test]
    fn test_json_export() {
        let mut buffer = LogBuffer::new(10);
        buffer.log(LogLevel::Warning, "Component", "Warning message".to_string(), None);
        
        let exporter = LogExporter::new(LogFormat::Json);
        let result = exporter.export(&buffer).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&result).unwrap();
        
        assert_eq!(json["metadata"]["total_entries"], 1);
        assert_eq!(json["entries"][0]["level"], "Warning");
    }
    
    #[test]
    fn test_csv_export() {
        let mut buffer = LogBuffer::new(10);
        buffer.log(LogLevel::Error, "Module", "Error occurred".to_string(), Some(vec![1, 2, 3]));
        
        let exporter = LogExporter::new(LogFormat::Csv);
        let result = exporter.export(&buffer).unwrap();
        let csv = String::from_utf8(result).unwrap();
        
        assert!(csv.contains("ERROR"));
        assert!(csv.contains("Module"));
        assert!(csv.contains("Error occurred"));
        assert!(csv.contains("3")); // Data size
    }
    
    #[test]
    fn test_html_export() {
        let mut buffer = LogBuffer::new(10);
        buffer.log(LogLevel::Fatal, "Critical", "System failure".to_string(), None);
        
        let exporter = LogExporter::new(LogFormat::Html);
        let result = exporter.export(&buffer).unwrap();
        let html = String::from_utf8(result).unwrap();
        
        assert!(html.contains("<table>"));
        assert!(html.contains("FATAL"));
        assert!(html.contains("System failure"));
        assert!(html.contains("class='fatal'"));
    }
    
    #[test]
    fn test_multiple_buffer_export() {
        let mut buffer1 = LogBuffer::new(10);
        buffer1.log(LogLevel::Info, "Buffer1", "Message 1".to_string(), None);
        
        let mut buffer2 = LogBuffer::new(10);
        buffer2.log(LogLevel::Error, "Buffer2", "Message 2".to_string(), None);
        
        let exporter = LogExporter::new(LogFormat::Json);
        let result = exporter.export_multiple(&[
            ("first", &buffer1),
            ("second", &buffer2),
        ]).unwrap();
        
        let json: serde_json::Value = serde_json::from_slice(&result).unwrap();
        assert!(json["buffers"]["first"].is_object());
        assert!(json["buffers"]["second"].is_object());
    }
}