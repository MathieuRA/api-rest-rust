use std::collections::HashMap;

pub struct IntlMessage<'a> {
    messages: HashMap<&'a str, &'a str>,
}

impl<'a> IntlMessage<'a> {
    pub fn new() -> Self {
        IntlMessage {
            messages: HashMap::from([
                ("authentication_failed", "Authentication process failed. User does not exist or invalid credentials."),
                ("authentication_required", "You must be authenticated."),
                ("authentication_success", "Authentication process success."),
                ("forbidden_operation", "Forbidden operation."),
                ("usr_created", "User creation process success."),
                ("usr_edited", "User edited successfully."),
                ("usr_founded", "User research process success.")
            ])
        }
    }

    pub fn get_by_intl_id(&self, intl_id: &str) -> (String, String) {
        let intl_message = self.messages.get_key_value(intl_id).unwrap();
        (intl_message.0.to_string(), intl_message.1.to_string())
    }
}