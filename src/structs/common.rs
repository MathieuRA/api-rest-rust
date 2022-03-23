use serde::{Deserialize, Serialize};

// That allow to impl optional struct. ie: impl FromRequest Optional<User>
#[derive(Deserialize, Serialize, Debug)]
pub struct Optional<T> {
    pub some: Option<T>,
}