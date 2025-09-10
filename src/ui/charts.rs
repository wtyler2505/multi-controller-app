use std::collections::VecDeque;
use std::sync::Arc;
use egui::{Ui, Color32, Stroke, Vec2};
use egui_plot::{Plot, PlotPoints, Line, Legend, Corner, GridMark, GridInput};
use egui_plot::{MarkerShape, Points, PlotBounds, AxisHints};
use parking_lot::RwLock;
use crate::telemetry::{TelemetryChannel, TelemetrySample, SampleValue};

/// Chart type for different visualization styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChartType {
    Line,
    Scatter,
    Area,
    StepLine,
}

/// Configuration for telemetry charts
#[derive(Debug, Clone)]
pub struct ChartConfig {
    /// Chart type
    pub chart_type: ChartType,
    
    /// Maximum number of points to display
    pub max_points: usize,
    
    /// Time window in seconds (0 = show all)
    pub time_window_seconds: f64,
    
    /// Enable auto-scaling for Y axis
    pub auto_scale_y: bool,
    
    /// Fixed Y axis range (if not auto-scaling)
    pub y_min: f64,
    pub y_max: f64,
    
    /// Show grid lines
    pub show_grid: bool,
    
    /// Show legend
    pub show_legend: bool,
    
    /// Line/point color
    pub color: Color32,
    
    /// Line width
    pub line_width: f32,
    
    /// Enable antialiasing
    pub antialiased: bool,
    
    /// Show tooltips on hover
    pub show_tooltips: bool,
    
    /// Update rate limiting (ms between updates)
    pub update_interval_ms: u32,
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self {
            chart_type: ChartType::Line,
            max_points: 300,  // Good balance for 30 FPS
            time_window_seconds: 10.0,  // Show last 10 seconds
            auto_scale_y: true,
            y_min: 0.0,
            y_max: 100.0,
            show_grid: true,
            show_legend: true,
            color: Color32::from_rgb(0, 200, 255),  // Cyan
            line_width: 1.5,
            antialiased: true,
            show_tooltips: true,
            update_interval_ms: 33,  // ~30 FPS
        }
    }
}

/// Real-time telemetry chart widget
pub struct TelemetryChart {
    /// Chart configuration
    config: ChartConfig,
    
    /// Chart name/label
    name: String,
    
    /// Data buffer for rendering (decimated)
    render_buffer: Arc<RwLock<Vec<[f64; 2]>>>,
    
    /// Last update timestamp
    last_update: std::time::Instant,
    
    /// Statistics
    stats: ChartStats,
}

#[derive(Debug)]
struct ChartStats {
    total_samples: u64,
    dropped_samples: u64,
    current_fps: f32,
    last_fps_update: std::time::Instant,
    frame_count: u32,
}

impl Default for ChartStats {
    fn default() -> Self {
        Self {
            total_samples: 0,
            dropped_samples: 0,
            current_fps: 0.0,
            last_fps_update: std::time::Instant::now(),
            frame_count: 0,
        }
    }
}

impl TelemetryChart {
    /// Create a new telemetry chart
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            config: ChartConfig::default(),
            name: name.into(),
            render_buffer: Arc::new(RwLock::new(Vec::with_capacity(300))),
            last_update: std::time::Instant::now(),
            stats: ChartStats::default(),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(name: impl Into<String>, config: ChartConfig) -> Self {
        Self {
            config,
            name: name.into(),
            render_buffer: Arc::new(RwLock::new(Vec::with_capacity(300))),
            last_update: std::time::Instant::now(),
            stats: ChartStats::default(),
        }
    }
    
    /// Update configuration
    pub fn set_config(&mut self, config: ChartConfig) {
        self.config = config;
    }
    
