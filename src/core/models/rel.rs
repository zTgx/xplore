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

#[derive(Debug, Deserialize)]
pub struct TimelineInstruction {
    #[serde(rename = "type")]
    pub instruction_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<TimelineEntry>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<TimelineEntry>,
}

#[derive(Debug, Deserialize)]
pub struct TimelineEntry {
    #[serde(rename = "entryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<String>,

    #[serde(rename = "sortIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_index: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<EntryContent>,
}

#[derive(Debug, Deserialize)]
pub struct EntryContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cursorType")]
    pub cursor_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entryType")]
    pub entry_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub __typename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<CursorContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "clientEventInfo")]
    pub client_event_info: Option<Value>, // Empty object
}

#[derive(Debug, Deserialize)]
pub struct ItemContent {
    #[serde(rename = "user_results")]
    pub user_results: Option<UserResults>,

    #[serde(rename = "userDisplayType")]
    pub user_display_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "itemType")]
    pub item_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserResults {
    pub result: UserResultData,
}

#[derive(Debug, Deserialize)]
pub struct UserResultData {
    #[serde(rename = "__typename")]
    pub typename: String,

    #[serde(rename = "affiliates_highlighted_label")]
    pub affiliates_highlighted_label: Value, // Empty object

    #[serde(rename = "has_graduated_access")]
    pub has_graduated_access: bool,

    pub id: String,

    #[serde(rename = "is_blue_verified")]
    pub is_blue_verified: bool,

    #[serde(rename = "legacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy: Option<UserLegacy>,

    pub professional: Option<Professional>,

    #[serde(rename = "profile_image_shape")]
    pub profile_image_shape: String,

    #[serde(rename = "rest_id")]
    pub rest_id: String,

    #[serde(rename = "super_follow_eligible")]
    pub super_follow_eligible: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct MediaColor {
    pub r: Option<ColorPalette>,
}

#[derive(Debug, Deserialize)]
pub struct ColorPalette {
    pub ok: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Url {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlInfo>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UrlInfo {
    #[serde(rename = "expanded_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserLegacy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses_count: Option<u32>,

    #[serde(rename = "id_str")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_str: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    #[serde(rename = "geo_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_enabled: Option<bool>,

    #[serde(rename = "pinned_tweet_ids_str")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_tweet_ids_str: Option<Vec<String>>,

    #[serde(rename = "profile_background_color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_color: Option<String>,

    #[serde(rename = "profile_banner_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_banner_url: Option<String>,

    #[serde(rename = "profile_image_url_https")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url_https: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,

    #[serde(rename = "screen_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    #[serde(rename = "has_custom_timelines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_custom_timelines: Option<bool>,

    #[serde(rename = "has_extended_profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_extended_profile: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "can_dm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_dm: Option<bool>,
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

    #[serde(rename = "cursorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "__typename")]
    pub typename: Option<String>,

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
