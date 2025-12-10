use crate::models::blog::BlogPost;
use leptos::prelude::*;
use leptos::{component, IntoView};
use leptos_meta::Meta;

/// Header component for blog post with metadata
#[component]
pub fn BlogPostHeader(post: BlogPost) -> impl IntoView {
    view! {
        <div class="mb-8">
            <a href="/blog" class="text-blue-600 hover:text-blue-800 transition inline-flex text-left gap-2 mb-6">
                <span>"← Back to Blog"</span>
            </a>

            {post.meta.cover_image.clone().map(|img| {
                view! {
                    <div class="aspect-video overflow-hidden rounded-2xl mb-8">
                        <img
                            src=format!("/assets/{}", img)
                            alt=post.meta.title.clone()
                            class="w-full h-full object-cover"
                        />
                    </div>
                }
            })}

            <h1 class="text-2xl md:text-5xl font-bold text-gray-900 mb-4">
                {post.meta.title.clone()}
            </h1>

            <div class="flex items-center gap-4 text-gray-600 mb-6">
                <span>{post.meta.author.clone()}</span>
                <span class="text-gray-300">"•"</span>
                <span>{post.meta.date.clone()}</span>
            </div>

            <div class="flex flex-wrap gap-2 mb-8">
                {post.meta.tags.iter().map(|tag| {
                    view! {
                        <span class="px-3 py-1 bg-blue-100 text-blue-700 text-sm rounded-full">
                            {tag.clone()}
                        </span>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Meta tags component for SEO
#[component]
pub fn BlogPostMetaTags(post: BlogPost) -> impl IntoView {
    let og_title = post
        .meta
        .og_title
        .clone()
        .unwrap_or_else(|| post.meta.title.clone());
    let og_description = post
        .meta
        .og_description
        .clone()
        .unwrap_or_else(|| post.meta.description.clone());
    let og_image = post
        .meta
        .og_image
        .clone()
        .or_else(|| {
            post.meta
                .cover_image
                .clone()
                .map(|img| format!("/assets/{}", img))
        })
        .unwrap_or_default();
    let og_type = post
        .meta
        .og_type
        .clone()
        .unwrap_or_else(|| "article".to_string());
    let site_url = "https://rustbase.site";
    let canonical_url = format!("{}/blog/{}", site_url, post.meta.slug);

    view! {
        // OpenGraph meta tags
        <Meta name="og:title" content=og_title.clone() />
        <Meta name="og:description" content=og_description.clone() />
        <Meta name="og:image" content=og_image.clone() />
        <Meta name="og:type" content=og_type />
        <Meta name="og:url" content=canonical_url.clone() />
        <Meta name="og:site_name" content="Rustbase" />

        // Twitter Card meta tags
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:title" content=og_title />
        <Meta name="twitter:description" content=og_description />
        <Meta name="twitter:image" content=og_image />

        // Standard meta tags
        <Meta name="description" content=post.meta.description.clone() />
        <Meta name="canonical" content=canonical_url />
        <Meta name="author" content=post.meta.author.clone() />
        <Meta name="publish_date" content=post.meta.date.clone() />
    }
}

/// Content component for rendered markdown
#[component]
pub fn BlogPostContent(html_content: String) -> impl IntoView {
    view! {
        <div
            class="prose-blog"
            inner_html=html_content
        />
    }
}

/// Loading state component
#[component]
pub fn BlogPostLoading() -> impl IntoView {
    view! {
        <div class="flex py-12">
            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
        </div>
    }
}

/// Not found component
#[component]
pub fn BlogPostNotFound() -> impl IntoView {
    view! {
        <div class="text-center py-12">
            <h1 class="text-3xl font-bold text-gray-900 mb-4">
                "Post Not Found"
            </h1>
            <p class="text-gray-600 mb-6">
                "The blog post you're looking for doesn't exist."
            </p>
            <a href="/blog" class="text-blue-600 hover:text-blue-800 transition">
                "← Back to Blog"
            </a>
        </div>
    }
}

/// Error component
#[component]
pub fn BlogPostError(error: String) -> impl IntoView {
    view! {
        <div class="text-center py-12">
            <p class="text-red-500 text-lg">
                {error}
            </p>
            <a href="/blog" class="text-blue-600 hover:text-blue-800 transition mt-4 inline-block">
                "← Back to Blog"
            </a>
        </div>
    }
}
