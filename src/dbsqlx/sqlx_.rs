use axum::{
    extract::State,
    http::StatusCode,
    
};
use serde::{Deserialize,Serialize};
use validator::Validate;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use axum::{response::{IntoResponse, Response}, Json};


#[derive(Deserialize,Serialize,Validate,FromRow)]
pub struct Users{
    id:i32,
    name:String,
    hair_color:String
}

pub struct ErrorResponse {
    errors: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "errors": self.errors,
        }));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

pub async fn get_one(State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

pub async fn get_many(State(pool): State<PgPool>) -> Result<Json<Vec<Users>>,ErrorResponse> {
    let user = sqlx::query_as::<_, Users>("SELECT * FROM users ")
        .fetch_all(&pool)
        .await;
    match user {
        Ok(user)=>{
            Ok(Json(user))
        },
        Err(err)=>{
            let errors = err.to_string();
            return Err(ErrorResponse{errors});
        }
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}