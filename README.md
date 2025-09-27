# GDCM - Git Diff to Commit Message API

A Cloudflare Worker-based web service that generates meaningful Git commit messages from Git diffs using Google's Gemini AI API. This is the web service version of the [gcm CLI tool](https://github.com/vinodnimbalkar/gcm).

## Features

- 🚀 **Fast & Serverless**: Built on Cloudflare Workers for global edge deployment
- 🤖 **AI-Powered**: Uses Google's Gemini 2.0 Flash model for intelligent commit message generation
- 📝 **Conventional Commits**: Generates commit messages following conventional commit standards
- 🎨 **Emoji Support**: Includes appropriate emojis based on change types
- 🌐 **Web Interface**: Includes a beautiful HTML interface for easy testing
- 🔒 **Secure**: API key can be provided via request or environment variable

## Quick Start

1. **Get a Gemini API Key**: Visit [Google AI Studio](https://makersuite.google.com/app/apikey) and create an API key

2. **Deploy the Worker**:
   ```bash
   git clone https://github.com/vinodnimbalkar/gdcm.git
   cd gdcm
   wrangler secret put GEMINI_API_KEY
   # Enter your API key when prompted
   wrangler deploy
   ```

3. **Use the Web Interface**: Open your deployed URL in a browser to access the beautiful web interface

4. **Or use with cURL**:
   ```bash
   curl -X POST https://your-worker-url.workers.dev/generate-commit \
     -H "Content-Type: application/json" \
     -d '{"diff": "your-git-diff-here"}'
   ```

## API Endpoints

### `POST /generate-commit`

Generates a commit message from a Git diff.

**Request Body:**
```json
{
  "diff": "your git diff content here",
  "gemini_api_key": "optional - your Gemini API key"
}
```

**Response (Success):**
```json
{
  "commit_message": "feat: ✨Add user authentication system\n\nImplement OAuth2 login flow with Google and GitHub providers",
  "service_used": "Gemini"
}
```

**Response (Error):**
```json
{
  "error": "Error message description"
}
```

### `GET /health`

Health check endpoint.

**Response:**
```
OK
```

## Setup & Deployment

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- Cloudflare account
- Google Gemini API key

### Local Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/vinodnimbalkar/gdcm.git
   cd gdcm
   ```

2. **Install dependencies:**
   ```bash
   cargo check
   ```

3. **Set up environment variables:**
   Create a `.env` file or set the environment variable:
   ```bash
   export GEMINI_API_KEY="your_gemini_api_key_here"
   ```

4. **Build the project:**
   ```bash
   wrangler build
   ```

5. **Run locally:**
   ```bash
   wrangler dev
   ```

   The service will be available at `http://localhost:8787`

### Production Deployment

1. **Set up secrets in Cloudflare:**
   ```bash
   wrangler secret put GEMINI_API_KEY
   # Enter your Gemini API key when prompted
   ```

2. **Deploy to Cloudflare Workers:**
   ```bash
   wrangler deploy
   ```

## Usage Examples

### Using cURL

```bash
# Basic usage
curl -X POST https://your-worker-url.workers.dev/generate-commit \
  -H "Content-Type: application/json" \
  -d '{
    "diff": "diff --git a/src/main.rs b/src/main.rs\nindex 1234567..abcdefg 100644\n--- a/src/main.rs\n+++ b/src/main.rs\n@@ -1,3 +1,4 @@\n fn main() {\n+    println!(\"Hello, World!\");\n     // existing code\n }"
  }'

# With API key in request
curl -X POST https://your-worker-url.workers.dev/generate-commit \
  -H "Content-Type: application/json" \
  -d '{
    "diff": "your git diff here",
    "gemini_api_key": "your_api_key_here"
  }'
```

### Using the Web Interface

The API includes a beautiful web interface that you can access by visiting your deployed worker URL in a browser.

**🌐 Web Interface Location:**
The web interface is automatically served when you visit the root URL (`/`) in any web browser. It's built right into the Worker and includes:

- **A large textarea** labeled "Git Diff" where you paste your `git diff` output
- **An optional API key field** for your Gemini API key
- **Generate button** to create the commit message
- **Copy to clipboard** functionality for easy use

**How it works:**
- When you visit the root URL (`/`) in a browser, it automatically serves the HTML interface
- When you make API calls (like with `curl`), it returns plain text API documentation

**Steps to use the web interface:**

1. **Navigate to your deployed worker URL** in a web browser:
   - Local development: `http://localhost:8787`
   - Production: `https://your-worker-name.your-subdomain.workers.dev`

2. **You'll see a form with two main fields:**
   - **Gemini API Key field** (optional if set as environment variable)
   - **Large Git Diff textarea** - this is where you paste your git diff content

3. **Get your Git diff and paste it**:
   - Run `git diff` or `git diff --staged` in your terminal
   - Copy the entire output
   - Paste it into the **"Git Diff" textarea** on the web page
   - Or click "Load Sample Diff" to test with example data

4. **Click "Generate Commit Message"**:
   - The interface shows a loading spinner while processing
   - Results appear below with the generated commit message

5. **Copy and use the generated message**:
   - Click "Copy to Clipboard" button
   - Use with `git commit -m "paste-message-here"`

**Features of the web interface:**
- ✨ Beautiful, responsive design that works on mobile and desktop
- 🔄 Real-time API calls with loading indicators
- 💾 Remembers your API key in browser storage
- 📋 One-click copy to clipboard functionality
- 🎯 Sample diff loader for quick testing
- ❌ Clear error messages and validation
- 🎨 Syntax highlighting for commit messages

**Need more help with the web interface?**
📖 See the detailed [Web Interface Guide](WEB_INTERFACE_GUIDE.md) for:
- Visual layout diagram showing exactly where the textarea is
- Step-by-step screenshots and workflow
- Troubleshooting common issues
- Technical implementation details

**Quick reference - The textarea code is located in:**
- File: `gdcm/public/index.html`
- Element: `<textarea id="diffInput" placeholder="Paste your git diff here..." required></textarea>`
- The HTML is embedded in the Worker using `include_str!("../../public/index.html")` in `src/handlers/basic.rs`

### Integration with Git Workflow

You can integrate this with your Git workflow using a shell function:

```bash
# Add to your .bashrc, .zshrc, etc.
gcm-api() {
    local diff_content
    diff_content=$(git diff --staged)

    if [ -z "$diff_content" ]; then
        echo "No staged changes found. Use 'git add' first."
        return 1
    fi

    local commit_msg
    commit_msg=$(curl -s -X POST "https://your-worker-url.workers.dev/generate-commit" \
        -H "Content-Type: application/json" \
        -d "{\"diff\": $(echo "$diff_content" | jq -Rs .)}" | \
        jq -r '.commit_message')

    if [ "$commit_msg" != "null" ] && [ -n "$commit_msg" ]; then
        echo "Generated commit message:"
        echo "$commit_msg"
        echo ""
        echo "Use this message? (y/n)"
        read -r response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            git commit -m "$commit_msg"
        fi
    else
        echo "Failed to generate commit message"
    fi
}
```

## Supported Commit Types & Emojis

| Type | Emoji | Description |
|------|-------|-------------|
| `feat` | ✨ | New feature |
| `fix` | 🐛 | Bug fix |
| `docs` | 📚 | Documentation |
| `style` | 💄 | Formatting, styling |
| `refactor` | 🔨 | Code refactoring |
| `test` | ✅ | Adding tests |
| `chore` | 🔧 | Maintenance tasks |
| `perf` | 🐎 | Performance improvement |
| `ci` | 💚 | Continuous integration |
| `build` | 📦 | Build system |
| `revert` | ⏪ | Reverting changes |
| `security` | 🔒 | Security improvements |
| `deps` | ⬆️ | Dependency updates |
| `remove` | 🔥 | Removing code/files |
| `wip` | 🚧 | Work in progress |

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google Gemini API key (required)

### Wrangler Configuration

The `wrangler.toml` file contains the Cloudflare Worker configuration:

```toml
name = "gdcm"
main = "build/index.js"
compatibility_date = "2025-09-27"

[build]
command = "cargo install -q worker-build && worker-build --release"
```

## Getting a Gemini API Key

1. Go to [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Click "Create API Key"
3. Copy the generated key
4. Set it as an environment variable or provide it in requests

## Error Handling

The API returns appropriate HTTP status codes:

- `200`: Success
- `400`: Bad request (invalid input)
- `500`: Internal server error (API failure, etc.)

Common error scenarios:
- Empty diff provided
- No API key provided
- Invalid API key
- Gemini API rate limits exceeded
- Network connectivity issues

## Development

### Project Structure

```
gdcm/
├── src/
│   ├── lib.rs          # Main entry point
│   ├── types.rs        # Request/Response types
│   ├── handlers/       # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── basic.rs    # Root and health handlers
│   │   └── commit.rs   # Commit generation handler
│   ├── services/       # Business logic services
│   │   ├── mod.rs
│   │   └── gemini.rs   # Gemini API service
│   └── routes/         # Router configuration
│       └── mod.rs
├── public/
│   └── index.html      # Web interface
├── test_api.js         # API testing script
├── WEB_INTERFACE_GUIDE.md  # Detailed web interface guide
├── CHANGELOG.md        # Project changelog
├── Cargo.toml          # Rust dependencies
├── wrangler.toml       # Cloudflare Worker config
└── README.md           # This file
```

### Testing

Run tests locally:
```bash
cargo test
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Related Projects

- [gcm CLI](https://github.com/vinodnimbalkar/gcm) - The original command-line version
- [Conventional Commits](https://www.conventionalcommits.org/) - Commit message specification

## Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/vinodnimbalkar/gdcm/issues) section
2. Create a new issue with detailed information
3. Include error messages, request/response examples, and environment details
