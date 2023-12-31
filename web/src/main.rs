use leptos::{leptos_dom::logging::console_log, *};
use leptos_icons::Icon;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::components::card::Card;
use leptos_icons::BiIcon::BiLeftArrowAltRegular;
use leptos_icons::BiIcon::BiRightArrowAltRegular;

mod components;

pub const SERVER: &str = "https://painless.final-assignment.zip";
//pub const SERVER: &str = "http://localhost:3000";

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
#[derive(Copy, Clone)]
struct InfoContext(RwSignal<bool>);
#[derive(Copy, Clone)]
struct ToastContext(RwSignal<(String, i32)>);

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
    provide_context(InfoContext(show_info));

    spawn_local(async move {
        if user_id.get_untracked().is_none() {
            let res = reqwasm::http::Request::get(&format!("{SERVER}/api/user/create"))
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
                let res = reqwasm::http::Request::get(&format!("{SERVER}/api/user/{id}"))
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
                                if last_touch_y.get() != 0 && currenty < last_touch_y.get() - 50
                                    && !show_info.get()
                                {
                                    show_info.set(true);
                                } else if last_touch_y.get() != 0
                                    && currenty > last_touch_y.get() + 50 && show_info.get()
                                {
                                    show_info.set(false)
                                }
                                if currentx < last_touch_x.get() - 50 && last_touch_x.get() != 0 {
                                    card.refetch()
                                }
                            }
                        >

                            <div class="previous" on:click=move |_e| card.refetch()>
                                <Icon
                                    icon=Icon::from(BiLeftArrowAltRegular)
                                    width="4em"
                                    height="4em"
                                />
                            </div>
                            <Card card=card_res/>
                            <div class="next" on:click=move |_e| card.refetch()>
                                <Icon
                                    icon=Icon::from(BiRightArrowAltRegular)
                                    width="4em"
                                    height="4em"
                                />
                            </div>
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
    let res: Card = reqwasm::http::Request::get(&format!("{SERVER}/api/card"))
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}