    /// Update chart data from telemetry channel
    pub fn update_from_channel(&mut self, channel: &TelemetryChannel) {
        // Check if we should update (rate limiting)
        let now = std::time::Instant::now();
        let elapsed_ms = now.duration_since(self.last_update).as_millis() as u32;
        
        if elapsed_ms < self.config.update_interval_ms {
            return;  // Skip update to maintain frame rate
        }
        
        // Get decimated data from channel
        let chart_data = channel.chart_data(self.config.max_points);
        
        // Convert to plot format
        let mut buffer = self.render_buffer.write();
        buffer.clear();
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        for (timestamp, value) in chart_data {
            // Convert timestamp from milliseconds to seconds
            let timestamp_secs = timestamp as f64 / 1000.0;
            
            // Apply time window filter if configured
            if self.config.time_window_seconds > 0.0 {
                let age = current_time - timestamp_secs;
                if age > self.config.time_window_seconds {
                    continue;
                }
            }
            
            buffer.push([timestamp_secs, value as f64]);
        }
        
        // Update statistics
        self.stats.total_samples = channel.get_stats().total_samples;
        self.stats.dropped_samples = channel.get_stats().samples_dropped;
        
        // Update FPS counter
        self.stats.frame_count += 1;
        let fps_elapsed = now.duration_since(self.stats.last_fps_update).as_secs_f32();
        if fps_elapsed >= 1.0 {
            self.stats.current_fps = self.stats.frame_count as f32 / fps_elapsed;
            self.stats.frame_count = 0;
            self.stats.last_fps_update = now;
        }
        
        self.last_update = now;
    }
    
    /// Update chart with raw data points
    pub fn update_with_data(&mut self, data: &[(f64, f64)]) {
        let now = std::time::Instant::now();
        let elapsed_ms = now.duration_since(self.last_update).as_millis() as u32;
        
        if elapsed_ms < self.config.update_interval_ms {
            return;
        }
        
        let mut buffer = self.render_buffer.write();
        buffer.clear();
        
        // Apply decimation if needed
        let step = if data.len() > self.config.max_points {
            data.len() / self.config.max_points
        } else {
            1
        };
        
        for (i, &(x, y)) in data.iter().enumerate() {
            if i % step == 0 {
                buffer.push([x, y]);
            }
        }
        
        self.last_update = now;
    }
    
    /// Render the chart
    pub fn show(&mut self, ui: &mut Ui) -> egui::Response {
        // Get current data
        let data = self.render_buffer.read().clone();
        
        // Create plot
        let plot = Plot::new(&self.name)
            .height(200.0)
            .auto_bounds([true, self.config.auto_scale_y].into())
            .show_grid(self.config.show_grid)
            .allow_drag(true)
            .allow_zoom(true)
            .allow_scroll(true)
            .label_formatter(|name, value| {
                if self.config.show_tooltips {
                    format!("{}: x={:.2}, y={:.2}", name, value.x, value.y)
                } else {
                    String::new()
                }
            });
        
        // Add legend if configured
        let plot = if self.config.show_legend {
            plot.legend(Legend::default().position(Corner::RightTop))
        } else {
            plot
        };
        
        // Note: In egui_plot 0.29, bounds are set differently
        // We'll use auto_bounds for now and can add manual bounds later if needed
        let plot = plot;
        
        // Render the plot
        let response = plot.show(ui, |plot_ui| {
            // Create line/scatter based on chart type
            match self.config.chart_type {
                ChartType::Line => {
                    let points: PlotPoints = data.into();
                    let line = Line::new(points)
                        .name(&self.name)
                        .color(self.config.color)
                        .width(self.config.line_width);
                    plot_ui.line(line);
                }
                ChartType::Scatter => {
                    let points: PlotPoints = data.into();
                    let scatter = Points::new(points)
                        .name(&self.name)
                        .color(self.config.color)
                        .radius(self.config.line_width * 2.0)
                        .shape(MarkerShape::Circle);
                    plot_ui.points(scatter);
                }
                ChartType::Area => {
                    let points: PlotPoints = data.into();
                    let line = Line::new(points)
                        .name(&self.name)
                        .color(self.config.color)
                        .width(self.config.line_width)
                        .fill(0.0);  // Fill to y=0
                    plot_ui.line(line);
                }
                ChartType::StepLine => {
                    // For step line, we need to create the stepped points
                    let stepped_points = self.create_stepped_points(&data);
                    let line = Line::new(stepped_points)
                        .name(&self.name)
                        .color(self.config.color)
                        .width(self.config.line_width);
                    plot_ui.line(line);
                }
            }
        }).response;
        
        response
    }
    
