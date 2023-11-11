use crate::wasm_bindgen::JsValue;
use leptos::leptos_dom::logging::console_log;
use leptos::{component, spawn_local, use_context, IntoView};
use leptos::{view, SignalGet};
use leptos_icons::BiIcon::BiLikeSolid;
use leptos_icons::BsIcon::BsRocketTakeoffFill;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};

use crate::{User, UserContext};

#[derive(Serialize, Deserialize)]
struct UserInfoPutRequest {
    inProgress: String,
    LikedCards: Vec<String>,
}

impl Into<leptos::wasm_bindgen::JsValue> for UserInfoPutRequest {
    fn into(self) -> leptos::wasm_bindgen::JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

#[component]
pub fn Buttons(card: String) -> impl IntoView {
    let user = use_context::<UserContext>().unwrap().0;
    move || {
        if let Some(user) = user.get() {
            view! {
                <div class="buttons">
                    <button class="button" on:click=move |e| {}>

                        <Icon icon=Icon::from(BiLikeSolid) width="2em" height="2em"/>
                    </button>
                    <div class="button">
                        <Icon icon=Icon::from(BsRocketTakeoffFill) width="2em" height="2em"/>
                    </div>
                </div>
            }
        } else {
            view! { <div></div> }
        }
    }
}

fn request(user: User, card: String) {
    spawn_local(async move {
        reqwasm::http::Request::put(&format!("http://localhost:3000/api/user/{}", user.id))
            .body(UserInfoPutRequest {
                inProgress: card,
                LikedCards: user.likedCards,
            })
            .send()
            .await;
    })
}

