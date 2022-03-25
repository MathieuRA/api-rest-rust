extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::http::Status;
use rocket::Request;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};

use crate::database::mongo::MongoDB;
use crate::routes::users;
use crate::structs::api_response::{ApiResponse, ApiResponseDetails};

mod database;
pub mod routes;
mod models;
mod structs;

#[catch(403)]
fn forbidden_error(req: &Request) -> ApiResponse {
    // local_cache is checking type value. He do not have "key":"value" system.
    // In case he do not found same value type in local_cache, he will store and return the given value
    // https://api.rocket.rs/v0.5-rc/rocket/request/macro.local_cache.html
    ApiResponse::forbidden(req.local_cache(|| ApiResponseDetails {
        intl_id: "forbidden_operation".to_string(),
        reason: "Forbidden operation.".to_string(),
        data: None,
    }).to_owned())
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    rocket::build()
        .register("/", catchers![forbidden_error])
        .mount("/api/v1", routes![
            users::get_user_rt,
            users::new_user_rt,
            users::login_user_rt,
            users::delete_user_rt,
            users::edit_user_rt
        ])
        .manage(MongoDB::new("rust-api").await)
        .launch()
        .await.expect("Rocket build instance");
}