//! In-process metrics for services that do not want a full metrics stack.

pub mod histogram;
pub mod registry;
pub mod window;

pub use histogram::Histogram;
pub use registry::Registry;
pub use window::RollingRate;
