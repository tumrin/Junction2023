use std::{vec, sync::{RwLock, Arc}};

use leptos::{*};

use crate::UserContext;

use super::card::Card;

#[component]
pub fn Profile() -> impl IntoView {
    let user = use_context::<UserContext>().unwrap().0;

    let likedCards = create_rw_signal::<Option<Arc<RwLock<Vec<Card>>>>>(None);
    let inProgress = create_rw_signal::<Option<Card>>(None);    
    
    move||{
        if let Some(user) = user.get()  {

            let user1 = user.clone();
            let likedCardsIds = user1.likedCards.unwrap_or(vec![]);
            if inProgress.get().is_none() {
                spawn_local(async move {
                    let res = reqwasm::http::Request::get(&format!("http://localhost:3000/api/card/{}",user1.inProgress))
                    .send()
                    .await
                    .unwrap()
                    .json::<Card>()
                    .await
                    .unwrap();
                    
                    inProgress.set(Some(res));
                });
            }
            if likedCards.get().is_none() {
                let likedCardsClone = likedCards.clone();
                spawn_local(async move {
                    let mut likedCards: Vec<Card> = vec![];
                    for card_id in likedCardsIds {
                        let res = reqwasm::http::Request::get(&format!("http://localhost:3000/api/card/{}", card_id))
                            .send()
                            .await
                            .unwrap()
                            .json::<Card>()
                            .await
                            .unwrap();
                        likedCards.push(res);
                    }
                    let liked_cards_arc = Arc::new(RwLock::new(likedCards));
                    likedCardsClone.set(Some(liked_cards_arc));
                });
            }
            let likedCards_data = likedCards.get();

            let likedCards = if let Some(liked_cards_arc) = likedCards_data {
                if let Ok(liked_cards) = liked_cards_arc.read() {
                    liked_cards.clone()
                } else {
           
                    vec![]
                }
            } else {
           
                vec![]
            };                    
            
            view! {
                <div class="profile">
                    <span>{user1.username}</span>
                    <span>Liked cards: {likedCards.len()}</span>

                    <ul class="likedCards">
                        <For each=move || likedCards.to_owned() key=|n| n.clone() let:liked>
                            <li>{liked.title}</li>
                        </For>

                    </ul>

                    <span>
                        In progress:
                        {inProgress
                            .get()
                            .unwrap_or(Card {
                                id: "".to_string(),
                                vidLink: "".to_string(),
                                reference: vec![],
                                title: "".to_string(),
                                content: "".to_string(),
                            })
                            .title}

                    </span>
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




























