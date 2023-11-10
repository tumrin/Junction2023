use leptos::*;

#[component]
pub fn CardInfo(content: String) -> impl IntoView {
    view! {
        <div class="cardinfo">
            <p>{content}</p>
        </div>
    }
}
