use macroquad::prelude::*;

use crate::simulation::{SHELL_COUNT, SURFACE_CUTOFF, Simulation, analytic_surface_radius};

pub fn draw(simulation: &Simulation, steps_this_frame: u32, convergence: &[(f64, f64); 3]) {
    clear_background(Color::from_rgba(10, 15, 26, 255));

    let width = screen_width();
    let height = screen_height();
    let surface = simulation.surface();
    let center = vec2(width * 0.31, height * 0.57);
    let pixels_per_unit = height.min(width * 0.58) * 0.32;

    draw_circle(
        center.x,
        center.y,
        surface.radius() as f32 * pixels_per_unit,
        Color::from_rgba(221, 104, 45, 255),
    );
    for (index, shell) in simulation.shells().iter().enumerate() {
        let color = if index + 1 == simulation.shells().len() {
            GOLD
        } else {
            Color::from_rgba(255, 194, 104, 125)
        };
        draw_circle_lines(
            center.x,
            center.y,
            shell.radius() as f32 * pixels_per_unit,
            1.0,
            color,
        );
    }
    draw_circle(center.x, center.y, 3.0, WHITE);

    let status = match simulation.status {
        None if simulation.paused => "PAUSED".to_owned(),
        None => "RUNNING".to_owned(),
        Some(reason) => format!("STOPPED: {reason}"),
    };
    draw_text("UNIFORM DUST COLLAPSE | NEWTONIAN", 24.0, 38.0, 25.0, WHITE);
    draw_text(&status, 24.0, 68.0, 20.0, GRAY);
    draw_text(
        format!("SURFACE R / R0: {:.4}", surface.radius()),
        24.0,
        112.0,
        20.0,
        GOLD,
    );
    draw_text(
        format!("SURFACE V / V0: {:.4}", surface.radial_velocity()),
        24.0,
        138.0,
        20.0,
        BLUE,
    );
    draw_text(
        format!("SURFACE a / a0: {:.4}", surface.radial_acceleration()),
        24.0,
        164.0,
        20.0,
        GOLD,
    );
    draw_text(
        format!("TIME t / t0: {:.3}", simulation.elapsed),
        24.0,
        190.0,
        20.0,
        BLUE,
    );
    draw_text(
        format!("MATERIAL BOUNDARIES: {SHELL_COUNT} | TOTAL MASS: 1"),
        24.0,
        216.0,
        18.0,
        WHITE,
    );
    draw_text(
        format!(
            "MAX SPECIFIC-ENERGY DRIFT: {:.3e}",
            simulation.max_specific_energy_error()
        ),
        24.0,
        242.0,
        18.0,
        GOLD,
    );
    draw_text(
        format!("ACCEPTED UPDATES THIS FRAME: {steps_this_frame}"),
        24.0,
        268.0,
        18.0,
        BLUE,
    );
    draw_text(
        format!("Numerical surface cutoff: {SURFACE_CUTOFF:.2} R0"),
        24.0,
        294.0,
        16.0,
        GRAY,
    );

    draw_trajectory_plot(simulation, convergence);

    draw_text(
        "Space: pause/resume | R: reset | Esc: close",
        24.0,
        height - 20.0,
        18.0,
        GRAY,
    );
}

fn draw_trajectory_plot(simulation: &Simulation, convergence: &[(f64, f64); 3]) {
    let width = screen_width();
    let height = screen_height();
    let plot_x = width * 0.60;
    let plot_y = height * 0.20;
    let plot_w = width * 0.35;
    let plot_h = height * 0.59;
    let time_max = std::f64::consts::PI / 8.0_f64.sqrt();
    let project = |time: f64, radius: f64| -> (f32, f32) {
        (
            plot_x + (time / time_max) as f32 * plot_w,
            plot_y + (1.0 - radius) as f32 * plot_h,
        )
    };

    draw_text(
        "SURFACE RADIUS VS MODEL TIME",
        plot_x,
        plot_y - 14.0,
        17.0,
        WHITE,
    );
    for tick in 0..=4 {
        let radius = tick as f64 * 0.25;
        let (_, y) = project(0.0, radius);
        draw_line(
            plot_x,
            y,
            plot_x + plot_w,
            y,
            1.0,
            Color::from_rgba(68, 78, 92, 160),
        );
        draw_text(format!("{radius:.2}"), plot_x - 36.0, y + 5.0, 13.0, GRAY);
    }
    draw_line(plot_x, plot_y, plot_x, plot_y + plot_h, 2.0, GRAY);
    draw_line(
        plot_x,
        plot_y + plot_h,
        plot_x + plot_w,
        plot_y + plot_h,
        2.0,
        GRAY,
    );

    let analytic_samples = 160;
    let mut previous = None;
    for sample in 0..=analytic_samples {
        let time = time_max * sample as f64 / analytic_samples as f64;
        if let Some(radius) = analytic_surface_radius(time) {
            let point = project(time, radius);
            if let Some((px, py)) = previous {
                draw_line(px, py, point.0, point.1, 2.0, ORANGE);
            }
            previous = Some(point);
        }
    }

    let mut previous = None;
    for point in simulation.trajectory() {
        let projected = project(point.time, point.radius);
        if let Some((px, py)) = previous {
            draw_line(px, py, projected.0, projected.1, 2.5, SKYBLUE);
        }
        previous = Some(projected);
    }
    draw_text("analytic", plot_x + 8.0, plot_y + 19.0, 14.0, ORANGE);
    draw_text("numerical", plot_x + 82.0, plot_y + 19.0, 14.0, SKYBLUE);
    draw_text(
        "t / t0",
        plot_x + plot_w - 45.0,
        plot_y + plot_h + 22.0,
        14.0,
        GRAY,
    );
    draw_text(
        "|surface radius error| at t/t0 = 0.5",
        plot_x,
        plot_y + plot_h + 46.0,
        14.0,
        WHITE,
    );
    for (index, (dt, error)) in convergence.iter().enumerate() {
        draw_text(
            format!("dt={dt:.4}: {error:.3e}"),
            plot_x,
            plot_y + plot_h + 65.0 + index as f32 * 17.0,
            13.0,
            SKYBLUE,
        );
    }
}
