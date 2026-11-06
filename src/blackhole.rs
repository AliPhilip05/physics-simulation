//! Schwarzschild black-hole image model in geometrized units (G = c = 1).
//! Here the black-hole mass length is M = 1, so Rs = 2M.

use std::f64::consts::{FRAC_PI_2, PI};
use std::sync::atomic::{AtomicBool, Ordering};

pub const MASS_LENGTH: f64 = 1.0;
pub const EVENT_HORIZON_RADIUS: f64 = 2.0 * MASS_LENGTH;
pub const PHOTON_SPHERE_RADIUS: f64 = 3.0 * MASS_LENGTH;
pub const CRITICAL_IMPACT_PARAMETER: f64 = 5.196_152_422_706_632 * MASS_LENGTH;
pub const DISK_INNER_RADIUS: f64 = 6.0 * MASS_LENGTH;
pub const DISK_OUTER_RADIUS: f64 = 20.0 * MASS_LENGTH;
pub const OBSERVER_RADIUS: f64 = 40.0 * MASS_LENGTH;
pub const IMAGE_WIDTH: u16 = 256;
pub const IMAGE_HEIGHT: u16 = 180;
const VERTICAL_FIELD_OF_VIEW: f64 = PI / 3.0;
const GEODESIC_STEP: f64 = 0.004;
const MAX_AZIMUTH: f64 = 32.0;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ImageSummary {
    pub captured_rays: usize,
    pub disk_rays: usize,
    pub escaped_rays: usize,
    pub critical_rays: usize,
}

pub struct BlackHoleImage {
    pub width: u16,
    pub height: u16,
    pub rgba: Vec<u8>,
    pub summary: ImageSummary,
}

#[derive(Clone, Copy)]
enum RayOutcome {
    Captured,
    Escaped { deflection: f64 },
    DiskHit { radius: f64, redshift: f64 },
    Critical,
}

/// Render a Schwarzschild shadow, lensed star field, and thin emitting disk.
/// Inclination is measured from the disk's rotation axis. Cancellation is
/// checked between image rows so obsolete frames stop promptly.
pub fn render_image_cancellable(
    width: u16,
    height: u16,
    inclination_degrees: f64,
    cancelled: &AtomicBool,
) -> BlackHoleImage {
    let width_usize = usize::from(width.max(1));
    let height_usize = usize::from(height.max(1));
    let inclination = inclination_degrees.to_radians().clamp(0.0, FRAC_PI_2);
    let sin_inclination = inclination.sin();
    let cos_inclination = inclination.cos();
    let half_tangent = (VERTICAL_FIELD_OF_VIEW * 0.5).tan();
    let mut rgba = vec![0_u8; width_usize * height_usize * 4];
    let mut summary = ImageSummary {
        captured_rays: 0,
        disk_rays: 0,
        escaped_rays: 0,
        critical_rays: 0,
    };

    for pixel_y in 0..height_usize {
        if cancelled.load(Ordering::Relaxed) {
            break;
        }
        for pixel_x in 0..width_usize {
            let screen_x = (2.0 * (pixel_x as f64 + 0.5) / width_usize as f64 - 1.0)
                * width_usize as f64
                / height_usize as f64;
            let screen_y = 1.0 - 2.0 * (pixel_y as f64 + 0.5) / height_usize as f64;
            let screen_radius = screen_x.hypot(screen_y);
            let image_angle = (half_tangent * screen_radius).atan();
            let impact_parameter = OBSERVER_RADIUS * image_angle.sin()
                / (1.0 - 2.0 * MASS_LENGTH / OBSERVER_RADIUS).sqrt();
            let azimuth = screen_y.atan2(screen_x);
            let outcome = trace_ray(impact_parameter, azimuth, sin_inclination, cos_inclination);
            let color = match outcome {
                RayOutcome::Captured => {
                    summary.captured_rays += 1;
                    [0, 0, 0]
                }
                RayOutcome::Escaped { deflection } => {
                    summary.escaped_rays += 1;
                    let source_angle = image_angle - deflection;
                    background_color(source_angle * azimuth.cos(), source_angle * azimuth.sin())
                }
                RayOutcome::DiskHit { radius, redshift } => {
                    summary.disk_rays += 1;
                    disk_color(radius, redshift)
                }
                RayOutcome::Critical => {
                    summary.critical_rays += 1;
                    [255, 190, 86]
                }
            };
            let offset = 4 * (pixel_y * width_usize + pixel_x);
            rgba[offset..offset + 4].copy_from_slice(&[color[0], color[1], color[2], 255]);
        }
    }

    BlackHoleImage {
        width,
        height,
        rgba,
        summary,
    }
}

