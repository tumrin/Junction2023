use leptos::*;
use leptos_icons::BiIcon::BiLikeSolid;
use leptos_icons::BsIcon::BsRocketTakeoffFill;
use leptos_icons::Icon;

#[component]
pub fn Buttons() -> impl IntoView {
    view! {
        <div class="buttons">
        <div class="button">
            <Icon icon=Icon::from(BiLikeSolid) width="2em" height="2em"/>
        </div>
        <div class="button">
            <Icon icon=Icon::from(BsRocketTakeoffFill) width="2em" height="2em"/>
        </div>
        </div>
    }
}
