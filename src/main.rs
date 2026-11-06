mod blackhole;
mod nbody;
mod relativity;
mod simulation;
mod view;
mod view_3d;
mod view_blackhole;
mod view_relativistic;

use macroquad::prelude::*;
use nbody::{PARTICLE_COUNTS, ParticleCloud, particle_number_report, timestep_convergence_report};
use relativity::RelativisticCollapse;
use simulation::{Simulation, radius_convergence_report};
use view_blackhole::BlackHoleView;

// Model time units (t / t0) per update; one real running second maps to one model unit.
const FIXED_DT: f64 = 0.002;

enum Model {
    Spherical(Simulation),
    ThreeDimensional(ParticleCloud),
    Relativistic(RelativisticCollapse),
    BlackHole,
}

impl Model {
    fn is_paused(&self) -> bool {
        match self {
            Self::Spherical(simulation) => simulation.paused,
            Self::ThreeDimensional(cloud) => cloud.paused,
            Self::Relativistic(collapse) => collapse.paused,
            Self::BlackHole => true,
        }
    }

    fn is_stopped(&self) -> bool {
        match self {
            Self::Spherical(simulation) => simulation.status.is_some(),
            Self::ThreeDimensional(cloud) => cloud.status.is_some(),
            Self::Relativistic(collapse) => collapse.status.is_some(),
            Self::BlackHole => false,
        }
    }

    fn toggle_pause(&mut self) {
        match self {
            Self::Spherical(simulation) => simulation.paused = !simulation.paused,
            Self::ThreeDimensional(cloud) => cloud.paused = !cloud.paused,
            Self::Relativistic(collapse) => collapse.paused = !collapse.paused,
            Self::BlackHole => {}
        }
    }

    fn reset(&mut self) {
        *self = match self {
            Self::Spherical(_) => Self::Spherical(Simulation::default()),
            Self::ThreeDimensional(cloud) => Self::ThreeDimensional(cloud.fresh_initial_state()),
            Self::Relativistic(_) => Self::Relativistic(RelativisticCollapse::default()),
            Self::BlackHole => Self::BlackHole,
        };
    }

    fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        match self {
            Self::Spherical(simulation) => simulation.step(dt),
            Self::ThreeDimensional(cloud) => cloud.step(dt),
            Self::Relativistic(collapse) => collapse.step(dt),
            Self::BlackHole => Ok(()),
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
    let mut model = Model::Relativistic(RelativisticCollapse::default());
    let spherical_convergence = radius_convergence_report();
    let particle_convergence = timestep_convergence_report()
        .expect("the built-in 3D timestep benchmark uses valid parameters");
    let particle_number_comparison =
        particle_number_report().expect("the built-in finite-particle study uses valid parameters");
    let mut black_hole_view: Option<BlackHoleView> = None;
    let mut accumulator: f64 = 0.0;
    loop {
        let mut steps_this_frame: u32 = 0;
        if is_key_pressed(KeyCode::Key3) {
            model = Model::ThreeDimensional(ParticleCloud::default());
            accumulator = 0.0;
        }
        if is_key_pressed(KeyCode::Key4) {
            model = Model::Relativistic(RelativisticCollapse::default());
            accumulator = 0.0;
        }
        if is_key_pressed(KeyCode::Key5) {
            model = Model::BlackHole;
            if black_hole_view.is_none() {
                black_hole_view = Some(BlackHoleView::default());
            }
            accumulator = 0.0;
        }
        if is_key_pressed(KeyCode::Key2) {
            model = Model::Spherical(Simulation::default());
            accumulator = 0.0;
        }
        if is_key_pressed(KeyCode::A) {
            let new_shape = match &model {
                Model::ThreeDimensional(cloud) => Some(!cloud.is_oblate()),
                Model::Spherical(_) => None,
                Model::Relativistic(_) => None,
                Model::BlackHole => None,
            };
            if let Some(oblate) = new_shape {
                if let Model::ThreeDimensional(cloud) = &model {
                    model = Model::ThreeDimensional(cloud.with_oblate_initial_shape(oblate));
                }
                accumulator = 0.0;
            }
        }
        if is_key_pressed(KeyCode::V) {
            let new_rotation = match &model {
                Model::ThreeDimensional(cloud) => Some(!cloud.is_rotating()),
                Model::Spherical(_) => None,
                Model::Relativistic(_) => None,
                Model::BlackHole => None,
            };
            if let Some(rotating) = new_rotation {
                if let Model::ThreeDimensional(cloud) = &model {
                    model = Model::ThreeDimensional(cloud.with_rotation_enabled(rotating));
                }
                accumulator = 0.0;
            }
        }
        if is_key_pressed(KeyCode::N) {
            let next_count = match &model {
                Model::ThreeDimensional(cloud) => {
                    let current = cloud.particle_count();
                    let index = PARTICLE_COUNTS
                        .iter()
                        .position(|&count| count == current)
                        .unwrap_or(0);
                    Some(PARTICLE_COUNTS[(index + 1) % PARTICLE_COUNTS.len()])
                }
                Model::Spherical(_) => None,
                Model::Relativistic(_) => None,
                Model::BlackHole => None,
            };
            if let Some(count) = next_count {
                if let Model::ThreeDimensional(cloud) = &model {
                    model = Model::ThreeDimensional(
                        cloud
                            .with_particle_count(count)
                            .expect("the selectable particle counts are valid"),
                    );
                }
                accumulator = 0.0;
            }
        }
        if matches!(&model, Model::BlackHole) {
            if (is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A))
                && let Some(view) = &mut black_hole_view
            {
                view.adjust_inclination(-5.0);
            }
            if (is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D))
                && let Some(view) = &mut black_hole_view
            {
                view.adjust_inclination(5.0);
            }
        }
        if is_key_pressed(KeyCode::Space) && !model.is_stopped() {
            model.toggle_pause();
        }
        if is_key_pressed(KeyCode::R) {
            model.reset();
            if matches!(&model, Model::BlackHole)
                && let Some(view) = &mut black_hole_view
            {
                view.reset();
            }
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
            Model::Spherical(simulation) => {
                view::draw(simulation, steps_this_frame, &spherical_convergence)
            }
            Model::ThreeDimensional(cloud) => view_3d::draw(
                cloud,
                steps_this_frame,
                &particle_convergence,
                &particle_number_comparison,
            ),
            Model::Relativistic(collapse) => view_relativistic::draw(collapse, steps_this_frame),
            Model::BlackHole => {
                if let Some(view) = &mut black_hole_view {
                    view.draw();
                }
            }
        }
        next_frame().await;
    }
}
