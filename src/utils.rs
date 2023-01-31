use std::{error::Error};
use http_body::Full;
use hyper::body::Bytes;
use hyper::{Response , StatusCode};

pub async fn get_file(filename: &str) -> Result<Response<Full<Bytes>>,Box<dyn Error>> {
    if let Ok(contents) = tokio::fs::read(filename).await {
        let body = contents.into();
        return Ok(Response::new(Full::new(body)));
    }
    Ok(not_found().await)
}

pub async fn not_found() -> Response<Full<Bytes>>{
    if let Ok(contents) = tokio::fs::read("public/404.html").await {
        let body = contents.into();
        return Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(body))
        .unwrap()
    }
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new("not found".into()))
        .unwrap()
}

