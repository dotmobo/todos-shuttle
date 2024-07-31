use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{delete, get, post},
    Json, Router,
};
use sqlx::PgPool;
use tokio::fs::read_to_string;

mod models;
use models::{Todo, TodoNew, MyState};

// Add a function to retrieve all todos
async fn retrieve_all(
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&state.pool)
        .await
    {
        Ok(todos) => Ok((StatusCode::OK, Json(todos))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn retrieve(
    Path(id): Path<i32>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn add(
    State(state): State<MyState>,
    Json(data): Json<TodoNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("INSERT INTO todos (note) VALUES ($1) RETURNING id, note")
        .bind(&data.note)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

// function to remove a todo
async fn remove(
    Path(id): Path<i32>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn serve_index() -> impl IntoResponse {
    match read_to_string("templates/index.html").await {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load HTML file").into_response(),
    }
}


#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = MyState { pool };
    let router = Router::new()
        .route("/", get(serve_index))
        .route("/todos", get(retrieve_all))
        .route("/todos", post(add))
        .route("/todos/:id", get(retrieve))
        .route("/todos/:id", delete(remove))
        .with_state(state);

    Ok(router.into())
}