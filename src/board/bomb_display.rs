use leptos::*;
use super::BoardState;

#[component]
pub fn BombDisplay(cx: Scope) -> impl IntoView {
    let num_flags = expect_context::<RwSignal<isize>>(cx);
    let num_bombs = move || {
        expect_context::<BoardState>(cx).params.get_untracked().mines as isize
    };
    
    view! { cx,
        <div class="bomb display">
            {move || num_bombs() - num_flags.get()}
        </div>
    }
}