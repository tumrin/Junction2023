use leptos::*;

use crate::components::{card::Card, profile::Profile};

mod components;
mod pages;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Profile/>
        <Card/>
    }
}

