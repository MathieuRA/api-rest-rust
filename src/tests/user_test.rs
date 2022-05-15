






use crate::{IntlMessage, MongoDB};
use crate::models::user::{InsertableUser};
use crate::tests::client;

const UNIT_TEST_INSERT_ONE: &str = "unit_test_one@mycleargames.com";

#[rocket::async_test]
async fn insertable_user_insert_one() {
    let mongo_db = client().await.rocket().state::<MongoDB>().unwrap();
    let intl_message = IntlMessage::new();
    let fake_user = InsertableUser {
        name: "".to_string(),
        email: UNIT_TEST_INSERT_ONE.to_string(),
        password: "".to_string(),
    };

    // Must success
    match InsertableUser::insert_one(mongo_db, fake_user.clone()).await {
        Ok(user) => assert_eq!(user.email, UNIT_TEST_INSERT_ONE),
        Err(_) => assert_eq!(false, true)
    }

    // Must fail, already exist
    match InsertableUser::insert_one(mongo_db, fake_user).await {
        Ok(_) => assert_eq!(false, true),
        Err(err) => {
            let api_resp_detail = err.get_content();
            let email_already_exist_msg = intl_message.get_by_intl_id("email_already_exist");
            assert_eq!(api_resp_detail.intl_id, email_already_exist_msg.0);
            assert_eq!(api_resp_detail.reason, email_already_exist_msg.1);
            assert_eq!(api_resp_detail.data, None);
        }
    }
}