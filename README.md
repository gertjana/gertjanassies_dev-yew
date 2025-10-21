# Personal Blog - Yew & WASM Edition

This is my personal blog built with [Yew](https://yew.rs/) and compiled to WebAssembly (WASM) using [Trunk](https://trunkrs.dev/). It's both a platform for sharing my thoughts on technology, coding, maker projects, and life, as well as a playground for experimenting with new web technologies.

## About This Project

This blog serves dual purposes:
- **Personal Space**: Where I share insights about technology, programming tutorials, maker projects, and random thoughts
- **Tech Exploration**: A testing ground for new frameworks and approaches - in this case, diving deep into Rust's web ecosystem with Yew and WASM

## Why Yew & WASM?

I use this blog to experiment with cutting-edge web technologies. This iteration explores:
- **Yew Framework**: Rust's component-based web framework
- **WebAssembly**: Running Rust code in the browser with near-native performance
- **Modern Web Patterns**: Single-page applications, component architecture

## Getting Started

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Add WASM target: `rustup target add wasm32-unknown-unknown`
3. Install Trunk: `cargo install trunk wasm-bindgen-cli`

### Development

The Makefile has plenty of hints

```
> make help
Available targets:
  help                 Show this help message
  build                Build the project
  build-release        Build in release mode
  build-frontend       Build frontend only
  build-server         Build server only
  build-frontend-web   Build frontend with Trunk (release)
  test                 Run all tests
  clean                Clean build artifacts
  serve-frontend       Start frontend development server
  run-server           Run the page stats server
  docker-build         Build Docker image
  check                Check workspace for errors
```

### Adding Content

1. Create a new `.md` file in `static/posts/` with the format: `YYMMDD_post_title.md`
2. Add YAML frontmatter with title, date, tags, etc.
3. Write your content in Markdown
4. Use custom components like `<Image />` for enhanced interactivity
5. Rebuild - posts are automatically discovered at compile time!


## Custom Components in Markdown

This blog features a custom markdown parser that allows embedding Yew components directly in markdown files:

### Available Components

- **`<Image />`**: Interactive image component with modal overlay
  - Props: `path`, `alt`, `thumbnail_width`, `class`
  - Example: `<Image path="/static/images/photo.png" alt="Description" thumbnail_width="600" />`



### Adding New Components

1. Create your component in `src/components/`
2. Implement the `MarkdownRenderable` trait
3. Register it in `src/markdown.rs` in the `COMPONENT_REGISTRY`
4. Use it in any markdown file with the component syntax

See `src/components/image.rs` for a reference implementation.

## Technical Highlights

- **Component Architecture**: Clean separation with individual files for each page component
- **Component-Aware Markdown**: Custom parser that supports embedding Yew components in markdown content
- **Interactive Image Modals**: Built-in `<Image />` component with thumbnail/fullsize modal viewing
- **Compile-time Post Discovery**: Build script automatically finds and indexes all blog posts
- **Custom Syntax Highlighting**: Tailored Prism.js implementation optimized for WASM
- **Math Rendering**: KaTeX integration for rendering LaTeX equations in posts
- **Modern Typography**: Custom web fonts - RedHat Text for body, FiraCode for code blocks
- **Modern CSS**: Flexbox/Grid layouts with mobile-first responsive design
- **Optimized Watch Configuration**: Smart file watching to prevent rebuild loops during development

## Docker Deployment

The blog includes a combined Docker container with both the frontend and a Rust-based API server for page statistics.

### Environment Variables

Configure the Redis backend using these environment variables:

- `REDIS_URL`: Redis connection URL (default: `redis://127.0.0.1:6379`)
- `APP_ENV`: Environment prefix for Redis keys (default: `production`)
- `PAGE_STATS_PORT`: Internal API server port (default: `3001`)
- `PAGE_STATS_HOST`: Internal API server host (default: `127.0.0.1`)

### Quick Start with Docker Compose

```bash
# Build the image
trunk build --release
docker build -f deploy/Dockerfile -t gertjanassies-blog .

# Run with Redis
docker-compose up
```

### Manual Docker Run

```bash
# Basic usage
docker run -p 80:80 gertjanassies-blog

# With external Redis
docker run -p 80:80 \
  -e REDIS_URL=redis://my-redis:6379 \
  -e APP_ENV=production \
  gertjanassies-blog
```

See `deploy/ENV_VARIABLES.md` for detailed configuration options.

## License

MIT
