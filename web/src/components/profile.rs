use std::{vec, fmt::format, time::Duration};

use leptos::{*, leptos_dom::logging::console_log};
use serde::Deserialize;

use super::card::Card;

#[derive(Clone, PartialEq, Debug, Deserialize)]
struct Profile{
    id: String,
    username: String,
    inProgress: String,
    likedCards: Vec<String>

}


#[component]
pub fn Profile() -> impl IntoView {
    let user_id = create_rw_signal("654e83471029b952630a92dd");
    let profile = create_local_resource(move|| user_id.get(), fetch_profile);
    let likedCards = create_rw_signal::<Vec<String>>(vec![]);
    let showLikes = create_rw_signal(false);

    move||{
        if let Some(profile) = profile.get()  {
            match profile {
                Ok(profile) => {
                    likedCards.set(profile.likedCards.clone());
                    view! {
                        <div class="profile">
                            <span>{profile.username}</span>
                            <span on:click=move |_| {
                                showLikes.set(!showLikes.get())
                            }>likes: {profile.likedCards.len()}</span>
                            <ul class="likedCards">
                                <AnimatedShow
                                    when=showLikes
                                    hide_delay=Duration::from_millis(500)
                                    show_class="slide"
                                    hide_class="hide"
                                >
                                    <For
                                        each=move || likedCards.get()
                                        key=|n| n.to_owned()
                                        let:liked
                                    >

                                        <li>{liked}</li>
                                    </For>
                                </AnimatedShow>
                            </ul>
                            <span>on progress: {profile.inProgress}</span>
                        </div>
                    }
                }
                Err(e) => {
                    view! { <div>{"Error loading profile"}</div> }},
                
            }
        }else {
            view! { <div class="profile"></div> }
        }
    }
}


async fn fetch_profile(user_id: &str) -> Result<Profile, error::Error> {
    let res: Profile =
    reqwasm::http::Request::get(&format!("http://127.0.0.1:3000/api/user/{user_id}"))
    .send()
    .await?
    .json()
    .await?;


    Ok(res)
}

async fn fetch_card(card_id: &str) -> Result<Card, error::Error> {
    let res: Card = reqwasm::http::Request::get(&format!("http://127.0.0.1:3000/api/card/{card_id}"))
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}















































































































































































