use polytunnel_maven::{Coordinate, MavenClient, MavenError};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn spawn_single_response_server(
    expected_path: String,
    status: u16,
    body: Vec<u8>,
    content_type: &'static str,
) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buffer = vec![0_u8; 8192];
        let size = socket.read(&mut buffer).await.unwrap();
        let request = String::from_utf8_lossy(&buffer[..size]);
        let first_line = request.lines().next().unwrap_or_default();
        let path = first_line.split_whitespace().nth(1).unwrap_or_default();

        assert_eq!(path, expected_path);

        let reason = match status {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Status",
        };

        let response_head = format!(
            "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\nConnection: close\r\n\r\n",
            body.len()
        );

        socket.write_all(response_head.as_bytes()).await.unwrap();
        socket.write_all(&body).await.unwrap();
    });

    (format!("http://{addr}"), handle)
}

#[test]
fn test_default_client_is_constructible() {
    let _client = MavenClient::default();
}

#[tokio::test]
async fn test_reqwest_transport_non_2xx_maps_to_http_status() {
    let coord = Coordinate::parse("org.test:missing:1.0.0").unwrap();
    let expected_path = "/org/test/missing/1.0.0/missing-1.0.0.pom".to_string();
    let (base_url, server) =
        spawn_single_response_server(expected_path, 500, b"server error".to_vec(), "text/plain")
            .await;

    let client = MavenClient::with_base_url(&base_url);
    let err = client.fetch_pom_content(&coord).await.unwrap_err();

    match err {
        MavenError::HttpStatus { status, url } => {
            assert_eq!(status, 500);
            assert!(url.contains("/org/test/missing/1.0.0/missing-1.0.0.pom"));
        }
        other => panic!("expected HttpStatus error, got {other:?}"),
    }

    server.await.unwrap();
}

#[tokio::test]
async fn test_search_reqwest_path_is_url_encoded() {
    let query = "g:\"org.test\" AND a:\"lib\"";
    let encoded = urlencoding::encode(query);
    let expected_path = format!("/solrsearch/select?q={encoded}&rows=1&wt=json");
    let body = br#"{"response":{"numFound":1,"docs":[{"id":"org.test:lib:1.0.0","g":"org.test","a":"lib","v":"1.0.0","latestVersion":"1.0.0"}]}}"#.to_vec();
    let (base_url, server) =
        spawn_single_response_server(expected_path, 200, body, "application/json").await;
    let search_url = format!("{base_url}/solrsearch/select");

    let client = MavenClient::with_base_url(&base_url).with_search_url(&search_url);
    let docs = client.search(query, 1).await.unwrap();

    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0].id, "org.test:lib:1.0.0");
    assert_eq!(docs[0].g, "org.test");
    assert_eq!(docs[0].a, "lib");

    server.await.unwrap();
}

#[tokio::test]
async fn test_search_non_2xx_maps_to_http_status() {
    let query = "g:\"org.test\" AND a:\"missing\"";
    let encoded = urlencoding::encode(query);
    let expected_path = format!("/solrsearch/select?q={encoded}&rows=1&wt=json");
    let (base_url, server) =
        spawn_single_response_server(expected_path, 503, b"unavailable".to_vec(), "text/plain")
            .await;
    let search_url = format!("{base_url}/solrsearch/select");

    let client = MavenClient::with_base_url(&base_url).with_search_url(&search_url);
    let err = client.search(query, 1).await.unwrap_err();

    match err {
        MavenError::HttpStatus { status, url } => {
            assert_eq!(status, 503);
            assert!(url.contains("rows=1&wt=json"));
        }
        other => panic!("expected HttpStatus error, got {other:?}"),
    }

    server.await.unwrap();
}

#[tokio::test]
async fn test_fetch_pom_content_invalid_utf8_maps_error() {
    let coord = Coordinate::parse("org.test:utf8:1.0.0").unwrap();
    let expected_path = "/org/test/utf8/1.0.0/utf8-1.0.0.pom".to_string();
    let (base_url, server) =
        spawn_single_response_server(expected_path, 200, vec![0xff, 0xfe, 0xfd], "text/plain")
            .await;

    let client = MavenClient::with_base_url(&base_url);
    let err = client.fetch_pom_content(&coord).await.unwrap_err();

    assert!(matches!(err, MavenError::InvalidUtf8 { .. }));
    server.await.unwrap();
}
