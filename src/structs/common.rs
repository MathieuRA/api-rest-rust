use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
/// That allow to impl optional struct.
/// ```
/// impl<'r> FromRequest<'r> for Optional<User>{
///  ...
/// }
/// ```
pub struct Optional<T> {
    pub some: Option<T>,
}