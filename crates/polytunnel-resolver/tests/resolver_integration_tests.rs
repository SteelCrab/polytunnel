//! Integration tests for dependency resolution using a deterministic transport.

use polytunnel_maven::{
    Coordinate, HttpResponse, HttpTransportFuture, MavenClient, MavenTransport,
};
use polytunnel_resolver::Resolver;
use std::collections::HashMap;
use std::sync::Arc;

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

fn transitive_routes(base_url: &str) -> Vec<(String, u16, String)> {
    vec![
        (
            format!("{base_url}/org/app/app/1.0.0/app-1.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.app</groupId>
  <artifactId>app</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>com.example</groupId>
      <artifactId>core-lib</artifactId>
      <version>1.0.0</version>
      <scope>compile</scope>
    </dependency>
    <dependency>
      <groupId>org.tests</groupId>
      <artifactId>ignored-test-lib</artifactId>
      <version>2.0.0</version>
      <scope>test</scope>
    </dependency>
    <dependency>
      <groupId>org.unavailable</groupId>
      <artifactId>missing-lib</artifactId>
      <version>1.0.0</version>
      <scope>compile</scope>
      <optional>true</optional>
    </dependency>
  </dependencies>
</project>
"#
            .to_string(),
        ),
        (
            format!("{base_url}/com/example/core-lib/1.0.0/core-lib-1.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>com.example</groupId>
  <artifactId>core-lib</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.slf4j</groupId>
      <artifactId>slf4j-api</artifactId>
      <version>2.0.9</version>
      <scope>compile</scope>
    </dependency>
  </dependencies>
</project>
"#
            .to_string(),
        ),
        (
            format!("{base_url}/org/slf4j/slf4j-api/2.0.9/slf4j-api-2.0.9.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.slf4j</groupId>
  <artifactId>slf4j-api</artifactId>
  <version>2.0.9</version>
</project>
"#
            .to_string(),
        ),
    ]
}

fn override_routes(base_url: &str) -> Vec<(String, u16, String)> {
    vec![
        (
            format!("{base_url}/org/app/root/9.9.9/root-9.9.9.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.app</groupId>
  <artifactId>root</artifactId>
  <version>9.9.9</version>
  <dependencies>
    <dependency>
      <groupId>org.lib</groupId>
      <artifactId>lib</artifactId>
      <version>2.0.0</version>
      <scope>compile</scope>
    </dependency>
  </dependencies>
</project>
"#
            .to_string(),
        ),
        (
            format!("{base_url}/org/lib/lib/2.0.0/lib-2.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.lib</groupId>
  <artifactId>lib</artifactId>
  <version>2.0.0</version>
</project>
"#
            .to_string(),
        ),
    ]
}

fn chain_routes(base_url: &str) -> Vec<(String, u16, String)> {
    vec![
        (
            format!("{base_url}/org/one/root/1.0.0/root-1.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.one</groupId>
  <artifactId>root</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.two</groupId>
      <artifactId>middle</artifactId>
      <version>1.0.0</version>
      <scope>compile</scope>
    </dependency>
  </dependencies>
</project>
"#
            .to_string(),
        ),
        (
            format!("{base_url}/org/two/middle/1.0.0/middle-1.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.two</groupId>
  <artifactId>middle</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.three</groupId>
      <artifactId>leaf</artifactId>
      <version>1.0.0</version>
      <scope>compile</scope>
    </dependency>
  </dependencies>
</project>
"#
            .to_string(),
        ),
        (
            format!("{base_url}/org/three/leaf/1.0.0/leaf-1.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.three</groupId>
  <artifactId>leaf</artifactId>
  <version>1.0.0</version>
</project>
"#
            .to_string(),
        ),
    ]
}

fn resilient_routes(base_url: &str) -> Vec<(String, u16, String)> {
    vec![
        (
            format!("{base_url}/org/faulty/root/1.0.0/root-1.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.faulty</groupId>
  <artifactId>root</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.good</groupId>
      <artifactId>with-missing-parent</artifactId>
      <version>1.0.0</version>
      <scope>compile</scope>
    </dependency>
    <dependency>
      <groupId>org.bad</groupId>
      <artifactId>missing</artifactId>
      <version>1.0.0</version>
      <scope>compile</scope>
    </dependency>
  </dependencies>
</project>
"#
            .to_string(),
        ),
        (
            format!("{base_url}/org/good/with-missing-parent/1.0.0/with-missing-parent-1.0.0.pom"),
            200,
            r#"
<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.good</groupId>
  <artifactId>with-missing-parent</artifactId>
  <version>1.0.0</version>
  <parent>
    <groupId>org.missing</groupId>
    <artifactId>parent</artifactId>
    <version>1.0.0</version>
  </parent>
  <dependencies>
    <dependency>
      <groupId>org.good</groupId>
      <artifactId>missing-transitive</artifactId>
      <version>1.0.0</version>
      <scope>compile</scope>
    </dependency>
  </dependencies>
</project>
"#
            .to_string(),
        ),
    ]
}

#[test]
fn test_resolver_new_uses_default_client() {
    let resolver = Resolver::new();
    assert_eq!(resolver.graph.nodes().count(), 0);
}

#[tokio::test]
async fn test_resolve_transitive_dependencies_with_graph() {
    let base_url = "https://repo.example.test";
    let mut resolver = Resolver::with_client(MavenClient::with_transport(
        base_url,
        Arc::new(MockTransport::new(transitive_routes(base_url))),
    ));
    let root = Coordinate::parse("org.app:app:1.0.0").unwrap();
    let tree = resolver
        .resolve(&[root])
        .await
        .expect("resolver should succeed");

    let mut coords: Vec<_> = tree
        .all_dependencies
        .into_iter()
        .map(|coord| coord.to_string())
        .collect();
    coords.sort();

    assert!(coords.contains(&"org.app:app:1.0.0".to_string()));
    assert!(coords.contains(&"com.example:core-lib:1.0.0".to_string()));
    assert!(coords.contains(&"org.slf4j:slf4j-api:2.0.9".to_string()));
    assert!(!coords.contains(&"org.tests:ignored-test-lib:2.0.0".to_string()));

    assert!(resolver.graph.get("org.app:app:1.0.0").is_some());
    assert!(resolver.graph.get("com.example:core-lib:1.0.0").is_some());
    assert!(resolver.graph.get("org.slf4j:slf4j-api:2.0.9").is_some());
}

#[tokio::test]
async fn test_resolve_applies_root_overrides() {
    let base_url = "https://repo.example.test";
    let mut resolver = Resolver::with_client(MavenClient::with_transport(
        base_url,
        Arc::new(MockTransport::new(override_routes(base_url))),
    ));
    let root = Coordinate::parse("org.app:root:1.0.0").unwrap();
    let override_root = Coordinate::parse("org.app:root:9.9.9").unwrap();
    let tree = resolver
        .resolve(&[root, override_root])
        .await
        .expect("resolver should apply root override");

    let app_versions: Vec<_> = tree
        .all_dependencies
        .into_iter()
        .filter(|coord| coord.group_id == "org.app" && coord.artifact_id == "root")
        .map(|coord| coord.version)
        .collect();

    assert_eq!(app_versions, vec!["9.9.9".to_string()]);
    assert!(resolver.graph.get("org.app:root:9.9.9").is_some());
}

#[tokio::test]
async fn test_dependency_graph_tracks_direct_and_transitive_nodes() {
    let base_url = "https://repo.example.test";
    let mut resolver = Resolver::with_client(MavenClient::with_transport(
        base_url,
        Arc::new(MockTransport::new(chain_routes(base_url))),
    ));
    let root = Coordinate::parse("org.one:root:1.0.0").unwrap();
    let tree = resolver
        .resolve(&[root])
        .await
        .expect("resolver should resolve transitive chain");

    let expected = [
        "org.one:root:1.0.0",
        "org.two:middle:1.0.0",
        "org.three:leaf:1.0.0",
    ];
    let resolved: Vec<_> = tree
        .all_dependencies
        .into_iter()
        .map(|coord| coord.to_string())
        .collect();

    for coord in expected {
        assert!(resolved.contains(&coord.to_string()));
    }

    assert!(resolver.graph.get("org.one:root:1.0.0").is_some());
    assert!(resolver.graph.get("org.two:middle:1.0.0").is_some());
    assert!(resolver.graph.get("org.three:leaf:1.0.0").is_some());
}

#[tokio::test]
async fn test_resolver_ignores_missing_parent_and_transitive_errors() {
    let base_url = "https://repo.example.test";
    let mut resolver = Resolver::with_client(MavenClient::with_transport(
        base_url,
        Arc::new(MockTransport::new(resilient_routes(base_url))),
    ));
    let root = Coordinate::parse("org.faulty:root:1.0.0").unwrap();
    let tree = resolver
        .resolve(&[root])
        .await
        .expect("resolver should continue after non-fatal failures");

    let coords: Vec<_> = tree
        .all_dependencies
        .into_iter()
        .map(|coord| coord.to_string())
        .collect();

    assert!(coords.contains(&"org.faulty:root:1.0.0".to_string()));
    assert!(coords.contains(&"org.good:with-missing-parent:1.0.0".to_string()));
    assert!(!coords.contains(&"org.good:missing-transitive:1.0.0".to_string()));
    assert!(!coords.contains(&"org.bad:missing:1.0.0".to_string()));
    assert!(
        resolver
            .graph
            .get("org.good:with-missing-parent:1.0.0")
            .is_some()
    );
}
