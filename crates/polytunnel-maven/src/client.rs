//! Maven Central API client

use crate::coordinate::Coordinate;
use crate::error::{MavenError, Result};
use crate::pom::Pom;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

const MAVEN_CENTRAL_URL: &str = "https://repo1.maven.org/maven2";
const MAVEN_SEARCH_URL: &str = "https://search.maven.org/solrsearch/select";

pub type HttpTransportFuture = Pin<Box<dyn Future<Output = Result<HttpResponse>> + Send>>;

#[derive(Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

/// Pluggable transport for testability and integration boundaries.
pub trait MavenTransport: Send + Sync {
    fn get(&self, url: String) -> HttpTransportFuture;
}

#[derive(Clone)]
struct ReqwestTransport {
    client: Client,
}

impl ReqwestTransport {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl MavenTransport for ReqwestTransport {
    fn get(&self, url: String) -> HttpTransportFuture {
        let client = self.client.clone();
        Box::pin(async move {
            let response = client.get(&url).send().await?;
            let status = response.status().as_u16();
            let body = response.bytes().await?.to_vec();

            Ok(HttpResponse { status, body })
        })
    }
}

/// Maven Central HTTP client
#[derive(Clone)]
pub struct MavenClient {
    http: Arc<dyn MavenTransport>,
    base_url: String,
    search_url: String,
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
    pub g: String, // groupId
    pub a: String, // artifactId
    #[serde(rename = "latestVersion")]
    pub latest_version: Option<String>,
    #[serde(rename = "v")]
    pub version: Option<String>,
}

impl MavenClient {
    pub fn new() -> Self {
        Self::with_transport(MAVEN_CENTRAL_URL, Arc::new(ReqwestTransport::new()))
    }

    pub fn with_base_url(base_url: &str) -> Self {
        Self::with_transport(base_url, Arc::new(ReqwestTransport::new()))
    }

    pub fn with_transport(base_url: &str, transport: Arc<dyn MavenTransport>) -> Self {
        Self {
            http: transport,
            base_url: base_url.to_string(),
            search_url: MAVEN_SEARCH_URL.to_string(),
        }
    }

    pub fn with_search_url(mut self, search_url: &str) -> Self {
        self.search_url = search_url.to_string();
        self
    }

    async fn read_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self.http.get(url.to_string()).await?;
        if !(200..=299).contains(&response.status) {
            return Err(MavenError::HttpStatus {
                status: response.status,
                url: url.to_string(),
            });
        }

        serde_json::from_slice(&response.body).map_err(|error| MavenError::JsonParse {
            message: error.to_string(),
        })
    }

    async fn read_text(&self, url: &str) -> Result<String> {
        let response = self.http.get(url.to_string()).await?;
        if !(200..=299).contains(&response.status) {
            return Err(MavenError::HttpStatus {
                status: response.status,
                url: url.to_string(),
            });
        }

        String::from_utf8(response.body).map_err(|error| MavenError::InvalidUtf8 {
            message: error.to_string(),
        })
    }

    /// Search artifacts by query
    pub async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchDoc>> {
        let url = format!(
            "{}?q={}&rows={}&wt=json",
            self.search_url,
            urlencoding::encode(query),
            limit
        );
        let response: SearchResponse = self.read_json(&url).await?;

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
        self.read_text(&url).await
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
            self.search_url,
            urlencoding::encode(&query)
        );
        let response: SearchResponse = self.read_json(&url).await?;

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
    pub async fn download_jar(
        &self,
        coord: &Coordinate,
        dest: &PathBuf,
        verbose: bool,
    ) -> Result<()> {
        let url = self.jar_url(coord);
        let request_url = url.clone();

        if verbose {
            println!("   Downloading {}", coord);
        }

        let response = self.http.get(request_url).await?;
        if !(200..=299).contains(&response.status) {
            return Err(MavenError::HttpStatus {
                status: response.status,
                url,
            });
        }

        std::fs::write(dest, response.body)?;
        Ok(())
    }
}

impl Default for MavenClient {
    fn default() -> Self {
        Self::new()
    }
}
