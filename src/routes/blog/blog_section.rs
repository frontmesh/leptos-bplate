use crate::components::Layout;
use crate::models::blog::BlogPostMeta;
use leptos::prelude::*;
use leptos::{component, IntoView};

#[cfg(feature = "ssr")]
use crate::server_functions::posts::process_posts;

/// Fetches the list of blog posts from KV
#[server(FetchBlogList, "/api")]
pub async fn fetch_blog_list() -> Result<Vec<BlogPostMeta>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let blog_path = "content/blog";
        let posts = process_posts(blog_path);
        let meta: Vec<BlogPostMeta> = posts.into_iter().map(|p| p.meta).collect();
        Ok(meta)
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new(
            "This function is only available on the server",
        ))
    }
}

#[component]
pub fn BlogListPage() -> impl IntoView {
    let posts = Resource::new(|| (), |_| fetch_blog_list());

    view! {
        <Layout>
            <div class="pt-24 pb-20 px-4 sm:px-6 lg:px-8">
                <div class="max-w-5xl mx-auto">
                    <div class="mb-12">
                        <h1 class="text-5xl font-bold text-gray-900 mb-4">
                            "Blog"
                        </h1>
                        <p class="text-xl text-gray-600">
                            "Thoughts, tutorials, and updates from our team."
                        </p>
                    </div>

                    <Suspense fallback=move || view! {
                        <div class="flex justify-center py-12">
                            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                        </div>
                    }>
                        {move || {
                            match posts.get() {
                                Some(Ok(blog_posts)) => {
                                    if blog_posts.is_empty() {
                                        view! {
                                            <div class="py-12">
                                                <p class="text-gray-500 text-lg">
                                                    "No blog posts yet. Check back soon!"
                                                </p>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="space-y-8">
                                                {blog_posts.into_iter().map(|post| {
                                                    view! {
                                                        <BlogPostCard post=post />
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        }.into_any()
                                    }
                                },
                                _ => view! {
                                    <div class="py-12">
                                        <p class="text-red-500 text-lg">
                                            "Error loading posts"
                                        </p>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </Suspense>
                </div>
            </div>
        </Layout>
    }
}

#[component]
fn BlogPostCard(post: BlogPostMeta) -> impl IntoView {
    let href = format!("/blog/{}", post.slug);

    view! {
        <article class="bg-white border border-gray-200 rounded-2xl overflow-hidden shadow-all text-left">
            <div class="p-6">
                <a href=href class="group">
                    <h2 class="text-2xl font-bold text-gray-900 group-hover:text-blue-600 transition mb-3">
                        {post.title.clone()}
                    </h2>

                    <p class="text-gray-600 mb-4">
                        {post.description.clone()}
                    </p>
                </a>
                {post.cover_image.clone().map(|image| {
                    view! {
                        <div class="aspect-video overflow-hidden mb-4">
                            <img
                                src=format!("/assets/{}", image)
                                alt=post.title.clone()
                                class="w-full h-full object-cover"
                            />
                        </div>
                    }
                })}
                <div class="flex flex-wrap gap-2">
                    {post.tags.iter().map(|tag| {
                        view! {
                            <span class="px-3 py-1 bg-gray-100 text-gray-700 text-sm rounded-full">
                                {tag.clone()}
                            </span>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </article>
    }
}
