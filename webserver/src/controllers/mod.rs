pub mod v1;

mod api_view
{
    use actix_web::{HttpResponse};
    use actix_web::http::StatusCode;

    use crate::error::{ApiError};

    pub fn view_with_status<S: Into<String>>(body: S, status: StatusCode) -> Result<HttpResponse, ApiError>
    {
        Ok(HttpResponse::build(status).content_type("application/json").body(body.into()))
    }

    pub fn view<S: Into<String>>(data: S) -> Result<HttpResponse, ApiError>
    {
        view_with_status(data, StatusCode::OK)
    }
}

pub mod handlers
{
    use actix_web::{HttpResponse};
    use actix_web::http::{StatusCode};
    use serde_json::json;
    use crate::error::{ApiError};

    pub async fn page_not_found() -> Result<HttpResponse, ApiError>
    {
        let response = json!({
            "error": 1,
            "message": "Not Found",
        });
        super::api_view::view_with_status( response.to_string(), StatusCode::NOT_FOUND)
    }

    

    //@todo implement 500 handler
}
