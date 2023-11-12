use leptos::*;

use crate::components::card::buttons::Buttons;
use crate::{InfoContext, SERVER};

#[component]
pub fn CardTitle(title: String, video: String, id: String) -> impl IntoView {
    let show_info = use_context::<InfoContext>().unwrap().0;
    view! {
        <div class="card-title">
            <h1>{title.clone()}</h1>
            <div class="video-wrapper"
                    on:click=move |_e| {
                    show_info.set(!show_info.get());
            }

        >
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
