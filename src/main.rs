use leptos::*;

mod board;

use board::Board;

fn main() {
    mount_to_body(|cx| view! { cx, <Board/> })
}