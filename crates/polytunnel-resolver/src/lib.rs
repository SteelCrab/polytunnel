//! Dependency resolution engine for Polytunnel

mod graph;
mod resolve;

pub use graph::{DependencyGraph, DependencyNode};
pub use resolve::Resolver;
