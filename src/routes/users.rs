use std::any::Any;

use mongodb::bson::doc;
use mongodb::error::{Error, ErrorKind, WriteFailure};
use mongodb::options::InsertOneOptions;
use rocket::futures::{Stream, StreamExt, TryStreamExt};
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::State;
use rocket_contrib::json;

use crate::{IntlMessage, MongoDB};
use crate::models::user::{AuthTokenUser, CredentialUser, EditableUser, InsertableUser, ResponseUser, User};
use crate::structs::api_response::{ApiResponse, ApiResponseDetails};

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
    let new_user = User::from_insertable((*user).clone());
    /**
    FIXME: SECURITY POINT
    In case email already exist, we return a success response to avoid user enumeration from
    the register functionality. But this have an high impact on the UX.
    Define with the whole team, what behaviour should be adopted.
    If UX is a prior, remove the FAKE_USER creation on the user_login route
    coz it voluntary affect the app performance to avoid user enumeration from the login
    functionality.
    There is no sense to have a secure login route, but not the register route.
    If we want to return an error when email already take, we can do above
          match mongo_db.get_users_coll().insert_one(&new_user, None).await {
              Ok(_) => {
                  // return API RESPONSE OK
              }
              Err(err) => {
                  match *err.kind {
                      ErrorKind::Write(err) => {
                          match err {
                              WriteFailure::WriteError(error) => {
                                  // error code for duplicate keys (email)
                                  if error.code == 11000 {
                                      // return API RESPONSE to say email already taken
                                  }
                              }
                              _ => {
                                  // ERROR 500
                              }
                          }
                      }
                      _ => {
                          // ERROR 500
                      }
                  }
              }
          }
     */
    mongo_db.get_users_coll().insert_one(&new_user, None).await;
    ApiResponse::created(
        intl_message.get_by_intl_id("usr_created"),
        Some(vec![json!(ResponseUser::from_user(new_user))]),
    )
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
    mongo_db.get_users_coll()
        .find_one_and_replace(doc! {"_id": &filtered_updated_user._id}, &filtered_updated_user, None)
        .await;

    ApiResponse::ok(
        intl_message.get_by_intl_id("usr_edited"),
        Some(vec![json!(ResponseUser::from_user(filtered_updated_user))]),
    )
}

#[delete("/users")]
pub async fn delete_user_rt(mongo_db: &State<MongoDB>, user: User) -> ApiResponse {
    mongo_db.get_users_coll().find_one_and_delete(doc! { "_id": user._id}, None).await;
    ApiResponse::no_content()
}