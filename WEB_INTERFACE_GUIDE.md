# Web Interface Guide

## 🌐 How to Access the Web Interface

The web interface is **automatically embedded** in your Cloudflare Worker and served when you visit the root URL in a browser.

### Access URLs:
- **Local Development**: `http://localhost:8787`
- **Production**: `https://your-worker-name.your-subdomain.workers.dev`

---

## 📋 Web Interface Layout

```
┌─────────────────────────────────────────────────────┐
│                🚀 GCM                               │
│        AI-Powered Git Commit Message Generator      │
└─────────────────────────────────────────────────────┘
│                                                     │
│  ┌─ Gemini API Key (optional) ─────────────────┐   │
│  │ [Enter your Gemini API key or leave empty]  │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  ┌─ Git Diff ──────────────────────────────────┐   │
│  │                                             │   │
│  │  [Paste your git diff here...]              │   │
│  │                                             │   │ ← **THIS IS THE TEXTAREA**
│  │  ↑ LARGE TEXTAREA FOR YOUR GIT DIFF ↑      │   │
│  │                                             │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  [Load Sample Diff] ← Click to test with example   │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │        Generate Commit Message              │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  📖 Example Usage:                                  │
│  1. Run `git diff` or `git diff --staged`          │
│  2. Copy the output and paste above ↑              │
│  3. Click "Generate Commit Message"                 │
│  4. Copy generated message for git commit          │
└─────────────────────────────────────────────────────┘
```

---

## 🔄 Step-by-Step Workflow

### Step 1: Get Your Git Diff
```bash
# For unstaged changes
git diff

# For staged changes (recommended)
git diff --staged

# Example output:
# diff --git a/src/main.rs b/src/main.rs
# index 1234567..abcdefg 100644
# --- a/src/main.rs
# +++ b/src/main.rs
# @@ -1,3 +1,4 @@
#  fn main() {
# +    println!("Hello, World!");
#      // existing code
#  }
```

### Step 2: Copy & Paste into Web Interface
1. **Select all** the git diff output from your terminal
2. **Copy** (Ctrl+C / Cmd+C)
3. **Open** your Worker URL in a browser
4. **Paste** into the large "Git Diff" textarea
5. **Optionally** enter your Gemini API key (if not set in environment)

### Step 3: Generate & Use
1. Click **"Generate Commit Message"**
2. Wait for AI processing (shows spinner)
3. **Copy** the generated commit message
4. Use with: `git commit -m "paste-generated-message-here"`

---

## 🛠 Technical Details

### How the Web Interface Works:
- **Smart Detection**: Browsers get HTML, APIs get JSON
- **Embedded HTML**: The interface is compiled into the Worker binary
- **No External Files**: Everything runs from the single Worker script
- **Auto-Deploy**: Interface updates when you deploy the Worker

### Code Location:
```
gdcm/
├── public/index.html          ← Web interface source
├── src/handlers/basic.rs      ← Handler that serves the HTML
└── build/index.js             ← Compiled Worker (includes HTML)
```

### Key Code Snippets:

**In `src/handlers/basic.rs`:**
```rust
// Detects if request wants HTML (browser) vs JSON (API)
let accept_header = _req.headers().get("Accept").unwrap_or_default();

if accept_header.map_or(false, |h| h.contains("text/html")) {
    // Serve web interface
    let html_content = include_str!("../../public/index.html");
    // ... serve HTML
} else {
    // Serve API docs
    Response::ok("Git Diff to Commit Message API...")
}
```

**In `public/index.html`:**
```html
<textarea id="diffInput" 
          placeholder="Paste your git diff here..." 
          required>
</textarea>
```

---

## 🎯 Quick Test

Want to test immediately? Use this sample diff:

```
diff --git a/README.md b/README.md
index 1234567..abcdefg 100644
--- a/README.md
+++ b/README.md
@@ -1 +1,2 @@
 # My Project
+This project does amazing things!
```

1. Copy the above diff
2. Open your Worker URL in browser
3. Paste into the textarea
4. Click "Generate Commit Message"
5. Should generate something like: `docs: 📚Add project description`

---

## ❓ Troubleshooting

**Can't see the web interface?**
- ✅ Make sure you're opening the URL in a **web browser** (not curl)
- ✅ Check the URL is correct (should be your Worker domain)
- ✅ Try clearing browser cache

**Textarea not working?**
- ✅ JavaScript should be enabled in your browser
- ✅ Try the "Load Sample Diff" button to test
- ✅ Check browser console for any errors

**API key not working?**
- ✅ Get a valid key from [Google AI Studio](https://makersuite.google.com/app/apikey)
- ✅ Or set `GEMINI_API_KEY` environment variable in Cloudflare
- ✅ The interface saves your key in browser localStorage

---

That's it! The textarea you're looking for is right in the web interface at your Worker's root URL! 🎉