mod card_info;
mod card_title;

use std::time::Duration;

use crate::components::card::card_info::CardInfo;
use crate::components::card::card_title::CardTitle;
use leptos::*;
use reqwasm::Error;

#[derive(Clone, PartialEq)]
struct Card {
    title: String,
}

#[component]
pub fn Card() -> impl IntoView {
    let show_info = create_rw_signal(false);
    let (card, set_card) = create_signal::<Option<Card>>(None);
    let cards = create_local_resource(card, fetch_card);
    view! {
        <div class="card" on:click=move |_e| show_info.set(!show_info.get())>
            <CardTitle/>
        <AnimatedShow  when=show_info hide_delay=Duration::from_millis(500) show_class="slide" hide_class="hide">
            <CardInfo/>
        </AnimatedShow>
        </div>
    }
}

async fn fetch_card(card: Option<Card>) -> Result<Card, Error> {
    reqwasm::http::Request::get("http://localhost:9000")
        .send()
        .await;
    Ok(Card {
        title: "".to_string(),
    })
}
