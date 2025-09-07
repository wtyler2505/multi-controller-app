// Multi-Controller App Library
// This file makes our modules available as a library crate

pub mod device;
pub mod drivers;
pub mod transport;
// pub mod scripting; // Temporarily disabled due to lifetime issues
pub mod telemetry;
pub mod ui;
pub mod profile;
pub mod logging;
pub mod performance;