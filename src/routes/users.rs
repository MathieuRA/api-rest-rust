use rocket::serde::json::Json;
use rocket::State;

use crate::models::user::{InsertableUser, User};
use crate::MongoDB;

#[post("/users", format = "json", data = "<user>")]
pub async fn new_user_rt(mongo_db: &State<MongoDB>, user: Json<InsertableUser>) -> Json<User> {
    let new_user = User::from_insertable((*user).clone());
    mongo_db.get_users_col()
        .insert_one(&new_user, None)
        .await
        .unwrap();
    Json(new_user)
}