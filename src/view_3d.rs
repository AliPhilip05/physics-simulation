use macroquad::prelude::*;

use crate::nbody::{PARTICLE_COUNT, PLUMMER_SOFTENING, ParticleCloud};

pub fn draw(cloud: &ParticleCloud, steps_this_frame: u32) {
    clear_background(Color::from_rgba(8, 12, 22, 255));
    set_camera(&Camera3D {
        position: vec3(2.8, 2.2, 3.2),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    });

    draw_grid(12, 0.25, DARKGRAY, Color::from_rgba(53, 64, 82, 255));
    draw_line_3d(vec3(-1.15, 0.0, 0.0), vec3(1.15, 0.0, 0.0), RED);
    draw_line_3d(vec3(0.0, -1.15, 0.0), vec3(0.0, 1.15, 0.0), GREEN);
    draw_line_3d(vec3(0.0, 0.0, -1.15), vec3(0.0, 0.0, 1.15), BLUE);

    for particle in cloud.particles() {
        let position = particle.position();
        let color = Color::new(
            (0.82 + 0.16 * position[2]) as f32,
            (0.38 + 0.18 * (position[1] + 1.0)) as f32,
            (0.20 + 0.12 * (position[0] + 1.0)) as f32,
            1.0,
        );
        draw_sphere(
            vec3(position[0] as f32, position[1] as f32, position[2] as f32),
            0.025,
            None,
            color,
        );
    }

    set_default_camera();
    let status = match cloud.status {
        None if cloud.paused => "PAUSED".to_owned(),
        None => "RUNNING".to_owned(),
        Some(reason) => format!("STOPPED: {reason}"),
    };
    draw_rectangle(14.0, 12.0, 475.0, 194.0, Color::from_rgba(8, 12, 22, 220));
    draw_text(
        "3D NEWTONIAN COLLAPSE | N-BODY MODEL",
        26.0,
        40.0,
        23.0,
        WHITE,
    );
    draw_text(&status, 26.0, 68.0, 19.0, GRAY);
    draw_text(
        format!("PARTICLES: {PARTICLE_COUNT} | TOTAL MASS: 1"),
        26.0,
        98.0,
        18.0,
        GOLD,
    );
    draw_text(
        format!("TIME t / t0: {:.3}", cloud.elapsed),
        26.0,
        124.0,
        18.0,
        BLUE,
    );
    draw_text(
        format!("RMS RADIUS / R0: {:.4}", cloud.rms_radius()),
        26.0,
        150.0,
        18.0,
        GOLD,
    );
    draw_text(
        format!(
            "ABSOLUTE TOTAL-ENERGY ERROR: {:.3e}",
            cloud.absolute_energy_error()
        ),
        26.0,
        176.0,
        17.0,
        GOLD,
    );
    draw_text(
        format!(
            "PLUMMER SOFTENING: {:.3} R0 | UPDATES THIS FRAME: {steps_this_frame}",
            PLUMMER_SOFTENING
        ),
        26.0,
        202.0,
        15.0,
        GRAY,
    );
    draw_text(
        "3: 3D particles | 2: spherical-shell model | Space: pause | R: reset | Esc: close",
        20.0,
        screen_height() - 20.0,
        17.0,
        WHITE,
    );
}
