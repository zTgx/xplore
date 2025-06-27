use dotenv::dotenv;
use std::env;
use xplore::Xplore;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");
    let xplore = Xplore::new(&cookie).await.unwrap();

    // Get the list of users that the authenticated user is following
    let user_id = "1222365222934962177"; // Example user ID (Twitter's @elonmusk)

    let following_response = xplore.get_following(user_id, 1, None).await.expect("Failed to get following list");

    println!("Following count: {}", following_response.1.unwrap_or("No next cursor".to_string()));

    // Print the usernames of the profiles that the user is following
    println!("Following profiles:");
    for profile in following_response.0 {
        println!("Following: {:#?}", profile.username);
    }

    // Get the list of users that are following the authenticated user
    let followers_response = xplore.get_followers(user_id, 1, None).await.expect("Failed to get followers list");
    println!("Followers count: {}", followers_response.1.unwrap_or("No next cursor".to_string()));
    // Print the usernames of the profiles that are following the user
    println!("Followers profiles:");
    for profile in followers_response.0 {
        println!("Follower: {:#?}", profile.username);
    }
}
