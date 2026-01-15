//! Polytunnel Core Library
//!
//! Core types and utilities for the polytunnel Java dependency manager.

mod config;
mod error;

pub use config::*;
#[allow(deprecated)]
pub use error::{AppError, CoreError, Result};
