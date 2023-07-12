use gloo_timers::future::TimeoutFuture;
use leptos::*;

use super::BoardState;
use super::GameStatus;

#[component]
pub fn TimerDisplay(cx: Scope) -> impl IntoView {
    let (time, set_time) = create_signal(cx, 0);
    let board_state = expect_context::<BoardState>(cx);

    spawn_local(async move {
        loop {
            TimeoutFuture::new(1000).await;
            if let GameStatus::InProgress= board_state.game_status.get_untracked() {
                set_time.update(|time| *time += 1);
            }
        }
    });

    create_effect(cx, move |_| {
        board_state.cells.track();
        set_time(0);
    });

    view! { cx,
        <div class="timer"> 
            {time}
        </div>
    }
}