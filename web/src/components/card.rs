mod card_info;
mod card_title;

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
    let (show_info, set_show_info) = create_signal(false);
    let (card, set_card) = create_signal::<Option<Card>>(None);
    let cards = create_local_resource(card, fetch_card);
    view! {
        <div class="card" on:click=move |_e| set_show_info(true)>
            <CardTitle/>
            <Show when=move || show_info.get()>
                <CardInfo/>
            </Show>
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


