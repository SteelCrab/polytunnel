//! Dependency resolution engine for Polytunnel

mod error;
mod graph;
mod resolve;

pub use error::{ResolverError, Result};
pub use graph::{DependencyGraph, DependencyNode};
pub use resolve::{ResolvedTree, Resolver};
