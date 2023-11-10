use leptos::*;

#[component]
pub fn CardTitle(title: String) -> impl IntoView {
    view! {
        <div class="card-title">
            <h1>{title}</h1>
            <p>asdf</p>
        </div>
    }
}
