mod buttons;
mod card_info;
mod card_title;

use crate::components::card::card_info::CardInfo;
use crate::components::card::card_title::CardTitle;
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use serde::Deserialize;
use std::time::Duration;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Card {
    id: String,
    vidLink: String,
    reference: Vec<String>,
    title: String,
    content: String,
}

#[component]
pub fn Card(card: Card, show_info: bool) -> impl IntoView {
    view! {
        <div class="card">
            <CardTitle title=card.title.clone() video=card.vidLink.clone() id=card.id/>
            <AnimatedShow
                when=show_info
                hide_delay=Duration::from_millis(500)
                show_class="slide"
                hide_class="hide"
            >
                <CardInfo content=card.content.clone()/>
            </AnimatedShow>
        </div>
    }
}

