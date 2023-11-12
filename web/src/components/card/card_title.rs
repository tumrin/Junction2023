use leptos::*;

use crate::components::card::buttons::Buttons;
use crate::SERVER;

#[component]
pub fn CardTitle(title: String, video: String, id: String) -> impl IntoView {
    view! {
        <div class="card-title">
            <h1>{title.clone()}</h1>
            <div class="video-wrapper">
                <video
                    loop=true
                    autoplay=true
                    mute=true
                    src=format!("{SERVER}/api/video/{video}")
                ></video>
            </div>
            <Buttons card=id/>
        </div>
    }
}
