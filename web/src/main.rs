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
    let show_info = create_rw_signal(false);

    provide_context(UserContext(user));

    spawn_local(async move {
        if user_id.get_untracked().is_none() {
            let res = reqwasm::http::Request::get("http://localhost:3000/api/user/create")
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
                    reqwasm::http::Request::get(&format!("http://localhost:3000/api/user/{id}"))
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

    let last_touch_x = create_rw_signal(0);
    let last_touch_y = create_rw_signal(0);

    move || {
        // is some when request completes
        if let Some(card_res) = card.get() {
            // Check if request was succesfull
            match card_res {
                Ok(card_res) => {
                    view! {
                        <div
                            class="app"
                            on:touchstart=move |e| {
                                let currenty = e.touches().get(0).unwrap().screen_y();
                                let currentx = e.touches().get(0).unwrap().screen_x();
                                last_touch_y.set(currenty);
                                last_touch_x.set(currentx);
                            }

                            on:touchmove=move |e| {
                                let currenty = e.touches().get(0).unwrap().screen_y();
                                let currentx = e.touches().get(0).unwrap().screen_x();
                                if last_touch_y.get() != 0 && currenty < last_touch_y.get() - 50 {
                                    show_info.set(true);
                                } else if last_touch_y.get() != 0 && currenty > last_touch_y.get() + 50
                                    && show_info.get()
                                {
                                    show_info.set(false)
                                }
                                if currentx < last_touch_x.get() - 50 && last_touch_x.get() != 0 {
                                    card.refetch()
                                }
                            }
                        >

                            <Card card=card_res show_info=show_info.get()/>
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
async fn fetch_card(_: ()) -> Result<Card, leptos::error::Error> {
    let res: Card = reqwasm::http::Request::get("http://localhost:3000/api/card")
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

