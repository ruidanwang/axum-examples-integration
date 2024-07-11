use axum::{extract::Query, http::{HeaderMap, StatusCode}};
use std::collections::HashMap;

pub async fn handler_get_dept() -> String{
    String::from("hello dept")
}

pub async  fn fallback() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, String::from("你访问的页面丢了哦！！！"))
}

pub async fn get_all_header(headers: HeaderMap) -> String {
    print!("{:#?}",headers);
    String::from("hello headers")
}

pub async fn get_body_string(body: String) -> String{
    String::from("Hello ") + &body
}

pub async fn get_mutil_extra(Query(params): Query<HashMap<String, String>>,headers: HeaderMap) -> String {
    println!("{:#?}",params);
    println!("{:#?}",headers);
    String::from("Hello mutil extractors")
}