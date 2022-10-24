use axum::{
    body::{self, Empty, Full},
    extract::{Path},
    http::{header::{self, HeaderValue}, StatusCode},
    response::{IntoResponse, Response},
    routing::{get},
    Router,
    Server,
};

use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("$OUT_DIR/pkg");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/pkg/*path", get(client));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body(body::boxed(Full::from(r#"
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
</html>"#)))
        .unwrap()
}

async fn client(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
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
