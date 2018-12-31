use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use crate::model::{
    Agent, GameState, KeyEvent, MobData, UserEvent, ViewChars, GAME_COLS, GAME_ROWS, VIEW_COLS,
    VIEW_ROWS,
};
use crate::utils::numerics::Clamp;

pub fn make_state_updater(
    update_sender: Sender<ViewChars>,
    user_input_receiver: Receiver<UserEvent>,
) {
    thread::spawn(move || {
        let mut game_state = GameState::default();

        loop {
            thread::sleep(Duration::from_millis(50));

            while let Ok(user_event) = user_input_receiver.try_recv() {
                handle_user_input(&mut game_state, user_event);
            }

            update_state(&mut game_state);

            update_sender
                .send(game_state.to_display_string())
                .expect("UI should be receiving these messages");
        }
    });
}

fn handle_user_input(state: &mut GameState, user_event: UserEvent) {
    match user_event {
        UserEvent::KeyPress(key) => handle_key_press(state, key),
    }
}

fn handle_key_press(state: &mut GameState, key_press: KeyEvent) {
    match key_press {
        KeyEvent::LEFT => move_viewpoint(state, -1, 0),
        KeyEvent::RIGHT => move_viewpoint(state, 1, 0),
        KeyEvent::UP => move_viewpoint(state, 0, -1),
        KeyEvent::DOWN => move_viewpoint(state, 0, 1),
    }
}

fn update_state(state: &mut GameState) {
    let new_agents = state
        .agents
        .iter()
        .filter_map(|agent| agent.update(state))
        .collect::<Vec<Agent>>();

    state.agents = new_agents;
    // pass
}

trait Update<T> {
    fn update(&self, game_state: &GameState) -> Option<T>;
}

impl Update<Agent> for Agent {
    fn update(&self, game_state: &GameState) -> Option<Agent> {
        match self {
            Agent::Mob(mob_data) => mob_data
                .update(game_state)
                .map(|new_mob| Agent::Mob(new_mob)),
        }
    }
}

impl Update<MobData> for MobData {
    fn update(&self, _game_state: &GameState) -> Option<MobData> {
        let mut out = { *self };
        out.x = (out.x as i32 + out.x_vel).clamp(0, GAME_COLS as i32 - 1) as usize;
        out.y = (out.y as i32 + out.y_vel).clamp(0, GAME_ROWS as i32 - 1) as usize;

        if out.x >= GAME_COLS - 1 {
            out.x_vel = -out.x_vel.abs();
        } else if out.x <= 0 {
            out.x_vel = out.x_vel.abs();
        }

        if out.y >= GAME_ROWS - 1 {
            out.y_vel = -out.y_vel.abs();
        } else if out.y <= 0 {
            out.y_vel = out.y_vel.abs();
        }

        Some(out)
    }
}

pub fn move_viewpoint(game_state: &mut GameState, dx: i32, dy: i32) {
    let mut view = &mut game_state.view_port;

    let vx = view.x as i32;
    let vy = view.y as i32;

    let adj_x = (vx + dx).clamp(0, (GAME_COLS - VIEW_COLS) as i32 - 1) as usize;
    let adj_y = (vy + dy).clamp(0, (GAME_ROWS - VIEW_ROWS) as i32 - 1) as usize;

    view.x = adj_x;
    view.y = adj_y;
}
