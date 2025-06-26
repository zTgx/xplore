use dotenv::dotenv;
use std::env;
use xplore::{IProfile, ITweet, Xplore};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");
    let xplore = Xplore::new(&cookie).await.unwrap();

    // Screen name of the user whose tweets we want to fetch
    let screen_name = "elonmusk"; // Replace with the desired screen name
    println!("Getting tweets for: {screen_name}");

    // Fetching the user profile
    let user_profile = xplore.get_profile(screen_name).await.unwrap();

    // Fetching the user's pinned tweets by twitter ID
    let tweet = xplore.read_tweet(&user_profile.pinned_tweet_id.unwrap()).await.unwrap();

    // Displaying the tweet details
    println!("Pinned Tweet ID: {:#?}", tweet.id);
    println!("Pinned Tweet Text: {:#?}", tweet.text);
}
