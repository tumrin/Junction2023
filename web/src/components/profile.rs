use leptos::{*, leptos_dom::logging::console_log};
use serde::Deserialize;

#[derive(Clone, PartialEq, Debug, Deserialize)]
struct Profile{
    id: String,
    username: String,
    in_progress: String,
    liked_cards: Vec<String>

}

#[component]
pub fn Profile() -> impl IntoView {
    let user_id = create_rw_signal("654e83471029b952630a92dd");
    let profile = create_local_resource(move|| user_id.get(), fetch_profile);

    console_log(&format!("profile: {:?}", profile));

    move||{
        if let Some(profile) = profile.get()  {
            match profile {
                Ok(profile) => {
                    view! {
                        <div class="profile">
                            <span>{profile.username}</span>
                            <span>likes: {profile.liked_cards.len()}</span>
                            <span>{profile.in_progress}</span>
                        </div>
                    }
                }
                Err(_e) => view! { <div>{"Error loading profile"}</div> },
                
            }
        }else {
            view! { <div class="profile"></div> }
        }
    }
}


async fn fetch_profile(user_id: &str) -> Result<Profile, error::Error> {
    console_log("fetching profile");
    let res: Profile =
    reqwasm::http::Request::get(&format!("http://127.0.0.1:3000/api/user/{user_id}"))
    .send()
    .await?
    .json()
    .await?;
    console_log(&format!("profile: {:?}", res));


    Ok(res)
}

















































































































