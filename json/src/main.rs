use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_user_handler));
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user_handler() -> Result<axum::Json<User>, axum::http::StatusCode> {
    let user = User {
        id: 1,
        name: "Ivan".to_string(),
        email: "irayspacii@gmail.com".to_string(),
    };
    Ok(axum::Json(user))
}
