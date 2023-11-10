use leptos::*;

use crate::components::card::buttons::Buttons;

#[component]
pub fn CardTitle(title: String) -> impl IntoView {
    view! {
        <div class="card-title">
            <h1>{title}</h1>
            <Buttons/>
        </div>
    }
}
