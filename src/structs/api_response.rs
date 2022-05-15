
use std::fmt::Debug;

use rocket::{Request, response, Response};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;

use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: ApiResponseDetails,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResponseDetails {
    pub intl_id: String,
    pub reason: String,
    pub data: Option<Vec<JsonValue>>,
}

impl ApiResponse {
    pub fn ok(message: (String, String), data: Option<Vec<JsonValue>>) -> Self {
        ApiResponse {
            status: Status::Ok,
            message: ApiResponseDetails {
                intl_id: message.0,
                reason: message.1,
                data,
            },
        }
    }
    pub fn conflict(message: (String, String), data: Option<Vec<JsonValue>>) -> Self {
        ApiResponse {
            status: Status::Conflict,
            message: ApiResponseDetails {
                intl_id: message.0,
                reason: message.1,
                data,
            },
        }
    }
    pub fn created(message: (String, String), data: Option<Vec<JsonValue>>) -> Self {
        ApiResponse {
            status: Status::Created,
            message: ApiResponseDetails {
                intl_id: message.0,
                reason: message.1,
                data,
            },
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
    pub fn not_found(message: (String, String), data: Option<Vec<JsonValue>>) -> Self {
        ApiResponse {
            status: Status::NotFound,
            message: ApiResponseDetails {
                intl_id: message.0,
                reason: message.1,
                data,
            },
        }
    }
    pub fn forbidden(message: (String, String), data: Option<Vec<JsonValue>>) -> Self {
        ApiResponse {
            status: Status::Forbidden,
            message: ApiResponseDetails {
                intl_id: message.0,
                reason: message.1,
                data,
            },
        }
    }

    pub fn internal_error(message: (String, String), data: Option<Vec<JsonValue>>) -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: ApiResponseDetails {
                intl_id: message.0,
                reason: message.1,
                data,
            },
        }
    }

    pub fn get_content(self) -> ApiResponseDetails {
        self.message
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