<div align="center">

[Discord](https://discord.gg/9b4YQmjh)

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

## Quick start
```rust
use dotenv::dotenv;
use std::env;
use xplore::{IProfile, Xplore};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let screen_name = "zTgx5";

    let cookie = env::var("X_COOKIE_STRING").expect("X_COOKIE_STRING");

    let xplore = Xplore::new(&cookie).await.unwrap();
    let profile_data = xplore.get_profile_by_screen_name(screen_name).await.unwrap();

    println!("{:#?}", profile_data);
}
```

## Reference projects
* https://github.com/cornip/Rina  

This project was refactored based on the above project. Thank you to the developers for their open-source spirit!

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=solagent-rs/xplore&type=Date)](https://star-history.com/#solagent-rs/xplore&Date)
