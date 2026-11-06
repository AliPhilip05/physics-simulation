use macroquad::prelude::*;

use crate::relativity::{
    FREE_FALL_TIME, INITIAL_COMPACTNESS, MIN_DISPLAY_SCALE, RelativisticCollapse,
};

pub fn draw(collapse: &RelativisticCollapse, steps_this_frame: u32) {
    clear_background(Color::from_rgba(4, 7, 18, 255));
    draw_3d_collapse_scene(collapse);
    let height = screen_height();

    let status = match collapse.status {
        Some(reason) => reason,
        None if collapse.horizon_crossed() => "SURFACE CROSSED Rs | BLACK HOLE FORMED",
        None if collapse.paused => "PAUSED",
        None => "COLLAPSING",
    };
    draw_rectangle(16.0, 14.0, 530.0, 308.0, Color::from_rgba(7, 12, 25, 226));
    draw_text("OPPENHEIMER–SNYDER DUST COLLAPSE", 29.0, 42.0, 22.0, WHITE);
    draw_text(status, 29.0, 72.0, 17.0, SKYBLUE);
    draw_text(
        format!("SURFACE R / R0: {:.4}", collapse.surface_radius_ratio()),
        29.0,
        108.0,
        18.0,
        GOLD,
    );
    draw_text(
        format!(
            "SCHWARZSCHILD Rs / R0: {:.3}",
            collapse.schwarzschild_radius_ratio()
        ),
        29.0,
        133.0,
        18.0,
        SKYBLUE,
    );
    draw_text(
        format!(
            "SURFACE COMPACTNESS 2GM/(Rc²): {:.3}",
            collapse.compactness()
        ),
        29.0,
        158.0,
        17.0,
        GOLD,
    );
    draw_text(
        format!("COMOVING PROPER TIME t/t0: {:.3}", collapse.elapsed),
        29.0,
        183.0,
        17.0,
        WHITE,
    );
    draw_text(
        format!(
            "SURFACE HORIZON-CROSSING TIME: {:.3} t0",
            RelativisticCollapse::horizon_crossing_time()
        ),
        29.0,
        208.0,
        15.0,
        WHITE,
    );
    draw_text(
        format!("ACCEPTED UPDATES THIS FRAME: {steps_this_frame}"),
        29.0,
        233.0,
        15.0,
        GRAY,
    );
    draw_text(
        "Uniform pressureless dust | analytic GR | no light ray tracing",
        29.0,
        268.0,
        14.0,
        GRAY,
    );
    draw_text(
        format!("Display stops at R/R0 = {MIN_DISPLAY_SCALE:.2}, before singularity"),
        29.0,
        291.0,
        14.0,
        GRAY,
    );

    draw_surface_history(collapse);
    draw_text(
        "4: collapse replay | 5: black-hole ray trace | 2: shells | 3: particles | Space: pause | R: reset | Esc: close",
        22.0,
        height - 20.0,
        15.0,
        WHITE,
    );
}

