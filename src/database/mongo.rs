use mongodb::{Client, Collection, Database, IndexModel};
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, IndexOptions};

use crate::models::user::User;

pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    pub async fn new(db_name: &str) -> Self {
        let unique_index = IndexOptions::builder().unique(true).build();
        let index_user_email = IndexModel::builder()
            .keys(doc! {"email": 1})
            .options(unique_index)
            .build();

        let mut client_options = ClientOptions::parse(dotenv!("MONGO_ADDRESS"))
            .await
            .expect("BDD CONNECTION ");
        client_options.app_name = Some(String::from("rust-api"));
        let client = Client::with_options(client_options)
            .expect("Valid mongodb options");
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .expect("Mongodb database pinged");
        client
            .database("rust-api")
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