use rocket::http::{ContentType, Status};

use crate::{ApiResponseDetails, IntlMessage};
use crate::tests::client;

const NEW_USER_BODY: &str = r##"{
"name":"",
"email":"unit_test_endpoint@mycleargames.com",
"password":"Toto"
}"##;

#[rocket::async_test]
async fn user_rt() {
    let intl_message = IntlMessage::new();
    let client = client().await;

    // ---- REGISTER NEW USER
    let resp = client
        .post("/api/v1/users")
        .header(ContentType::JSON)
        .body(NEW_USER_BODY)
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

    // ---- LOGIN USER
}