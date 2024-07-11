use axum::http::HeaderMap;
use axum::http::header::{self};


pub async fn get_new_headers(header:HeaderMap) ->HeaderMap{
    let mut headers = HeaderMap::new();
    headers.clone_from(&header);
    headers.insert(header::SERVER, "axum".parse().unwrap());
    headers.insert("PrivateKey", "some-value".parse().unwrap());
    headers
}