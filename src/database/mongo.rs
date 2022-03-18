use mongodb::{Client, Collection, Database};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;

use crate::models::user::User;

pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    pub async fn new(db_name: &str) -> Self {
        let mut client_options = ClientOptions::parse(dotenv!("MONGO_ADDRESS"))
            .await
            .expect("BDD CONNECTION ");
        client_options.app_name = Some(String::from("rust-api"));
        let client = Client::with_options(client_options)
            .expect("Valid mongodb options");
        client.database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .expect("Mongodb database pinged");
        MongoDB {
            db: client.database(db_name)
        }
    }

    pub fn get_users_col(&self) -> Collection<User> {
        self.db.collection("users")
    }
}