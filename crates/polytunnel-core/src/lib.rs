//! Polytunnel Core Library
//!
//! Core types and utilities for the polytunnel Java dependency manager.

#![warn(missing_docs)]

mod config;
mod error;

pub use config::*;
pub use error::{CoreError, Result};
