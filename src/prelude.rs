//! Crate prelude

pub use crate::error::AppError;

/// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, AppError>;
