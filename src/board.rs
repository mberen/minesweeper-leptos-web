mod options_menu;
mod bomb_display;
mod reset_button;
mod timer_display;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use leptos::*;
use leptos::ev::MouseEvent;
use rand::thread_rng;
use rand::seq::SliceRandom;
use web_sys::console;
use uuid::Uuid;

use options_menu::OptionsMenu;
use bomb_display::BombDisplay;
use reset_button::ResetButton;
use timer_display::TimerDisplay;

#[component]
pub fn Board(cx: Scope) -> impl IntoView {
    let default_params = BoardParams {height: 10, width:10, mines:10};

    let board_state = BoardState::new(cx, default_params);
    provide_context(cx, board_state);
    let board_state = expect_context::<BoardState>(cx);

    let num_flags = create_rw_signal(cx, 0isize);
    provide_context(cx, num_flags);

    let style = move || {
        let width = board_state.params.get().width;
        format!("--row-length: {}", width)
    };

    view! { cx,
        <div 
            class="board" 
            style=style
            on:contextmenu=|e| e.prevent_default()
        >
            <div class="options">
                <OptionsMenu/>
            </div>
            <div class="displayPanel">
                <BombDisplay/>
                <ResetButton/>
                <TimerDisplay/>
            </div>
            <div class="cellGrid"> 
                <ul>
                    <For
                        each=board_state.cells
                        key=|cell| cell.with(|cell| (*cell).get_hash())
                        view=move |cx, cell| {
                            view! { cx,
                                <li
                                    class=cell.get().get_id()
                                    on:click=move |_| BoardState::cell_click(cx, board_state, cell)
                                    on:auxclick=move |click| BoardState::cell_aux_click(cx, cell, click)
                                >
                                    {match cell.get() {
                                        Cell::Mine{cell_status: CellStatus::Flagged, ..} => "🚩".to_string(),
                                        Cell::Empty{cell_status: CellStatus::Flagged, ..} => "🚩".to_string(),
                                        Cell::Mine{cell_status: CellStatus::Hidden, ..} => "🔳".to_string(),
                                        Cell::Empty{cell_status: CellStatus::Hidden, ..} => "🔳".to_string(),
                                        Cell::Mine{cell_status: CellStatus::Revealed, ..} => "💥".to_string(),
                                        Cell::Empty{cell_status: CellStatus::Revealed, mines: n, ..} => {
                                            let c = char::from_digit(n, 10).expect("Adjacency always 9 or less");
                                            format!("{c} ")
                                        }
                                    }}
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
        </div>
    }


}

#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    cells: RwSignal<Vec<RwSignal<Cell>>>,
    params: RwSignal<BoardParams>,
    game_status: RwSignal<GameStatus>,

}

impl BoardState {
    fn new (cx: Scope, params: BoardParams) -> Self {
        let BoardParams {height, width, mines} = params;
        let mut cells = vec!(Cell::Empty{cell_status: CellStatus::Hidden, mines: 0, id: 0, uuid: Uuid::default()}; height*width);

        cells.iter_mut()
            .take(mines)
            .for_each(|cell| *cell = Cell::Mine{cell_status: CellStatus::Hidden, id: 0,  uuid: Uuid::default()});

        cells.shuffle(&mut thread_rng());
        for (i, cell) in cells.iter_mut().enumerate() {
            match cell {
                Cell::Mine{cell_status: s, id: _, uuid: _} => *cell=Cell::Mine{cell_status: (*s).clone(), id: i, uuid: Uuid::new_v4()},
                Cell::Empty{cell_status: s, mines:m, id: _, uuid: _} => *cell=Cell::Empty{cell_status: (*s).clone(), mines: *m, id: i, uuid: Uuid::new_v4()},
            }
        }

        let cells: Vec<RwSignal<Cell>> = 
            cells
            .into_iter()
            .map(|cell| create_rw_signal(cx, cell))
            .collect();
    
        let board_state = BoardState {
            cells: create_rw_signal(cx, cells),
            params: create_rw_signal(cx, params),
            game_status: create_rw_signal(cx, GameStatus::InProgress),
        };
    
        for i in 0..board_state.cells.get_untracked().len() {
            if let Cell::Empty{uuid, ..} = board_state.cells.get_untracked()[i].get_untracked() {
                let adjacent_bombs = board_state.get_adjacent_bombs(i);
                board_state.cells.get_untracked()[i].set(Cell::Empty{cell_status: CellStatus::Hidden, mines: adjacent_bombs, id: i, uuid});
            }
        }
    
        board_state
    }

    fn reset(&self, cx: Scope, params: &BoardParams) {
        let num_flags = expect_context::<RwSignal<isize>>(cx);
        num_flags.set(0);

        let new_state = Self::new(cx, *params);
        self.cells.set(new_state.cells.get_untracked());
        self.params.set(new_state.params.get_untracked());
        self.game_status.set(new_state.game_status.get_untracked());
    }

    fn cell_click(cx: Scope, board_state: BoardState, cell: RwSignal<Cell>) {
        /*let board_state_option = use_context::<BoardState>(cx);
        if board_state_option.is_none() {
            let msg = "panic!";
            unsafe { console::log_1(&msg.into()); }
        }
        let board_state = board_state_option.unwrap(); */

        if board_state.game_status.get_untracked() == GameStatus::InProgress {
            match cell.get_untracked() {
                ref c @ Cell::Empty{cell_status: CellStatus::Hidden, mines: 0, id: self_idx, ..} => {
                    cell.set(c.new_status(CellStatus::Revealed));

                    let adjacent_list = board_state.get_adjacent(self_idx);
                    for (x, y) in adjacent_list {
                        let adj_idx = board_state.get_coord_index(&Coordinate(x, y));
                        if self_idx != adj_idx {
                            //let msg = format!("Sending click to {:?}", board_state.cells.get_untracked()[adj_idx].get());
                            //unsafe { console::log_1(&msg.into()); }
                            BoardState::cell_click(cx, board_state, board_state.cells.get_untracked()[adj_idx]);
                        }
                    } 
                },
                ref c @ Cell::Empty{cell_status: CellStatus::Hidden, ..} => cell.set(c.new_status(CellStatus::Revealed)),
                ref c @ Cell::Mine{cell_status: CellStatus::Hidden, ..}=> {
                    cell.set(c.new_status(CellStatus::Revealed));
                    board_state.game_status.set(GameStatus::Lost);
                },
                _ => (),
            }
        }

        if board_state.game_won() { board_state.game_status.set(GameStatus::Won) }
    }

    fn cell_aux_click(cx: Scope, cell: RwSignal<Cell>, click: MouseEvent) {
        let button = click.button();
        let num_flags = expect_context::<RwSignal<isize>>(cx);
        match (cell.get(), button) {
            (c @ Cell::Mine{cell_status: CellStatus::Flagged, ..}, 2) => {
                cell.set(c.new_status(CellStatus::Hidden));
                num_flags.set(num_flags.get_untracked() - 1);
            },
            (c @ Cell::Empty{cell_status: CellStatus::Flagged, ..}, 2) => {
                cell.set(c.new_status(CellStatus::Hidden));
                num_flags.set(num_flags.get_untracked() - 1);
            },
            (c @ Cell::Mine{cell_status: CellStatus::Hidden, ..}, 2) => {
                cell.set(c.new_status(CellStatus::Flagged));
                num_flags.set(num_flags.get_untracked() + 1);
            },
            (c @ Cell::Empty{cell_status: CellStatus::Hidden, ..}, 2) => {
                cell.set(c.new_status(CellStatus::Flagged));
                num_flags.set(num_flags.get_untracked() + 1);
            },
            _ => (),
        };
    }

    pub fn get_coord_index(&self, Coordinate(x, y): &Coordinate) -> usize {
        y*self.params.get_untracked().width + x % self.params.get_untracked().height
    }

    pub fn get_coord_from_index(&self, index: usize) -> Coordinate {
        let x = index % self.params.get_untracked().width;
        let y = index / self.params.get_untracked().height;

        Coordinate(x, y)
    }

    pub fn get_adjacent(&self, i: usize) -> Vec<(usize, usize)>{
        let Coordinate(x, y) = self.get_coord_from_index(i);

        let adjacent_matrix: [(isize, isize); 9] = [ (-1, -1), (0, -1), (1, -1),
                                                    (-1, 0), (0, 0), (1, 0),
                                                    (-1, 1), (0, 1), (1, 1)];
    
        adjacent_matrix
            .iter()
            .map(|(dx, dy)| (x as isize+dx, y as isize+dy))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| { (x as usize, y as usize) })
            .filter(|(x, y)| *x < self.params.get_untracked().width && *y < self.params.get_untracked().height)
            .collect()
    }    

    pub fn get_adjacent_bombs(&self, i: usize) -> u32 {
        let adjacent_list =self.get_adjacent(i);

        let mut count = 0;
        for (x, y) in adjacent_list {
            let c = Coordinate(x, y);
            if let Cell::Mine{..} = self.cells.get_untracked()[self.get_coord_index(&c)].get_untracked()  {
                count += 1;
            };
        };
        count
    }

    fn game_won (&self) -> bool {
        let cells = self.cells.get();
        for cell in cells.iter() {
            if let Cell::Empty{cell_status: CellStatus::Hidden, ..} = cell.get() { return false }
        }
        true
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BoardParams {
    pub height: usize,
    pub width: usize,
    pub mines: usize,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Cell {
    Mine {cell_status: CellStatus, id: usize, uuid: Uuid},
    Empty {cell_status: CellStatus, mines: u32, id: usize, uuid: Uuid},
}

impl Cell {
    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
    fn new_status(&self, status: CellStatus) -> Self {
        match self {
            Cell::Mine {cell_status: _, id: i, uuid} => Cell::Mine {cell_status: status, id: *i, uuid: *uuid},
            Cell::Empty {cell_status: _, mines: n, id: i, uuid} => Cell::Empty {cell_status: status, mines: *n, id: *i, uuid: *uuid},
        }
    }
    fn get_id(&self) -> usize {
        match self {
            Cell::Mine {id: i, ..} => *i,
            Cell::Empty {id: i, ..} => *i,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum CellStatus {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameStatus {
    Won,
    Lost,
    InProgress,
}

#[derive(Debug, Clone)]
pub struct Coordinate(pub usize, pub usize);