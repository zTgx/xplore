use {crate::core::error::XploreError, serde::Deserialize, serde_json::Value};

#[derive(Debug, Deserialize)]
pub struct RelationshipResponse {
    pub data: Option<RelationshipData>,
    #[serde(skip)]
    pub errors: Option<Vec<XploreError>>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipData {
    pub user: UserRelationships,
}

#[derive(Debug, Deserialize)]
pub struct UserRelationships {
    pub result: UserResult,
}

#[derive(Debug, Deserialize)]
pub struct UserResult {
    pub timeline: Timeline,
    pub rest_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Timeline {
    pub timeline: TimelineData,
}

#[derive(Debug, Deserialize)]
pub struct TimelineData {
    pub instructions: Vec<TimelineInstruction>,
}

// #[derive(Debug, Deserialize)]
// #[serde(tag = "type")]
// pub enum TimelineInstruction {
//     #[serde(rename = "TimelineAddEntries")]
//     AddEntries { entries: Vec<TimelineEntry> },
//     #[serde(rename = "TimelineReplaceEntry")]
//     ReplaceEntry { entry: TimelineEntry },
// }

// TODO: fix TimelineInstruction to struct
#[derive(Debug, Deserialize)]
// #[serde(tag = "type")]
pub struct TimelineInstruction {
    #[serde(rename = "type")]
    pub instruction_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    pub entries: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize)]
pub struct TimelineEntry {
    pub content: EntryContent,
    pub entry_id: String,
    pub sort_index: String,
}

#[derive(Debug, Deserialize)]
pub struct EntryContent {
    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,
    pub cursor: Option<CursorContent>,
}

#[derive(Debug, Deserialize)]
pub struct ItemContent {
    #[serde(rename = "user_results")]
    pub user_results: Option<UserResults>,
    #[serde(rename = "userDisplayType")]
    pub user_display_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserResults {
    pub result: UserResultData,
}

#[derive(Debug, Deserialize)]
pub struct UserResultData {
    #[serde(rename = "typename")]
    pub type_name: Option<String>,
    #[serde(rename = "mediaColor")]
    pub media_color: Option<MediaColor>,
    pub id: Option<String>,
    pub rest_id: Option<String>,
    pub affiliates_highlighted_label: Option<Value>,
    pub has_graduated_access: Option<bool>,
    pub is_blue_verified: Option<bool>,
    pub profile_image_shape: Option<String>,
    pub legacy: Option<UserLegacy>,
    pub professional: Option<Professional>,
}

#[derive(Debug, Deserialize)]
pub struct MediaColor {
    pub r: Option<ColorPalette>,
}

#[derive(Debug, Deserialize)]
pub struct ColorPalette {
    pub ok: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct UserLegacy {
    pub following: Option<bool>,
    pub followed_by: Option<bool>,
    pub screen_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub protected: Option<bool>,
    pub verified: Option<bool>,
    pub followers_count: Option<i32>,
    pub friends_count: Option<i32>,
    pub statuses_count: Option<i32>,
    pub listed_count: Option<i32>,
    pub created_at: Option<String>,
    pub profile_image_url_https: Option<String>,
    pub profile_banner_url: Option<String>,
    pub pinned_tweet_ids_str: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Professional {
    pub rest_id: Option<String>,
    pub professional_type: Option<String>,
    pub category: Option<Vec<ProfessionalCategory>>,
}

#[derive(Debug, Deserialize)]
pub struct ProfessionalCategory {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CursorContent {
    pub value: String,
    pub cursor_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimeline {
    pub data: Option<RelationshipTimelineData>,
    pub errors: Option<Vec<XploreError>>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimelineData {
    pub user: UserData,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub result: RelationshipUserResult,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipUserResult {
    pub timeline: Timeline,
}

#[derive(Debug, Deserialize)]
pub struct InnerTimeline {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Instruction {
    #[serde(rename = "TimelineAddEntries")]
    AddEntries { entries: Vec<RelationshipTimelineEntry> },
    #[serde(rename = "TimelineReplaceEntry")]
    ReplaceEntry { entry: RelationshipTimelineEntry },
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimelineEntry {
    pub content: EntryContent,
    pub entry_id: String,
    pub sort_index: String,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimelineContainer {
    pub timeline: InnerTimeline,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipTimelineWrapper {
    pub timeline: InnerTimeline,
}
