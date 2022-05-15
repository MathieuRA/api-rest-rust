use mongodb::{Client, Collection, Database, IndexModel};
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, IndexOptions};

use crate::models::user::User;

const DB_QA: &str = "mcg-api-qa";
const DB_DEV: &str = "mcg-api-dev";

pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    pub async fn new(db_prod: &str) -> Self {
        let db_name = match dotenv!("ENV") {
            "DEV" => DB_DEV,
            "TEST" => DB_QA,
            "PROD" => db_prod,
            _ => panic!("Must set ENV variable before start a database instance.")
        };
        let unique_index = IndexOptions::builder().unique(true).build();
        let index_user_email = IndexModel::builder()
            .keys(doc! {"email": 1})
            .options(unique_index)
            .build();

        let mut client_options = ClientOptions::parse(dotenv!("MONGO_ADDRESS"))
            .await
            .expect("BDD CONNECTION ");
        client_options.app_name = Some(String::from(db_name));
        let client = Client::with_options(client_options)
            .expect("Invalid mongodb options");
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .expect("Mongodb database pinged");

        if db_name == DB_QA {
            client.database(db_name).drop(None).await.unwrap();
        }

        client
            .database(db_name)
            .collection::<User>("users")
            .create_index(index_user_email, None)
            .await
            .expect("Error creating index!");
        MongoDB {
            db: client.database(db_name)
        }
    }

    pub fn get_users_coll(&self) -> Collection<User> {
        self.db.collection("users")
    }
}