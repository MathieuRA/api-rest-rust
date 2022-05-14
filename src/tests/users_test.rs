use rocket::futures::TryFutureExt;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket_contrib::json;
use serde::{Deserializer, Serialize};

use crate::{ApiResponse, ApiResponseDetails, IntlMessage};
use crate::models::user::ResponseUser;
use crate::tests::client;

#[rocket::async_test]
async fn new_user_rt_test() {
    let intl_message = IntlMessage::new();
    let client = client().await;
    let resp = client
        .post("/api/v1/users")
        .header(ContentType::JSON)
        .body(r##"{
        "name":"Toto",
        "email":"Toto",
        "password":"Toto"
        }"##)
        .dispatch().await;

    assert_eq!(resp.status(), Status::Created);
    assert_eq!(resp.content_type(), Some(ContentType::JSON));

    let api_response = serde_json::from_str::<ApiResponseDetails>(
        resp.into_string().await.unwrap().as_str()
    ).unwrap();
    let msg = intl_message.get_by_intl_id("usr_created");

    assert_eq!(api_response.intl_id, msg.0);
    assert_eq!(api_response.reason, msg.1);
    assert_ne!(api_response.data, None);
}