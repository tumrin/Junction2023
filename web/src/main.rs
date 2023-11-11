use leptos::{leptos_dom::logging::console_log, *};
use leptos_use::storage::use_local_storage;
use serde::Deserialize;

use crate::components::card::Card;

mod components;
mod pages;

#[derive(Deserialize)]
struct User {
    id: String,
}

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let card = create_local_resource(move || (), fetch_card);
    let (user_id, set_user_id, _) = use_local_storage("user", None);
    let user_request = create_rw_signal(None);

    create_effect(move |_| {
        if user_id.get().is_none() {
            user_request.set(Some(create_local_resource(move || (), generate_user)));
        }
    });

    create_effect(move |_| {
        if let Some(user_request) = user_request.get() {
            if let Some(Ok(user_id)) = user_request.get() {
                set_user_id(Some(user_id))
            }
        }
    });

    move || {
        // is some when request completes
        if let Some(card_res) = card.get() {
            // Check if request was succesfull
            match card_res {
                Ok(card_res) => {
                    view! {
                        <div class="app">
                            <Card card=card_res/>
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
async fn generate_user(_: ()) -> Result<String, error::Error> {
    let res: String = reqwasm::http::Request::get("http://127.0.0.1:3000/api/user/create")
        .send()
        .await?
        .json::<User>()
        .await?
        .id;
    Ok(res)
}

async fn fetch_card(_: ()) -> Result<Card, error::Error> {
    let res: Card = reqwasm::http::Request::get("http://127.0.0.1:3000/api/card")
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}
