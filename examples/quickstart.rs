use dotenv::dotenv;
use std::env;
use xplore::Xplore;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut xplore = Xplore::new(None).await.unwrap();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");
    xplore.set_cookie(&cookie).await;

    let screen_name = "zTgx5"; // Replace with the desired screen name
    println!("Getting profile for: {screen_name}");
    let profile = xplore.get_profile(screen_name).await.expect("Failed to get profile");
    println!("Profile: {profile:#?}");
}
