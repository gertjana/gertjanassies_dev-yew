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

## Features

- **📝 Markdown-based Content**: All posts written in Markdown with YAML frontmatter
- **🎨 Syntax Highlighting**: Custom Prism.js integration for code blocks
- **🏗️ Build-time Optimization**: Automatic post discovery during compilation
- **📱 Responsive Design**: Mobile-first design with clean typography
- **🔍 Content Filtering**: Filter posts by categories, tags, and featured status
- **⚡ Fast Performance**: WASM-compiled Rust for optimal performance

## Architecture

```
src/
├── app.rs              # Main app and routing
├── components/         # Modular components
│   ├── header.rs       # Navigation header
│   ├── footer.rs       # Site footer
│   ├── homepage.rs     # Homepage with featured posts
│   ├── blogpage.rs     # Blog archive page
│   ├── aboutpage.rs    # About page
│   ├── notfoundpage.rs # 404 error page
│   └── posts.rs        # Post listing and individual post views
└── main.rs             # Application entry point

static/
├── posts/              # Markdown blog posts
├── images/             # Post images and assets
└── styles/             # Additional CSS/themes
```

## Getting Started

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Add WASM target: `rustup target add wasm32-unknown-unknown`
3. Install Trunk: `cargo install trunk wasm-bindgen-cli`

### Development

```bash
# Start development server with hot reload
trunk serve

# Build for production
trunk build --release

# Run tests
cargo test
```

### Adding Content

1. Create a new `.md` file in `static/posts/` with the format: `YYMMDD_post_title.md`
2. Add YAML frontmatter with title, date, tags, etc.
3. Write your content in Markdown
4. Rebuild - posts are automatically discovered at compile time!

## Technical Highlights

- **Component Architecture**: Clean separation with individual files for each page component
- **Compile-time Post Discovery**: Build script automatically finds and indexes all blog posts
- **Custom Syntax Highlighting**: Tailored Prism.js implementation optimized for WASM
- **Responsive Typography**: Roboto fonts for headers, FiraCode for code blocks
- **Modern CSS**: Flexbox/Grid layouts with mobile-first responsive design


## License

MIT

---

*This blog is a continuous experiment in web technologies and personal expression. Expect occasional technical deep-dives alongside random musings about code, the maker space, and life.*
# Automated Deployments Enabled
