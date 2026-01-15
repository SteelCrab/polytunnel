//! Polytunnel Maven Library
//!
//! Maven Central client and POM parser

mod client;
mod coordinate;
mod pom;

pub use client::{MavenClient, SearchDoc, SearchResponse};
pub use coordinate::{Coordinate, CoordinateError};
pub use pom::{parse_pom, DependencyScope, Exclusion, Pom, PomDependency};
