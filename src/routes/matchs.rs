use mongodb::bson::{doc, Document};
use mongodb::Database;
use rocket::{Request, request, State};
use rocket::futures::TryStreamExt;
use rocket::request::{FromRequest, Outcome};

use crate::models::match_model::Match_Model;
use crate::MongoDB;

pub struct AuthUser {
    pub id: u8,
}

// Guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("From request");
        let mongoDB = request.rocket().state::<MongoDB>().expect("Error stat");
        for name in mongoDB.db.list_collection_names(None).await.expect("DB") {
            println!("{}", name)
        };
        let col = mongoDB.db.collection::<Document>("Toto");
        col.insert_one(doc! {"test":"fromGuard"}, None).await.expect("INSERT");
        Outcome::Success({ AuthUser { id: 1 } })
    }
}

#[get("/matchs")]
pub async fn matchs_rt(mongodb: &State<MongoDB>, user: AuthUser) -> String {
    let col = mongodb.db.collection::<Match_Model>("matchs");
    let mut cursor = col.find(None, None).await.expect("BDD");

    let mut array = vec![];
    while let Some(matchdb) = cursor.try_next().await.expect("Wile") {
        array.push(matchdb)
    };

    // let resut = serde_json::to_string(&array).expect("RESULT");
    user.id.to_string()
}

