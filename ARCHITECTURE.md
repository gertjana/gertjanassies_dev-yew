# Architecture Diagram - gertjanassies.dev

```
┌────────────────────────────────────────────────────────────────────────────┐
│                                 Browser                                    │
│                                                                            │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │                         Single Page Application                       │ │
│  │                                                                       │ │
│  │  ┌─────────────────────────────────────────────────────────────────┐  │ │
│  │  │              Yew Frontend (WASM Module)                         │  │ │
│  │  │                                                                 │  │ │
│  │  │  • Components (Header, Footer, BlogPage, etc.)                  │  │ │
│  │  │  • Routing (yew-router)                                         │  │ │
│  │  │  • Markdown Rendering                                           │  │ │
│  │  │  • Authentication Wrapper                                       │  │ │
│  │  │  • Page Stats Display                                           │  │ │
│  │  │                                                                 │  │ │
│  │  │  Compiled from: frontend/src/*.rs                               │  │ │
│  │  └─────────────────────────────────────────────────────────────────┘  │ │
│  │                                  │                                    │ │
│  │                                  │ HTTP Requests                      │ │
│  │                                  ▼                                    │ │
│  │                          ┌───────────────┐                            │ │
│  │                          │  Static Files │                            │ │
│  │                          │   (Nginx)     │                            │ │
│  │                          └───────────────┘                            │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────────┘
                                      │
                          HTTP Requests (Port 80)
                                      │
                                      ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Server Infrastructure                            │
│                                                                             │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                         Nginx Web Server                               │ │
│  │                                                                        │ │
│  │  • Serves static content (HTML, WASM, CSS, JS)                         │ │
│  │  • Serves markdown files from /content/                                │ │
│  │  • Reverse proxy to page-stats-server                                  │ │
│  │  • Configured via: deploy/nginx.conf                                   │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                      │                                      │
│                      ┌───────────────┴───────────────┐                      │
│                      │                               │                      │
│                      ▼                               ▼                      │
│  ┌─────────────────────────────────┐   ┌─────────────────────────────┐      │
│  │        Static Content           │   │   API Requests (/api/*)     │      │
│  │                                 │   │                             │      │
│  │  /content/                      │   │   Proxied to:               │      │
│  │  ├── images/                    │   │   localhost:3001            │      │
│  │  │   ├── badges/                │   │                             │      │
│  │  │   └── *.png, *.jpg           │   └─────────────┬───────────────┘      │
│  │  ├── pages/                     │                 │                      │
│  │  │   ├── about.md               │                 │                      │
│  │  │   ├── blog.md                │                 │                      │
│  │  │   └── home.md                │                 ▼                      │
│  │  └── posts/                     │   ┌──────────────────────────────┐     │
│  │      ├── 210609_*.md            │   │  Page Stats Server (Axum)    │     │
│  │      ├── 220528_*.md            │   │                              │     │
│  │      ├── 240101_*.md            │   │  • GET /api/stats/:page      │     │
│  │      └── 251202_*.md            │   │  • POST /api/stats/:page     │     │
│  │                                 │   │  • Track views & likes       │     │
│  └─────────────────────────────────┘   │  • Built with Rust/Axum      │     │
│                                        │                              │     │
│  /frontend/dist/                       │  Port: 3001                  │     │
│  ├── index.html                        │  Source: backend/src/        │     │
│  ├── *.wasm                            └──────────────┬───────────────┘     │
│  ├── *.js (wasm-bindgen glue)                         │                     │
│  └── styles.css                                       │ Redis Protocol      │
│                                                       │                     │
│  /static/                                             ▼                     │
│  ├── fonts/                            ┌──────────────────────────────┐     │
│  ├── js/                               │      Redis Database          │     │
│  │   ├── copy-code.js                  │                              │     │
│  │   └── katex-init.js                 │  • Stores page statistics    │     │
│  └── styles/                           │  • Keys: {env}:stats:{page}  │     │
│                                        │  • Data: views, likes        │     │
│                                        │  • Persistent storage        │     │
│                                        │                              │     │
│                                        │  Port: 6379 (internal)       │     │
│                                        │  Volume: redis_data:/data    │     │
│                                        └──────────────────────────────┘     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                            Build Process                                    │
│                                                                             │
│  Frontend Build (Trunk):                                                    │
│  • frontend/src/**/*.rs  ──[cargo + wasm-pack]──> frontend/dist/*.wasm      │
│  • frontend/index.html   ──[trunk build]───────> frontend/dist/index.html   │
│  • frontend/index.scss   ──[sass]─────────────> frontend/dist/styles.css    │
│                                                                             │
│  Backend Build (Cargo):                                                     │
│  • backend/src/**/*.rs   ──[cargo build]──────> page-stats-server binary    │
│                                                                             │
│  Docker Build:                                                              │
│  • deploy/Dockerfile     ──[multi-stage]──────> Combined Nginx + Server     │
│  • docker-compose.yml    ──[orchestration]────> Redis + Blog services       │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                          Key Technologies                                   │
│                                                                             │
│  Frontend:                 Backend:                  Infrastructure:        │
│  • Rust + Yew             • Rust + Axum             • Nginx (web server)    │
│  • WebAssembly            • Redis client            • Redis (cache/store)   │
│  • wasm-bindgen           • Tower HTTP              • Docker + Compose      │
│  • yew-router             • Serde (JSON)            • Supervisor            │
│  • pulldown-cmark         • Tokio (async)                                   │
│  • web-sys APIs           • Tracing (logging)                               │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                          Data Flow                                          │
│                                                                             │
│  1. User visits URL (e.g., /blog, /post/rust_on_esp32)                      │
│  2. Browser loads index.html + WASM module from Nginx                       │
│  3. Yew app initializes, router determines which component to render        │
│  4. Component fetches markdown content from /content/posts/*.md             │
│  5. Component calls /api/stats/:page?track_view=true                        │
│  6. Page stats server queries/updates Redis                                 │
│  7. Stats returned to WASM, displayed in PageStatsDisplay component         │
│  8. User can like/unlike, triggers POST to /api/stats/:page                 │
│                                                                             │
│  Private Content Flow (e.g., /private/resume):                              │
│  1. Router directs to Private route                                         │
│  2. AuthWrapper checks localStorage['Authorization'] header                 │
│  3. If valid Basic Auth: renders wrapped Page component                     │
│  4. If invalid: shows "Access denied" message                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Component Details

### Frontend Components (`frontend/src/components/`)
- `header.rs` - Navigation header with logo and links
- `footer.rs` - Page footer with social links
- `homepage.rs` - Landing page content
- `blogpage.rs` - Blog post listing with filtering
- `aboutpage.rs` - About/bio page
- `posts.rs` - Individual blog post viewer
- `page.rs` - Generic markdown page renderer
- `auth_wrapper.rs` - Authentication wrapper for private content
- `page_stats_display.rs` - View/like counter display
- `image.rs` - Image component with lazy loading
- `notfoundpage.rs` - 404 error page

### Backend Services (`backend/src/`)
- `main.rs` - Axum web server, routing, CORS
- `redis_client.rs` - Redis connection pool and operations

### Configuration Files
- `Cargo.toml` - Workspace dependencies
- `Trunk.toml` - Frontend build configuration
- `deploy/nginx.conf` - Web server configuration
- `deploy/supervisord.conf` - Process management
- `docker-compose.yml` - Container orchestration
