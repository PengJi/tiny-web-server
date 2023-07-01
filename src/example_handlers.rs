use std::collections::HashMap;
use log::{debug, info, warn, error};

use axum::{
    body::{self, Body},
    debug_handler,
    extract::{Form, Path, Query},
    http::header::{HeaderMap, HeaderName, HeaderValue},
    response::{Html, IntoResponse, Json, Response},
};
use http::{StatusCode, Uri};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// ==================== extract from url path ====================

// eg: /user/30
#[debug_handler]
pub async fn user_info(Path(id): Path<i32>) -> String {
    format!("user id: {}", id)
}

// eg: /user2/30
#[debug_handler]
pub async fn user_info_2(id: Path<i32>) -> String {
    format!("user id: {}", id.0)
}

// eg: /person/123/30, get: id: 123, age: 30
#[debug_handler]
pub async fn person(Path((id, age)): Path<(i32, i32)>) -> String {
    format!("id: {}, age: {}", id, age)
}

#[derive(Deserialize)]
pub struct SomeRequest {
    a: String,
    b: i32,
    c: String,
    d: u32,
}

// eg: path_req/a1/b1/c1/d1
#[debug_handler]
pub async fn path_req(Path(req): Path<SomeRequest>) -> String {
    format!("a: {}, b: {}, c: {}, d: {}", req.a, req.b, req.c, req.d)
}

// ==================== extract from query string ====================

//eg: query_req/?a=test&b=2&c=abc&d=80
#[debug_handler]
pub async fn query_req(Query(args): Query<SomeRequest>) -> String {
    format!("a: {}, b: {}, c: {}, d: {}", args.a, args.b, args.c, args.d)
}

#[derive(Deserialize)]
pub struct SomeRequest2 {
    a: Option<String>,
    b: Option<i32>,
    c: Option<String>,
    d: Option<u32>,
}

//eg: query_req2?a=abc&c=中华人民共和国&d=123
#[debug_handler]
pub async fn query_req2(Query(args): Query<SomeRequest2>) -> String {
    format!(
        "a: {}, b: {}, c: {}, d: {}",
        args.a.unwrap_or_default(),
        args.b.unwrap_or(-1), // default value for b is -1
        args.c.unwrap_or_default(),
        args.d.unwrap_or_default()
    )
}

//eg: query?a=1&b=1.0&c=xxx
#[debug_handler]
pub async fn query(Query(params): Query<HashMap<String, String>>) -> String {
    for (key, value) in &params {
        println!("key: {}, value: {}", key, value);
    }
    format!("{:?}", params)
}

// ==================== extract from form ====================

// 表单提交
#[debug_handler]
pub async fn form_request(Form(model): Form<SomeRequest2>) -> String {
    format!(
        "a:{},b:{},c:{},d:{}",
        model.a.unwrap_or_default(),
        model.b.unwrap_or(-1), //b缺省值指定为-1
        model.c.unwrap_or_default(),
        model.d.unwrap_or_default()
    )
}

// ==================== extract from json ====================

#[debug_handler]
pub async fn json_request(Json(model): Json<SomeRequest>) -> String {
    format!(
        "a: {}, b: {}, c: {}, d:{}",
        model.a, model.b, model.c, model.d
    )
}

// ==================== extract http header ====================

// get all header
#[debug_handler]
pub async fn get_all_header(headers: HeaderMap) -> String {
    for (key, value) in &headers {
        println!("key: {:?} , value: {:?}", key, value);
    }
    format!("{:?}", headers)
}

// ==================== read or write cookie ====================

// set cookie and redirect
#[debug_handler]
pub async fn set_cookie_and_redirect(mut headers: HeaderMap) -> (StatusCode, HeaderMap, ()) {
    //设置cookie，blog_url为cookie的key
    headers.insert(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str("blog_url=http://yjmyzz.cnblogs.com/").unwrap(),
    );

    //重设LOCATION，跳到新页面
    headers.insert(
        axum::http::header::LOCATION,
        HeaderValue::from_str("/get_cookie").unwrap(),
    );
    //302重定向
    (StatusCode::FOUND, headers, ())
}

