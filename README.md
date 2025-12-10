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
