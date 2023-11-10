use leptos::{*, leptos_dom::logging::console_log};
use reqwasm::Error;


#[derive(Clone, PartialEq, Debug)]
struct Profile{
    id: String,
    username: String,
    in_progress: String,
    liked_cards: Vec<String>

}

#[component]
pub fn Profile() -> impl IntoView {
    let show_info = create_rw_signal(false);
    let profile = create_rw_signal::<Option<Profile>>(None);
    create_local_resource(profile, fetch_profile);
    view! {
        <div class="profile">
            <span>Username</span>
            <span>Likes</span>
            <span>Current task</span>
        </div>
    }
}


async fn fetch_profile(_: Option<Profile>) -> Result<Profile, Error> {
    let res = reqwasm::http::Request::get("http://localhost:3000/api/user/654e83471029b952630a92dd")
        .send()
        .await
        .unwrap();
    
    let json = res.json::<Profile>().await.unwrap();
    console_log(&format!("{:?}", res));
 

    Ok(Profile {
        id: "".to_string(),
        username: "".to_string(),
        in_progress: "".to_string(),
        liked_cards: vec![],
    })
}




































































