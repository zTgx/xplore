<div align="center">

# xplore   
X for Rust

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

## Usage
Two method to authenticate with Cookie:
* By using `login`
```rust
let mut xplore = Xplore::new(None).await.unwrap();

xplore.login(username, password, email, two_factor_secret).await;
```

OR

* By using `set_cookie`
```rust
let mut xplore = Xplore::new(None).await.unwrap();

let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");
xplore.set_cookie(&cookie).await;
```

--- 

### A quick start
```rust
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
```

## Star History
[![Star History Chart](https://api.star-history.com/svg?repos=zTgx/xplore&type=Date)](https://www.star-history.com/#zTgx/xplore&Date)