fn trace_ray(impact: f64, azimuth: f64, sin_inclination: f64, cos_inclination: f64) -> RayOutcome {
    if impact <= f64::EPSILON {
        return RayOutcome::Captured;
    }
    let inverse_radius_start = 1.0 / OBSERVER_RADIUS;
    let radial_term = 1.0 / impact.powi(2) - inverse_radius_start.powi(2)
        + 2.0 * MASS_LENGTH * inverse_radius_start.powi(3);
    if radial_term <= 0.0 {
        return RayOutcome::Captured;
    }

    // u = 1/r obeys u'' + u = 3 M u² for a Schwarzschild null geodesic.
    let mut inverse_radius = inverse_radius_start;
    let mut derivative = radial_term.sqrt();
    let mut phi = 0.0;
    let mut previous_z = cos_inclination * OBSERVER_RADIUS;
    let mut previous_radius = OBSERVER_RADIUS;
    let e_t_z = -azimuth.sin() * sin_inclination;
    let max_steps = (MAX_AZIMUTH / GEODESIC_STEP) as usize;

    for _ in 0..max_steps {
        let (new_inverse_radius, new_derivative) =
            rk4_step(inverse_radius, derivative, GEODESIC_STEP);
        phi += GEODESIC_STEP;
        inverse_radius = new_inverse_radius;
        derivative = new_derivative;
        if !inverse_radius.is_finite() || inverse_radius <= 0.0 || !derivative.is_finite() {
            return RayOutcome::Captured;
        }
        if inverse_radius >= 1.0 / EVENT_HORIZON_RADIUS {
            return RayOutcome::Captured;
        }

        let radius = 1.0 / inverse_radius;
        let z = radius * (cos_inclination * phi.cos() + e_t_z * phi.sin());
        if (previous_z >= 0.0 && z < 0.0) || (previous_z < 0.0 && z >= 0.0) {
            let fraction = previous_z / (previous_z - z);
            let disk_radius = previous_radius + fraction * (radius - previous_radius);
            if (DISK_INNER_RADIUS..=DISK_OUTER_RADIUS).contains(&disk_radius) {
                let hit_phi = phi - GEODESIC_STEP * (1.0 - fraction);
                let x = disk_radius * hit_phi.sin() * azimuth.cos();
                let y = disk_radius
                    * (hit_phi.cos() * sin_inclination
                        + hit_phi.sin() * azimuth.sin() * cos_inclination);
                let orbital_speed = (MASS_LENGTH / (disk_radius - 2.0 * MASS_LENGTH)).sqrt();
                let gamma = 1.0 / (1.0 - orbital_speed.powi(2)).sqrt();
                let toward_observer = orbital_speed * (x / disk_radius) * sin_inclination;
                let gravitational = (1.0 - 2.0 * MASS_LENGTH / disk_radius).sqrt();
                let redshift = gravitational / (gamma * (1.0 - toward_observer));
                let _disk_azimuth = y.atan2(x);
                return RayOutcome::DiskHit {
                    radius: disk_radius,
                    redshift,
                };
            }
        }

        if radius >= OBSERVER_RADIUS && derivative < 0.0 && phi > FRAC_PI_2 {
            let local_impact_sine =
                impact * (1.0 - 2.0 * MASS_LENGTH / OBSERVER_RADIUS).sqrt() / OBSERVER_RADIUS;
            let finite_distance_correction = 2.0 * local_impact_sine.clamp(-1.0, 1.0).asin();
            let deflection = (phi - PI + finite_distance_correction).max(0.0);
            return RayOutcome::Escaped { deflection };
        }

        previous_z = z;
        previous_radius = radius;
    }
    RayOutcome::Critical
}

