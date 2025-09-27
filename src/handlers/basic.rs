use worker::*;

pub async fn handle_root(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // Check if the request accepts HTML
    let accept_header = _req.headers().get("Accept").unwrap_or_default();

    if accept_header.map_or(false, |h| h.contains("text/html")) {
        // Serve the web interface for browser requests
        let html_content = include_str!("../../public/index.html");
        let headers = Headers::new();
        headers.set("Content-Type", "text/html")?;
        Ok(Response::ok(html_content)?.with_headers(headers))
    } else {
        // Serve API information for non-browser requests (like curl)
        Response::ok("Git Diff to Commit Message API\n\nPOST /generate-commit\n{\n  \"diff\": \"your git diff content\",\n  \"gemini_api_key\": \"optional - if not provided, uses environment variable GEMINI_API_KEY\"\n}\n\nGET /health - Health check\n\nGET / (with Accept: text/html) - Web interface")
    }
}

pub async fn handle_health(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("OK")
}
