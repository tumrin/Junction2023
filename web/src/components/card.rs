mod buttons;
mod card_info;
mod card_title;

use crate::components::card::card_info::CardInfo;
use crate::components::card::card_title::CardTitle;
use crate::InfoContext;
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
pub fn Card(card: Card) -> impl IntoView {
    let show_info = use_context::<InfoContext>().unwrap().0;
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
