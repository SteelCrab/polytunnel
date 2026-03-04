//! Polytunnel Maven Library
//!
//! Maven Central client and POM parser

#![warn(missing_docs)]

mod client;
mod coordinate;
mod error;
mod pom;

pub use client::{
    HttpResponse, HttpTransportFuture, MavenClient, MavenTransport, SearchDoc, SearchResponse,
};
pub use coordinate::{Coordinate, CoordinateError};
pub use error::{MavenError, Result};
pub use pom::{DependencyScope, Exclusion, Pom, PomDependency, parse_pom};
