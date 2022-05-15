extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate rocket;


use dotenv::dotenv;
use rocket::{Build, Request, Rocket};

use crate::database::mongo::MongoDB;
use crate::intl_message::IntlMessage;
use crate::routes::{ping, user};
use crate::structs::api_response::{ApiResponse, ApiResponseDetails};

mod database;
pub mod routes;
mod models;
mod structs;
mod intl_message;
#[cfg(test)]
mod tests;

#[catch(403)]
fn forbidden_error(req: &Request) -> ApiResponse {
    // local_cache is checking type value. He do not have "key":"value" system.
    // In case he do not found same value type in local_cache, he will store and return the given value
    // https://api.rocket.rs/v0.5-rc/rocket/request/macro.local_cache.html
    let intl_message = req.rocket().state::<IntlMessage>().unwrap()
        .get_by_intl_id("forbidden_operation");
    let api = req.local_cache(|| ApiResponseDetails {
        intl_id: intl_message.0,
        reason: intl_message.1,
        data: None,
    }).to_owned();
    ApiResponse::forbidden((api.intl_id, api.reason), api.data)
}

#[rocket::main]
async fn main() {
    dotenv().ok();
    rocket_builder()
        .await.launch()
        .await.expect("Rocket build instance");
}

async fn rocket_builder() -> Rocket<Build> {
    rocket::build()
        .register("/", catchers![forbidden_error])
        .mount("/", routes![
            ping::ping_rt
        ])
        .mount("/api/v1", routes![
            user::get_user_rt,
            user::new_user_rt,
            user::login_user_rt,
            user::delete_user_rt,
            user::edit_user_rt
        ])
        .manage(IntlMessage::new())
        .manage(MongoDB::new("rust-api").await)
}