extern crate cursive;

mod controller;
mod model;
mod utils;
mod view;

use self::controller::make_state_updater;
use self::view::run_view;

use std::sync::mpsc::channel;

fn main() {
    let (state_tx, state_rx) = channel();
    let (user_input_tx, user_input_rx) = channel();

    make_state_updater(state_tx, user_input_rx);

    run_view(state_rx, user_input_tx);

    eprintln!("All done");
}
