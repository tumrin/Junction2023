use leptos::*;
use leptos_icons::BiIcon::BiLikeSolid;
use leptos_icons::BsIcon::BsRocketTakeoffFill;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};

use crate::SERVER;
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

enum ButtonAction {
    LikeButton,
    StartButton,
}

#[component]
pub fn Buttons(card: String) -> impl IntoView {
    let user = use_context::<UserContext>().unwrap().0;
    let card = create_rw_signal(card);

    move || {
        if let Some(user) = user.get() {
            let user1 = user.clone();
            view! {
                <div class="buttons">
                    <div class="button" on:click=move |_e| { button_action(&user, card.get(), ButtonAction::LikeButton) }>

                        <Icon icon=Icon::from(BiLikeSolid) width="2em" height="2em"/>
                    </div>
                    <div class="button" on:click=move |_e| { button_action(&user1, card.get(), ButtonAction::StartButton) }>

                        <Icon icon=Icon::from(BsRocketTakeoffFill) width="2em" height="2em"/>
                    </div>
                </div>
            }
        } else {
            view! { <div></div> }
        }
    }
}

fn button_action(user: &User, card: String, button_action: ButtonAction) {
    let id = user.id.to_string();
    let user = user.clone();
    spawn_local(async move {
        let body = match button_action {
            ButtonAction::LikeButton => UserInfoPutRequest {
                inProgress: card,
                likedCards: Some(user.likedCards.unwrap_or_default()),
            },
            ButtonAction::StartButton => {
                let mut liked_cards = user.likedCards.unwrap_or_default();
                liked_cards.push(card);
                UserInfoPutRequest {
                    inProgress: user.inProgress,
                    likedCards: Some(liked_cards),
                }
            }
        };
        reqwasm::http::Request::put(&format!("{SERVER}/api/user/{id}"))
            .header("Content-Type", "application/json")
            .body(serde_json::to_value(body).unwrap().to_string())
            .send()
            .await
            .unwrap();
    });
}
