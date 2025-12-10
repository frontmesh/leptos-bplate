use leptos::prelude::*;
use leptos::IntoView;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <nav class="fixed w-full top-0 z-50 bg-white bg-opacity-95 backdrop-blur-md border-b border-gray-200">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    <div class="flex items-center">
                        <div class="text-lg font-bold text-blue-600">
                            "LeptosCloudflareWorkers"
                        </div>
                    </div>
                    <div class="hidden md:flex items-center space-x-8">
                        <a href="/#features" class="text-gray-600 hover:text-gray-900 font-medium transition">
                            "Features"
                        </a>
                        <a href="/#how-it-works" class="text-gray-600 hover:text-gray-900 font-medium transition">
                            "How It Works"
                        </a>
                        <a href="/blog" class="text-gray-600 hover:text-gray-900 font-medium transition">
                            "Blog"
                        </a>
                        <a href="https://github.com/frontmesh/leptos-cloudflare-workers" target="_blank" rel="noopener noreferrer" class="px-6 py-2 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition">
                            "Get Started"
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
