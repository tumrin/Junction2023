use leptos::*;

#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <div class="profile">
            <span>Username</span>
            <span>Likes</span>
            <span>Current task</span>
        </div>
    }
}

