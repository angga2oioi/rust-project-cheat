use actix_web::{ HttpResponse, HttpRequest };
use actix_web::web;
use crate::controllers::api_view::{ view };
use crate::model::user_model::{ User };
use crate::error::{ ApiError };
use serde_json::{ json };
use qstring::QString;

pub struct UserController;

impl UserController {
    pub async fn index() -> Result<HttpResponse, ApiError> {
        Err(ApiError::InternalError)
    }

    pub async fn get_id(req: HttpRequest) -> Result<HttpResponse, ApiError> {
        let id = req.match_info().query("id").parse::<String>().unwrap(); // dynamic routing
        let query_str = req.query_string(); // query string
        let qs = QString::from(query_str);
        let field = qs.get("field").unwrap();

        println!("{:?}", field);

        Err(ApiError::InternalError)
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