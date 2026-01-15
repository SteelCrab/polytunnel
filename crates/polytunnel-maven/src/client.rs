//! Maven Central API client

use crate::coordinate::Coordinate;
use crate::pom::Pom;
use polytunnel_core::Result;
use reqwest::Client;
use std::path::PathBuf;

const MAVEN_CENTRAL_URL: &str = "https://repo1.maven.org/maven2";
const MAVEN_SEARCH_URL: &str = "https://search.maven.org/solrsearch/select";

/// Maven Central HTTP client
pub struct MavenClient {
    http: Client,
    base_url: String,
}

/// Search result from Maven Central
#[derive(Debug, serde::Deserialize)]
pub struct SearchResponse {
    pub response: SearchResponseBody,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchResponseBody {
    #[serde(rename = "numFound")]
    pub num_found: u32,
    pub docs: Vec<SearchDoc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchDoc {
    pub id: String,
    pub g: String,           // groupId
    pub a: String,           // artifactId
    #[serde(rename = "latestVersion")]
    pub latest_version: Option<String>,
    #[serde(rename = "v")]
    pub version: Option<String>,
}

impl MavenClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
            base_url: MAVEN_CENTRAL_URL.to_string(),
        }
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Search artifacts by query
    pub async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchDoc>> {
        let url = format!(
            "{}?q={}&rows={}&wt=json",
            MAVEN_SEARCH_URL, query, limit
        );

        let response: SearchResponse = self.http
            .get(&url)
            .send()
            .await
            .map_err(|e| polytunnel_core::AppError::Io(std::io::Error::other(e.to_string())))?
            .json()
            .await
            .map_err(|e| polytunnel_core::AppError::Io(std::io::Error::other(e.to_string())))?;

        Ok(response.response.docs)
    }

    /// Fetch POM file content
    pub async fn fetch_pom_content(&self, coord: &Coordinate) -> Result<String> {
        let url = format!(
            "{}/{}/{}",
            self.base_url,
            coord.repo_path(),
            coord.pom_filename()
        );

        let content = self.http
            .get(&url)
            .send()
            .await
            .map_err(|e| polytunnel_core::AppError::Io(std::io::Error::other(e.to_string())))?
            .text()
            .await
            .map_err(|e| polytunnel_core::AppError::Io(std::io::Error::other(e.to_string())))?;

        Ok(content)
    }

    /// Fetch and parse POM
    pub async fn fetch_pom(&self, coord: &Coordinate) -> Result<Pom> {
        let content = self.fetch_pom_content(coord).await?;
        crate::pom::parse_pom(&content)
    }

    /// Get list of available versions for an artifact
    pub async fn list_versions(&self, group_id: &str, artifact_id: &str) -> Result<Vec<String>> {
        let query = format!("g:\"{}\" AND a:\"{}\"", group_id, artifact_id);
        let url = format!(
            "{}?q={}&core=gav&rows=100&wt=json",
            MAVEN_SEARCH_URL,
            urlencoding::encode(&query)
        );

        let response: SearchResponse = self.http
            .get(&url)
            .send()
            .await
            .map_err(|e| polytunnel_core::AppError::Io(std::io::Error::other(e.to_string())))?
            .json()
            .await
            .map_err(|e| polytunnel_core::AppError::Io(std::io::Error::other(e.to_string())))?;

        let versions: Vec<String> = response
            .response
            .docs
            .into_iter()
            .filter_map(|d| d.version)
            .collect();

        Ok(versions)
    }

    /// Get JAR download URL
    pub fn jar_url(&self, coord: &Coordinate) -> String {
        format!(
            "{}/{}/{}",
            self.base_url,
            coord.repo_path(),
            coord.jar_filename()
        )
    }

    /// Download JAR to a path
    pub async fn download_jar(&self, coord: &Coordinate, dest: &PathBuf) -> Result<()> {
        let url = self.jar_url(coord);
        
        let bytes = self.http
            .get(&url)
            .send()
            .await
            .map_err(|e| polytunnel_core::AppError::Io(std::io::Error::other(e.to_string())))?
            .bytes()
            .await
            .map_err(|e| polytunnel_core::AppError::Io(std::io::Error::other(e.to_string())))?;

        std::fs::write(dest, bytes)?;
        Ok(())
    }
}

impl Default for MavenClient {
    fn default() -> Self {
        Self::new()
    }
}
