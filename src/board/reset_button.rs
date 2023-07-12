use leptos::*;

use super::GameStatus;
use super::BoardState;


#[component]
pub fn ResetButton(cx: Scope) -> impl IntoView {
    let board_state = expect_context::<BoardState>(cx);

    view! { cx, 
        <div class="resetButton"> 
            <button on:click=move |_| handle_click(cx)>
            {move || match board_state.game_status.get() {
                GameStatus::Won => "😎",
                GameStatus::Lost => "☹️",
                GameStatus::InProgress => "🔄"
            }}
            </button>
        </div>
    }
}

fn handle_click(cx:Scope) {
    let board_state = expect_context::<BoardState>(cx);    
    let params = board_state.params.get_untracked();
    board_state.reset(cx, &params);
}