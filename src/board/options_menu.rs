use leptos::*;
use super::BoardState;
use super::BoardParams;

#[component]
pub fn OptionsMenu(cx: Scope) -> impl IntoView {
    let board_state = expect_context::<BoardState>(cx);

    let (option_clicked, option_clicked_set) = create_signal(cx, false);
    let (height, set_height) = create_signal(cx, "10".to_string());
    let (width, set_width) = create_signal(cx, "10".to_string());
    let (mines, set_mines) = create_signal(cx, "10".to_string());

    let max_bombs = move || {
        let height: usize = height().parse().unwrap();
        let width: usize = width().parse().unwrap();
        height * width - 1
    };

    let handle_option_click =move |_| {
        option_clicked_set(true);
    };

    let handle_cancel = move |_| {
        option_clicked_set(false);
    };

    let handle_new_game = move |_| {
        let height: usize = height.get().parse().unwrap();
        let width: usize = width.get().parse().unwrap();
        let mines: usize = mines.get().parse().unwrap();

        board_state.reset(cx, &BoardParams{height, width, mines});
        option_clicked_set(false);
    };

    view! { cx,
        {move || if ! option_clicked() {
            view! { cx,
                <ul>
                    <li on:click=handle_option_click>"Options"</li>
                </ul>
            }.into_any()
        }
        else {
            view! { cx,
                <div class="game_options">
                    <div>
                        <label for="height">"Height"</label>
                        <input 
                            type="number" 
                            min="1" 
                            id="height" 
                            prop:value={height}
                            on:input=move |ev| set_height(event_target_value(&ev))
                        />
                    </div>
                    <div>
                        <label for="width">"Width"</label>
                        <input 
                            type="number" 
                            min="1" 
                            id="width" 
                            prop:value={width}
                            on:input=move |ev| set_width(event_target_value(&ev))
                        />
                    </div>
                    <div>
                        <label for="mines">"Mines"</label>
                        <input 
                            type="number" 
                            min="1" 
                            max=max_bombs
                            id="mines" 
                            prop:value={mines}
                            on:input=move |ev| set_mines(event_target_value(&ev))
                        />
                    </div>
                    <button on:click=handle_new_game.clone()>"New Game"</button>
                    <button on:click=handle_cancel.clone()>"Cancel"</button>
                </div>
            }.into_any()
        }
    }}
}