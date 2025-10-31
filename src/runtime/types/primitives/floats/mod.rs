// Floating point types module
pub mod float32;
pub mod float64;
pub mod decimal;

// Re-export all float types
pub use decimal::DecimalType;
pub use float32::Float32Type;
pub use float64::Float64Type;

// Generic float type (backwards compatibility)
pub use float64::FloatType;
