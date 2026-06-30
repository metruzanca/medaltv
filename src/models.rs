use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub(crate) struct AuthResponse {
    pub user: UserId,
    pub auth: AuthKey,
}

#[derive(Deserialize, Debug)]
pub(crate) struct UserId {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct AuthKey {
    pub key: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "gameSessions")]
    pub game_sessions: Vec<GameSession>,
}

#[derive(Deserialize, Debug)]
pub struct GameSession {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "categoryId")]
    pub category_id: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    #[serde(rename = "expiresAt")]
    pub expires_at: i64,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    #[serde(rename = "categoryName")]
    pub category_name: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct RecentGame {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub category: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: serde_json::Value,
    #[serde(rename = "expiresAt")]
    pub expires_at: i64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ApiClip {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "contentTitle")]
    pub content_title: String,
    #[serde(rename = "videoLengthSeconds")]
    pub video_length_seconds: f64,
    pub views: i64,
    pub likes: i64,
    pub comments: i64,
    #[serde(rename = "publishedAt")]
    pub published_at: i64,
    #[serde(rename = "contentUrl720p")]
    pub content_url_720p: String,
    #[serde(rename = "thumbnail720p")]
    pub thumbnail_720p: String,
    pub category: ClipCategory,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ClipCategory {
    #[serde(rename = "categoryName")]
    pub category_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Clip {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "contentTitle")]
    pub content_title: String,
    #[serde(rename = "videoLengthSeconds")]
    pub video_length_seconds: f64,
    pub views: i64,
    pub likes: i64,
    pub comments: i64,
    #[serde(rename = "publishedAt")]
    pub published_at: i64,
    #[serde(rename = "contentUrl720p")]
    pub content_url_720p: String,
    #[serde(rename = "thumbnail720p")]
    pub thumbnail_720p: String,
    pub category: String,
}
