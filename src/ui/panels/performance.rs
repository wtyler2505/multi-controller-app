//! Enterprise-grade real-time performance dashboard
//! 
//! This module provides comprehensive performance visualization with
//! real-time charts, status indicators, and alert displays.

use egui::{self, Context, Ui, Color32, RichText, Vec2, Stroke, FontId, FontFamily};
use egui_plot::{Plot, PlotPoints, Line, Legend, Corner, Text as PlotText};
use crate::performance::{
    monitor::{PerformanceMonitor, PerformanceAlert, AlertSeverity},
    metrics::{SystemMetrics, ProcessMetrics, ResourceUsage},
    startup::{StartupReport, StartupPhase, tracker},
    profiler::{profiler, FlameGraph},
};
use std::collections::VecDeque;
use std::sync::Arc;
use parking_lot::RwLock;
use std::time::{Duration, Instant};

/// Maximum number of data points in charts
const MAX_CHART_POINTS: usize = 300;

/// Chart update interval (30 FPS)
const CHART_UPDATE_INTERVAL: Duration = Duration::from_millis(33);

/// Performance dashboard panel
pub struct PerformancePanel {
    monitor: Arc<PerformanceMonitor>,
    
    // Chart data buffers (ring buffers for efficiency)
    cpu_history: Arc<RwLock<VecDeque<(f64, f64)>>>,
    memory_history: Arc<RwLock<VecDeque<(f64, f64)>>>,
    thread_history: Arc<RwLock<VecDeque<(f64, f64)>>>,
    
    // Alert history
    recent_alerts: Arc<RwLock<VecDeque<(Instant, PerformanceAlert)>>>,
    max_alerts: usize,
    
    // Update tracking
    last_update: Instant,
    time_offset: f64,
    
    // UI state
    show_advanced: bool,
    show_flame_graph: bool,
    show_startup_report: bool,
    
    // Cached data
    cached_startup_report: Option<StartupReport>,
    cached_flame_graph: Option<FlameGraph>,
}

