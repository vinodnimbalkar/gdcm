use crate::services::GeminiService;
use crate::types::{CommitMessageResponse, ErrorResponse, GitDiffRequest};
use worker::*;

pub async fn handle_generate_commit(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Parse JSON body
    let payload: GitDiffRequest = match req.json().await {
        Ok(data) => data,
        Err(_) => {
            let error = ErrorResponse {
                error: "Invalid JSON in request body".to_string(),
            };
            return Ok(Response::from_json(&error)?.with_status(400));
        }
    };

    // Validate diff
    if payload.diff.trim().is_empty() {
        let error = ErrorResponse {
            error: "Empty diff provided".to_string(),
        };
        return Ok(Response::from_json(&error)?.with_status(400));
    }

    // Get API key from request or environment
    let api_key = payload
        .gemini_api_key
        .or_else(|| ctx.env.var("GEMINI_API_KEY").ok().map(|v| v.to_string()))
        .unwrap_or_else(|| std::env::var("GEMINI_API_KEY").unwrap_or_default());

    if api_key.is_empty() {
        let error = ErrorResponse {
            error: "No Gemini API key provided. Set GEMINI_API_KEY environment variable or include in request.".to_string(),
        };
        return Ok(Response::from_json(&error)?.with_status(400));
    }

    // Generate commit message
    match GeminiService::generate_commit_message(&payload.diff, &api_key).await {
        Ok(commit_message) => {
            let response = CommitMessageResponse {
                commit_message,
                service_used: "Gemini".to_string(),
            };
            Ok(Response::from_json(&response)?)
        }
        Err(e) => {
            let error = ErrorResponse {
                error: format!("Failed to generate commit message: {}", e),
            };
            Ok(Response::from_json(&error)?.with_status(500))
        }
    }
}
