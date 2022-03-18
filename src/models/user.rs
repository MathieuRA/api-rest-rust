use argon2::{Config, hash_encoded};
use mongodb::bson::Uuid;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub _id: Uuid,
    pub email: String,
    pub name: String,
    pub password: String,
    pub salt: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableUser {
    pub name: String,
    pub email: String,
    pub password: String,
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
}