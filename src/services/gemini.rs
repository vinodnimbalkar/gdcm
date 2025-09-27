use crate::types::{GeminiContent, GeminiPart, GeminiRequest, GeminiResponse};
use worker::*;

pub struct GeminiService;

impl GeminiService {
    pub async fn generate_commit_message(
        diff: &str,
        api_key: &str,
    ) -> std::result::Result<String, String> {
        let prompt = Self::create_commit_prompt(diff);

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            api_key
        );

        let request_body = GeminiRequest {
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
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.trim().to_string())
            .ok_or_else(|| "No commit message generated".to_string())?;

        Ok(commit_message)
    }

    fn create_commit_prompt(diff: &str) -> String {
        format!(
            r#"You are an expert developer tasked with creating meaningful Git commit messages based on code diffs.

Given the following Git diff, generate a single commit message that follows these guidelines:

1. Use conventional commit format: <type>: <emoji><message>
2. Include appropriate emoji based on the change type
3. Keep the summary line under 50 characters when possible
4. Be specific and descriptive about what changed
5. If there are multiple changes, focus on the most significant one

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
- "feat: ✨Add user authentication system"
- "fix: 🐛Resolve memory leak in data processing"
- "docs: 📚Update API documentation"
- "refactor: 🔨Simplify user service logic"

Git diff:
```
{}
```

Generate only the commit message, nothing else:"#,
            diff
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_commit_prompt() {
        let diff = "diff --git a/src/main.rs b/src/main.rs\nindex 1234567..abcdefg 100644\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1,3 +1,4 @@\n fn main() {\n+    println!(\"Hello, World!\");\n     // existing code\n }";
        let prompt = GeminiService::create_commit_prompt(diff);
        assert!(prompt.contains("Git diff:"));
        assert!(prompt.contains(diff));
        assert!(prompt.contains("conventional commit format"));
    }
}
