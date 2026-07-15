use crate::models::blog::{BlogMeta, BlogPost};
use leptos::prelude::*;

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<BlogPost>, ServerFnError> {
    use std::fs;

    let mut posts = Vec::new();
    let paths = match fs::read_dir("content/blog") {
        Ok(paths) => paths,
        Err(_) => return Ok(posts),
    };

    for path in paths.flatten() {
        let path_buf = path.path();
        if path_buf.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Ok(content) = fs::read_to_string(&path_buf) {
                // Extract frontmatter manually by finding --- boundaries
                let mut lines = content.lines();
                if let Some(first_line) = lines.next() {
                    if first_line.trim() == "---" {
                        let mut yaml_str = String::new();
                        let mut markdown_content = String::new();
                        let mut in_frontmatter = true;

                        for line in lines {
                            if in_frontmatter {
                                if line.trim() == "---" {
                                    in_frontmatter = false;
                                } else {
                                    yaml_str.push_str(line);
                                    yaml_str.push('\n');
                                }
                            } else {
                                markdown_content.push_str(line);
                                markdown_content.push('\n');
                            }
                        }

                        if let Ok(meta) = serde_yaml::from_str::<BlogMeta>(&yaml_str) {
                            let slug = path_buf.file_stem().unwrap().to_str().unwrap().to_string();

                            use pulldown_cmark::{html, Parser};
                            let parser = Parser::new(&markdown_content);
                            let mut html_output = String::new();
                            html::push_html(&mut html_output, parser);

                            posts.push(BlogPost {
                                slug,
                                meta,
                                content_html: html_output,
                            });
                        }
                    }
                }
            }
        }
    }

    // Sort by date descending
    posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
    Ok(posts)
}

#[server(GetPost, "/api")]
pub async fn get_post(slug: String) -> Result<Option<BlogPost>, ServerFnError> {
    let posts = get_posts().await?;
    Ok(posts.into_iter().find(|p| p.slug == slug))
}