    /// Create stepped points for step line chart
    fn create_stepped_points(&self, data: &[[f64; 2]]) -> PlotPoints {
        let mut stepped = Vec::with_capacity(data.len() * 2);
        
        for i in 0..data.len() {
            if i > 0 {
                // Add horizontal step
                stepped.push([data[i][0], data[i-1][1]]);
            }
            stepped.push([data[i][0], data[i][1]]);
        }
        
        stepped.into()
    }
    
    /// Get current statistics
    pub fn stats(&self) -> ChartStatsSummary {
        ChartStatsSummary {
            total_samples: self.stats.total_samples,
            dropped_samples: self.stats.dropped_samples,
            current_fps: self.stats.current_fps,
            buffer_size: self.render_buffer.read().len(),
        }
    }
}

/// Statistics summary for external use
#[derive(Debug, Clone)]
pub struct ChartStatsSummary {
    pub total_samples: u64,
    pub dropped_samples: u64,
    pub current_fps: f32,
    pub buffer_size: usize,
}

/// Multi-chart container for side-by-side or stacked charts
pub struct MultiChart {
    charts: Vec<TelemetryChart>,
    layout: ChartLayout,
}

#[derive(Debug, Clone, Copy)]
pub enum ChartLayout {
    Vertical,    // Stack charts vertically
    Horizontal,  // Place charts side by side
    Grid(usize), // Grid with N columns
}

impl MultiChart {
    /// Create a new multi-chart container
    pub fn new(layout: ChartLayout) -> Self {
        Self {
            charts: Vec::new(),
            layout,
        }
    }
    
    /// Add a chart
    pub fn add_chart(&mut self, chart: TelemetryChart) {
        self.charts.push(chart);
    }
    
    /// Update all charts from their respective channels
    pub fn update_all(&mut self, channels: &[&TelemetryChannel]) {
        for (chart, channel) in self.charts.iter_mut().zip(channels.iter()) {
            chart.update_from_channel(channel);
        }
    }
    
    /// Render all charts
    pub fn show(&mut self, ui: &mut Ui) {
        match self.layout {
            ChartLayout::Vertical => {
                for chart in &mut self.charts {
                    chart.show(ui);
                    ui.separator();
                }
            }
            ChartLayout::Horizontal => {
                ui.horizontal(|ui| {
                    for chart in &mut self.charts {
                        ui.vertical(|ui| {
                            chart.show(ui);
                        });
                    }
                });
            }
            ChartLayout::Grid(cols) => {
                let chunks: Vec<_> = self.charts.chunks_mut(cols).collect();
                for row in chunks {
                    ui.horizontal(|ui| {
                        for chart in row {
                            ui.vertical(|ui| {
                                chart.show(ui);
                            });
                        }
                    });
                }
            }
        }
    }
    
    /// Get all chart statistics
    pub fn all_stats(&self) -> Vec<ChartStatsSummary> {
        self.charts.iter().map(|c| c.stats()).collect()
    }
}

/// Helper function to format time axis labels
pub fn format_time_axis(value: f64) -> String {
    let seconds = value % 60.0;
    let minutes = (value / 60.0) as i64 % 60;
    let hours = (value / 3600.0) as i64;
    
    if hours > 0 {
        format!("{}:{:02}:{:02.1}", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}:{:04.1}", minutes, seconds)
    } else {
        format!("{:.1}s", seconds)
    }
}

/// Helper to create a simple test chart with sine wave data
pub fn create_test_chart(name: &str) -> TelemetryChart {
    let mut chart = TelemetryChart::new(name);
    
    // Generate test data
    let mut data = Vec::new();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    
    for i in 0..300 {
        let t = i as f64 * 0.1;
        let value = 50.0 + 30.0 * (t * 0.5).sin();
        data.push((now - 30.0 + t, value));
    }
    
    chart.update_with_data(&data);
    chart
}