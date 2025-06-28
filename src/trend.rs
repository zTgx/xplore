use {
    crate::{
        api,
        auth::UserAuth,
        timeline_v1::{TimelineModuleItemWrapper, TimelineV1},
        Result, XploreError,
    },
    reqwest::Method,
    serde_json::json,
};

pub async fn get_trend(auth: &mut UserAuth) -> Result<Vec<String>> {
    let params = json!(
        {
            "count": "20",
            "candidate_source": "trends",
            "include_page_configuration": "false",
            "entity_tokens": "false",
        }
    );
    let url = format!("https://api.x.com/2/guide.json?{}", urlencoding::encode(&params.to_string()));

    let (response, _) = api::send_request::<TimelineV1>(auth, &url, Method::GET, None).await?;

    let instructions = response
        .timeline
        .as_ref()
        .and_then(|t| t.instructions.as_ref())
        .ok_or_else(|| XploreError::Api("Missing instructions in response".to_string()))?;

    if instructions.len() < 2 {
        return Err(XploreError::Api("Insufficient instructions entries".to_string()));
    }

    let entries = instructions[1]
        .add_entries
        .as_ref()
        .and_then(|e| e.entries.as_ref())
        .ok_or_else(|| XploreError::Api("Missing entries in instructions".to_string()))?;

    if entries.len() < 2 {
        return Err(XploreError::Api("Insufficient entries".to_string()));
    }

    let items = entries[1]
        .content
        .as_ref()
        .and_then(|c| c.timeline_module.as_ref())
        .and_then(|m| m.items.as_ref())
        .ok_or_else(|| XploreError::Api("Missing items in entry content".to_string()))?;

    // 收集所有趋势名称
    let trends: Vec<String> = items.iter().filter_map(extract_trend_name).collect();

    if trends.is_empty() {
        Err(XploreError::Api("No valid trends found".to_string()))
    } else {
        Ok(trends)
    }
}

fn extract_trend_name(item: &TimelineModuleItemWrapper) -> Option<String> {
    item.item
        .as_ref()
        .and_then(|i| i.client_event_info.as_ref())
        .and_then(|c| c.details.as_ref())
        .and_then(|d| d.guide_details.as_ref())
        .and_then(|g| g.transparent_guide_details.as_ref())
        .and_then(|t| t.trend_metadata.as_ref())
        .and_then(|m| m.trend_name.as_ref())
        .cloned()
}
