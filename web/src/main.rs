use leptos::{leptos_dom::logging::console_log, *};
use leptos_use::storage::use_local_storage;
use serde::Deserialize;

use crate::components::card::Card;

mod components;
mod pages;

#[derive(Debug, Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    inProgress: String,
    likedCards: Vec<String>,
}
#[derive(Copy, Clone)]
struct UserContext(ReadSignal<Option<User>>);

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let card = create_local_resource(move || (), fetch_card);
    let (user_id, set_user_id, _) = use_local_storage("user", None);
    let user_creation_request = create_rw_signal(None);
    let user_request = create_rw_signal(None);
    let (user, set_user) = create_signal(None);
    provide_context(UserContext(user));

    create_effect(move |_| {
        if user_id.get().is_none() {
            user_creation_request.set(Some(create_local_resource(move || (), generate_user)));
        } else {
            user_request.set(Some(create_local_resource(
                move || user_id.get().unwrap(),
                fetch_user,
            )));
        }
    });

    create_effect(move |_| {
        if let Some(user_request) = user_creation_request.get() {
            if let Some(Ok(user_id)) = user_request.get() {
                set_user_id(Some(user_id))
            }
        }
        if let Some(user_request) = user_request.get() {
            if let Some(Ok(user)) = user_request.get() {
                set_user(Some(user))
            }
        }
    });

    let mut last_touch = 0;

    move || {
        // is some when request completes
        if let Some(card_res) = card.get() {
            // Check if request was succesfull
            match card_res {
                Ok(card_res) => {
                    view! {
                        <div
                            class="app"
                            on:touchmove=move |e| {
                                let current = e.touches().get(0).unwrap().screen_x();
                                if current < last_touch {
                                    card.refetch()
                                } else {
                                    last_touch = current
                                }
                            }
                        >

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
async fn generate_user(_: ()) -> Result<String, leptos::error::Error> {
    let res: String = reqwasm::http::Request::get("http://127.0.0.1:3000/api/user/create")
        .send()
        .await?
        .json::<User>()
        .await?
        .id;
    Ok(res)
}

async fn fetch_user(id: String) -> Result<User, leptos::error::Error> {
    let res: User = reqwasm::http::Request::get(&format!("http://127.0.0.1:3000/api/user/{id}"))
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(res)
}
async fn fetch_card(_: ()) -> Result<Card, leptos::error::Error> {
    let res: Card = reqwasm::http::Request::get("http://127.0.0.1:3000/api/card")
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}
