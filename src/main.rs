mod simulation;
mod view;

use macroquad::prelude::*;
use simulation::Simulation;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics Simulation | Stellar Collapse".to_owned(),
        window_width: 1000,
        window_height: 700,
        window_resizable: true,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut simulation = Simulation::default();
       loop {
        if is_key_pressed(KeyCode::Space) {
            simulation.paused = !simulation.paused;
        }
        if is_key_pressed(KeyCode::R) {
            simulation = Simulation::default();
        }
        if !simulation.paused {
            let clock = f64::from(get_frame_time());
            simulation.elapsed += clock;
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Your first exercise adds input and state updates here.
        view::draw(&simulation);
        next_frame().await;
    }
}
