<div align="center">

# xplore   
X/Twitter for Rust

[![Version](https://img.shields.io/crates/v/xplore)](https://crates.io/crates/xplore)
![Crates Downloads](https://img.shields.io/crates/d/xplore?logo=rust)
![GitHub License](https://img.shields.io/github/license/solagent-rs/xplore)

</div>

>> 💥 PRs are welcome.   
>> ❗We are still in the early development phase, so please be aware that the interfaces may evolve as we continue to refine the project.

## Features
- Authentication with cookies
- Comprehensive user profile management
- Timeline retrieval
- Tweet interactions (like, retweet, post)
- Advanced search capabilities
- User relationship management (follow/unfollow)

## Installation
```toml
[dependencies]
xplore = "0.1"
```

## Quick start
```rust
use dotenv::dotenv;
use std::env;
use xplore::{IProfile, Xplore};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");

    let xplore = Xplore::new(&cookie).await.unwrap();
    let user_id = xplore.get_user_id("elonmusk").await.unwrap();
    println!("user id: {user_id}");
}
```

### Get screen name's profile

```rust
use dotenv::dotenv;
use std::env;
use xplore::{IProfile, Xplore};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");
    let xplore = Xplore::new(&cookie).await.unwrap();
    
    let screen_name = "elonmusk"; // Replace with the desired screen name
    println!("Getting profile for: {screen_name}");
    let profile = get_profile(&xplore, screen_name)
        .await;
    println!("Profile: {profile:#?}");
}

async fn get_profile(xplore: &Xplore, screen_name: &str) -> xplore::core::models::profile::Profile {
    // This function retrieves the profile of a user by their screen name.
    // It uses the Xplore instance to call the get_profile method.
    // The screen_name parameter is the user's handle on the platform.
    // The function returns a Profile object containing the user's profile information.
    let profile = xplore.get_profile(screen_name)
        .await
        .expect("Failed to get profile");

    profile
}
```

### Get screen name's user ID
```rust
use dotenv::dotenv;
use std::env;
use xplore::{IProfile, Xplore};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");
    let xplore = Xplore::new(&cookie).await.unwrap();
    
    let screen_name = "elonmusk"; // Replace with the desired screen name
    println!("Getting profile for: {screen_name}");
    let user_id = get_user_id(&xplore, screen_name)
        .await;
    println!("{screen_name}'s User ID: {user_id:?}");
}

async fn get_user_id(xplore: &Xplore, screen_name: &str) -> String {
    // This function retrieves the user ID of a user by their screen name.
    // It uses the Xplore instance to call the get_user_id method.
    // The screen_name parameter is the user's handle on the platform.
    // The function returns a String containing the user's ID.
    let user_id = xplore.get_user_id(screen_name)
        .await
        .expect("Failed to get profile by ID");

        user_id
}
```

### Get pinned Tweet content
```rust
use dotenv::dotenv;
use std::env;
use xplore::{IProfile, Xplore, ITweet};

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
```

## Star History
[![Star History Chart](https://api.star-history.com/svg?repos=zTgx/xplore&type=Date)](https://www.star-history.com/#zTgx/xplore&Date)