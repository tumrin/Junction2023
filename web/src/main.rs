use leptos::*;

use crate::components::{card::Card, profile::Profile, top_bar::TopBar};

mod components;
mod pages;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <TopBar/>
        <Profile/>
        <Card/>
    }
}

