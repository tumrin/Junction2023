use leptos::*;

use crate::components::{card::Card, profile::Profile};

mod components;
mod pages;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let card = create_local_resource(move || (), fetch_card);
    move || {
        // is some when request completes
        if let Some(card_res) = card.get() {
            // Check if request was succesfull
            match card_res {
                Ok(card_res) => {
                    view! {
                        <div class="app">
                            <Profile/>
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



