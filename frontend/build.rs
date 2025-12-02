use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("post_slugs.rs");

    // Path to the posts directory
    let posts_dir = Path::new("../content/posts");

    let mut slugs = Vec::new();

    if posts_dir.exists() {
        // Read all .md files from the posts directory
        if let Ok(entries) = fs::read_dir(posts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "md" {
                        if let Some(file_stem) = path.file_stem() {
                            if let Some(slug) = file_stem.to_str() {
                                slugs.push(slug.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort slugs for consistent ordering
    slugs.sort();

    // Generate the Rust code
    let mut file = fs::File::create(&dest_path).unwrap();
    writeln!(file, "// Auto-generated list of post slugs").unwrap();
    writeln!(file, "pub fn get_all_post_slugs() -> Vec<&'static str> {{").unwrap();
    writeln!(file, "    vec![").unwrap();

    for slug in &slugs {
        writeln!(file, "        \"{}\",", slug).unwrap();
    }

    writeln!(file, "    ]").unwrap();
    writeln!(file, "}}").unwrap();

    // Tell cargo to rerun this build script if the posts directory changes
    println!("cargo:rerun-if-changed=../content/posts");

    // Generate authentication credentials from environment variable
    generate_auth_config(&out_dir);

    println!("Generated post slugs: {:?}", slugs);
}

fn generate_auth_config(out_dir: &str) {
    let dest_path = Path::new(out_dir).join("auth_config.rs");
    let mut file = fs::File::create(&dest_path).unwrap();

    // Read AUTH_CREDENTIALS environment variable (format: "username:password")
    let auth_credentials =
        env::var("AUTH_CREDENTIALS").unwrap_or_else(|_| "demo:demo123".to_string());

    // Encode to base64
    let encoded = base64_encode(&auth_credentials);
    let expected_header = format!("Basic {}", encoded);

    writeln!(file, "// Generated authentication configuration").unwrap();
    writeln!(file, "pub fn get_expected_auth_header() -> &'static str {{").unwrap();
    writeln!(file, "    \"{}\"", expected_header).unwrap();
    writeln!(file, "}}").unwrap();

    println!("cargo:rerun-if-env-changed=AUTH_CREDENTIALS");
    println!(
        "Generated auth config with credentials: {}",
        auth_credentials
    );
}

fn base64_encode(input: &str) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let bytes = input.as_bytes();

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(CHARSET[((b >> 18) & 63) as usize] as char);
        result.push(CHARSET[((b >> 12) & 63) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARSET[((b >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARSET[(b & 63) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}
