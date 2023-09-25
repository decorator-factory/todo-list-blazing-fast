use std::convert::Infallible;

use axum::response::IntoResponse;

pub trait WebError {
    fn code(&self) -> &'static str;

    fn status(&self) -> axum::http::StatusCode {
        axum::http::StatusCode::BAD_REQUEST
    }

    fn detail(&self) -> serde_json::Value {
        serde_json::Value::Null
    }
}

impl WebError for Infallible {
    fn code(&self) -> &'static str {
        unimplemented!()
    }
}

impl WebError for &'static str {
    fn code(&self) -> &'static str {
        self
    }
}

#[derive(Debug)]
pub enum Answer<T, E = Infallible> {
    Ok(T),
    Err(E),
}

impl<T: serde::Serialize, E: WebError> IntoResponse for Answer<T, E> {
    fn into_response(self) -> axum::response::Response {
        let (body, status) = match self {
            Answer::Ok(value) => (
                serde_json::json!({
                    "status": "ok",
                    "payload": value
                }),
                axum::http::StatusCode::OK,
            ),
            Answer::Err(e) => (
                serde_json::json!({
                    "status": "error",
                    "error": e.code(),
                    "detail": e.detail(),
                }),
                e.status(),
            ),
        };

        (status, axum::Json(body)).into_response()
    }
}
