use async_once::AsyncOnce;
use lazy_static::lazy_static;
use rocket::local::asynchronous::Client;

use crate::rocket_builder;

#[cfg(test)]
mod user_test;
#[cfg(test)]
mod endpoint;

pub fn client() -> &'static AsyncOnce<Client> {
    lazy_static! {
        static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
            Client::tracked(rocket_builder().await).await.unwrap()
        });
    }
    &*CLIENT
}