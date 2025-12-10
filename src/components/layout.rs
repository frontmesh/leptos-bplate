use leptos::prelude::*;
use leptos::IntoView;

use crate::components::{Footer, Header};

/// Layout component that wraps pages with Header and Footer
///
/// # Props
/// * `children` - The page content to render between Header and Footer
///
/// # Example
/// ```ignore
/// <Layout>
///   <YourPageComponent />
/// </Layout>
/// ```
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen bg-white">
            <Header />

            <main class="flex-1">
                {children()}
            </main>

            <Footer />
        </div>
    }
}
