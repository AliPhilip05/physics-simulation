use macroquad::prelude::*;

use crate::simulation::Simulation;

pub fn draw(simulation: &Simulation, steps_this_frame: u32) {
    clear_background(Color::from_rgba(10, 15, 26, 255));

    let center = vec2(screen_width() * 0.5, screen_height() * 0.52);
    let pixels_per_unit = screen_width().min(screen_height()) * 0.24;
    let radius_px = simulation.star.radius as f32 * pixels_per_unit;
    let milestone_message: String = {
        if simulation.paused {
            String::from("PAUSED")
        } else {
            String::from("RUNNING")
        }
    };

    draw_circle(center.x, center.y, radius_px, ORANGE);
    draw_circle_lines(center.x, center.y, radius_px, 2.0, GOLD);
    draw_circle(center.x, center.y, 3.0, WHITE);

    draw_text("STELLAR COLLAPSE", 24.0, 40.0, 28.0, WHITE);
    draw_text(&milestone_message, 24.0, 70.0, 22.0, GRAY);
    draw_text(
        format!("R / R0 = {:.2}", simulation.star.radius),
        24.0,
        100.0,
        22.0,
        GOLD,
    );
    draw_text(
        format!("UPDATES THIS FRAME: {}", steps_this_frame),
        24.0,
        124.0,
        22.0,
        BLUE,
    );
    draw_text(
        format!(
            "RADIAL VELOCITY (V / V0): {:.2}",
            simulation.star.radial_velocity
        ),
        24.0,
        168.0,
        22.0,
        BLUE,
    );
    draw_text(
        format!("SIMULATION TIME (T / T0) {:.2}", simulation.elapsed),
        24.0,
        144.0,
        22.0,
        BLUE,
    );
    draw_text(
        format!(
            "RADIAL ACCELERATION (a / a0) {:.2}",
            simulation.star.radial_acceleration()
        ),
        24.0,
        192.0,
        22.0,
        GOLD,
    );
    draw_text(
        "Space: pause/resume | R: reset | Esc: close | Resize: check scaling",
        24.0,
        screen_height() - 24.0,
        20.0,
        GRAY,
    );
}
