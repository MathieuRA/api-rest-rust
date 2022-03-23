extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use crate::database::mongo::MongoDB;
use crate::routes::users;

mod database;
pub mod routes;
mod models;
mod structs;

#[rocket::main]
async fn main() {
    dotenv().ok();
    rocket::build()
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