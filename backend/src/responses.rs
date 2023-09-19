use std::convert::Infallible;

use actix_web::{web, Responder};

pub trait WebError {
    fn code(&self) -> &'static str;

    fn status(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
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

impl<T: serde::Serialize, E: WebError> Responder for Answer<T, E> {
    type Body = <web::Json<T> as Responder>::Body;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let (body, status) = match self {
            Answer::Ok(value) => (
                web::Json(serde_json::json!({
                    "status": "ok",
                    "payload": value
                })),
                actix_web::http::StatusCode::OK,
            ),
            Answer::Err(e) => (
                web::Json(serde_json::json!({
                    "status": "error",
                    "error": e.code(),
                    "detail": e.detail(),
                })),
                e.status(),
            ),
        };

        (body, status).respond_to(req)
    }
}
