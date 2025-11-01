use crate::models::User;
use axum::{extract::Path, http::StatusCode, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub age: u8,
}

pub async fn get_users() -> Json<Vec<User>> {
    Json(vec![
        User {
            id: 1,
            name: "Alice".into(),
            age: 22,
        },
        User {
            id: 2,
            name: "Bob".into(),
            age: 30,
        },
    ])
}

pub async fn get_user_by_id(Path(id): Path<u32>) -> Json<User> {
    Json(User {
        id,
        name: format!("User{}", id),
        age: 25,
    })
}

pub async fn create_user(Json(body): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 999,
        name: body.name,
        age: body.age,
    };
    (StatusCode::CREATED, Json(user))
}

pub async fn update_user(Path(id): Path<u32>, Json(body): Json<CreateUser>) -> Json<User> {
    Json(User {
        id,
        name: body.name,
        age: body.age,
    })
}

pub async fn delete_user(Path(id): Path<u32>) -> (StatusCode, String) {
    (StatusCode::NO_CONTENT, format!("Deleted user {}", id))
}
