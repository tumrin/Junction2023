use leptos::*;

use crate::components::card::Card;

mod components;
mod pages;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {<div class="app"> <Card/> </div>}
}
