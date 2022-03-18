use rocket::State;
use rocket::serde::json::Json;
use rocket_contrib::json;

use crate::models::user::{InsertableUser, ResponseUser, User};
use crate::MongoDB;
use crate::structs::api_response::ApiResponse;

#[post("/users", format = "json", data = "<user>")]
pub async fn new_user_rt(mongo_db: &State<MongoDB>, user: Json<InsertableUser>) -> ApiResponse {
    let new_user = User::from_insertable((*user).clone());
    mongo_db.get_users_col()
        .insert_one(&new_user, None)
        .await
        .unwrap();
    ApiResponse::ok(json!(ResponseUser::from_user(new_user)))
}