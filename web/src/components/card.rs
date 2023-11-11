mod buttons;
mod card_info;
mod card_title;

use crate::components::card::card_info::CardInfo;
use crate::components::card::card_title::CardTitle;
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
pub fn Card() -> impl IntoView {
    let show_info = create_rw_signal(false);
    let card = create_local_resource(move || (), fetch_card);

    move || {
        // is some when request completes
        if let Some(card) = card.get() {
            // Check if request was succesfull
            match card {
                Ok(card) => {
                    view! {
                        <div class="card" on:click=move |_e| show_info.set(!show_info.get())>
                            <CardTitle title=card.title.clone()/>
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
                Err(_e) => view! { <div>{"Error loading card"}</div> },
            }
        } else {
            view! { <div class="card"></div> }
        }
    }
}

async fn fetch_card(_: ()) -> Result<Card, error::Error> {
    let res: Card = reqwasm::http::Request::get("http://127.0.0.1:3000/api/card")
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}



