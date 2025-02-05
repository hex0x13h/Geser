use anyhow::{Result, anyhow};
use tokio::fs;
use pulldown_cmark::{Parser, Event, Tag, Options};
use crate::cache::Cache;

/// Serves a Markdown file from the pages directory.
/// If the safe_path is "/" then load "pages_dir/index.md", otherwise load "pages_dir{safe_path}.md".
pub async fn serve_markdown(pages_dir: &str, safe_path: &str, cache: Cache) -> Result<String> {
    let file_path = if safe_path == "/" {
        format!("{}/index.md", pages_dir)
    } else {
        format!("{}{}.md", pages_dir, safe_path)
    };

    // Check cache first.
    if let Some(content) = cache.get_text(&file_path) {
        return Ok(content);
    }

    let content = fs::read_to_string(&file_path).await
        .map_err(|e| anyhow!("Failed to read file {}: {:?}", file_path, e))?;
    
    // Use pulldown-cmark to parse Markdown content.
    let parser = Parser::new_ext(&content, Options::all());
    let mut output = String::new();

    // State variables for handling links.
    let mut in_link = false;
    let mut link_url = String::new();
    let mut link_text = String::new();

    // Process the parsed events and convert them to Gemini formatted output.
    for event in parser {
        match event {
            // Process headings: match with 3 fields and output the appropriate number of '#' characters.
            Event::Start(Tag::Heading(level, _, _)) => {
                output.push('\n');
                for _ in 0..level as usize {
                    output.push('#');
                }
                output.push(' ');
            },
            Event::End(Tag::Heading(_, _, _)) => {
                output.push('\n');
            },
            // Process paragraphs.
            Event::Start(Tag::Paragraph) => {
                // Insert a newline before paragraph.
                output.push('\n');
            },
            Event::End(Tag::Paragraph) => {
                output.push('\n');
            },
            // Process links: collect link text and target URL, then convert to a Gemini link line.
            Event::Start(Tag::Link(_link_type, url, _title)) => {
                in_link = true;
                link_url = url.to_string();
                link_text.clear();
            },
            Event::End(Tag::Link(_, _, _)) => {
                in_link = false;
                // Output Gemini link on a separate line.
                let text = if link_text.trim().is_empty() {
                    link_url.clone()
                } else {
                    link_text.clone()
                };
                output.push_str(&format!("\n=> {} {}\n", link_url, text));
            },
            // Process images: convert Markdown image syntax directly to a Gemini link line.
            Event::Start(Tag::Image(_link_type, url, title)) => {
                output.push_str(&format!("\n=> {} {}\n", url, title));
            },
            // Process inline code: wrap with backticks.
            Event::Code(code) => {
                output.push_str(&format!("`{}`", code));
            },
            // Process text: if inside a link, accumulate into link_text; otherwise, output directly.
            Event::Text(text) => {
                if in_link {
                    link_text.push_str(&text);
                } else {
                    output.push_str(&text);
                }
            },
            // Handle line breaks.
            Event::SoftBreak | Event::HardBreak => {
                output.push('\n');
            },
            // Other events are ignored for simplicity.
            _ => {}
        }
    }
    // Cache the converted content.
    cache.set_text(file_path, output.clone());
    Ok(output)
}

/// Serves a static file (e.g., an image) from the pages directory.
/// The safe_path corresponds to a file inside pages_dir.
pub async fn serve_static_file(pages_dir: &str, safe_path: &str, cache: Cache) -> Result<(Vec<u8>, &'static str)> {
    let file_path = format!("{}{}", pages_dir, safe_path);
    // Check cache for binary file.
    if let Some(data) = cache.get_binary(&file_path) {
        let mime = get_mime_type(safe_path);
        return Ok((data, mime));
    }
    let data = fs::read(&file_path).await
        .map_err(|e| anyhow!("Failed to read file {}: {:?}", file_path, e))?;
    let mime = get_mime_type(safe_path);
    cache.set_binary(file_path, data.clone());
    Ok((data, mime))
}

fn get_mime_type(path: &str) -> &'static str {
    if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".gif") {
        "image/gif"
    } else {
        "application/octet-stream"
    }
}
