

use mongodb::bson::doc;



use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::State;
use rocket_contrib::json;

use crate::{IntlMessage, MongoDB};
use crate::models::user::{AuthTokenUser, CredentialUser, EditableUser, InsertableUser, ResponseUser, User};
use crate::structs::api_response::{ApiResponse};

const FAKE_EMAIL: &str = "to_prevent_time_based_account_enumeration";
const FAKE_PASSWORD: &str = "$argon2i$v=19$m=4096,t=3,p=1$TWJKeTdoZ3pPWDdaS2dNTnpZN2g$TkFyv+ZHQVlER2hWlMBnq+DHTJvanckCTgx+ULeObAs";

#[get("/users/<id>")]
pub async fn get_user_rt(intl_message: &State<IntlMessage<'_>>, id: Uuid, user: User) -> ApiResponse {
    if user._id.to_string() != id.to_string() {
        ApiResponse::forbidden(
            intl_message.get_by_intl_id("forbidden_operation"),
            None,
        )
    } else {
        ApiResponse::ok(
            intl_message.get_by_intl_id("usr_founded"),
            Some(vec![json!(ResponseUser::from_user(user))]),
        )
    }
}

#[post("/users", format = "json", data = "<user>")]
pub async fn new_user_rt(mongo_db: &State<MongoDB>, intl_message: &State<IntlMessage<'_>>, user: Json<InsertableUser>) -> ApiResponse {
    match InsertableUser::insert_one(mongo_db, user.into_inner()).await {
        Ok(new_user) =>
            ApiResponse::created(
                intl_message.get_by_intl_id("usr_created"),
                Some(vec![json!(ResponseUser::from_user(new_user))]),
            ),
        Err(err) => err
    }
}

#[post("/users/login", format = "json", data = "<credential>")]
pub async fn login_user_rt(mongo_db: &State<MongoDB>, intl_message: &State<IntlMessage<'_>>, cookies: &CookieJar<'_>, credential: Json<CredentialUser>) -> ApiResponse {
    let user = match mongo_db.get_users_coll()
        .find_one(doc! { "name" : &credential.name }, None).await.unwrap() {
        Some(user) => user,
        None =>
            User {
                _id: Default::default(),
                email: FAKE_EMAIL.to_string(),
                name: "".to_string(),
                password: FAKE_PASSWORD.to_string(),
                salt: "".to_string(),
            }
    };

    if user.match_password(&credential.password) && user.email != FAKE_EMAIL.to_string() {
        // Rocket ensure cookie cannot be tamperer
        // https://api.rocket.rs/master/rocket/http/struct.CookieJar.html#private-cookies
        cookies.add_private(
            Cookie::build(
                "session_id",
                AuthTokenUser { _id: user._id }.to_string(),
            )
                .path("/")
                // Only for production
                //.secure(true)
                .http_only(true)
                .finish()
        );
        ApiResponse::ok(
            intl_message.get_by_intl_id("authentication_success"),
            Some(vec![json!(ResponseUser::from_user(user))]),
        )
    } else {
        ApiResponse::not_found(
            intl_message.get_by_intl_id("authentication_failed"),
            None,
        )
    }
}

#[put("/users", format = "json", data = "<updated_user>")]
pub async fn edit_user_rt(mongo_db: &State<MongoDB>, intl_message: &State<IntlMessage<'_>>, user: User, updated_user: Json<EditableUser>) -> ApiResponse {
    let filtered_updated_user = user.update((*updated_user).clone());
    // TODO: Handle error possibility
    mongo_db.get_users_coll()
        .find_one_and_replace(doc! {"_id": &filtered_updated_user._id}, &filtered_updated_user, None)
        .await
        .unwrap();

    ApiResponse::ok(
        intl_message.get_by_intl_id("usr_edited"),
        Some(vec![json!(ResponseUser::from_user(filtered_updated_user))]),
    )
}

#[delete("/users")]
pub async fn delete_user_rt(mongo_db: &State<MongoDB>, user: User) -> ApiResponse {
    // TODO: Must throw an error in case the deletion throw
    mongo_db.get_users_coll().find_one_and_delete(doc! { "_id": user._id}, None).await.unwrap();
    ApiResponse::no_content()
}