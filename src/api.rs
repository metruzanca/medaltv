use crate::error::MedalError;
use crate::models::*;
use log::error;
use reqwest::blocking::Client;
use uuid::Uuid;

const MEDAL_API_BASE: &str = "https://medal.tv/api";

pub struct MedalAPI {
    client: Client,
    token: String,
}

impl MedalAPI {
    pub fn new() -> Result<Self, MedalError> {
        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                headers.insert(
                    "Medal-User-Agent",
                    "Medal-web/1.0 (string_id; simplified_signup; no_upscale; markdown)"
                        .parse()
                        .unwrap(),
                );
                headers
            })
            .build()?;

        let token = Self::authenticate_inner(&client)?;
        Ok(Self { client, token })
    }

    pub fn with_token(token: String) -> Result<Self, MedalError> {
        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                headers.insert(
                    "Medal-User-Agent",
                    "Medal-web/1.0 (string_id; simplified_signup; no_upscale; markdown)"
                        .parse()
                        .unwrap(),
                );
                headers
            })
            .build()?;

        Ok(Self { client, token })
    }

    fn auth_header_value(&self) -> String {
        self.token.clone()
    }

    fn authenticate_inner(client: &Client) -> Result<String, MedalError> {
        let uid = Self::make_uid();
        let body = serde_json::json!({
            "email": "guest",
            "userName": "guest",
            "password": uid,
        });

        let resp: AuthResponse = client
            .post(format!("{MEDAL_API_BASE}/users"))
            .json(&body)
            .send()
            .map_err(|e| {
                error!("Connection to medal.tv failed.");
                MedalError::Http(e)
            })?
            .json()
            .map_err(|e| {
                error!("Medal did not return valid JSON data.");
                MedalError::Http(e)
            })?;

        Ok(format!("{},{}", resp.user.user_id, resp.auth.key))
    }

    fn make_uid() -> String {
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let full = uuid1.as_bytes();
        let full2 = uuid2.as_bytes();

        let hex_bytes: [u8; 31] = {
            let mut arr = [0u8; 31];
            let mut idx = 0;
            for &b in full.iter().chain(full2.iter()) {
                if idx >= 31 {
                    break;
                }
                arr[idx] = b;
                idx += 1;
            }
            arr
        };
        let hex: String = hex_bytes.iter().map(|b| format!("{b:02x}")).collect();
        // Format: 8-4-4-4-11
        format!(
            "{}-{}-{}-{}-{}",
            &hex[0..8],
            &hex[8..12],
            &hex[12..16],
            &hex[16..20],
            &hex[20..31]
        )
    }

    pub fn get_user(&self, username: &str) -> Result<Vec<User>, MedalError> {
        let resp = self
            .client
            .get(format!("{MEDAL_API_BASE}/users"))
            .query(&[("username", username)])
            .header("X-Authentication", self.auth_header_value())
            .send()
            .map_err(|e| {
                error!("Connection to medal.tv failed.");
                MedalError::Http(e)
            })?;

        let users: Vec<User> = resp.json().map_err(|e| {
            error!("Medal did not return valid JSON data.");
            MedalError::Http(e)
        })?;

        Ok(users)
    }

    pub fn get_category(&self, category_id: &str) -> Result<Category, MedalError> {
        let resp = self
            .client
            .get(format!("{MEDAL_API_BASE}/categories/{category_id}"))
            .header("X-Authentication", self.auth_header_value())
            .send()
            .map_err(|e| {
                error!("Connection to medal.tv failed.");
                MedalError::Http(e)
            })?;

        let category: Category = resp.json().map_err(|e| {
            error!("Medal did not return valid JSON data.");
            MedalError::Http(e)
        })?;

        Ok(category)
    }

    pub fn get_recent_games(
        &self,
        username: &str,
    ) -> Result<Vec<RecentGame>, MedalError> {
        let users = self.get_user(username)?;
        let user = users
            .first()
            .ok_or_else(|| MedalError::UserNotFound(username.to_string()))?;

        let mut results = Vec::with_capacity(user.game_sessions.len());
        for session in &user.game_sessions {
            let category_name = self
                .get_category(&session.category_id)
                .map(|c| c.category_name)
                .unwrap_or_else(|_| "unknown".to_string());

            let end_time = match session.end_time {
                Some(t) => serde_json::Value::Number(t.into()),
                None => serde_json::Value::String(String::new()),
            };

            results.push(RecentGame {
                session_id: session.session_id.clone(),
                category: category_name,
                start_time: session.start_time,
                end_time,
                expires_at: session.expires_at,
            });
        }

        Ok(results)
    }

    pub fn get_recent_clips(
        &self,
        username: &str,
        limit: u32,
    ) -> Result<Vec<Clip>, MedalError> {
        let users = self.get_user(username)?;
        let user = users
            .first()
            .ok_or_else(|| MedalError::UserNotFound(username.to_string()))?;

        let resp = self
            .client
            .get(format!("{MEDAL_API_BASE}/content"))
            .query(&[("userId", &user.user_id), ("limit", &limit.to_string())])
            .header("X-Authentication", self.auth_header_value())
            .send()
            .map_err(|e| {
                error!("Connection to medal.tv failed.");
                MedalError::Http(e)
            })?;

        let api_clips: Vec<ApiClip> = resp.json().map_err(|e| {
            error!("Medal did not return valid JSON data.");
            MedalError::Http(e)
        })?;

        let clips = api_clips
            .into_iter()
            .map(|c| Clip {
                content_id: c.content_id,
                content_title: c.content_title,
                video_length_seconds: c.video_length_seconds,
                views: c.views,
                likes: c.likes,
                comments: c.comments,
                published_at: c.published_at,
                content_url_720p: c.content_url_720p,
                thumbnail_720p: c.thumbnail_720p,
                category: c.category.category_name,
            })
            .collect();

        Ok(clips)
    }
}
