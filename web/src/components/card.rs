mod buttons;
mod card_info;
mod card_title;

use crate::components::card::card_info::CardInfo;
use crate::components::card::card_title::CardTitle;
use crate::components::profile::Profile;
use crate::InfoContext;
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use serde::Deserialize;
use std::time::Duration;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Debug)]
pub struct Card {
    pub id: String,
    pub vidLink: String,
    pub reference: Vec<String>,
    pub title: String,
    pub content: String,
}

#[component]
pub fn Card(card: Card, show_profile: bool) -> impl IntoView {
    let show_info = use_context::<InfoContext>().unwrap().0;
    view! {
        <div class="card">
            <AnimatedShow
                when=show_profile
                hide_delay=Duration::from_millis(500)
                show_class="slide"
                hide_class="hide"
            >
                <Profile/>
            </AnimatedShow>

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