fn draw_3d_collapse_scene(collapse: &RelativisticCollapse) {
    let surface_radius = (1.35 * collapse.surface_radius_ratio()) as f32;
    let horizon_radius = (1.35 * INITIAL_COMPACTNESS) as f32;
    set_camera(&Camera3D {
        position: vec3(3.7, 2.5, 4.4),
        target: vec3(0.55, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    });

    draw_starfield_3d();
    draw_grid(
        18,
        0.5,
        Color::from_rgba(21, 32, 51, 110),
        Color::from_rgba(31, 43, 64, 150),
    );
    if collapse.horizon_crossed() {
        draw_sphere(
            vec3(0.0, 0.0, 0.0),
            horizon_radius,
            None,
            Color::from_rgba(0, 0, 5, 255),
        );
        draw_wire_sphere(
            vec3(0.0, 0.0, 0.0),
            horizon_radius * 1.025,
            16,
            12,
            Color::from_rgba(84, 191, 255, 235),
        );
    } else {
        let temperature = (1.0 - collapse.surface_radius_ratio() as f32).clamp(0.0, 1.0);
        for layer in (1..=5).rev() {
            let glow_radius = surface_radius * (1.0 + layer as f32 * 0.035);
            draw_sphere(
                vec3(0.0, 0.0, 0.0),
                glow_radius,
                None,
                Color::new(1.0, 0.2 + temperature * 0.24, 0.02, 0.025),
            );
        }
        let color = Color::new(
            1.0,
            0.58 - temperature * 0.22,
            0.13 - temperature * 0.08,
            1.0,
        );
        draw_sphere(vec3(0.0, 0.0, 0.0), surface_radius, None, color);
        draw_wire_sphere(
            vec3(0.0, 0.0, 0.0),
            surface_radius * 1.003,
            18,
            14,
            Color::from_rgba(255, 220, 147, 105),
        );
        draw_wire_sphere(
            vec3(0.0, 0.0, 0.0),
            horizon_radius,
            14,
            10,
            Color::from_rgba(111, 205, 255, 210),
        );
    }
    set_default_camera();
}

fn draw_wire_sphere(
    center: Vec3,
    radius: f32,
    longitude_count: usize,
    latitude_count: usize,
    color: Color,
) {
    let segments = 48;
    for latitude_index in 1..latitude_count {
        let latitude = -std::f32::consts::FRAC_PI_2
            + std::f32::consts::PI * latitude_index as f32 / latitude_count as f32;
        for segment in 0..segments {
            let phi0 = std::f32::consts::TAU * segment as f32 / segments as f32;
            let phi1 = std::f32::consts::TAU * (segment + 1) as f32 / segments as f32;
            draw_line_3d(
                sphere_point(center, radius, latitude, phi0),
                sphere_point(center, radius, latitude, phi1),
                color,
            );
        }
    }
    for longitude in 0..longitude_count {
        let phi = std::f32::consts::TAU * longitude as f32 / longitude_count as f32;
        for segment in 0..segments {
            let lat0 = -std::f32::consts::FRAC_PI_2
                + std::f32::consts::PI * segment as f32 / segments as f32;
            let lat1 = -std::f32::consts::FRAC_PI_2
                + std::f32::consts::PI * (segment + 1) as f32 / segments as f32;
            draw_line_3d(
                sphere_point(center, radius, lat0, phi),
                sphere_point(center, radius, lat1, phi),
                color,
            );
        }
    }
}

fn sphere_point(center: Vec3, radius: f32, latitude: f32, longitude: f32) -> Vec3 {
    let horizontal = radius * latitude.cos();
    vec3(
        center.x + horizontal * longitude.cos(),
        center.y + radius * latitude.sin(),
        center.z + horizontal * longitude.sin(),
    )
}

fn draw_starfield_3d() {
    for index in 0..84 {
        let x = ((index * 173 + 23) % 997) as f32 / 997.0 * 15.0 - 7.5;
        let y = ((index * 311 + 57) % 991) as f32 / 991.0 * 10.0 - 5.0;
        let z = -4.0 - (index % 7) as f32 * 0.7;
        let brightness = 0.35 + (index % 4) as f32 * 0.12;
        draw_sphere(
            vec3(x, y, z),
            0.012 + (index % 3) as f32 * 0.005,
            None,
            Color::new(0.52, 0.72, 1.0, brightness),
        );
    }
}

fn draw_surface_history(collapse: &RelativisticCollapse) {
    let x = screen_width() * 0.61;
    let y = screen_height() * 0.20;
    let width = screen_width() * 0.34;
    let height = screen_height() * 0.57;
    let plot_x = x + 35.0;
    let plot_y = y + 24.0;
    let plot_w = width - 48.0;
    let plot_h = height - 48.0;
    let project = |time: f64, radius: f64| {
        (
            plot_x + (time / FREE_FALL_TIME) as f32 * plot_w,
            plot_y + (1.0 - radius) as f32 * plot_h,
        )
    };

    draw_text("SURFACE RADIUS AND HORIZON", x, y - 10.0, 16.0, WHITE);
    for tick in 0..=4 {
        let radius = tick as f64 * 0.25;
        let (_, tick_y) = project(0.0, radius);
        draw_line(
            x + 35.0,
            tick_y,
            x + width,
            tick_y,
            1.0,
            Color::from_rgba(63, 76, 98, 140),
        );
        draw_text(format!("{radius:.2}"), x, tick_y + 4.0, 12.0, GRAY);
    }
    draw_line(plot_x, plot_y, plot_x, plot_y + plot_h, 1.5, GRAY);
    draw_line(
        plot_x,
        plot_y + plot_h,
        plot_x + plot_w,
        plot_y + plot_h,
        1.5,
        GRAY,
    );
    let (_, horizon_y) = project(0.0, INITIAL_COMPACTNESS);
    draw_line(plot_x, horizon_y, plot_x + plot_w, horizon_y, 1.5, SKYBLUE);

    let mut previous = None;
    for point in collapse.trajectory() {
        let projected = project(point.time, point.radius);
        if let Some((previous_x, previous_y)) = previous {
            draw_line(previous_x, previous_y, projected.0, projected.1, 2.5, GOLD);
        }
        previous = Some(projected);
    }
    draw_text("surface", plot_x + 8.0, plot_y + 18.0, 13.0, GOLD);
    draw_text("Rs", plot_x + 72.0, horizon_y - 4.0, 13.0, SKYBLUE);
    draw_text(
        "comoving proper time / t0",
        plot_x + 48.0,
        plot_y + plot_h + 22.0,
        13.0,
        GRAY,
    );
}
