use dotenv::dotenv;
use std::env;
use xplore::{search::SearchMode, ISearch, Xplore};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");
    let xplore = Xplore::new(&cookie).await.unwrap();

    // Search for tweets by the user
    let tweets_response = xplore
        .search_tweets("The Democratic Party", 2, SearchMode::Latest, None)
        .await
        .expect("Failed to search tweets");

    for tweet in tweets_response.tweets {
        println!("Tweet ID: {:#?}, Text: {:#?}", tweet.id, tweet.text);
    }
}
