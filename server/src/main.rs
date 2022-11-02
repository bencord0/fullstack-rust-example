#[cfg(not(target_arch = "wasm32"))]
use axum::{
    body::{self, Empty, Full},
    extract::{Path},
    http::{header::{self, HeaderValue}, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get},
    Router,
    Server,
};

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/pkg/*path", get(client));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Running server: {}", addr.to_string());
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
async fn index() -> impl IntoResponse {
        Html(r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>fullstack-rust-example</title>
    <script type="module">
      import init, { start } from "/pkg/client.js";
      init().then(() => {
        start();
      });
    </script>
  </head>

  <body></body>
</html>"#)
}

#[cfg(not(target_arch = "wasm32"))]
async fn client(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match ::client::get_file(path) {
        None =>Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),

        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}
