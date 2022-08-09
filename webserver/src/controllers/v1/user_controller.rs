use actix_web::{ HttpResponse, HttpRequest };
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web::web;
use crate::controllers::api_view::{ view };
use crate::model::user_model::{ User, UserQueryParams };
use crate::error::{ ApiError };
use serde_json::{ json };
use serde_qs as qs;

pub struct UserController;

impl UserController {
    pub async fn index(req: HttpRequest) -> Result<HttpResponse, ApiError> {
        let query_str = req.query_string(); // query string
        let params: UserQueryParams = qs::from_str(query_str).unwrap();

        let response = json!({
            "params":params
        });

        println!("{:?}", params);

        view(response.to_string())
    }

    pub async fn get_user_by_id(req: HttpRequest) -> Result<HttpResponse, ApiError> {
        let id = req.match_info().query("id").parse::<String>().unwrap(); // dynamic routing

        let response = json!({
            "id":id,
        });

        println!("{:?}", id);

        view(response.to_string())
    }

    pub async fn post_user(req: HttpRequest,body : String,auth: BearerAuth) -> Result<HttpResponse, ApiError> {
        
        let user: User = serde_json::from_str(&body)?;

        let response =
            json!({
            "error": 0,
            "message": "About get",
            "user":user,
            "token":auth.token()
        });

        view(response.to_string())
    }
}