impl PerformancePanel {
    /// Create new performance panel
    pub fn new(monitor: Arc<PerformanceMonitor>) -> Self {
        // Register alert callback
        let alerts = Arc::new(RwLock::new(VecDeque::new()));
        let alerts_clone = alerts.clone();
        
        let monitor_clone = monitor.clone();
        tokio::spawn(async move {
            monitor_clone.register_alert_callback(move |alert| {
                let mut alerts = alerts_clone.write();
                alerts.push_back((Instant::now(), alert));
                
                // Keep only recent alerts
                while alerts.len() > 50 {
                    alerts.pop_front();
                }
            }).await;
        });
        
        Self {
            monitor,
            cpu_history: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_CHART_POINTS))),
            memory_history: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_CHART_POINTS))),
            thread_history: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_CHART_POINTS))),
            recent_alerts: alerts,
            max_alerts: 50,
            last_update: Instant::now(),
            time_offset: 0.0,
            show_advanced: false,
            show_flame_graph: false,
            show_startup_report: false,
            cached_startup_report: None,
            cached_flame_graph: None,
        }
    }
    
    /// Update dashboard data
    pub fn update(&mut self) {
        // Check if update is needed (30 FPS rate limiting)
        if self.last_update.elapsed() < CHART_UPDATE_INTERVAL {
            return;
        }
        
        // Get current metrics asynchronously
        let monitor = self.monitor.clone();
        let cpu_history = self.cpu_history.clone();
        let memory_history = self.memory_history.clone();
        let thread_history = self.thread_history.clone();
        let time_offset = self.time_offset;
        
        tokio::spawn(async move {
            let (sys_metrics, proc_metrics) = monitor.current_metrics().await;
            
            // Update CPU history
            {
                let mut history = cpu_history.write();
                history.push_back((time_offset, sys_metrics.cpu_usage as f64));
                while history.len() > MAX_CHART_POINTS {
                    history.pop_front();
                }
            }
            
            // Update memory history
            {
                let mut history = memory_history.write();
                let memory_mb = proc_metrics.memory_mb();
                history.push_back((time_offset, memory_mb));
                while history.len() > MAX_CHART_POINTS {
                    history.pop_front();
                }
            }
            
            // Update thread history
            {
                let mut history = thread_history.write();
                history.push_back((time_offset, proc_metrics.thread_count as f64));
                while history.len() > MAX_CHART_POINTS {
                    history.pop_front();
                }
            }
        });
        
        // Update time tracking
        self.time_offset += self.last_update.elapsed().as_secs_f64();
        self.last_update = Instant::now();
    }
    
    /// Render the performance dashboard
    pub fn render(&mut self, ctx: &Context, ui: &mut Ui) {
        // Update data
        self.update();
        
        // Main layout
        ui.heading("🎯 Performance Monitor");
        ui.separator();
        
        // Control buttons
        ui.horizontal(|ui| {
            if ui.button("📊 Advanced Metrics").clicked() {
                self.show_advanced = !self.show_advanced;
            }
            
            if ui.button("🔥 Flame Graph").clicked() {
                self.show_flame_graph = !self.show_flame_graph;
                if self.show_flame_graph {
                    self.cached_flame_graph = Some(profiler().generate_flame_graph());
                }
            }
            
            if ui.button("🚀 Startup Report").clicked() {
                self.show_startup_report = !self.show_startup_report;
                if self.show_startup_report {
                    self.cached_startup_report = Some(tracker().generate_report());
                }
            }
            
            ui.separator();
            
            if ui.button("🔄 Reset Charts").clicked() {
                self.cpu_history.write().clear();
                self.memory_history.write().clear();
                self.thread_history.write().clear();
                self.time_offset = 0.0;
            }
        });
        
        ui.separator();
        
        // Main dashboard grid
        ui.columns(2, |columns| {
            // Left column - Charts
            columns[0].group(|ui| {
                ui.label(RichText::new("Real-time Metrics").size(16.0).strong());
                self.render_charts(ui);
            });
            
            // Right column - Status and alerts
            columns[1].group(|ui| {
                ui.label(RichText::new("System Status").size(16.0).strong());
                self.render_status(ui);
                
                ui.separator();
                
                ui.label(RichText::new("Recent Alerts").size(16.0).strong());
                self.render_alerts(ui);
            });
        });
        
        // Advanced sections
        if self.show_advanced {
            ui.separator();
            self.render_advanced_metrics(ui);
        }
        
        if self.show_flame_graph {
            ui.separator();
            self.render_flame_graph(ui);
        }
        
        if self.show_startup_report {
            ui.separator();
            self.render_startup_report(ui);
        }
        
        // Request repaint for continuous updates
        ctx.request_repaint_after(CHART_UPDATE_INTERVAL);
    }
    
    /// Render performance charts
    fn render_charts(&self, ui: &mut Ui) {
        let chart_height = 150.0;
        
        // CPU Usage Chart
        ui.group(|ui| {
            ui.label("CPU Usage (%)");
            
            let cpu_data: Vec<[f64; 2]> = self.cpu_history.read()
                .iter()
                .map(|(t, v)| [*t, *v])
                .collect();
            
            Plot::new("cpu_plot")
                .height(chart_height)
                .legend(Legend::default().position(Corner::RightTop))
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new(PlotPoints::from(cpu_data))
                            .color(Color32::from_rgb(255, 128, 0))
                            .name("CPU %")
                    );
                    
                    // Add threshold line at 2% (idle budget)
                    plot_ui.hline(
                        egui_plot::HLine::new(2.0)
                            .color(Color32::from_rgb(255, 0, 0))
                            .width(1.0)
                            .name("Idle Budget")
                    );
                });
        });
        
        // Memory Usage Chart
        ui.group(|ui| {
            ui.label("Memory Usage (MB)");
            
            let memory_data: Vec<[f64; 2]> = self.memory_history.read()
                .iter()
                .map(|(t, v)| [*t, *v])
                .collect();
            
            Plot::new("memory_plot")
                .height(chart_height)
                .legend(Legend::default().position(Corner::RightTop))
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new(PlotPoints::from(memory_data))
                            .color(Color32::from_rgb(0, 128, 255))
                            .name("Memory MB")
                    );
                    
                    // Add threshold line at 150 MB
                    plot_ui.hline(
                        egui_plot::HLine::new(150.0)
                            .color(Color32::from_rgb(255, 0, 0))
                            .width(1.0)
                            .name("Memory Budget")
                    );
                });
        });
        
        // Thread Count Chart
        ui.group(|ui| {
            ui.label("Thread Count");
            
            let thread_data: Vec<[f64; 2]> = self.thread_history.read()
                .iter()
                .map(|(t, v)| [*t, *v])
                .collect();
            
            Plot::new("thread_plot")
                .height(chart_height)
                .legend(Legend::default().position(Corner::RightTop))
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new(PlotPoints::from(thread_data))
                            .color(Color32::from_rgb(128, 255, 128))
                            .name("Threads")
                    );
                });
        });
    }
    
    /// Render system status
    fn render_status(&self, ui: &mut Ui) {
        // Get current metrics synchronously for display
        let monitor = self.monitor.clone();
        
        // We'll use cached values for now
        ui.vertical(|ui| {
            // Status indicators with color coding
            let cpu_usage = self.cpu_history.read()
                .back()
                .map(|(_, v)| *v as f32)
                .unwrap_or(0.0);
            
            let memory_mb = self.memory_history.read()
                .back()
                .map(|(_, v)| *v as f32)
                .unwrap_or(0.0);
            
            let threads = self.thread_history.read()
                .back()
                .map(|(_, v)| *v as u32)
                .unwrap_or(0);
            
            // CPU Status
            let cpu_color = if cpu_usage <= 2.0 {
                Color32::GREEN
            } else if cpu_usage <= 10.0 {
                Color32::YELLOW
            } else {
                Color32::RED
            };
            
            ui.horizontal(|ui| {
                ui.label("CPU:");
                ui.label(RichText::new(format!("{:.1}%", cpu_usage)).color(cpu_color));
                ui.label(format!("/ 2.0% idle"));
            });
            
            // Memory Status
            let memory_color = if memory_mb <= 150.0 {
                Color32::GREEN
            } else if memory_mb <= 200.0 {
                Color32::YELLOW
            } else {
                Color32::RED
            };
            
            ui.horizontal(|ui| {
                ui.label("Memory:");
                ui.label(RichText::new(format!("{:.1} MB", memory_mb)).color(memory_color));
                ui.label(format!("/ 150 MB"));
            });
            
            // Thread Status
            ui.horizontal(|ui| {
                ui.label("Threads:");
                ui.label(format!("{}", threads));
            });
            
            ui.separator();
            
            // Performance Score
            let score = calculate_performance_score(cpu_usage, memory_mb);
            let score_color = if score >= 90.0 {
                Color32::GREEN
            } else if score >= 70.0 {
                Color32::YELLOW
            } else {
                Color32::RED
            };
            
            ui.label(RichText::new(format!("Performance Score: {:.0}%", score))
                .size(18.0)
                .color(score_color)
                .strong());
        });
    }
    
    /// Render recent alerts
    fn render_alerts(&self, ui: &mut Ui) {
        let alerts = self.recent_alerts.read();
        
        if alerts.is_empty() {
            ui.label("No recent alerts");
        } else {
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    for (timestamp, alert) in alerts.iter().rev().take(10) {
                        let age = timestamp.elapsed();
                        let age_str = format_duration(age);
                        
                        let (icon, color) = match alert {
                            PerformanceAlert::Memory { severity, .. } |
                            PerformanceAlert::Cpu { severity, .. } |
                            PerformanceAlert::Generic { severity, .. } => {
                                match severity {
                                    AlertSeverity::Info => ("ℹ", Color32::LIGHT_BLUE),
                                    AlertSeverity::Warning => ("⚠", Color32::YELLOW),
                                    AlertSeverity::Critical => ("🔴", Color32::RED),
                                }
                            }
                            PerformanceAlert::StartupTime { .. } => ("🚀", Color32::RED),
                        };
                        
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(icon).color(color));
                            ui.label(format!("[{}]", age_str));
                            ui.label(alert.to_log_message());
                        });
                    }
                });
        }
    }
    
    /// Render advanced metrics
    fn render_advanced_metrics(&self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new("Advanced Metrics").size(18.0).strong());
            
            ui.columns(3, |columns| {
                // System Information
                columns[0].group(|ui| {
                    ui.label(RichText::new("System Info").strong());
                    ui.separator();
                    
                    ui.label(format!("CPU Cores: {}", num_cpus::get()));
                    ui.label(format!("Platform: {}", std::env::consts::OS));
                    ui.label(format!("Architecture: {}", std::env::consts::ARCH));
                });
                
                // Process Information
                columns[1].group(|ui| {
                    ui.label(RichText::new("Process Info").strong());
                    ui.separator();
                    
                    ui.label(format!("PID: {}", std::process::id()));
                    ui.label(format!("Uptime: {}", format_duration(self.last_update.elapsed())));
                });
                
                // Performance Budgets
                columns[2].group(|ui| {
                    ui.label(RichText::new("Budgets").strong());
                    ui.separator();
                    
                    ui.label("Startup: < 2000ms");
                    ui.label("CPU Idle: ≤ 2%");
                    ui.label("Memory: ≤ 150 MB");
                });
            });
        });
    }
    
    /// Render flame graph visualization
    fn render_flame_graph(&self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new("Flame Graph Profiler").size(18.0).strong());
            
            if let Some(graph) = &self.cached_flame_graph {
                ui.label(format!("Total samples: {}", graph.sample_count));
                ui.label(format!("Total duration: {:?}", graph.total_duration));
                
                // Show top functions by time
                ui.separator();
                ui.label(RichText::new("Top Functions by Time").strong());
                
                let mut top_functions: Vec<_> = graph.function_stats.values().collect();
                top_functions.sort_by_key(|s| std::cmp::Reverse(s.total_time));
                
                for stat in top_functions.iter().take(10) {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", stat.function_name));
                        ui.label(format!("{}ms", stat.total_time.as_millis()));
                        ui.label(format!("({} calls)", stat.call_count));
                    });
                }
            } else {
                ui.label("No profiling data available");
            }
        });
    }
    
    /// Render startup performance report
    fn render_startup_report(&self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new("Startup Performance Report").size(18.0).strong());
            
            if let Some(report) = &self.cached_startup_report {
                // Summary
                let status_color = if report.budget_exceeded {
                    Color32::RED
                } else {
                    Color32::GREEN
                };
                
                ui.horizontal(|ui| {
                    ui.label("Total Duration:");
                    ui.label(RichText::new(format!("{} ms", report.total_duration_ms)).color(status_color));
                    ui.label(format!("/ {} ms budget", report.budget_ms));
                });
                
                // Phase breakdown
                ui.separator();
                ui.label(RichText::new("Phase Breakdown").strong());
                
                for phase in &report.phases {
                    let color = if phase.over_budget {
                        Color32::RED
                    } else {
                        Color32::GREEN
                    };
                    
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", phase.phase.display_name()));
                        ui.label(RichText::new(format!("{} ms", phase.duration_ms)).color(color));
                        ui.label(format!("/ {} ms", phase.budget_ms));
                    });
                }
                
                // Bottlenecks
                if !report.bottlenecks.is_empty() {
                    ui.separator();
                    ui.label(RichText::new("Bottlenecks").strong().color(Color32::RED));
                    
                    for bottleneck in &report.bottlenecks {
                        ui.label(format!("⚠ {}: {} ms over budget",
                            bottleneck.phase.display_name(),
                            bottleneck.duration_ms - bottleneck.budget_ms as u128
                        ));
                    }
                }
                
                // Recommendations
                if !report.recommendations.is_empty() {
                    ui.separator();
                    ui.label(RichText::new("Recommendations").strong());
                    
                    for rec in &report.recommendations {
                        ui.label(format!("• {}", rec));
                    }
                }
            } else {
                ui.label("No startup data available");
            }
        });
    }
}

/// Calculate overall performance score
fn calculate_performance_score(cpu_usage: f32, memory_mb: f32) -> f32 {
    let cpu_score = if cpu_usage <= 2.0 {
        100.0
    } else if cpu_usage <= 10.0 {
        100.0 - ((cpu_usage - 2.0) * 5.0)
    } else {
        60.0 - ((cpu_usage - 10.0) * 2.0).min(50.0)
    };
    
    let memory_score = if memory_mb <= 150.0 {
        100.0
    } else if memory_mb <= 200.0 {
        100.0 - ((memory_mb - 150.0) * 1.0)
    } else {
        50.0 - ((memory_mb - 200.0) * 0.5).min(40.0)
    };
    
    (cpu_score * 0.5 + memory_score * 0.5).max(0.0).min(100.0)
}

/// Format duration for display
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}