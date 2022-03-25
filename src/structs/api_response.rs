use std::any::Any;
use std::fmt::Debug;

use rocket::{Request, response, Response};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: ApiResponseDetails,
}

#[derive(Debug, Serialize, Clone)]
pub struct ApiResponseDetails {
    pub intl_id: String,
    pub reason: String,
    pub data: Option<Vec<JsonValue>>,
}

impl ApiResponse {
    pub fn ok(message: ApiResponseDetails) -> Self {
        ApiResponse {
            status: Status::Ok,
            message,
        }
    }
    pub fn created(message: ApiResponseDetails) -> Self {
        ApiResponse {
            status: Status::Created,
            message,
        }
    }
    pub fn no_content() -> Self {
        ApiResponse {
            status: Status::NoContent,
            message: ApiResponseDetails {
                intl_id: "no_content".to_string(),
                reason: "No content.".to_string(),
                data: None,
            },
        }
    }
    pub fn not_found(message: ApiResponseDetails) -> Self {
        ApiResponse {
            status: Status::NotFound,
            message,
        }
    }
    pub fn forbidden(message: ApiResponseDetails) -> Self {
        ApiResponse {
            status: Status::Forbidden,
            message,
        }
    }

    pub fn internal_error(message: ApiResponseDetails) -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message,
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let body = serde_json::to_string(&self.message).unwrap();
        Response::build_from(body.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}