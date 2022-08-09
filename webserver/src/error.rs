use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::Serialize;

// Struct which defines the structure of the error response
#[derive(Serialize)]
struct FormattedErrorResponse {
    error: u16,
    message: String,
}

// Custom data type for the error response
#[derive(Debug, Display, Error)]
pub enum ApiError {
    // Formatting the validation error message
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
    // Formatting the internatal server error error message
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    // Formatting the bad request error message
    #[display(fmt = "Bad request")]
    BadClientData,
    #[display(fmt = "Not found!")]
    NotFound,
}

impl ApiError {
    fn name(&self) -> String {
        match self {
            ApiError::ValidationError { .. } => "Validation Error".to_string(),
            ApiError::InternalError => "Internal Server Error".to_string(),
            ApiError::BadClientData => "Bad request".to_string(),
            ApiError::NotFound => "Not found".to_string(),
        }
    }
}

// Implement ResponseError trait for the custom struct
impl ResponseError for ApiError {
    // Function to generate the error response
    fn error_response(&self) -> HttpResponse {
        let error_response = FormattedErrorResponse {
            //status_code: self.status_code().as_u16(),
            //error: self.to_string(),
            error: self.status_code().as_u16(),
            message: self.name(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
    // Function to generate the error code
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ApiError::BadClientData => StatusCode::BAD_REQUEST,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> ApiError {
        ApiError::ValidationError{field:err.to_string()}
    }
}




