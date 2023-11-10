use leptos::*;

use crate::components::{card::Card, top_bar::TopBar};

mod components;
mod pages;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <TopBar/>
        <Card/>
    }
}

