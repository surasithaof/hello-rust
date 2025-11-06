use axum::{response::Html, routing::get, Router};
use tokio::fs;

pub fn openapi_preview() -> Router {
    Router::new()
        .route("/api-docs", get(|| async { Html(generate_scalar_html()) }))
        .route(
            "/api-docs/spec",
            get(|| async { fs::read_to_string("./docs/openapi.yaml").await.unwrap() }),
        )
}

fn generate_scalar_html() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>OpenAPI Preview</title>
    <meta charset="utf-8" />
</head>
<body>
    <div id="app"></div>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
    <script>
        fetch('/api-docs/spec')
            .then(r => r.text())
            .then(content => {{
                Scalar.createApiReference('#app', {{
                    spec: {{ content: content }},
                    layout: 'modern'
                }});
            }});
    </script>
</body>
</html>
    "#
    )
}
