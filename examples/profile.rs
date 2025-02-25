use xplore::XploreX;

#[tokio::main]
async fn main() {
    let xplore = XploreX::new().await;

    let screen_name = "zTgx5";
    let profile_data = xplore
        .get_profile_by_screen_name(screen_name)
        .await
        .unwrap();
    println!("{:#?}", profile_data);

    // Profile {
    //     id: "1222365222934962177",
    //     username: "ztgx5",
    //     name: "zTgx",
    //     description: Some(
    //         "building https://t.co/yfkClZ4PcD",
    //     ),
    //     location: Some(
    //         "Beijing, China",
    //     ),
    //     url: Some(
    //         "https://solagent.rs",
    //     ),
    //     protected: false,
    //     verified: false,
    //     followers_count: 372,
    //     following_count: 549,
    //     tweets_count: 168,
    //     listed_count: 8,
    //     created_at: 2020-01-29T03:46:03Z,
    //     profile_image_url: Some(
    //         "https://pbs.twimg.com/profile_images/1877661287120797696/Pt9QEdqN.jpg",
    //     ),
    //     profile_banner_url: Some(
    //         "https://pbs.twimg.com/profile_banners/1222365222934962177/1736862489",
    //     ),
    //     pinned_tweet_id: None,
    //     is_blue_verified: Some(
    //         false,
    //     ),
    // }
}
