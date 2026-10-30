mod nbody;
mod simulation;
mod view;
mod view_3d;

use macroquad::prelude::*;
use nbody::ParticleCloud;
use simulation::{Simulation, radius_convergence_report};

// Model time units (t / t0) per update; one real running second maps to one model unit.
const FIXED_DT: f64 = 0.002;

enum Model {
    Spherical(Simulation),
    ThreeDimensional(ParticleCloud),
}

impl Model {
    fn is_paused(&self) -> bool {
        match self {
            Self::Spherical(simulation) => simulation.paused,
            Self::ThreeDimensional(cloud) => cloud.paused,
        }
    }

    fn is_stopped(&self) -> bool {
        match self {
            Self::Spherical(simulation) => simulation.status.is_some(),
            Self::ThreeDimensional(cloud) => cloud.status.is_some(),
        }
    }

    fn toggle_pause(&mut self) {
        match self {
            Self::Spherical(simulation) => simulation.paused = !simulation.paused,
            Self::ThreeDimensional(cloud) => cloud.paused = !cloud.paused,
        }
    }

    fn reset(&mut self) {
        *self = match self {
            Self::Spherical(_) => Self::Spherical(Simulation::default()),
            Self::ThreeDimensional(_) => Self::ThreeDimensional(ParticleCloud::default()),
        };
    }

    fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        match self {
            Self::Spherical(simulation) => simulation.step(dt),
            Self::ThreeDimensional(cloud) => cloud.step(dt),
        }
    }
}

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
    let mut model = Model::ThreeDimensional(ParticleCloud::default());
    let convergence = radius_convergence_report();
    let mut accumulator: f64 = 0.0;
    loop {
        let mut steps_this_frame: u32 = 0;
        if is_key_pressed(KeyCode::Key3) {
            model = Model::ThreeDimensional(ParticleCloud::default());
            accumulator = 0.0;
        }
        if is_key_pressed(KeyCode::Key2) {
            model = Model::Spherical(Simulation::default());
            accumulator = 0.0;
        }
        if is_key_pressed(KeyCode::Space) && !model.is_stopped() {
            model.toggle_pause();
        }
        if is_key_pressed(KeyCode::R) {
            model.reset();
            accumulator = 0.0;
        }
        if !model.is_paused() {
            accumulator += f64::from(get_frame_time());
            while accumulator >= FIXED_DT {
                match model.step(FIXED_DT) {
                    Ok(()) => {
                        steps_this_frame += 1;
                        accumulator -= FIXED_DT;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        match &model {
            Model::Spherical(simulation) => view::draw(simulation, steps_this_frame, &convergence),
            Model::ThreeDimensional(cloud) => view_3d::draw(cloud, steps_this_frame),
        }
        next_frame().await;
    }
}
