use {serde::Deserialize, serde_json::Value};

#[derive(Debug, Deserialize)]
pub struct HomeTimelineResponse {
    pub data: Option<HomeData>,
}

#[derive(Debug, Deserialize)]
pub struct HomeData {
    pub home: Home,
}

#[derive(Debug, Deserialize)]
pub struct Home {
    #[serde(rename = "home_timeline_urt")]
    pub home_timeline: HomeTimeline,
}

#[derive(Debug, Deserialize)]
pub struct HomeTimeline {
    pub instructions: Vec<TimelineInstruction>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TimelineInstruction {
    #[serde(rename = "TimelineAddEntries")]
    AddEntries { entries: Vec<TimelineEntry> },
    // Add other variants as needed
}

#[derive(Debug, Deserialize)]
pub struct TimelineEntry {
    pub content: EntryContent,
}

#[derive(Debug, Deserialize)]
pub struct EntryContent {
    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,
}

#[derive(Debug, Deserialize)]
pub struct ItemContent {
    pub tweet_results: Option<TweetResults>,
}

#[derive(Debug, Deserialize)]
pub struct TweetResults {
    pub result: Option<Value>,
}
