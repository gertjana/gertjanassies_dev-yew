use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("post_slugs.rs");

    // Path to the posts directory
    let posts_dir = Path::new("static/posts");

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
    println!("cargo:rerun-if-changed=static/posts");

    println!("Generated post slugs: {:?}", slugs);
}
