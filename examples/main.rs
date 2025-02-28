use dotenv::dotenv;
use std::env;
use xplore::{IProfile, Xplore};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");

    let xplore = Xplore::new(&cookie).await.unwrap();
    let user_id = xplore.get_user_id("zTgx5").await.unwrap();
    println!("user id: {user_id}"); // 1222365222934962177
}
