use std::io::Write;
use tempfile::NamedTempFile;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub async fn setup_mock_server() -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/admin"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("Admin panel content here with multiple words for counting")
                .insert_header("content-type", "text/html"),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("{\"status\": \"ok\"}")
                .insert_header("content-type", "application/json"),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/images"))
        .respond_with(ResponseTemplate::new(301).insert_header("location", "/images/"))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/images/"))
        .respond_with(ResponseTemplate::new(200).set_body_string("Directory listing for images"))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.htaccess"))
        .respond_with(ResponseTemplate::new(403).set_body_string("Forbidden"))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/secret"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&server)
        .await;

    server
}

pub fn create_wordlist(words: &[&str]) -> NamedTempFile {
    let mut f = NamedTempFile::new().unwrap();
    for word in words {
        writeln!(f, "{}", word).unwrap();
    }
    f
}
