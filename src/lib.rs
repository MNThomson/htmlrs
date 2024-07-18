pub use leptos::{component, view, IntoView};

#[macro_export]
macro_rules! html {
    ($($view:tt)*) => {{
        leptos::ssr::render_to_string(|| view! { $($view)* }).to_string()
    }};
}
