use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GitDiffRequest {
    pub diff: String,
    #[serde(default)]
    pub gemini_api_key: Option<String>,
}

#[derive(Serialize)]
pub struct CommitMessageResponse {
    pub commit_message: String,
    pub service_used: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
}

#[derive(Serialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiContentResponse,
}

#[derive(Deserialize)]
pub struct GeminiContentResponse {
    pub parts: Vec<GeminiPartResponse>,
}

#[derive(Deserialize)]
pub struct GeminiPartResponse {
    pub text: String,
}
