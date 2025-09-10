/// Transport layer test suite

#[cfg(test)]
mod reconnection;

#[cfg(test)]
mod error_handling;

#[cfg(test)]
mod latency;

// Re-export test utilities for use in integration tests
#[cfg(test)]
pub use crate::transport::mock::{MockTransport, MockConfig};