use cursive::traits::*;
use cursive::views::{BoxView, Canvas, Dialog, IdView};
use cursive::Cursive;

use cursive::event::{Event, EventResult, Key};

use crate::model::{GameState, KeyEvent, UserEvent, ViewChars, VIEW_COLS, VIEW_ROWS};

use std::sync::mpsc::{Receiver, Sender, TryRecvError};

const GAME_CANVAS_ID: &str = "game_canvas";

type GameCanvas = Canvas<ViewChars>;

/// Entry point to the UI. This constructs a UI, then listens on the given receiver
/// for state changes (which trigger UI redraws).
pub fn run_view(state_receiver: Receiver<ViewChars>, user_input_sender: Sender<UserEvent>) {
    let mut siv = get_view(user_input_sender);
    siv.set_fps(20); // draw FPS -- unrelated to update FPS (separate thread entirely)

    while siv.is_running() {
        // Burn through all updates in the queue; only keep the last one
        // Stop burning through the queue when it's empty (no reason to draw old frames)
        let mut next_state = None;
        loop {
            match state_receiver.try_recv() {
                Ok(state) => {
                    next_state = Some(state);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => {
                    eprintln!("Disconnected from update queue");
                    siv.quit();
                    break;
                }
            }
        }

        if let Some(state) = next_state {
            eprintln!("Forcing update, hopefully");
            set_state(&mut siv, state);
            siv.step();
        }
    }
}

fn get_view(user_input_sender: Sender<UserEvent>) -> Cursive {
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(
        Dialog::around(game_canvas(GAME_CANVAS_ID, user_input_sender))
            .title("This game is totally something that exists"),
    );

    siv
}

fn set_state(s: &mut Cursive, game_state: ViewChars) {
    s.call_on_id(GAME_CANVAS_ID, |canvas: &mut GameCanvas| {
        *canvas.state_mut() = game_state;
    })
    .expect("Should have found it!")
}

fn game_canvas(id: &str, user_input_sender: Sender<UserEvent>) -> BoxView<IdView<GameCanvas>> {
    let mut canvas = Canvas::new(GameState::default().to_display_string());

    canvas.set_draw(|state, printer| {
        for row_ind in 0..state.len() {
            // coordinates are X, Y (positive directions are right and down)
            printer.print((0, row_ind), &state[row_ind].iter().collect::<String>());
        }
    });

    canvas.set_on_event(move |_state, event| match event {
        Event::Key(key) => match key {
            Key::Left => {
                user_input_sender
                    .send(UserEvent::KeyPress(KeyEvent::LEFT))
                    .expect("Receiver should be getting the key presses");
                EventResult::Consumed(None)
            }
            Key::Right => {
                user_input_sender
                    .send(UserEvent::KeyPress(KeyEvent::RIGHT))
                    .expect("Receiver should be getting the key presses");
                EventResult::Consumed(None)
            }
            Key::Up => {
                user_input_sender
                    .send(UserEvent::KeyPress(KeyEvent::UP))
                    .expect("Receiver should be getting the key presses");
                EventResult::Consumed(None)
            }
            Key::Down => {
                user_input_sender
                    .send(UserEvent::KeyPress(KeyEvent::DOWN))
                    .expect("Receiver should be getting the key presses");
                EventResult::Consumed(None)
            }
            _ => EventResult::Ignored,
        },
        _ => EventResult::Ignored,
    });

    canvas.with_id(id).fixed_size((VIEW_COLS, VIEW_ROWS))
}
