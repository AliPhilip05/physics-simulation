use macroquad::prelude::*;

use crate::nbody::{
    PARTICLE_STUDY_DT, PARTICLE_STUDY_TIME, PLUMMER_SOFTENING, ParticleCloud, ParticleCountResult,
    TimestepResult, VALIDATION_TIME,
};

pub fn draw(
    cloud: &ParticleCloud,
    steps_this_frame: u32,
    convergence: &[TimestepResult; 3],
    particle_number_comparison: &[ParticleCountResult; 4],
) {
    clear_background(Color::from_rgba(8, 12, 22, 255));
    let camera_distance = (cloud.outer_radius() * 3.5).clamp(0.75, 8.0) as f32;
    let particle_radius = (cloud.outer_radius() * 0.025).clamp(0.003, 0.025) as f32;
    set_camera(&Camera3D {
        position: vec3(
            0.60 * camera_distance,
            0.47 * camera_distance,
            0.68 * camera_distance,
        ),
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
            (0.82 + 0.16 * position[2]).clamp(0.0, 1.0) as f32,
            (0.38 + 0.18 * (position[1] + 1.0)).clamp(0.0, 1.0) as f32,
            (0.20 + 0.12 * (position[0] + 1.0)).clamp(0.0, 1.0) as f32,
            1.0,
        );
        draw_sphere(
            vec3(position[0] as f32, position[1] as f32, position[2] as f32),
            particle_radius,
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
    draw_rectangle(14.0, 12.0, 530.0, 370.0, Color::from_rgba(8, 12, 22, 220));
    draw_text(
        "3D NEWTONIAN COLLAPSE | N-BODY MODEL",
        26.0,
        40.0,
        23.0,
        WHITE,
    );
    draw_text(&status, 26.0, 68.0, 19.0, GRAY);
    draw_text(
        format!("PARTICLES: {} | TOTAL MASS: 1", cloud.particle_count()),
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
        format!(
            "CENTER-OF-MASS OFFSET: {:.3e}",
            cloud.center_of_mass_offset()
        ),
        26.0,
        228.0,
        16.0,
        SKYBLUE,
    );
    draw_text(
        format!(
            "TOTAL MOMENTUM MAGNITUDE: {:.3e}",
            cloud.total_momentum_magnitude()
        ),
        26.0,
        252.0,
        16.0,
        SKYBLUE,
    );
    draw_text(
        "Collisionless particles | no pressure | Newtonian gravity",
        26.0,
        278.0,
        14.0,
        GRAY,
    );
    let extents = cloud.principal_rms_extents();
    draw_text(
        format!(
            "START: {} | SOLID-BODY ROTATION: {}",
            if cloud.is_oblate() {
                "OBLATE"
            } else {
                "SPHERICAL"
            },
            if cloud.is_rotating() { "ON" } else { "OFF" }
        ),
        26.0,
        306.0,
        14.0,
        SKYBLUE,
    );
    draw_text(
        format!(
            "PRINCIPAL RMS EXTENTS: {:.3} {:.3} {:.3}",
            extents[0], extents[1], extents[2]
        ),
        26.0,
        328.0,
        14.0,
        SKYBLUE,
    );
    draw_text(
        format!(
            "MINOR / MAJOR: {:.3} | ANGULAR MOMENTUM: {:.3e}",
            cloud.minor_to_major_axis_ratio(),
            cloud.total_angular_momentum_magnitude()
        ),
        26.0,
        350.0,
        14.0,
        SKYBLUE,
    );

    draw_rectangle(580.0, 12.0, 405.0, 135.0, Color::from_rgba(8, 12, 22, 220));
    draw_text(
        format!("COMMON-TIME STUDY AT t/t0 = {VALIDATION_TIME:.2}"),
        594.0,
        40.0,
        16.0,
        WHITE,
    );
    draw_text(
        "dt       RMS-radius error      |energy error|",
        594.0,
        64.0,
        14.0,
        GRAY,
    );
    for (index, result) in convergence.iter().enumerate() {
        draw_text(
            format!(
                "{:.4}      {:.3e}          {:.3e}",
                result.dt, result.rms_radius_error, result.absolute_energy_error
            ),
            594.0,
            88.0 + index as f32 * 19.0,
            14.0,
            SKYBLUE,
        );
    }
    draw_history_panel(cloud, 580.0, 162.0, 405.0, 220.0);
    draw_particle_number_panel(particle_number_comparison, 580.0, 394.0, 405.0, 220.0);
    draw_text(
        "3: particles | 2: shells | 4: collapse | 5: black hole | A: shape | V: spin | N: count | Space: pause | R: reset",
        20.0,
        screen_height() - 20.0,
        17.0,
        WHITE,
    );
}

fn draw_history_panel(cloud: &ParticleCloud, x: f32, y: f32, width: f32, height: f32) {
    draw_rectangle(x, y, width, height, Color::from_rgba(8, 12, 22, 220));
    draw_text("RECENT COLLAPSE HISTORY", x + 14.0, y + 22.0, 15.0, WHITE);
    draw_text("RMS radius / initial", x + 14.0, y + 42.0, 12.0, GOLD);
    draw_text("minor / major", x + 170.0, y + 42.0, 12.0, SKYBLUE);

    let history = cloud.diagnostic_history();
    if history.len() < 2 {
        return;
    }
    let left = x + 28.0;
    let right = x + width - 14.0;
    let top = y + 56.0;
    let bottom = y + height - 28.0;
    draw_line(left, top, left, bottom, 1.0, GRAY);
    draw_line(left, bottom, right, bottom, 1.0, GRAY);
    let first_time = history.front().unwrap().time;
    let last_time = history.back().unwrap().time;
    let time_span = (last_time - first_time).max(f64::EPSILON);
    let initial_radius = history.front().unwrap().rms_radius.max(f64::EPSILON);
    let point = |sample: &crate::nbody::DiagnosticSample, shape: bool| {
        let fraction = ((sample.time - first_time) / time_span) as f32;
        let value = if shape {
            sample.minor_to_major_ratio
        } else {
            (sample.rms_radius / initial_radius).min(1.05)
        } as f32;
        let maximum = if shape { 1.0 } else { 1.05 };
        (
            left + fraction * (right - left),
            bottom - value / maximum * (bottom - top),
        )
    };
    for (shape, color) in [(false, GOLD), (true, SKYBLUE)] {
        for pair in history.as_slices().0.windows(2) {
            let (x1, y1) = point(&pair[0], shape);
            let (x2, y2) = point(&pair[1], shape);
            draw_line(x1, y1, x2, y2, 2.0, color);
        }
        let wrapped = history.as_slices().1;
        for pair in wrapped.windows(2) {
            let (x1, y1) = point(&pair[0], shape);
            let (x2, y2) = point(&pair[1], shape);
            draw_line(x1, y1, x2, y2, 2.0, color);
        }
        if !history.as_slices().0.is_empty() && !history.as_slices().1.is_empty() {
            let (x1, y1) = point(history.as_slices().0.last().unwrap(), shape);
            let (x2, y2) = point(history.as_slices().1.first().unwrap(), shape);
            draw_line(x1, y1, x2, y2, 2.0, color);
        }
    }
    draw_text(
        format!("t/t0 {:.2} to {:.2}", first_time, last_time),
        left,
        y + height - 8.0,
        11.0,
        GRAY,
    );
}

fn draw_particle_number_panel(
    comparison: &[ParticleCountResult; 4],
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    draw_rectangle(x, y, width, height, Color::from_rgba(8, 12, 22, 220));
    draw_text(
        format!("FINITE-N STUDY | t/t0={PARTICLE_STUDY_TIME:.2} | dt={PARTICLE_STUDY_DT:.3}"),
        x + 14.0,
        y + 24.0,
        14.0,
        WHITE,
    );
    draw_text(
        "N       |delta RMS radius|      |delta shape ratio|",
        x + 14.0,
        y + 48.0,
        12.0,
        GRAY,
    );
    for (index, result) in comparison.iter().enumerate() {
        draw_text(
            format!(
                "{:<4}    {:.3e}              {:.3e}",
                result.count, result.rms_radius_difference, result.shape_ratio_difference
            ),
            x + 14.0,
            y + 75.0 + index as f32 * 23.0,
            13.0,
            SKYBLUE,
        );
    }
    draw_text(
        "Differences use the largest sample as reference; not a continuum solution.",
        x + 14.0,
        y + height - 14.0,
        10.0,
        GRAY,
    );
}
