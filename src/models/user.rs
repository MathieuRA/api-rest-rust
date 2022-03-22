use argon2::{Config, hash_encoded};
use mongodb::bson::{doc, Uuid as Uuid_mongo};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rocket::futures::TryFutureExt;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

use crate::MongoDB;

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

    fn hash_password(password: &String, salt: &String) -> String {
        match hash_encoded(password.as_bytes(), salt.as_bytes(), &Config::default()) {
            Ok(hashed_password) => hashed_password,
            // FIXME: Handle case that error triggered
            Err(err) => panic!("{:?}", err)
        }
    }

    pub fn match_password(&self, password: &String) -> bool {
        // FIXME: Handle error case
        argon2::verify_encoded(&self.password, password.as_bytes()).unwrap()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private("session_id") {
            Some(cookie) => {
                match serde_json::from_str::<AuthTokenUser>(cookie.value()) {
                    Ok(token) => {
                        let mongoDB = request.rocket().state::<MongoDB>().unwrap();
                        match mongoDB.get_users_coll()
                            .find_one(doc! {"_id": token._id}, None).await.unwrap() {
                            Some(user) => Outcome::Success(user),
                            None => Outcome::Failure((Status::NotFound, ()))
                        }
                    }
                    // Token is not following the good format.
                    Err(_) => Outcome::Failure((Status::BadRequest, ()))
                }
            }
            // No cookie found
            None => Outcome::Failure((Status::Forbidden, ()))
        }
    }
}
