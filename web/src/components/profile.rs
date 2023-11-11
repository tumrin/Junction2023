use std::{vec, fmt::format, time::Duration};

use leptos::{*, leptos_dom::logging::console_log};
use serde::Deserialize;

use crate::UserContext;

use super::card::Card;

#[component]
pub fn Profile() -> impl IntoView {
    let user = use_context::<UserContext>().unwrap().0;

   // let user_id = create_rw_signal("654e83471029b952630a92dd");
    //let profile = create_local_resource(move|| user_id.get(), fetch_profile);
    let likedCards = create_rw_signal::<Vec<String>>(vec![]);
    let showLikes = create_rw_signal(false);
    //let a = fetch_cards(likedCards.get().clone());*/
    
    move||{
        if let Some(user) = user.get()  {
            let user1 = user.clone();
            console_log(&format!("{:?}", user1));
            // create rw signal likecards
            likedCards.set(user1.likedCards.unwrap_or(vec![]));
      
                    view! {
                        <div class="profile">
                            <span>{user1.username}</span>
                            <span on:click=move |_| {}>likes: {likedCards.get().len()}</span>
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
                            <span>on progress: {fetch_card(user1.inProgress)}</span>
                        </div>
                    }
                
     
        }else {
            view! {
                <div class="profile">
                    <span>loading...</span>
                </div>
            }
        }
    }
}




async fn fetch_cards(card_ids: Vec<String>) -> Result<Vec<Card>, error::Error> {

    let mut cards: Vec<Card> = vec![];
    for card_id in card_ids {
        let res: Card = reqwasm::http::Request::get(&format!("http://127.0.0.1:3000/api/card/{card_id}", card_id=card_id))
        .send()
        .await?
        .json()
        .await?;
        cards.push(res);
    }

    Ok(cards)
}



/*async fn fetch_card(card_id: String) -> Result<Card, error::Error> {

    let res: Card = reqwasm::http::Request::get(&format!("http://127.0.0.1:3000/api/card/{card_id}", card_id=card_id))
    .send()
    .await?
    .json()
    .await?;

    Ok(res)
}*/

fn fetch_card(card_id: String) {
    spawn_local(async move {
        reqwasm::http::Request::get(&format!("http://localhost:3000/api/card/{card_id}",))
            .send()
            .await
            .unwrap();
    })
}





































































































































































































































































