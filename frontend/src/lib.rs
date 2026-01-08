mod api;
mod components;
mod models;

use leptos::*;

use crate::components::auth::Auth;
use crate::components::dashboard::Dashboard;

#[component]
fn App() -> impl IntoView {
    let (is_authenticated, set_is_authenticated) = create_signal(api::get_token().is_some());

    view! {
        <div>
            {move || if is_authenticated.get() {
                view! { <Dashboard on_logout=move || set_is_authenticated.set(false) /> }.into_view()
            } else {
                view! { <Auth on_auth=move || set_is_authenticated.set(true) /> }.into_view()
            }}
        </div>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
