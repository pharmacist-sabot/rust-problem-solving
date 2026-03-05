use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};

fn main() {
    let src_dir = Path::new("src");
    if !src_dir.exists() || !src_dir.is_dir() {
        eprintln!("Error: 'src' directory not found.");
        std::process::exit(1);
    }

    // Find all part-* directories
    let mut part_dirs: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(src_dir).expect("Failed to read src directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("part-") {
                    part_dirs.push(path);
                }
            }
        }
    }

    // For each part directory, find markdown files, extract titles, and update index.md
    for dir in part_dirs {
        update_index_for_dir(&dir);
    }

    println!("Successfully updated all auto-indexes.");
}

fn update_index_for_dir(dir: &Path) {
    let index_file = dir.join("index.md");
    if !index_file.exists() {
        return;
    }

    // Collect info from all md files except index.md
    let mut lessons: Vec<(String, String)> = Vec::new(); // (Title, Filename)

    for entry in fs::read_dir(dir).expect("Failed to read part directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            if filename == "index.md" {
                continue;
            }

            if let Some(title) = extract_title(&path) {
                lessons.push((title, filename));
            } else {
                // If no heading found, just use the filename without extension
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                lessons.push((name, filename));
            }
        }
    }

    // Sort lessons alphabetically by title
    lessons.sort_by(|a, b| a.0.cmp(&b.0));

    // Read index.md
    let mut content = String::new();
    fs::File::open(&index_file)
        .expect("Failed to open index.md")
        .read_to_string(&mut content)
        .expect("Failed to read index.md");

    // Replace everything between markers
    let start_marker = "<!-- AUTO-INDEX-START -->";
    let end_marker = "<!-- AUTO-INDEX-END -->";

    let start_idx = content.find(start_marker);
    let end_idx = content.find(end_marker);

    if let (Some(start), Some(end)) = (start_idx, end_idx) {
        if start < end {
            let mut new_content = String::new();
            new_content.push_str(&content[..start + start_marker.len()]);
            new_content.push('\n');
            
            for (title, filename) in lessons {
                new_content.push_str(&format!("- [{}]({})\n", title, filename));
            }
            
            new_content.push_str(&content[end..]);

            // Write back to index.md
            let mut file = fs::File::create(&index_file).expect("Failed to create index.md");
            file.write_all(new_content.as_bytes()).expect("Failed to write index.md");
            println!("Updated auto-index in {}", index_file.display());
        } else {
            eprintln!("Warning: Markers are out of order in {}", index_file.display());
        }
    } else {
        eprintln!("Warning: Auto-index markers not found in {}", index_file.display());
    }
}

fn extract_title(path: &Path) -> Option<String> {
    let mut content = String::new();
    if fs::File::open(path).and_then(|mut f| f.read_to_string(&mut content)).is_ok() {
        for line in content.lines() {
            if line.starts_with("# ") {
                let title = line[2..].trim();
                return Some(clean_markdown_features(title));
            } else if line.starts_with("## ") {
                let title = line[3..].trim();
                return Some(clean_markdown_features(title));
            }
        }
    }
    None
}

// Strip some common Markdown formatting from the extracted title for clean links
fn clean_markdown_features(text: &str) -> String {
    text.replace("**", "").replace("__", "").replace("`", "")
}