#[get("/matchs/post")]
pub async fn post_matchs_rt(mongodb: &State<MongoDB>) -> String {
    // 106
    let col = mongodb.db.collection::<Match_Model>("matchs");
    let mut docs = vec![];
    for _i in 1..=107 {
        docs.push(Match_Model {
            stadium: String::from("PSG"),
            fixture_id: 10,
            league_id: 10,
            away_team_id: 10,
            home_team_id: 10,
            away_team_name: String::from("PSG"),
            home_team_name: String::from("PSG"),
            timestamp_kick_off: 1000000,
            status: String::from("PSG"),

            stat_1_home_name: String::from("PSG"),
            stat_1_away_name: String::from("PSG"),
            stat_1_home_value: 10,
            stat_1_away_value: 10,
            stat_2_home_name: String::from("PSG"),
            stat_2_away_name: String::from("PSG"),
            stat_2_home_value: 10,
            stat_2_away_value: 10,
            stat_3_home_name: String::from("PSG"),
            stat_3_away_name: String::from("PSG"),
            stat_3_home_value: 10,
            stat_3_away_value: 10,
            stat_4_home_name: String::from("PSG"),
            stat_4_away_name: String::from("PSG"),
            stat_4_home_value: 10,
            stat_4_away_value: 10,
            stat_5_home_name: String::from("PSG"),
            stat_5_away_name: String::from("PSG"),
            stat_5_home_value: 10,
            stat_5_away_value: 10,
            stat_6_home_name: String::from("PSG"),
            stat_6_away_name: String::from("PSG"),
            stat_6_home_value: 10,
            stat_6_away_value: 10,
            stat_7_home_name: String::from("PSG"),
            stat_7_away_name: String::from("PSG"),
            stat_7_home_value: 10,
            stat_7_away_value: 10,
            stat_8_home_name: String::from("PSG"),
            stat_8_away_name: String::from("PSG"),
            stat_8_home_value: 10,
            stat_8_away_value: 10,
            stat_9_home_name: String::from("PSG"),
            stat_9_away_name: String::from("PSG"),
            stat_9_home_value: 10,
            stat_9_away_value: 10,
            stat_10_home_name: String::from("PSG"),
            stat_10_away_name: String::from("PSG"),
            stat_10_home_value: 10,
            stat_10_away_value: 10,

            stat_11_home_name: String::from("PSG"),
            stat_11_away_name: String::from("PSG"),
            stat_11_home_value: 10,
            stat_11_away_value: 10,
            stat_12_home_name: String::from("PSG"),
            stat_12_away_name: String::from("PSG"),
            stat_12_home_value: 10,
            stat_12_away_value: 10,
            stat_13_home_name: String::from("PSG"),
            stat_13_away_name: String::from("PSG"),
            stat_13_home_value: 10,
            stat_13_away_value: 10,
            stat_14_home_name: String::from("PSG"),
            stat_14_away_name: String::from("PSG"),
            stat_14_home_value: 10,
            stat_14_away_value: 10,
            stat_15_home_name: String::from("PSG"),
            stat_15_away_name: String::from("PSG"),
            stat_15_home_value: 10,
            stat_15_away_value: 10,
            stat_16_home_name: String::from("PSG"),
            stat_16_away_name: String::from("PSG"),
            stat_16_home_value: 10,
            stat_16_away_value: 10,
            stat_17_home_name: String::from("PSG"),
            stat_17_away_name: String::from("PSG"),
            stat_17_home_value: 10,
            stat_17_away_value: 10,
            stat_18_home_name: String::from("PSG"),
            stat_18_away_name: String::from("PSG"),
            stat_18_home_value: 10,
            stat_18_away_value: 10,
            stat_19_home_name: String::from("PSG"),
            stat_19_away_name: String::from("PSG"),
            stat_19_home_value: 10,
            stat_19_away_value: 10,
            stat_20_home_name: String::from("PSG"),
            stat_20_away_name: String::from("PSG"),
            stat_20_home_value: 10,
            stat_20_away_value: 10,

            stat_21_home_name: String::from("PSG"),
            stat_21_away_name: String::from("PSG"),
            stat_21_home_value: 10,
            stat_21_away_value: 10,
            stat_22_home_name: String::from("PSG"),
            stat_22_away_name: String::from("PSG"),
            stat_22_home_value: 10,
            stat_22_away_value: 10,
            stat_23_home_name: String::from("PSG"),
            stat_23_away_name: String::from("PSG"),
            stat_23_home_value: 10,
            stat_23_away_value: 10,
            stat_24_home_name: String::from("PSG"),
            stat_24_away_name: String::from("PSG"),
            stat_24_home_value: 10,
            stat_24_away_value: 10,
            stat_25_home_name: String::from("PSG"),
            stat_25_away_name: String::from("PSG"),
            stat_25_home_value: 10,
            stat_25_away_value: 10,
            stat_26_home_name: String::from("PSG"),
            stat_26_away_name: String::from("PSG"),
            stat_26_home_value: 10,
            stat_26_away_value: 10,
            stat_27_home_name: String::from("PSG"),
            stat_27_away_name: String::from("PSG"),
            stat_27_home_value: 10,
            stat_27_away_value: 10,
            stat_28_home_name: String::from("PSG"),
            stat_28_away_name: String::from("PSG"),
            stat_28_home_value: 10,
            stat_28_away_value: 10,
            stat_29_home_name: String::from("PSG"),
            stat_29_away_name: String::from("PSG"),
            stat_29_home_value: 10,
            stat_29_away_value: 10,
            stat_30_home_name: String::from("PSG"),
            stat_30_away_name: String::from("PSG"),
            stat_30_home_value: 30,
            stat_30_away_value: 10,

            stat_31_home_name: String::from("PSG"),
            stat_31_away_name: String::from("PSG"),
            stat_31_home_value: 10,
            stat_31_away_value: 10,
            stat_32_home_name: String::from("PSG"),
            stat_32_away_name: String::from("PSG"),
            stat_32_home_value: 10,
            stat_32_away_value: 10,
            stat_33_home_name: String::from("PSG"),
            stat_33_away_name: String::from("PSG"),
            stat_33_home_value: 10,
            stat_33_away_value: 10,
            stat_34_home_name: String::from("PSG"),
            stat_34_away_name: String::from("PSG"),
            stat_34_home_value: 10,
            stat_34_away_value: 10,
            stat_35_home_name: String::from("PSG"),
            stat_35_away_name: String::from("PSG"),
            stat_35_home_value: 10,
            stat_35_away_value: 10,
            stat_36_home_name: String::from("PSG"),
            stat_36_away_name: String::from("PSG"),
            stat_36_home_value: 10,
            stat_36_away_value: 10,
            stat_37_home_name: String::from("PSG"),
            stat_37_away_name: String::from("PSG"),
            stat_37_home_value: 10,
            stat_37_away_value: 10,
            stat_38_home_name: String::from("PSG"),
            stat_38_away_name: String::from("PSG"),
            stat_38_home_value: 10,
            stat_38_away_value: 10,
            stat_39_home_name: String::from("PSG"),
            stat_39_away_name: String::from("PSG"),
            stat_39_home_value: 10,
            stat_39_away_value: 10,
            stat_40_home_name: String::from("PSG"),
            stat_40_away_name: String::from("PSG"),
            stat_40_home_value: 40,
            stat_40_away_value: 10,
            home_team_score: 0,
            away_team_score: 0,
            minute: 0,
            league_rank: 0,
            league_name: String::from("Toto"),
        });
    }
    col.insert_many(&docs, None).await.expect("INSERT");
    String::from("Posted")
}

