use argon2::{Config, hash_encoded};
use mongodb::bson::{doc, Uuid as Uuid_mongo};
use mongodb::error::{ErrorKind, WriteFailure};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

use crate::{ApiResponseDetails, IntlMessage, MongoDB};
use crate::structs::api_response::ApiResponse;
use crate::structs::common::Optional;

type Uuid = Uuid_mongo;

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthTokenUser {
    pub _id: Uuid,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CredentialUser {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditableUser {
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUser {
    pub _id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub _id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
    pub salt: String,
}

impl AuthTokenUser {
    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_str(str: &str) -> serde_json::error::Result<AuthTokenUser> {
        serde_json::from_str::<AuthTokenUser>(str)
    }
}

impl InsertableUser {
    pub async fn insert_one(mongo_db: &MongoDB, new_user: InsertableUser) -> Result<User, ApiResponse> {
        let user = User::from_insertable(new_user);
        match mongo_db.get_users_coll().insert_one(&user, None).await {
            Ok(_) => {}
            Err(error) => {
                let intl_message = IntlMessage::new();
                let internal_error = ApiResponse::internal_error(
                    intl_message.get_by_intl_id("internal_error"),
                    None,
                );
                return match *error.kind {
                    ErrorKind::Write(err) => {
                        return match err {
                            WriteFailure::WriteError(e) => {
                                if e.code == 11000 {
                                    return Err(ApiResponse::conflict(
                                        intl_message.get_by_intl_id("email_already_exist"),
                                        None,
                                    ));
                                }
                                Err(internal_error)
                            }
                            _ => Err(internal_error)
                        };
                    }
                    _ => Err(internal_error)
                };
            }
        };
        Ok(user)
    }
}

impl ResponseUser {
    pub fn from_user(user: User) -> Self {
        ResponseUser {
            _id: user._id,
            name: user.name,
            email: user.email,
        }
    }
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        let salt: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        let hashed_password = User::hash_password(&password, &salt);

        User {
            _id: Uuid::new(),
            email,
            name,
            password: hashed_password,
            salt,
        }
    }

    pub fn from_insertable(insertable: InsertableUser) -> Self {
        User::new(insertable.name, insertable.email, insertable.password)
    }

    pub async fn from_request(request: &Request<'_>) -> Option<Self> {
        match request.cookies().get_private("session_id") {
            Some(cookie) => {
                match AuthTokenUser::from_str(cookie.value()) {
                    Ok(token) => {
                        request.rocket().state::<MongoDB>().unwrap().get_users_coll()
                            .find_one(doc! {"_id": token._id}, None).await.unwrap()
                    }
                    Err(_) => None
                }
            }
            None => None
        }
    }

    fn hash_password(password: &String, salt: &String) -> String {
        match hash_encoded(password.as_bytes(), salt.as_bytes(), &Config::default()) {
            Ok(hashed_password) => hashed_password,
            // FIXME: Handle case that error triggered
            Err(err) => panic!("{:?}", err)
        }
    }

    pub fn update(mut self, editable: EditableUser) -> Self {
        match editable.password {
            Some(pass) => self.password = User::hash_password(&pass, &self.salt),
            None => {}
        };
        match editable.email {
            Some(mail) => self.email = mail,
            None => {}
        };
        match editable.name {
            Some(name) => self.name = name,
            None => {}
        };
        self
    }

    pub fn match_password(&self, password: &String) -> bool {
        argon2::verify_encoded(&self.password, password.as_bytes()).unwrap()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Optional<User> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Optional {
            some: User::from_request(request).await
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match User::from_request(request).await {
            Some(user) => Outcome::Success(user),
            None => {
                let intl_message = request.rocket().state::<IntlMessage>().unwrap()
                    .get_by_intl_id("authentication_required");
                request.local_cache(|| ApiResponseDetails {
                    intl_id: intl_message.0,
                    reason: intl_message.1,
                    data: None,
                });
                Outcome::Failure((Status::Forbidden, ()))
            }
        }
    }
}


