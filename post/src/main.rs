use axum::{routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(posts_create));
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn posts_create(
    axum::Json(params): axum::Json<CreatePost>,
) -> Result<axum::Json<Post>, axum::http::StatusCode> {
    let post = Post {
        id: 0,
        title: params.title,
        content: params.content,
    };
    Ok(axum::Json(post))
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct CreatePost {
    title: String,
    content: String,
}
