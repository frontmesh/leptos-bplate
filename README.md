# Leptos Boilerplate

A production-ready boilerplate for building full-stack web applications with **Leptos**, **Rust**, **Tailwind CSS**, and **Docker**. Includes a built-in markdown blog system and optimized for performance.

<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" width="200">
</picture>

## ✨ Key Features

- **🦀 Full-Stack Rust** - Write your entire application in Rust with complete type safety
- **⚡ Server-Side Rendering** - Fast initial page loads with SSR and seamless client-side hydration
- **📝 Built-in Blog System** - Markdown-based blog with YAML frontmatter, ready to customize
- **🎨 Tailwind CSS** - Pre-configured with utility-first styling
- **🐳 Production Docker** - Lightweight Docker image (~30MB) for easy deployment
- **🔒 Type Safe** - Compile-time verification across your entire application
- **📦 Reactive Components** - Automatic reactivity tracking without boilerplate
- **🚀 Zero Config Setup** - Get started in minutes with sensible defaults

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (nightly toolchain)
- [cargo-leptos](https://github.com/akesson/cargo-leptos) - `cargo install cargo-leptos --locked`
- Node.js / npm (for Tailwind CSS)

### Installation

1. Clone this repository:
```bash
git clone <your-repo-url>
cd leptos-boilerplate
```

2. Install the WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Run the development server:
```bash
cargo leptos watch
```

Your app will be available at `http://localhost:3000`

## 📁 Project Structure

```
src/
├── app.rs                 # Main app component and routing
├── components/            # Reusable components
│   ├── blog.rs           # Blog-related components
│   ├── header.rs         # Header component
│   ├── footer.rs         # Footer component
│   └── layout.rs         # Layout wrapper
├── routes/               # Page components
│   ├── home_page.rs      # Homepage
│   └── blog/             # Blog pages
├── models/               # Data models
├── server_functions/     # Server-side functions
└── styles/               # CSS with Tailwind
content/
└── blog/                 # Markdown blog posts with YAML frontmatter
```

## 📝 Writing Blog Posts

Create a new `.md` file in `content/blog/` with YAML frontmatter:

```yaml
---
slug: my-first-post
title: My First Post
description: A short description
author: Your Name
date: 2025-01-15
tags:
  - rust
  - leptos
og_title: My First Post
og_description: A short description
og_type: article
---

Your markdown content here...
```

## 🐳 Docker Deployment

Build and run with Docker:

```bash
# Build the Docker image
docker build -t leptos-app .

# Run the container
docker run -p 3000:3000 leptos-app
```

The optimized Docker image is **~30MB** and production-ready.

## 🏗️ Production Build

Create a production build:

```bash
cargo leptos build --release
```

Output files:
- Server binary: `target/server/release/leptos-boilerplate`
- Site files: `target/site/`

### Deploying to a Remote Server

1. Build for release: `cargo leptos build --release`
2. Copy `target/server/release` and `target/site` to your server
3. Set environment variables:
```bash
export LEPTOS_OUTPUT_NAME="leptos-boilerplate"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="0.0.0.0:3000"
```
4. Run the server binary

## 🛠️ Development

### Useful Commands

- `cargo leptos watch` - Start dev server with hot reload
- `cargo leptos build` - Build for development
- `cargo leptos build --release` - Build for production
- `cargo test` - Run tests

### Modifying Styles

Edit `style/tailwind.css` and `style/main.scss`. Tailwind CSS is automatically processed during development.

### Adding New Routes

Edit `src/app.rs` and add your routes:

```rust
<Route path=StaticSegment("my-page") view=MyPage/>
```

Then create the component in `src/routes/my_page.rs`.

## 📚 Learning Resources

- [Leptos Book](https://leptos.dev) - Official documentation
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn Rust
- [Tailwind CSS Docs](https://tailwindcss.com/docs) - Styling guide

## 🤝 Customization

- **Change the site name**: Update `<Meta>` tags in `src/app.rs`
- **Customize blog styling**: Edit `.prose-blog` class in `style/tailwind.css`
- **Add authentication**: Use `leptos_axum` with auth middleware
- **Database**: Add any Rust database library (SQLx, Diesel, etc.)

## 📄 License

This template is released under the Unlicense. Replace with your preferred license.

## 🎯 What's Next?

1. Customize the homepage in `src/routes/home_page.rs`
2. Add your first blog post in `content/blog/`
3. Create custom components in `src/components/`
4. Deploy to your hosting platform

Happy building! 🚀
```

Now let me update the home_page.rs:

```
use crate::components::Layout;
use leptos::prelude::*;
use leptos::{component, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Layout>
            <div class="pt-32 pb-20 px-4 sm:px-6 lg:px-8">
                <div class="max-w-7xl mx-auto">
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                        <div class="space-y-8">
                            <div class="space-y-4">
                                <div class="inline-block px-4 py-2 bg-blue-100 rounded-full">
                                    <span class="text-blue-700 text-sm font-semibold">
                                        "✨ Full-Stack Rust Web Framework"
                                    </span>
                                </div>
                                <h1 class="text-6xl font-bold text-gray-900 leading-tight">
                                    "Build Modern Web Apps with "
                                    <span class="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-cyan-600">
                                        "Leptos"
                                    </span>
                                </h1>
                            </div>
                            <p class="text-xl text-gray-600 leading-relaxed max-w-lg">
                                "A production-ready boilerplate for building full-stack web applications in Rust. "
                                "Includes server-side rendering, reactive components, Tailwind CSS, and a markdown blog system."
                            </p>
                            <div class="flex flex-wrap gap-4 pt-4">
                                <a href="/blog" class="inline-block px-8 py-4 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition shadow-lg hover:shadow-xl hover:shadow-blue-600/20">
                                    "Read Blog"
                                </a>
                                <a href="https://leptos.dev" target="_blank" rel="noopener noreferrer" class="inline-block px-8 py-4 border-2 border-gray-300 text-gray-900 font-semibold rounded-lg hover:border-gray-400 hover:bg-gray-50 transition">
                                    "View Docs"
                                </a>
                            </div>
                        </div>

                        <div class="relative hidden lg:block h-96">
                            <div class="absolute inset-0 flex items-center justify-center">
                                <div class="absolute w-32 h-32 bg-gradient-to-r from-blue-400 to-blue-600 rounded-full blur-3xl opacity-20 animate-blob"></div>
                                <div class="absolute w-32 h-32 bg-gradient-to-r from-cyan-400 to-blue-400 rounded-full blur-3xl opacity-20 animate-blob animation-delay-2000"></div>
                                <div class="absolute w-32 h-32 bg-gradient-to-r from-blue-300 to-cyan-300 rounded-full blur-3xl opacity-20 animate-blob animation-delay-4000"></div>

                                <div class="absolute inset-0 flex items-center justify-center">
                                    <div class="w-24 h-24 border-2 border-blue-200 rounded-full" style="animation: spin-slow 20s linear infinite;"></div>
                                    <div class="absolute w-16 h-16 border-2 border-cyan-200 rounded-full" style="animation: spin-reverse 15s linear infinite;"></div>
                                    <div class="absolute w-8 h-8 bg-gradient-to-r from-blue-600 to-cyan-600 rounded-full"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div id="features" class="py-24 bg-gray-50 border-t border-gray-200">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-20">
                        <h2 class="text-5xl font-bold text-gray-900 mb-6">
                            "Everything You Need"
                        </h2>
                        <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                            "A complete starter template with all the essentials for modern web development."
                        </p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-blue-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"🦀"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Full-Stack Rust"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Write both frontend and backend in Rust. Get compile-time safety and type checking across your entire application."
                            </p>
                        </div>

                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-cyan-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"⚡"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Lightning Fast"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Server-side rendering for instant page loads. Client-side hydration for seamless interactivity. Optimized for performance."
                            </p>
                        </div>

                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-blue-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"🎨"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Tailwind CSS"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Pre-configured Tailwind CSS for beautiful, responsive designs. Utility-first approach for rapid development."
                            </p>
                        </div>

                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-cyan-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"📝"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Markdown Blog"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Built-in blog system with markdown support, YAML frontmatter, and beautiful typography. Ready to publish."
                            </p>
                        </div>

                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-blue-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"🐳"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Docker Ready"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Production-ready Dockerfile with optimized ~30MB image size. Deploy anywhere with confidence."
                            </p>
                        </div>

                        <div class="bg-white p-8 rounded-2xl border border-gray-200 hover:border-cyan-300 hover:shadow-lg transition-all">
                            <div class="w-14 h-14 bg-blue-100 rounded-xl flex items-center justify-center mb-6">
                                <span class="text-2xl">"🚀"</span>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-900 mb-4">
                                "Production Ready"
                            </h3>
                            <p class="text-gray-600 leading-relaxed">
                                "Clean architecture, best practices, and sensible defaults. Start building immediately without boilerplate."
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            <div id="how-it-works" class="py-24 bg-white">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-20">
                        <h2 class="text-5xl font-bold text-gray-900 mb-6">
                            "How It Works"
                        </h2>
                        <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                            "Simple, powerful, and elegant. From development to production."
                        </p>
                    </div>

                    <div class="space-y-12">
                        <div class="flex gap-8 items-start">
                            <div class="flex-shrink-0">
                                <div class="flex items-center justify-center h-16 w-16 rounded-full bg-blue-600 text-white font-bold text-xl">
                                    "1"
                                </div>
                            </div>
                            <div class="flex-1 pt-2">
                                <h3 class="text-2xl font-bold text-gray-900 mb-3">
                                    "Write Your App in Rust"
                                </h3>
                                <p class="text-gray-600 text-lg leading-relaxed">
                                    "Use Leptos to write your frontend and backend in Rust. Enjoy the full power of "
                                    "type safety, pattern matching, and zero-cost abstractions. Components are simple, "
                                    "reactive functions that automatically track dependencies."
                                </p>
                            </div>
                        </div>

                        <div class="flex gap-8 items-start">
                            <div class="flex-shrink-0">
                                <div class="flex items-center justify-center h-16 w-16 rounded-full bg-cyan-600 text-white font-bold text-xl">
                                    "2"
                                </div>
                            </div>
                            <div class="flex-1 pt-2">
                                <h3 class="text-2xl font-bold text-gray-900 mb-3">
                                    "Compile to WebAssembly"
                                </h3>
                                <p class="text-gray-600 text-lg leading-relaxed">
                                    "Leptos compiles your Rust code to ultra-efficient WebAssembly. Server-side rendering "
                                    "generates fast initial HTML responses. Client-side code hydrates in seconds with minimal JavaScript."
                                </p>
                            </div>
                        </div>

                        <div class="flex gap-8 items-start">
                            <div class="flex-shrink-0">
                                <div class="flex items-center justify-center h-16 w-16 rounded-full bg-blue-600 text-white font-bold text-xl">
                                    "3"
                                </div>
                            </div>
                            <div class="flex-1 pt-2">
                                <h3 class="text-2xl font-bold text-gray-900 mb-3">
                                    "Deploy Anywhere"
                                </h3>
                                <p class="text-gray-600 text-lg leading-relaxed">
                                    "With Docker support and a ~30MB image size, deploy your application anywhere. "
                                    "Use traditional servers, Kubernetes, cloud platforms, or container registries."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="py-24 bg-blue-600">
                <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <h2 class="text-5xl font-bold text-white mb-6">
                        "Ready to Build?"
                    </h2>
                    <p class="text-2xl text-blue-100 mb-12 max-w-2xl mx-auto">
                        "Start building your next web application with this Leptos boilerplate today."
                    </p>
                    <a href="/blog" class="inline-block px-10 py-4 bg-white text-blue-600 font-bold text-lg rounded-xl hover:bg-gray-50 transition shadow-lg hover:shadow-xl hover:shadow-white/30">
                        "Explore Blog Posts"
                    </a>
                </div>
            </div>
        </Layout>
    }
}
