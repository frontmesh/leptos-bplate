use crate::components::blog::{
    BlogPostContent, BlogPostError, BlogPostHeader, BlogPostLoading, BlogPostMetaTags,
    BlogPostNotFound,
};
use crate::components::Layout;
use crate::server_functions::posts::fetch_blog_post;
use leptos::prelude::*;
use leptos::{component, IntoView};
use leptos_router::hooks::use_params_map;

/// Main blog post page component
#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let post = Resource::new(move || slug(), |slug| fetch_blog_post(slug));

    view! {
        <Layout>
            <div class="pt-24 pb-20 px-4 sm:px-6 lg:px-8">
                <div class="max-w-3xl mx-auto">
                    <Suspense fallback=move || view! { <BlogPostLoading /> }>
                        {move || {
                            match post.get() {
                                Some(Ok(Some(blog_post))) => {
                                    view! {
                                        <BlogPostMetaTags post=blog_post.clone() />

                                        <article>
                                            <BlogPostHeader post=blog_post.clone() />
                                            <BlogPostContent html_content=blog_post.content.clone() />
                                        </article>
                                    }.into_any()
                                },
                                Some(Ok(None)) => {
                                    view! { <BlogPostNotFound /> }.into_any()
                                },
                                Some(Err(e)) => {
                                    view! {
                                        <BlogPostError error=format!("Error loading post: {}", e) />
                                    }
                                    .into_any()
                                }
                                _ => view! { <BlogPostLoading /> }.into_any(),
                            }
                        }}
                    </Suspense>
                </div>
            </div>
        </Layout>
    }
}
