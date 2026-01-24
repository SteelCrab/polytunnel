//! Tests for Resolver logic using a local mock server

use polytunnel_maven::{Coordinate, MavenClient};
use polytunnel_resolver::Resolver;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// Start a simple mock HTTP server that simulates a dependency tree
// Root -> A -> B
async fn start_mock_repo() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        loop {
            if let Ok((mut socket, _)) = listener.accept().await {
                tokio::spawn(async move {
                    let mut buf = [0; 4096];
                    if let Ok(n) = socket.read(&mut buf).await {
                        if n == 0 {
                            return;
                        }

                        let request = String::from_utf8_lossy(&buf[..n]);

                        let response = if request
                            .contains("GET /org/test/root/1.0.0/root-1.0.0.pom")
                        {
                            // Root depends on A
                            let body = r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.test</groupId>
  <artifactId>root</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.test</groupId>
      <artifactId>lib-a</artifactId>
      <version>1.0.0</version>
    </dependency>
  </dependencies>
</project>
"#;
                            format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                                body.len(),
                                body
                            )
                        } else if request.contains("GET /org/test/lib-a/1.0.0/lib-a-1.0.0.pom") {
                            // A depends on B
                            let body = r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.test</groupId>
  <artifactId>lib-a</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.test</groupId>
      <artifactId>lib-b</artifactId>
      <version>1.0.0</version>
    </dependency>
  </dependencies>
</project>
"#;
                            format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                                body.len(),
                                body
                            )
                        } else if request.contains("GET /org/test/lib-b/1.0.0/lib-b-1.0.0.pom") {
                            // B has no dependencies
                            let body = r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.test</groupId>
  <artifactId>lib-b</artifactId>
  <version>1.0.0</version>
</project>
"#;
                            format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                                body.len(),
                                body
                            )
                        } else {
                            "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n".to_string()
                        };

                        let _ = socket.write_all(response.as_bytes()).await;
                    }
                });
            }
        }
    });

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn test_resolve_transitive_dependencies() {
    let base_url = start_mock_repo().await;
    let client = MavenClient::with_base_url(&base_url);

    let mut resolver = Resolver::with_client(client);

    let root = Coordinate::parse("org.test:root:1.0.0").unwrap();
    let result = resolver.resolve(&[root]).await;

    assert!(result.is_ok());
    let tree = result.unwrap();

    // Should include root, A, and B
    assert_eq!(tree.all_dependencies.len(), 3);

    let artifacts: Vec<String> = tree
        .all_dependencies
        .iter()
        .map(|d| d.artifact_id.clone())
        .collect();

    assert!(artifacts.contains(&"root".to_string()));
    assert!(artifacts.contains(&"lib-a".to_string()));
    assert!(artifacts.contains(&"lib-b".to_string()));
}

#[tokio::test]
async fn test_resolve_single_dependency() {
    let base_url = start_mock_repo().await;
    let client = MavenClient::with_base_url(&base_url);

    let mut resolver = Resolver::with_client(client);

    let dep = Coordinate::parse("org.test:lib-b:1.0.0").unwrap();
    let result = resolver.resolve(&[dep]).await;

    assert!(result.is_ok());
    let tree = result.unwrap();

    assert_eq!(tree.all_dependencies.len(), 1);
    assert_eq!(tree.all_dependencies[0].artifact_id, "lib-b");
}
