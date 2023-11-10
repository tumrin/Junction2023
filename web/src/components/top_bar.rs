use leptos::*;
use leptos_icons::{FiIcon::FiMenu, Icon};

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <div class="topbar">
            <Icon icon=Icon::from(FiMenu) width="2rem" height="2rem"/>
        </div>
    }
}

