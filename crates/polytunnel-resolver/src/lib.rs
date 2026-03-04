//! Dependency resolution engine for Polytunnel

#![warn(missing_docs)]

mod error;
mod graph;
mod resolve;

pub use error::{ResolverError, Result};
pub use graph::{DependencyGraph, DependencyNode};
pub use resolve::Resolver;
