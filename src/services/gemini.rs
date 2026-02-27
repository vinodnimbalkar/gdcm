use crate::types::{GeminiContent, GeminiPart, GeminiRequest, GeminiResponse};
use worker::*;

pub struct GeminiService;

impl GeminiService {
    pub async fn generate_commit_message(
        diff: &str,
        api_key: &str,
    ) -> std::result::Result<String, String> {
        let prompt = format!(
            "Generate a commit message for this Git diff:\n\n```\n{}\n```",
            diff
        );

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:generateContent?key={}",
            api_key
        );

        let request_body = GeminiRequest {
            system_instruction: Some(GeminiContent {
                parts: vec![GeminiPart {
                    text: Self::get_system_instruction().to_string(),
                }],
            }),
            contents: vec![GeminiContent {
                parts: vec![GeminiPart { text: prompt }],
            }],
        };

        // Create the request
        let headers = Headers::new();
        headers
            .set("Content-Type", "application/json")
            .map_err(|e| format!("Failed to set header: {:?}", e))?;

        let mut init = RequestInit::new();
        init.with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(
                serde_json::to_string(&request_body)
                    .map_err(|e| e.to_string())?
                    .into(),
            ));

        let request = Request::new_with_init(&url, &init)
            .map_err(|e| format!("Failed to create request: {:?}", e))?;

        // Make the request
        let mut response = Fetch::Request(request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {:?}", e))?;

        if !(200..300).contains(&response.status_code()) {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!(
                "API request failed with status {}: {}",
                response.status_code(),
                error_text
            ));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {:?}", e))?;

        let commit_message = gemini_response
            .candidates
            .first()
            .map(|c| {
                c.content
                    .parts
                    .iter()
                    .map(|p| p.text.as_str())
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .to_string()
            })
            .ok_or_else(|| "No commit message generated".to_string())?;

        Ok(commit_message)
    }

    fn get_system_instruction() -> &'static str {
        r#"You are an expert developer tasked with creating meaningful Git commit messages based on code diffs.

Generate commit messages that follow these guidelines:

1. Use conventional commit format with a summary and optional body:
   <type>: <emoji><summary>

   <body with detailed explanation if needed>

2. Include appropriate emoji based on the change type
3. Keep the summary line under 50 characters when possible
4. Add a blank line between summary and body
5. Use the body to explain WHAT changed and WHY (if not obvious)
6. Wrap body lines at 72 characters
7. For simple changes, a single-line message is acceptable

Commit Types and Emojis:
- feat: ✨ (new feature)
- fix: 🐛 (bug fix)
- docs: 📚 (documentation)
- style: 💄 (formatting, styling)
- refactor: 🔨 (code refactoring)
- test: ✅ (adding tests)
- chore: 🔧 (maintenance tasks)
- perf: 🐎 (performance improvement)
- ci: 💚 (continuous integration)
- build: 📦 (build system)
- revert: ⏪ (reverting changes)
- security: 🔒 (security improvements)
- deps: ⬆️ (dependency updates)
- remove: 🔥 (removing code/files)
- wip: 🚧 (work in progress)

Examples:
Simple change:
"feat: ✨Add user authentication system"

Complex change:
"feat: ✨Add user authentication system

Implement JWT-based authentication with role-based access control.
Includes login, logout, and token refresh endpoints."

Generate only the commit message, nothing else."#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_instruction() {
        let instruction = GeminiService::get_system_instruction();
        assert!(instruction.contains("conventional commit format"));
        assert!(instruction.contains("feat: ✨"));
        assert!(instruction.contains("Commit Types and Emojis"));
    }
}
