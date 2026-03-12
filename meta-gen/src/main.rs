//! meta-gen: Generates static HTML sidecar pages for social sharing.
//!
//! For each post in `content/posts/*.md`, writes a file at
//! `dist/post/{slug}/index.html` containing the correct OG and Twitter Card
//! meta tags derived from the post's YAML frontmatter, plus a JS redirect so
//! real browsers are immediately sent to the SPA route `/post/{slug}`.
//!
//! Usage:
//!   meta-gen [--content-dir <path>] [--dist-dir <path>]
//!
//! Defaults:
//!   --content-dir  content
//!   --dist-dir     dist

use std::fs;
use std::path::PathBuf;

const BASE_URL: &str = "https://gertjanassies.dev";
const TWITTER_HANDLE: &str = "@major7";
const FALLBACK_IMAGE: &str = "/static/logo_ga.svg";

// ---------------------------------------------------------------------------
// CLI argument parsing (no external deps)
// ---------------------------------------------------------------------------

struct Args {
    content_dir: PathBuf,
    dist_dir: PathBuf,
}

impl Args {
    fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut content_dir = PathBuf::from("content");
        let mut dist_dir = PathBuf::from("dist");

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--content-dir" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        content_dir = PathBuf::from(v);
                    }
                }
                "--dist-dir" => {
                    i += 1;
                    if let Some(v) = args.get(i) {
                        dist_dir = PathBuf::from(v);
                    }
                }
                other => {
                    eprintln!("Unknown argument: {other}");
                    std::process::exit(1);
                }
            }
            i += 1;
        }

        Args {
            content_dir,
            dist_dir,
        }
    }
}

// ---------------------------------------------------------------------------
// Frontmatter types — mirrors frontend/src/components/posts.rs
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct PostFrontmatter {
    title: String,
    date: String,
    summary: String,
    author: String,
    tags: Vec<String>,
    image: String,
    published: bool,
}

// ---------------------------------------------------------------------------
// YAML frontmatter parser — mirrors the hand-rolled parser in the frontend
// ---------------------------------------------------------------------------

fn parse_yaml_frontmatter(yaml_str: &str) -> PostFrontmatter {
    let mut fm = PostFrontmatter {
        published: true,
        ..Default::default()
    };

    for line in yaml_str.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            // A value may contain colons (e.g. in titles), so we split on the
            // first colon only and reassemble the rest.
            let value = value.trim().trim_matches('"').trim_matches('\'');

            match key {
                "title" => fm.title = value.to_string(),
                "date" => fm.date = value.trim_matches('"').trim_matches('\'').to_string(),
                "summary" => fm.summary = value.to_string(),
                "author" => fm.author = value.to_string(),
                "image" => fm.image = value.to_string(),
                "published" => fm.published = value.to_lowercase() == "true",
                "tags" => {
                    let raw = if value.starts_with('[') && value.ends_with(']') {
                        &value[1..value.len() - 1]
                    } else {
                        value
                    };
                    fm.tags = raw
                        .split(',')
                        .map(|t| t.trim().trim_matches('"').trim_matches('\'').to_string())
                        .filter(|t| !t.is_empty())
                        .collect();
                }
                _ => {}
            }
        }
    }

    fm
}

fn parse_post(raw: &str) -> Option<PostFrontmatter> {
    if !raw.starts_with("---\n") {
        return None;
    }

    let lines: Vec<&str> = raw.lines().collect();
    let mut frontmatter_end = None;
    let mut found_first = false;

    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "---" {
            if found_first {
                frontmatter_end = Some(i);
                break;
            } else {
                found_first = true;
            }
        }
    }

    frontmatter_end.map(|end| {
        let fm_str = lines[1..end].join("\n");
        parse_yaml_frontmatter(&fm_str)
    })
}

// ---------------------------------------------------------------------------
// Date normalisation: "20240101" → "2024-01-01", passthrough if already ISO
// ---------------------------------------------------------------------------

