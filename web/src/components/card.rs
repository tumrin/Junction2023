mod card_info;
mod card_title;

use leptos::*;

use crate::components::card::card_info::CardInfo;

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="card">
            <CardInfo/>
        </div>
    }
}

