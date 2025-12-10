use leptos::{prelude::ServerFnError, *};

#[allow(unused_imports)]
use crate::models::blog::{BlogPost, BlogPostMeta};

#[server(GetPosts, "/api")]
async fn get_posts() -> Result<Vec<BlogPost>, ServerFnError> {
    let blog_path = "content/blog";

    let posts = process_posts(blog_path);

    Ok(posts)
}

/// Fetches a single blog post by slug
#[server(FetchBlogPost, "/api")]
pub async fn fetch_blog_post(slug: String) -> Result<Option<BlogPost>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let blog_path = "content/blog";
        let posts = process_posts(blog_path);

        // Find the post with matching slug
        let post = posts.into_iter().find(|p| p.meta.slug == slug);
        Ok(post)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "This function is only available on the server",
        ))
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{
            fs::{self, DirEntry},
            path::Path,
        };
        use chrono::NaiveDate;

        pub fn get_posts_file<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
            match fs::read_dir(path) {
                Ok(entries) => {
                    entries
                        .filter_map(Result::ok)
                        .filter(|entry| {
                            if let Ok(file_type) = entry.file_type() {
                                file_type.is_file() && entry.path().extension() == Some("md".as_ref())
                            } else {
                                false
                            }
                        })
                        .collect()
                },
                Err(e) => {
                    eprintln!("Error reading blog directory: {}", e);
                    Vec::new()
                }
            }
        }

        pub fn read_post_content(entry: DirEntry) -> Option<String> {
            fs::read_to_string(entry.path()).ok()
        }

        pub fn parse_post_content(content: &str) -> Option<BlogPost> {
            use gray_matter::engine::YAML;
            use gray_matter::Matter;
            use pulldown_cmark::{html, Options, Parser};

            let matter = Matter::<YAML>::new();

            let post_data = match matter.parse(content) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error parsing markdown with gray_matter: {}", e);
                    return None;
                }
            };

            // Deserialize the frontmatter Value into BlogPostMeta
            let post_metadata: BlogPostMeta = match post_data.data {
                Some(data) => {
                    match serde_json::from_value::<BlogPostMeta>(data) {
                        Ok(meta) => meta,
                        Err(e) => {
                            eprintln!("Error parsing frontmatter into BlogPostMeta: {}", e);
                            return None;
                        }
                    }
                },
                None => {
                    eprintln!("No frontmatter found in post");
                    return None;
                }
            };

            // Convert markdown to HTML on the server
            let markdown_content = post_data.content;
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_TASKLISTS);

            let parser = Parser::new_ext(&markdown_content, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            Some(BlogPost {
                meta: post_metadata,
                content: html_output,
            })
        }

        pub fn sort_posts(posts: &mut [BlogPost]) {
            posts.sort_by(|a, b| {
                let a_date = NaiveDate::parse_from_str(&a.meta.date, "%Y-%m-%d").unwrap();
                let b_date = NaiveDate::parse_from_str(&b.meta.date, "%Y-%m-%d").unwrap();
                //reverse sorting
                a_date.cmp(&b_date)
            });
        }

        // Code that compiles only with ssr feature
        pub fn process_posts<P: AsRef<Path>>(path: P) -> Vec<BlogPost> {
            let posts_text = get_posts_file(path);
            let mut posts = Vec::new();

            for entry in posts_text {
                if let Some(content) = read_post_content(entry) {
                    if let Some(post) = parse_post_content(&content) {
                        posts.push(post);
                    }
                }
            }

            sort_posts(&mut posts);

            posts
        }
    }
}
