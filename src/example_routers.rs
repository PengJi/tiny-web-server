use axum::routing::{get, post};
use axum::Router;
use log::{debug, info, warn, error};

use crate::example_handlers::*;

pub fn get_router() -> Router {
    let app = Router::new()
        // request path
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user/:id", get(user_info))
        .route("/user2/:id", get(user_info_2))
        .route("/person/:id/:age", get(person))
        .route("/path_req/:a/:b/:c/:d", get(path_req))
        .route("/query_req", get(query_req))
        .route("/query_req2", get(query_req2))
        .route("/query", get(query))
        .route("/form", post(form_request))
        .route("/json", post(json_request))
        .route("/header", get(get_all_header))
        .route("/set_cookie", get(set_cookie_and_redirect))
        .route("/get_cookie", get(get_cookie))
        // response
        .route("/plain_text", get(plain_text))
        .route("/plain_text_string", get(plain_text_string))
        .route("/bytes", get(bytes))
        .route("/empty", get(empty))
        .route("/empty_with_status", get(empty_with_status))
        .route("/with_status", get(with_status))
        .route("/with_headers", get(with_headers))
        .route("/with_headers_and_status", get(with_headers_and_status))
        .route("/html", get(html))
        .route("/json", get(json))
        .route("/result", get(result))
        .route("/response", get(response))
        .route("/blog", get(blog_struct))
        .route("/blog_cn", get(blog_struct_cn))
        .route("/custom_error", get(custom_error));
    app
}
