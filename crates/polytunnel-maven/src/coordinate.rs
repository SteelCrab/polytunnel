//! Maven coordinate types

use serde::{Deserialize, Serialize};
use std::fmt;

/// Maven artifact coordinate (GAV)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coordinate {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    #[serde(default)]
    pub classifier: Option<String>,
    #[serde(default = "default_packaging")]
    pub packaging: String,
}

fn default_packaging() -> String {
    "jar".to_string()
}

impl Coordinate {
    pub fn new(group_id: &str, artifact_id: &str, version: &str) -> Self {
        Self {
            group_id: group_id.to_string(),
            artifact_id: artifact_id.to_string(),
            version: version.to_string(),
            classifier: None,
            packaging: default_packaging(),
        }
    }

    /// Parse from "groupId:artifactId:version" format
    pub fn parse(s: &str) -> Result<Self, CoordinateError> {
        let parts: Vec<&str> = s.split(':').collect();
        match parts.len() {
            3 => Ok(Self::new(parts[0], parts[1], parts[2])),
            4 => Ok(Self {
                group_id: parts[0].to_string(),
                artifact_id: parts[1].to_string(),
                packaging: parts[2].to_string(),
                version: parts[3].to_string(),
                classifier: None,
            }),
            5 => Ok(Self {
                group_id: parts[0].to_string(),
                artifact_id: parts[1].to_string(),
                packaging: parts[2].to_string(),
                classifier: Some(parts[3].to_string()),
                version: parts[4].to_string(),
            }),
            _ => Err(CoordinateError::InvalidFormat(s.to_string())),
        }
    }

    /// Convert group_id to path format (org.slf4j -> org/slf4j)
    pub fn group_path(&self) -> String {
        self.group_id.replace('.', "/")
    }

    /// Get the JAR filename
    pub fn jar_filename(&self) -> String {
        match &self.classifier {
            Some(c) => format!("{}-{}-{}.jar", self.artifact_id, self.version, c),
            None => format!("{}-{}.jar", self.artifact_id, self.version),
        }
    }

    /// Get the POM filename
    pub fn pom_filename(&self) -> String {
        format!("{}-{}.pom", self.artifact_id, self.version)
    }

    /// Get the repository path
    pub fn repo_path(&self) -> String {
        format!(
            "{}/{}/{}",
            self.group_path(),
            self.artifact_id,
            self.version
        )
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.group_id, self.artifact_id, self.version)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CoordinateError {
    #[error("Invalid coordinate format: {0}")]
    InvalidFormat(String),
}
