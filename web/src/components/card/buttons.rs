use leptos::*;
use leptos_icons::BiIcon::BiLikeSolid;
use leptos_icons::BsIcon::BsRocketTakeoffFill;
use leptos_icons::Icon;

#[component]
pub fn Buttons() -> impl IntoView {
    view! {
        <div class="buttons">
            <Icon icon=Icon::from(BiLikeSolid) width="2em" height="2em"/>
            <Icon icon=Icon::from(BsRocketTakeoffFill) width="2em" height="2em"/>
        </div>
    }
}