fn normalise_date(date: &str) -> String {
    let d = date.trim().trim_matches('"').trim_matches('\'');
    if d.len() == 8 && d.chars().all(|c| c.is_ascii_digit()) {
        // YYYYMMDD → YYYY-MM-DD
        format!("{}-{}-{}", &d[0..4], &d[4..6], &d[6..8])
    } else {
        d.to_string()
    }
}

// ---------------------------------------------------------------------------
// HTML generation
// ---------------------------------------------------------------------------

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn generate_html(slug: &str, fm: &PostFrontmatter) -> String {
    let title = escape_html(&fm.title);
    let full_title = if fm.title.is_empty() {
        "gertjanassies.dev".to_string()
    } else {
        format!("{} - gertjanassies.dev", title)
    };

    let description = escape_html(&fm.summary);
    let url = escape_html(&format!("{}/post/{}", BASE_URL, slug));

    let image = if fm.image.is_empty() {
        escape_html(&format!("{}{}", BASE_URL, FALLBACK_IMAGE))
    } else {
        escape_html(&format!("{}{}", BASE_URL, fm.image))
    };

    let author = escape_html(&fm.author);
    let date_iso = normalise_date(&fm.date);
    let published_time = if date_iso.is_empty() {
        String::new()
    } else {
        format!("{}T00:00:00Z", date_iso)
    };

    // Build article:tag lines
    let tag_metas: String = fm
        .tags
        .iter()
        .map(|t| {
            format!(
                "  <meta property=\"article:tag\" content=\"{}\" />\n",
                escape_html(t)
            )
        })
        .collect();

    let published_meta = if published_time.is_empty() {
        String::new()
    } else {
        format!(
            "  <meta property=\"article:published_time\" content=\"{}\" />\n",
            published_time
        )
    };

    let author_meta = if fm.author.is_empty() {
        String::new()
    } else {
        format!(
            "  <meta property=\"article:author\" content=\"{}\" />\n",
            author
        )
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>{full_title}</title>
  <meta name="description" content="{description}" />
  <meta name="robots" content="index, follow" />
  <link rel="canonical" href="{url}" />

  <!-- Open Graph -->
  <meta property="og:title" content="{full_title}" />
  <meta property="og:description" content="{description}" />
  <meta property="og:image" content="{image}" />
  <meta property="og:url" content="{url}" />
  <meta property="og:type" content="article" />
  <meta property="og:site_name" content="gertjanassies.dev" />
{author_meta}{published_meta}{tag_metas}
  <!-- Twitter Card -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:creator" content="{TWITTER_HANDLE}" />
  <meta name="twitter:site" content="{TWITTER_HANDLE}" />
  <meta name="twitter:title" content="{full_title}" />
  <meta name="twitter:description" content="{description}" />
  <meta name="twitter:image" content="{image}" />

  <!-- Redirect real browsers to the SPA immediately -->
  <script>window.location.replace(window.location.pathname.replace(/\/index\.html?$/, ''));</script>
</head>
<body>
  <noscript>
    <p><a href=".">Read: {title}</a></p>
  </noscript>
</body>
</html>
"#
    )
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let args = Args::parse();
    let posts_dir = args.content_dir.join("posts");
    let dist_post_dir = args.dist_dir.join("post");

    if !posts_dir.exists() {
        eprintln!(
            "Error: content directory not found: {}",
            posts_dir.display()
        );
        std::process::exit(1);
    }

    let mut entries: Vec<PathBuf> = fs::read_dir(&posts_dir)
        .unwrap_or_else(|e| {
            eprintln!("Failed to read {}: {}", posts_dir.display(), e);
            std::process::exit(1);
        })
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|ext| ext == "md").unwrap_or(false))
        .collect();

    entries.sort();

    let mut generated = 0usize;
    let mut skipped = 0usize;

    for path in &entries {
        let slug = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };

        let raw = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("  [WARN] Could not read {}: {}", path.display(), e);
                continue;
            }
        };

        let fm = match parse_post(&raw) {
            Some(f) => f,
            None => {
                eprintln!("  [WARN] No frontmatter in {}", path.display());
                continue;
            }
        };

        if !fm.published {
            println!("  [SKIP] {} (draft)", slug);
            skipped += 1;
            continue;
        }

        let out_dir = dist_post_dir.join(&slug);
        if let Err(e) = fs::create_dir_all(&out_dir) {
            eprintln!("  [WARN] Could not create {}: {}", out_dir.display(), e);
            continue;
        }

        let html = generate_html(&slug, &fm);
        let out_file = out_dir.join("index.html");

        match fs::write(&out_file, html) {
            Ok(_) => {
                println!("  [OK]   {}", out_file.display());
                generated += 1;
            }
            Err(e) => {
                eprintln!("  [WARN] Could not write {}: {}", out_file.display(), e);
            }
        }
    }

    println!(
        "\nmeta-gen: generated {} pages, skipped {} drafts.",
        generated, skipped
    );

    if generated == 0 && entries.is_empty() {
        eprintln!(
            "Warning: no markdown files found in {}",
            posts_dir.display()
        );
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalise_date_compact() {
        assert_eq!(normalise_date("20240101"), "2024-01-01");
    }

    #[test]
    fn test_normalise_date_iso_passthrough() {
        assert_eq!(normalise_date("2024-01-01"), "2024-01-01");
    }

    #[test]
    fn test_normalise_date_quoted() {
        assert_eq!(normalise_date("\"2024-03-10\""), "2024-03-10");
    }

    #[test]
    fn test_parse_yaml_frontmatter_basic() {
        let yaml = r#"title: My Post
date: "2024-01-01"
summary: A great article
author: Gertjan Assies
tags: rust, wasm, featured
image: /content/images/test.png
published: true"#;

        let fm = parse_yaml_frontmatter(yaml);
        assert_eq!(fm.title, "My Post");
        assert_eq!(fm.summary, "A great article");
        assert_eq!(fm.author, "Gertjan Assies");
        assert_eq!(fm.image, "/content/images/test.png");
        assert!(fm.published);
        assert_eq!(fm.tags, vec!["rust", "wasm", "featured"]);
    }

    #[test]
    fn test_parse_yaml_frontmatter_bracket_tags() {
        let yaml = "tags: [rust, wasm]";
        let fm = parse_yaml_frontmatter(yaml);
        assert_eq!(fm.tags, vec!["rust", "wasm"]);
    }

    #[test]
    fn test_parse_post_no_frontmatter() {
        let raw = "Just some content without frontmatter";
        assert!(parse_post(raw).is_none());
    }

    #[test]
    fn test_parse_post_with_frontmatter() {
        let raw = "---\ntitle: Hello\npublished: true\n---\n\nContent here";
        let fm = parse_post(raw).unwrap();
        assert_eq!(fm.title, "Hello");
        assert!(fm.published);
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("a & b < c > d"), "a &amp; b &lt; c &gt; d");
        assert_eq!(escape_html("say \"hi\""), "say &quot;hi&quot;");
    }

    #[test]
    fn test_generate_html_contains_meta_tags() {
        let fm = PostFrontmatter {
            title: "Test Post".to_string(),
            date: "20240101".to_string(),
            summary: "A test summary".to_string(),
            author: "Gertjan Assies".to_string(),
            tags: vec!["rust".to_string()],
            image: "/content/images/test.png".to_string(),
            published: true,
        };
        let html = generate_html("test_slug", &fm);

        assert!(html.contains("og:title"));
        assert!(html.contains("og:description"));
        assert!(html.contains("og:image"));
        assert!(html.contains("twitter:card"));
        assert!(html.contains("summary_large_image"));
        assert!(html.contains("article:published_time"));
        assert!(html.contains("2024-01-01T00:00:00Z"));
        assert!(html.contains("article:tag"));
        assert!(html.contains("window.location.replace('/post/test_slug')"));
        assert!(html.contains("Test Post - gertjanassies.dev"));
    }

    #[test]
    fn test_generate_html_fallback_image() {
        let fm = PostFrontmatter {
            title: "No Image Post".to_string(),
            image: String::new(),
            published: true,
            ..Default::default()
        };
        let html = generate_html("no_image", &fm);
        assert!(html.contains(FALLBACK_IMAGE));
    }
}
