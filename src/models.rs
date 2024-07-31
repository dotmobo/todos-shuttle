use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Deserialize)]
pub struct TodoNew {
    pub note: String,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub note: String,
}

#[derive(Clone)]
pub struct MyState {
    pub pool: PgPool,
}
