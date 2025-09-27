use crate::handlers::{handle_generate_commit, handle_health, handle_root};
use worker::*;

pub fn setup_routes() -> Router<'static, ()> {
    Router::new()
        .get_async("/", handle_root)
        .get_async("/health", handle_health)
        .post_async("/generate-commit", handle_generate_commit)
}
