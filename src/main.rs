extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use crate::database::mongo::MongoDB;
use crate::routes::{matchs, users};

mod database;
mod routes;
mod models;
mod structs;

#[rocket::main]
async fn main() {
    dotenv().ok();
    rocket::build()
        .mount("/api/v1", routes![
            matchs::matchs_rt,
            matchs::post_matchs_rt,
            users::new_user_rt
        ])
        .manage(MongoDB::new("rust-api").await)
        .launch()
        .await.expect("Rocket build instance");
}