use serde::{Deserialize, Serialize};

/// Metadata for a blog post, used for listing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogPostMeta {
    /// Unique identifier/slug for the blog post
    pub slug: String,
    /// Title of the blog post
    pub title: String,
    /// Short description/excerpt
    pub description: String,
    /// Author name
    pub author: String,
    /// Publication date in ISO 8601 format (YYYY-MM-DD)
    pub date: String,
    /// Optional cover image path (relative to /assets)
    pub cover_image: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// OpenGraph title (defaults to title if not specified)
    #[serde(default)]
    pub og_title: Option<String>,
    /// OpenGraph description (defaults to description if not specified)
    #[serde(default)]
    pub og_description: Option<String>,
    /// OpenGraph image URL (defaults to cover_image if not specified)
    #[serde(default)]
    pub og_image: Option<String>,
    /// OpenGraph type (e.g., "article", "website")
    #[serde(default)]
    pub og_type: Option<String>,
}

/// Full blog post including content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogPost {
    /// Metadata
    #[serde(flatten)]
    pub meta: BlogPostMeta,
    /// Markdown content of the blog post
    pub content: String,
}
