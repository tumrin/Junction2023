use leptos::*;
use leptos_use::storage::use_local_storage;
use web_sys::js_sys::Math::random;

use crate::components::card::Card;

mod components;
mod pages;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let card = create_local_resource(move || (), fetch_card);
    let (user_id, set_user_id, _) = use_local_storage("user", None);
    if user_id.get_untracked().is_none() {
        set_user_id(Some((random() * 1000.0).round()));
    }

    move || {
        // is some when request completes
        if let Some(card_res) = card.get() {
            // Check if request was succesfull
            match card_res {
                Ok(card_res) => {
                    view! {
                        <div class="app">
                            <button on:click=move |_e| card.refetch()>previous</button>
                            <Card card=card_res/>
                            <button on:click=move |_e| card.refetch()>next</button>
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
