//! Tests for `MavenClient` backed by a deterministic in-memory transport.

use polytunnel_maven::{
    Coordinate, HttpResponse, HttpTransportFuture, MavenClient, MavenTransport,
};
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::NamedTempFile;

#[derive(Clone)]
struct MockTransport {
    routes: HashMap<String, (u16, Vec<u8>)>,
}

impl MockTransport {
    fn new(routes: Vec<(String, u16, String)>) -> Self {
        let routes = routes
            .into_iter()
            .map(|(path, status, body)| (path, (status, body.into_bytes())))
            .collect();

        Self { routes }
    }
}

impl MavenTransport for MockTransport {
    fn get(&self, url: String) -> HttpTransportFuture {
        let response = self
            .routes
            .get(&url)
            .cloned()
            .unwrap_or((404, b"not found".to_vec()));

        Box::pin(async move {
            Ok(HttpResponse {
                status: response.0,
                body: response.1,
            })
        })
    }
}

fn routes(base_url: &str) -> Vec<(String, u16, String)> {
    let search_query_raw = "g:\"org.test\" AND a:\"lib\"";
    let query = urlencoding::encode("g:\"org.test\" AND a:\"lib\"");
    let search_url = format!("{base_url}/solrsearch/select");

    let search_query = format!("{search_url}?q={search_query_raw}&rows=1&wt=json");
    let list_versions_query = format!("{search_url}?q={query}&core=gav&rows=100&wt=json");

    vec![
        (
            format!("{base_url}/org/test/lib/1.0.0/lib-1.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.test</groupId>
  <artifactId>lib</artifactId>
  <version>1.0.0</version>
</project>
"#
            .to_string(),
        ),
        (
            format!("{base_url}/org/test/unknown/1.0.0/unknown-1.0.0.pom"),
            404,
            "not found".to_string(),
        ),
        (
            format!("{base_url}/org/test/lib/1.0.0/lib-1.0.0.jar"),
            200,
            "dummy jar content".to_string(),
        ),
        (
            search_query,
            200,
            r#"{"response":{"numFound":1,"docs":[{"id":"org.test:lib:1.0.0","g":"org.test","a":"lib","v":"1.0.0","latestVersion":"1.0.0"}]} }"#
                .to_string(),
        ),
        (
            list_versions_query,
            200,
            r#"{"response":{"numFound":1,"docs":[{"id":"org.test:lib:1.0.0","g":"org.test","a":"lib","v":"1.0.0","latestVersion":"1.0.0"}]} }"#
                .to_string(),
        ),
    ]
}

#[tokio::test]
async fn test_fetch_pom_success() {
    let base_url = "https://repo.example.test";
    let client =
        MavenClient::with_transport(base_url, Arc::new(MockTransport::new(routes(base_url))));
    let coord = Coordinate::parse("org.test:lib:1.0.0").unwrap();

    let pom = client
        .fetch_pom(&coord)
        .await
        .expect("fetch_pom should succeed");

    assert_eq!(pom.coordinate.group_id, "org.test");
    assert_eq!(pom.coordinate.artifact_id, "lib");
    assert_eq!(pom.coordinate.version, "1.0.0");
}

#[tokio::test]
async fn test_fetch_pom_not_found() {
    let base_url = "https://repo.example.test";
    let client =
        MavenClient::with_transport(base_url, Arc::new(MockTransport::new(routes(base_url))));
    let coord = Coordinate::parse("org.test:unknown:1.0.0").unwrap();

    assert!(client.fetch_pom(&coord).await.is_err());
}

#[tokio::test]
async fn test_download_jar() {
    let base_url = "https://repo.example.test";
    let client =
        MavenClient::with_transport(base_url, Arc::new(MockTransport::new(routes(base_url))));
    let coord = Coordinate::parse("org.test:lib:1.0.0").unwrap();
    let temp_file = NamedTempFile::new().unwrap();
    let destination = temp_file.path().to_path_buf();

    client
        .download_jar(&coord, &destination, false)
        .await
        .expect("download_jar should succeed");

    let content = std::fs::read_to_string(&destination).unwrap();
    assert_eq!(content, "dummy jar content");
}

#[tokio::test]
async fn test_search() {
    let base_url = "https://repo.example.test";
    let search_url = format!("{base_url}/solrsearch/select");
    let client =
        MavenClient::with_transport(base_url, Arc::new(MockTransport::new(routes(base_url))))
            .with_search_url(&search_url);

    let results = client
        .search("g:\"org.test\" AND a:\"lib\"", 1)
        .await
        .expect("search should succeed");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "org.test:lib:1.0.0");
    assert_eq!(results[0].version, Some("1.0.0".to_string()));
}

#[tokio::test]
async fn test_list_versions() {
    let base_url = "https://repo.example.test";
    let search_url = format!("{base_url}/solrsearch/select");
    let client =
        MavenClient::with_transport(base_url, Arc::new(MockTransport::new(routes(base_url))))
            .with_search_url(&search_url);

    let versions = client
        .list_versions("org.test", "lib")
        .await
        .expect("list_versions should succeed");

    assert_eq!(versions, vec!["1.0.0".to_string()]);
}

#[tokio::test]
async fn test_search_handles_invalid_json() {
    let base_url = "https://repo.example.test";
    let search_url = format!("{base_url}/solrsearch/select");
    let mut routes = routes(base_url);
    let raw_query = "g:\"org.test\" AND a:\"lib\"";
    let malformed_query = format!("{search_url}?q={q}&rows=1&wt=json", q = raw_query);
    routes.push((
        malformed_query,
        200,
        r#"{"response":{"numFound":1,"docs":"not-json"}}"#.to_string(),
    ));
    let client = MavenClient::with_transport(base_url, Arc::new(MockTransport::new(routes)))
        .with_search_url(&search_url);

    assert!(
        client
            .search("g:\"org.test\" AND a:\"lib\"", 1)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_fetch_pom_content_bad_status() {
    let base_url = "https://repo.example.test";
    let client =
        MavenClient::with_transport(base_url, Arc::new(MockTransport::new(routes(base_url))));
    let coord = Coordinate::parse("org.test:unknown:1.0.0").unwrap();

    assert!(client.fetch_pom_content(&coord).await.is_err());
}

#[tokio::test]
async fn test_download_jar_bad_status() {
    let base_url = "https://repo.example.test";
    let mut routes = routes(base_url);
    routes.push((
        format!("{base_url}/org/test/lib/1.0.0/lib-1.0.0.jar"),
        500,
        "server error".to_string(),
    ));
    let client = MavenClient::with_transport(base_url, Arc::new(MockTransport::new(routes)));
    let coord = Coordinate::parse("org.test:lib:1.0.0").unwrap();
    let destination = tempfile::tempdir().unwrap().path().join("artifact.jar");

    assert!(
        client
            .download_jar(&coord, &destination, true)
            .await
            .is_err()
    );
}
