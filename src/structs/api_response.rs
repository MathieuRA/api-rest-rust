use rocket::{Request, response, Response};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: JsonValue,
}

impl ApiResponse {
    pub fn ok(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Ok,
            message,
        }
    }
    pub fn created(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Created,
            message,
        }
    }
    pub fn no_content() -> Self {
        ApiResponse {
            status: Status::NoContent,
            message: json!({}),
        }
    }
    pub fn not_found(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::NotFound,
            message,
        }
    }
    pub fn forbidden(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Forbidden,
            message,
        }
    }

    pub fn err(message: JsonValue) -> Self {
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