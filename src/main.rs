mod simulation;
mod view;

use macroquad::prelude::*;
use simulation::Simulation;

// Model time units (t / t0) per update; one real running second maps to one model unit.
const FIXED_DT: f64 = 0.01;

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
    let mut accumulator: f64 = 0.0;
    loop {
        let mut steps_this_frame: u32 = 0;
        if is_key_pressed(KeyCode::Space) && simulation.status.is_none() {
            simulation.paused = !simulation.paused;
        }
        if is_key_pressed(KeyCode::R) {
            simulation = Simulation::default();
            accumulator = 0.0;
        }
        if !simulation.paused {
            accumulator += f64::from(get_frame_time());
            while accumulator >= FIXED_DT {
                match simulation.step(FIXED_DT) {
                    Ok(()) => {
                        steps_this_frame += 1;
                        accumulator -= FIXED_DT;
                    }
                    Err(_reason) => {
                        simulation.paused = true;
                        break;
                    }
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        view::draw(&simulation, steps_this_frame);
        next_frame().await;
    }
}
