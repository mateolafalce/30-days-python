use axum::{
    Router,
    extract::Path,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use comrak::Options;
use comrak::markdown_to_html_with_plugins;
use tokio::net::TcpListener;

const PORT: u16 = 5001;
const GITHUB_URL: &str = "https://github.com/mateolafalce/30-days-python";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/markdown/{*path}", get(markdown_handler))
        .nest_service(
            "/images",
            axum::routing::get_service(tower_http::services::ServeDir::new("./images")),
        );
    let listener = TcpListener::bind(&format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();

    println!("Listening on http://localhost:{}", PORT);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let md = convert_file2_string("./markdown/index.md");
    let html = convert_md2_html(&md);
    Html(html)
}

async fn markdown_handler(Path(path): Path<String>) -> Response {
    if path == "readme.md" {
        Redirect::to("/").into_response()
    } else {
        let file_path = format!("./markdown/{}", path);
        let md = convert_file2_string(&file_path);
        let html = convert_md2_html(&md);
        Html(html).into_response()
    }
}

fn convert_file2_string(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap_or_else(|_| String::from("Error reading file"))
}

fn convert_md2_html(md: &str) -> String {
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.header_ids = Some("".to_string());
    let plugins = comrak::Plugins::default();
    let css = std::fs::read_to_string("static/style.css").unwrap_or_default();

    format!(
        r#"
<link rel="stylesheet" href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css">
<link rel="icon" type="image/x-icon" href="/images/icon.ico">
<title>30 Days of Python</title>
<style>
{}
</style>
<script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
<script>hljs.highlightAll();</script>
{}
<footer style="height:100px; background:#000000; text-align:center; display:flex; align-items:center; justify-content:center; margin-top:40px;">
    <a href="{}" target="_blank" rel="noopener noreferrer" style="color: white; text-decoration: none; display: inline-flex; align-items: center;">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 16 16" style="margin-right: 8px;">
            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.01 8.01 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
        </svg>
        View on GitHub
    </a>
</footer>
"#,
        css,
        markdown_to_html_with_plugins(md, &options, &plugins),
        GITHUB_URL
    )
}
