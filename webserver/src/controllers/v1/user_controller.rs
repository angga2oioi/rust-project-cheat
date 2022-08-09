use actix_web::{ Responder, HttpResponse };
use actix_web::web;
use actix_web::http::StatusCode;
use crate::controllers::api_view::{ view, view_with_status };
use crate::model::user_model::{ User };
use crate::error::{ApiError};
use serde_json::{ json };

pub struct UserController;

impl UserController {
    pub async fn index() -> impl Responder {
        let response =
            json!({
            "error": 1,
            "message": "Unknown Error",
        });

        view_with_status(response.to_string(), StatusCode::from_u16(500).unwrap())
    }

    pub async fn get_id(path: web::Path<(String,)>) -> impl Responder {
        let id = path.into_inner().0;
        let response =
            json!({
            "error": 0,
            "message": "About get",
            "data":id
        });

        view(response.to_string())
    }

    pub async fn post_user(req: String) -> Result<HttpResponse, ApiError> {
        let user: User = serde_json::from_str(&req)?;

        let response =
            json!({
            "error": 0,
            "message": "About get",
            "user":user
        });

        view(response.to_string())
    }
}