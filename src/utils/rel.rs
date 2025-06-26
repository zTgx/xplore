use {
    crate::{
        core::client::Xplore,
        core::models::{profile::Profile, rel::RelationshipTimeline, timeline_v1::QueryProfilesResponse, Result},
        utils::home::{get_follower_timeline, get_following_timeline},
    },
    chrono::{DateTime, Utc},
};

pub async fn fetch_profile_following(
    xyz: &Xplore,
    user_id: &str,
    max_profiles: i32,
    cursor: Option<String>,
) -> Result<QueryProfilesResponse> {
    let timeline = get_following_timeline(xyz, user_id, max_profiles, cursor).await?;
    Ok(parse_relationship_timeline(&timeline))
}

pub async fn fetch_profile_followers(
    xyz: &Xplore,
    user_id: &str,
    max_profiles: i32,
    cursor: Option<String>,
) -> Result<QueryProfilesResponse> {
    let timeline = get_follower_timeline(xyz, user_id, max_profiles, cursor).await?;
    Ok(parse_relationship_timeline(&timeline))
}

fn parse_relationship_timeline(timeline: &RelationshipTimeline) -> QueryProfilesResponse {
    let mut profiles = Vec::new();
    let mut bottom_cursor = None;
    let mut top_cursor = None;

    if let Some(data) = &timeline.data {
        for instruction in &data.user.result.timeline.timeline.instructions {
            if instruction.instruction_type == "TimelineAddEntries"
                || instruction.instruction_type == "TimelineReplaceEntry"
            {
                // Handle case where instruction has a single entry (entry field)
                if let Some(entry_content) = &instruction.entry {
                    if let Some(content) = &entry_content.content {
                        if let Some(cursor_type) = &content.cursor_type {
                            if cursor_type == "Bottom" {
                                if let Some(value) = &content.value {
                                    bottom_cursor = Some(value.clone());
                                    continue;
                                }
                            } else if cursor_type == "Top" {
                                if let Some(value) = &content.value {
                                    top_cursor = Some(value.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }

                // Handle case where instruction has multiple entries (entries field)
                if let Some(entries) = &instruction.entries {
                    for entry in entries {
                        if let Some(item_content) = &entry.content.as_ref().and_then(|c| c.item_content.as_ref()) {
                            if item_content.user_display_type == Some("User".to_string()) {
                                if let Some(user_result_raw) = &item_content.user_results {
                                    if let Some(legacy) = &user_result_raw.result.legacy {
                                        let profile = Profile {
                                            username: legacy.screen_name.clone().unwrap_or_default(),
                                            name: legacy.name.clone().unwrap_or_default(),
                                            id: user_result_raw.result.rest_id.to_string(),
                                            description: legacy.description.clone(),
                                            location: legacy.location.clone(),
                                            url: legacy.url.clone(),
                                            protected: legacy.protected.unwrap_or_default(),
                                            verified: legacy.verified.unwrap_or_default(),
                                            followers_count: legacy.followers_count.unwrap_or_default(),
                                            following_count: legacy.friends_count.unwrap_or_default(),
                                            tweets_count: legacy.statuses_count.unwrap_or_default(),
                                            listed_count: legacy.listed_count.unwrap_or_default(),
                                            created_at: legacy
                                                .created_at
                                                .as_ref()
                                                .and_then(|date| {
                                                    DateTime::parse_from_str(date, "%a %b %d %H:%M:%S %z %Y")
                                                        .ok()
                                                        .map(|dt| dt.with_timezone(&Utc))
                                                })
                                                .unwrap_or_default(),
                                            profile_image_url: legacy.profile_image_url_https.clone(),
                                            profile_banner_url: legacy.profile_banner_url.clone(),
                                            pinned_tweet_id: legacy.pinned_tweet_ids_str.clone(),
                                            is_blue_verified: Some(user_result_raw.result.is_blue_verified),
                                        };

                                        profiles.push(profile);
                                    }
                                }
                            } else if let Some(cursor_type) =
                                &entry.content.as_ref().and_then(|c| c.cursor_type.as_ref())
                            {
                                if cursor_type.to_string() == "Bottom" {
                                    if let Some(value) = &entry.content.as_ref().and_then(|c| c.value.as_ref()) {
                                        bottom_cursor = Some(value.to_string());
                                    }
                                } else if cursor_type.to_string() == "Top" {
                                    if let Some(value) = &entry.content.as_ref().and_then(|c| c.value.as_ref()) {
                                        top_cursor = Some(value.to_string());
                                    }
                                }
                            }
                        }
                    }
                }

                // println!("Processing instruction: {:?}", instruction);
            }
        }
    }

    QueryProfilesResponse { profiles, next: bottom_cursor, previous: top_cursor }
}