fn rk4_step(u: f64, du: f64, step: f64) -> (f64, f64) {
    let acceleration =
        |inverse_radius: f64| 3.0 * MASS_LENGTH * inverse_radius.powi(2) - inverse_radius;
    let k1_u = du;
    let k1_du = acceleration(u);
    let k2_u = du + 0.5 * step * k1_du;
    let k2_du = acceleration(u + 0.5 * step * k1_u);
    let k3_u = du + 0.5 * step * k2_du;
    let k3_du = acceleration(u + 0.5 * step * k2_u);
    let k4_u = du + step * k3_du;
    let k4_du = acceleration(u + step * k3_u);
    (
        u + step * (k1_u + 2.0 * k2_u + 2.0 * k3_u + k4_u) / 6.0,
        du + step * (k1_du + 2.0 * k2_du + 2.0 * k3_du + k4_du) / 6.0,
    )
}

fn disk_color(radius: f64, redshift: f64) -> [u8; 3] {
    // Newtonian thin-disk flux profile, then bolometric I_obs = g^4 I_emit.
    let emitted = (radius.powi(-3) * (1.0 - (DISK_INNER_RADIUS / radius).sqrt()).max(0.0)
        / 0.00027)
        .clamp(0.0, 1.0);
    let brightness = (emitted * redshift.powi(4)).clamp(0.0, 1.0);
    let blue_fraction = ((redshift - 0.72) / 0.62).clamp(0.0, 1.0);
    let red = 255.0 * brightness * (1.0 - 0.68 * blue_fraction);
    let green = 74.0 * brightness + 142.0 * brightness * blue_fraction;
    let blue = 21.0 * brightness + 210.0 * brightness * blue_fraction;
    [red as u8, green as u8, blue as u8]
}

fn background_color(source_x: f64, source_y: f64) -> [u8; 3] {
    let grid_scale = 92.0;
    let grid_x = (source_x * grid_scale).floor() as i32;
    let grid_y = (source_y * grid_scale).floor() as i32;
    let hash = hash_cell(grid_x, grid_y);
    let jitter_x = ((hash & 0xffff) as f64 / 65535.0) * 0.76 + 0.12;
    let jitter_y = (((hash >> 16) & 0xffff) as f64 / 65535.0) * 0.76 + 0.12;
    let dx = (source_x * grid_scale).fract() - jitter_x;
    let dy = (source_y * grid_scale).fract() - jitter_y;
    let star = if hash % 173 < 2 && dx.hypot(dy) < 0.075 {
        185.0 + (hash % 70) as f64
    } else {
        0.0
    };
    let nebula = ((source_x * 17.0).sin() * (source_y * 13.0 + 0.7).cos()
        + (source_x * 7.0 - source_y * 9.0).sin())
    .max(0.0)
        * 3.0;
    [
        (3.0 + nebula * 1.2 + star) as u8,
        (5.0 + nebula * 1.5 + star * 0.88) as u8,
        (13.0 + nebula * 4.0 + star * 0.67) as u8,
    ]
}

fn hash_cell(x: i32, y: i32) -> u32 {
    let mut value = (x as u32).wrapping_mul(0x9e37_79b9) ^ (y as u32).wrapping_mul(0x85eb_ca6b);
    value ^= value >> 16;
    value = value.wrapping_mul(0x7feb_352d);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846c_a68b);
    value ^ (value >> 16)
}
