use axum::{
    extract::{State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[tokio::main]
async fn main() {
    let db = Db::default();
    let app = Router::new()
        .route("/tasks", get(tasks_index).post(tasks_create))
        .with_state(db);
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn tasks_index(State(db): State<Db>) -> impl IntoResponse {
    let tasks = db.read().unwrap();
    let tasks_json = tasks.values().cloned().collect::<Vec<_>>();
    Json(tasks_json)
}

#[derive(Deserialize)]
struct CreateTask {
    text: String,
}

async fn tasks_create(State(db): State<Db>, Json(input): Json<CreateTask>) -> impl IntoResponse {
    let tasks = db.read().unwrap();
    let task = Task {
        id: tasks.len() as u64,
        text: input.text,
    };

    // это необходимо для того, чтобы
    // освободить захваченный мьютекс RwLockReadGuard
    // и разрешить другим потокам получить доступ к данным
    drop(tasks);

    db.write().unwrap().insert(task.id, task.clone());
    (StatusCode::CREATED, Json(task))
}

type Db = Arc<RwLock<HashMap<u64, Task>>>;

#[derive(Debug, Serialize, Clone)]
struct Task {
    id: u64,
    text: String,
}
