use std::rc::Rc;

use leptos::{component, create_rw_signal, spawn_local, use_context, IntoView};
use leptos::{view, SignalGet};
use leptos_icons::BiIcon::BiLikeSolid;
use leptos_icons::BsIcon::BsRocketTakeoffFill;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};

use crate::{User, UserContext};

#[derive(Serialize, Deserialize, Debug)]
struct UserInfoPutRequest {
    inProgress: String,
    likedCards: Option<Vec<String>>,
}

impl From<UserInfoPutRequest> for leptos::wasm_bindgen::JsValue {
    fn from(val: UserInfoPutRequest) -> Self {
        serde_wasm_bindgen::to_value(&val).unwrap()
    }
}

#[component]
pub fn Buttons(card: String) -> impl IntoView {
    let user = use_context::<UserContext>().unwrap().0;
    let card = create_rw_signal(card);

    let user = Rc::new(user);

    move || {
        if let Some(user) = user.get() {
            let user1 = user.clone();
            view! {
                <div class="buttons">
                    <div class="button" on:click=move |e| { like_request(&user, card.get()) }>

                        <Icon icon=Icon::from(BiLikeSolid) width="2em" height="2em"/>
                    </div>
                    <div class="button" on:click=move |e| { start_request(&user1, card.get()) }>

                        <Icon icon=Icon::from(BsRocketTakeoffFill) width="2em" height="2em"/>
                    </div>
                </div>
            }
        } else {
            view! { <div></div> }
        }
    }
}

fn start_request(user: &User, card: String) {
    let id = user.id.to_string();
    let user = user.clone();
    spawn_local(async move {
        let body = UserInfoPutRequest {
            inProgress: card,
            likedCards: Some(user.likedCards.unwrap_or(vec![])),
        };
        reqwasm::http::Request::put(&format!("http://localhost:3000/api/user/{id}",))
            .header("Content-Type", "application/json")
            .body(serde_json::to_value(body).unwrap().to_string())
            .send()
            .await
            .unwrap();
    })
}
fn like_request(user: &User, card: String) {
    let id = user.id.to_string();
    let user = user.clone();
    spawn_local(async move {
        let mut likedCards = user.likedCards.unwrap_or(vec![]);
        likedCards.push(card);
        let body = UserInfoPutRequest {
            inProgress: user.inProgress,
            likedCards: Some(likedCards),
        };
        reqwasm::http::Request::put(&format!("http://localhost:3000/api/user/{id}",))
            .header("Content-Type", "application/json")
            .body(serde_json::to_value(body).unwrap().to_string())
            .send()
            .await
            .unwrap();
    })
}