// get cookie
#[debug_handler]
pub async fn get_cookie(headers: HeaderMap) -> (StatusCode, String) {
    //读取cookie，并转成字符串
    let cookies = headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or("".to_string());

    //cookie空判断
    if cookies.is_empty() {
        println!("cookie is empty!");
        return (StatusCode::OK, "cookie is empty".to_string());
    }

    //将cookie拆成列表
    let cookies: Vec<&str> = cookies.split(';').collect();
    println!("{:?}", cookies);
    for cookie in &cookies {
        //将内容拆分成k=v的格式
        let cookie_pair: Vec<&str> = cookie.split('=').collect();
        if cookie_pair.len() == 2 {
            let cookie_name = cookie_pair[0].trim();
            let cookie_value = cookie_pair[1].trim();
            println!("{:?}", cookie_pair);
            //判断其中是否有刚才设置的blog_url
            if cookie_name == "blog_url" && !cookie_value.is_empty() {
                println!("found:{}", cookie_value);
                return (StatusCode::OK, cookie_value.to_string());
            }
        }
    }
    return (StatusCode::OK, "empty".to_string());
}

// We've already seen returning &'static str
#[debug_handler]
pub async fn plain_text() -> &'static str {
    "foo"
}

// String works too and will get a `text/plain; charset=utf-8` content-type
#[debug_handler]
pub async fn plain_text_string(uri: Uri) -> String {
    format!("Hi from {}", uri.path())
}

// Bytes will get a `application/octet-stream` content-type
#[debug_handler]
pub async fn bytes() -> Vec<u8> {
    vec![1, 2, 3, 4]
}

// `()` gives an empty response
#[debug_handler]
pub async fn empty() {}

// `StatusCode` gives an empty response with that status code
#[debug_handler]
pub async fn empty_with_status() -> StatusCode {
    StatusCode::NOT_FOUND
}

// A tuple of `StatusCode` and something that implements `IntoResponse` can
// be used to override the status code
#[debug_handler]
pub async fn with_status() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
}

// A tuple of `HeaderMap` and something that implements `IntoResponse` can
// be used to override the headers
#[debug_handler]
pub async fn with_headers() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-foo"),
        HeaderValue::from_static("foo"),
    );
    (headers, "foo")
}

// You can also override both status and headers at the same time
#[debug_handler]
pub async fn with_headers_and_status() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-foo"),
        HeaderValue::from_static("foo"),
    );
    (StatusCode::INTERNAL_SERVER_ERROR, headers, "foo")
}

// `Html` gives a content-type of `text/html`
#[debug_handler]
pub async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
#[debug_handler]
pub async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

// `Result<T, E>` where `T` and `E` implement `IntoResponse` is useful for
// returning errors
#[debug_handler]
pub async fn result() -> Result<&'static str, StatusCode> {
    Ok("all good")
}

// `Response` gives full control
#[debug_handler]
pub async fn response() -> Response<Body> {
    Response::builder().body(Body::empty()).unwrap()
}

#[derive(Serialize)]
pub struct Blog {
    title: String,
    author: String,
    summary: String,
}

#[debug_handler]
pub async fn blog_struct() -> Json<Blog> {
    let blog = Blog {
        title: "axum笔记(2)-response".to_string(),
        author: "菩提树下的杨过".to_string(),
        summary: "response各种示例".to_string(),
    };
    Json(blog)
}

#[debug_handler]
pub async fn blog_struct_cn() -> (HeaderMap, Json<Blog>) {
    let blog = Blog {
        title: "axum笔记(2)-response".to_string(),
        author: "菩提树下的杨过".to_string(),
        summary: "response各种示例".to_string(),
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    (headers, Json(blog))
}

pub struct CustomError {
    msg: String,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let body = body::boxed(body::Full::from(self.msg));
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body)
            .unwrap()
    }
}

#[debug_handler]
pub async fn custom_error() -> Result<&'static str, CustomError> {
    Err(CustomError {
        msg: "Opps!".to_string(),
    })
}
