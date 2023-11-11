use leptos::{leptos_dom::logging::console_log, *};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::components::{card::Card};

mod components;
mod pages;

#[derive(Debug, Deserialize, Clone)]
struct User {
    id: String,
    username: String,
    inProgress: String,
    likedCards: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct UserIdResponse {
    id: String,
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
    let (user, set_user) = create_signal(None);
    provide_context(UserContext(user));

    spawn_local(async move {
        if user_id.get_untracked().is_none() {
            let res = reqwasm::http::Request::get("http://127.0.0.1:3000/api/user/create")
                .send()
                .await
                .unwrap()
                .json::<UserIdResponse>()
                .await
                .unwrap()
                .id;
            set_user_id(Some(res));
        }
    });

    create_effect(move |_| {
        if let Some(id) = user_id.get() {
            spawn_local(async move {
                let res =
                    reqwasm::http::Request::get(&format!("http://127.0.0.1:3000/api/user/{id}"))
                        .send()
                        .await
                        .unwrap()
                        .json::<User>()
                        .await
                        .map_err(|e| console_log(&format!("{e:?}")))
                        .unwrap();
                set_user(Some(res));
            })
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
async fn generate_user(_: ()) -> Result<User, leptos::error::Error> {
    let res: User = reqwasm::http::Request::get("http://127.0.0.1:3000/api/user/create")
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(res)
}

async fn fetch_user(id: String) -> Result<User, leptos::error::Error> {
    console_log("fetch");

    let res: User = reqwasm::http::Request::get(&format!("http://127.0.0.1:3000/api/user/{id}"))
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(res)
}
async fn fetch_card(_: ()) -> Result<Card, leptos::error::Error> {
    console_log("fetch2");
    let res: Card = reqwasm::http::Request::get("http://127.0.0.1:3000/api/card")
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

