use leptos::prelude::*;
use leptos::IntoView;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-900 text-gray-300 py-16 border-t border-gray-800">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-12 mb-12">
                    <div>
                        <div class="text-xl font-bold text-white mb-4">
                            "LeptosCloudflareWorkers"
                        </div>
                        <p class="text-gray-400">
                            "Full-stack web development in Rust."
                        </p>
                    </div>
                    <div>
                        <h4 class="text-white font-semibold mb-4">
                            "Product"
                        </h4>
                        <ul class="space-y-2">
                            <li>
                                <a href="#features" class="text-gray-400 hover:text-white transition">
                                    "Features"
                                </a>
                            </li>
                            <li>
                                <a href="#how-it-works" class="text-gray-400 hover:text-white transition">
                                    "How It Works"
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-white font-semibold mb-4">
                            "Resources"
                        </h4>
                        <ul class="space-y-2">
                            <li>
                                <a href="https://book.leptos.dev/" target="_blank" rel="noopener noreferrer" class="text-gray-400 hover:text-white transition">
                                    "Leptos Docs"
                                </a>
                            </li>
                            <li>
                                <a href="https://developers.cloudflare.com/workers/" target="_blank" rel="noopener noreferrer" class="text-gray-400 hover:text-white transition">
                                    "Workers Docs"
                                </a>
                            </li>
                            <li>
                                <a href="https://github.com/frontmesh/leptos-tailwind-docker" target="_blank" rel="noopener noreferrer" class="text-gray-400 hover:text-white transition">
                                    "GitHub"
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="text-white font-semibold mb-4">
                            "Company"
                        </h4>
                        <ul class="space-y-2">
                            <li>
                                <a href="#" class="text-gray-400 hover:text-white transition">
                                    "About"
                                </a>
                            </li>
                            <li>
                                <a href="/blog" class="text-gray-400 hover:text-white transition">
                                    "Blog"
                                </a>
                            </li>
                            <li>
                                <a href="#" class="text-gray-400 hover:text-white transition">
                                    "Contact"
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="border-t border-gray-800 pt-8">
                    <div class="flex flex-col md:flex-row justify-between items-center">
                        <p class="text-gray-400 text-sm">
                            "© 2025 Leptos. All rights reserved."
                        </p>
                        <div class="flex space-x-6 mt-4 md:mt-0">
                            <a href="#" class="text-gray-400 hover:text-white transition text-sm">
                                "Privacy"
                            </a>
                            <a href="#" class="text-gray-400 hover:text-white transition text-sm">
                                "Terms"
                            </a>
                            <a href="#" class="text-gray-400 hover:text-white transition text-sm">
                                "Status"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    }
}
