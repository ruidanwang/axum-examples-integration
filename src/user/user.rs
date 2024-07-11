
use axum::{extract::{Path, Query, Json},response::{IntoResponse, Response},http::StatusCode,};  
use std::collections::HashMap;  
use serde::{Deserialize,Serialize};
use validator::Validate;



//接收和返回如果是同一个对象需要进行序列化和反序列化，引入宏
#[derive(Deserialize,Serialize,Validate)]
pub struct User {
    id:u32,
    #[validate(length(min = 6, message = "name is length small than 6"))]
    username:String,
    #[validate(range(min = 18, max = 35,message = "年龄在18-35"))]
    age:u8
}


  
// Path路径，eg. /users/<id>  
pub async fn get_path_parm(Path(user_id): Path<u32>) -> String  {
    String::from("Hello ")+&user_id.to_string()
}  

pub async fn get_path_parm_mtil(Path(parms):Path<HashMap<String,String>>) -> String {
    let id = parms.get("id").unwrap();
    let name = parms.get("name").unwrap();
    String::from("hello ") + &id + &name
}
  
// Query参数，eg. /users?id=123&name=jim  
pub async fn get_query_parm(Query(params): Query<HashMap<String, String>>) -> String {
    print!("{:#?}",params);
    String::from("Hello Query Parm:")+&params.get("id").unwrap()+&params.get("name").unwrap()
    
}

// // Json 格式参数，一般用于 POST 请求  
pub async fn json(Json(obj): Json<User>) ->String{
    String::from("Hello Json")+&obj.id.to_string()+&obj.username+&obj.age.to_string()
}

 pub struct ValidationErrorResponse {
    errors: Vec<String>,
}

impl IntoResponse for ValidationErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "errors": self.errors,
        }));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

pub async fn validatejson(Json(obj): Json<User>) -> Result<Json<User>, ValidationErrorResponse> {
    if let Err(e) = obj.validate() {
        let errors = e.field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!("{}: {}", field, error.message.as_ref().unwrap())
                })
            })
            .collect();

        return Err(ValidationErrorResponse { errors });
    }

    // 处理通过验证的数据
    Ok(Json(obj))
}

pub async fn handle_list() -> &'static str {
    "Hello, user!"
}

pub async fn get_user(Path(user_id): Path<u32>) -> Json<User> {
    let user = User{
        id:user_id,
        username:String::from("wdq"),
        age:20,
    };
    Json(user)
}