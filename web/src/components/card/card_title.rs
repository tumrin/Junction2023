use leptos::*;

use crate::components::card::buttons::Buttons;

#[component]
pub fn CardTitle(title: String, video: String) -> impl IntoView {
    view! {
        <div class="card-title">
            <h1>{title}</h1>
            <h2>{video}</h2>
            <Buttons/>
        </div>
    }
}
