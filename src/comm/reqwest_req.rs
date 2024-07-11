

use serde::{Deserialize, Serialize};
use reqwest;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

/**
 * reqwest 返回的是枚举，关键是处理好枚举信息
 */



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

#[derive( Serialize, Deserialize)]
pub struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}



pub async fn typed_json() -> Json<Post>{
    let new_post = Post {
        id: None,
        title: "Reqwest.rs".into(),
        body: "https://docs.rs/reqwest".into(),
        user_id: 1,
    };
    let new_post: Post = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await
        .unwrap()
        .json()
        .await.unwrap();
    Json(new_post)
} 

pub async fn typed_json_resp() -> Result<Json<Post>, ErrorResponse> {
    let new_post = Post {
        id: None,
        title: "Reqwest.rs".into(),
        body: "https://docs.rs/reqwest".into(),
        user_id: 1,
    };
    match reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await
        {
            Ok(res) => {
                if res.status().is_success(){
                    match res.json().await {
                        Ok(post) =>{
                            return Ok(Json(post));
                        },
                        Err(err) => {
                            let errors = err.to_string();
                            return Err(ErrorResponse{errors});
                            
                        }
                    }
                }else{
                    let errors = String::from("请求异常");
                    return Err(ErrorResponse{errors});
                }
            },
            Err(err) => {
                let errors = err.to_string();
                return Err(ErrorResponse{errors});
            }
        };
}    