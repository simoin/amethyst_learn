use crate::state::Dot::{Block, BlockOnGoal, Goal, Man, ManOnGoal, Space};
use amethyst::{
    input::{get_key, is_close_requested, is_key_down, ElementState, VirtualKeyCode},
    prelude::*,
};
use log::info;
use std::string::ToString;

type Pos = (usize, usize);

pub struct MyState {
    map: Vec<Vec<Dot>>,
    size: (usize, usize),
    p: Pos,
    rest_dst: i32,
}

impl Default for MyState {
    fn default() -> Self {
        MyState {
            map: vec![
                vec![Space, Goal, Goal, Space, Man, Space],
                vec![Space, Block, Block, Space, Space, Space],
                vec![Space, Space, Space, Space, Space, Space],
            ],
            size: (3, 6),
            p: (0, 4),
            rest_dst: 2,
        }
    }
}

#[derive(Display, Debug, PartialEq, Eq)]
enum Dot {
    // #[strum(serialize = "#")]
    // Wall,
    #[strum(serialize = " ")]
    Space,
    #[strum(serialize = "o")]
    Block,
    #[strum(serialize = "o")]
    BlockOnGoal,
    #[strum(serialize = ".")]
    Goal,
    #[strum(serialize = "p")]
    ManOnGoal,
    #[strum(serialize = "p")]
    Man,
}

impl SimpleState for MyState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        create_console_ui(&self);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if let Some((key, elem_state)) = get_key(&event) {
                if let ElementState::Released = elem_state {
                    info!("handling key event: {:?}", event);
                    let res = handle_move(self, key);
                    create_console_ui(&self);
                    return res;
                }
            }
        }

        Trans::None
    }
}

fn create_console_ui(state: &MyState) {
    println!("########");
    for line in &state.map {
        print!("#");
        for d in line {
            print!("{}", d.to_string());
        }
        println!("#");
    }
    println!("########");
}

fn handle_move(state: &mut MyState, key: VirtualKeyCode) -> SimpleTrans {
    let (dx, dy) = match key {
        VirtualKeyCode::A => (0, -1),
        VirtualKeyCode::D => (0, 1),
        VirtualKeyCode::W => (-1, 0),
        VirtualKeyCode::S => (1, 0),
        _ => (0, 0),
    };

    let (x, y) = state.p;
    let tx = state.p.0 as i32 + dx;
    let ty = state.p.1 as i32 + dy;

    if tx < 0 || ty < 0 || tx >= state.size.0 as i32 || ty >= state.size.1 as i32 {
        return Trans::None;
    }
    let (tx_p, ty_p) = (tx as usize, ty as usize);

    if state.map[tx_p][ty_p] == Space || state.map[tx_p][ty_p] == Goal {
        state.map[tx_p][ty_p] = if state.map[tx_p][ty_p] == Goal {
            ManOnGoal
        } else {
            Man
        };
        state.p = (tx_p, ty_p);
        state.map[x][y] = if state.map[x][y] == Goal { Goal } else { Space };
    } else {
        let (tx2, ty2) = (tx + dx, ty + dy);
        if tx2 < 0 || ty2 < 0 || tx2 as usize >= state.size.0 || ty2 as usize >= state.size.1 {
            return Trans::None;
        }
        let (tx2_p, ty2_p) = (tx2 as usize, ty2 as usize);

        if state.map[tx2_p][ty2_p] == Space || state.map[tx2_p][ty2_p] == Goal {
            state.p = (tx_p, ty_p);
            state.map[tx2_p][ty2_p] = if state.map[tx2_p][ty2_p] == Goal {
                state.rest_dst -= 1;
                BlockOnGoal
            } else {
                Block
            };
            state.map[tx_p][ty_p] = if state.map[tx_p][ty_p] == BlockOnGoal {
                state.rest_dst += 1;
                ManOnGoal
            } else {
                Man
            };
            state.map[x][y] = if state.map[x][y] == ManOnGoal {
                Goal
            } else {
                Space
            };
        }
    }
    if state.rest_dst == 0 {
        return Trans::Quit;
    }

    Trans::None
}
