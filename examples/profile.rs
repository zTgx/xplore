use dotenv::dotenv;
use std::env;
use xplore::Xplore;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");
    let xplore = Xplore::new(&cookie).await.unwrap();

    let screen_name = "elonmusk"; // Replace with the desired screen name
    println!("Getting profile for: {screen_name}");
    let profile = get_profile(&xplore, screen_name).await;
    println!("Profile: {profile:#?}");

    let user_id = get_user_id(&xplore, screen_name).await;
    println!("{screen_name}'s User ID: {user_id:?}");

    // Getting profile for: elonmusk
    // Profile: Profile {
    //     id: "44196397",
    //     username: "elonmusk",
    //     name: "Elon Musk",
    //     description: Some(
    //         "",
    //     ),
    //     location: Some(
    //         "",
    //     ),
    //     url: None,
    //     protected: false,
    //     verified: false,
    //     followers_count: 221188763,
    //     following_count: 1146,
    //     tweets_count: 80515,
    //     listed_count: 162995,
    //     created_at: 2009-06-02T20:12:29Z,
    //     profile_image_url: Some(
    //         "https://pbs.twimg.com/profile_images/1936002956333080576/kqqe2iWO.jpg",
    //     ),
    //     profile_banner_url: Some(
    //         "https://pbs.twimg.com/profile_banners/44196397/1739948056",
    //     ),
    //     pinned_tweet_id: Some(
    //         "1936876178356490546",
    //     ),
    //     is_blue_verified: Some(
    //         true,
    //     ),
    // }
    // elonmusk's User ID: "44196397"
}

async fn get_profile(xplore: &Xplore, screen_name: &str) -> xplore::core::models::profile::Profile {
    // This function retrieves the profile of a user by their screen name.
    // It uses the Xplore instance to call the get_profile method.
    // The screen_name parameter is the user's handle on the platform.
    // The function returns a Profile object containing the user's profile information.
    let profile = xplore.get_profile(screen_name).await.expect("Failed to get profile");

    profile
}

async fn get_user_id(xplore: &Xplore, screen_name: &str) -> String {
    // This function retrieves the user ID of a user by their screen name.
    // It uses the Xplore instance to call the get_user_id method.
    // The screen_name parameter is the user's handle on the platform.
    // The function returns a String containing the user's ID.
    let user_id = xplore.get_user_id(screen_name).await.expect("Failed to get profile by ID");

    user_id
}
