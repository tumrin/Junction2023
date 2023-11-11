use leptos::*;

use crate::components::card::buttons::Buttons;

#[component]
pub fn CardTitle(title: String, video: String, id: String) -> impl IntoView {
    view! {
        <div class="card-title">
            <h1>{title.clone()}</h1>
            <div class="video-wrapper">
                <video
                    loop=true
                    autoplay=true
                    src=format!("http://127.0.0.1:3000/api/video/{video}")
                ></video>
            </div>
            <Buttons card=id/>
        </div>
    }
}
