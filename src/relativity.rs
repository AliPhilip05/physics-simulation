//! Analytic Oppenheimer–Snyder surface collapse for a uniform, pressureless sphere.
//! The animation uses comoving proper time and a dimensionless initial compactness.

use crate::simulation::{TrajectoryPoint, analytic_surface_radius};

pub const INITIAL_COMPACTNESS: f64 = 0.1;
pub const MIN_DISPLAY_SCALE: f64 = 0.02;
pub const FREE_FALL_TIME: f64 = std::f64::consts::PI / (2.0 * std::f64::consts::SQRT_2);

#[derive(Debug)]
pub struct RelativisticCollapse {
    /// Comoving proper time divided by t0 = sqrt(R0^3 / (G M)).
    pub elapsed: f64,
    pub paused: bool,
    pub status: Option<&'static str>,
    scale_factor: f64,
    horizon_crossed: bool,
    trajectory: Vec<TrajectoryPoint>,
}

impl Default for RelativisticCollapse {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            paused: true,
            status: None,
            scale_factor: 1.0,
            horizon_crossed: false,
            trajectory: vec![TrajectoryPoint {
                time: 0.0,
                radius: 1.0,
            }],
        }
    }
}

impl RelativisticCollapse {
    pub fn surface_radius_ratio(&self) -> f64 {
        self.scale_factor
    }

    pub fn schwarzschild_radius_ratio(&self) -> f64 {
        INITIAL_COMPACTNESS
    }

    pub fn compactness(&self) -> f64 {
        INITIAL_COMPACTNESS / self.scale_factor
    }

    pub fn horizon_crossed(&self) -> bool {
        self.horizon_crossed
    }

    pub fn horizon_crossing_time() -> f64 {
        time_at_scale(INITIAL_COMPACTNESS)
    }

    pub fn trajectory(&self) -> &[TrajectoryPoint] {
        &self.trajectory
    }

    /// Advance analytic surface evolution in comoving proper time.
    pub fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        if let Some(reason) = self.status {
            self.paused = true;
            return Err(reason);
        }
        if !dt.is_finite() || dt <= 0.0 {
            return self.stop("Timestep must be finite and positive");
        }

        let maximum_time = time_at_scale(MIN_DISPLAY_SCALE);
        let proposed_time = self.elapsed + dt;
        if !proposed_time.is_finite() {
            return self.stop("Proposed proper time is not finite");
        }
        let new_time = proposed_time.min(maximum_time);
        let Some(new_scale) = analytic_surface_radius(new_time) else {
            return self.stop("Analytic collapse time is outside its valid range");
        };
        if !new_scale.is_finite() || new_scale <= 0.0 {
            return self.stop("Analytic surface radius is invalid");
        }

        self.elapsed = new_time;
        self.scale_factor = new_scale;
        self.horizon_crossed = self.horizon_crossed || new_scale <= INITIAL_COMPACTNESS;
        self.trajectory.push(TrajectoryPoint {
            time: new_time,
            radius: new_scale,
        });
        if proposed_time >= maximum_time {
            self.status = Some("Black hole formed; near-singularity display cutoff");
            self.paused = true;
        }
        Ok(())
    }

    fn stop(&mut self, reason: &'static str) -> Result<(), &'static str> {
        self.status = Some(reason);
        self.paused = true;
        Err(reason)
    }
}

/// Proper time for a collapsing dust sphere to reach a given R/R0.
fn time_at_scale(scale: f64) -> f64 {
    let eta = (2.0 * scale - 1.0).clamp(-1.0, 1.0).acos();
    (eta + eta.sin()) / (2.0 * 2.0_f64.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begins_at_unit_radius_and_horizon_time_is_before_singularity() {
        let collapse = RelativisticCollapse::default();
        assert_eq!(collapse.surface_radius_ratio(), 1.0);
        assert!(collapse.schwarzschild_radius_ratio() < 1.0);
        assert!(RelativisticCollapse::horizon_crossing_time() < FREE_FALL_TIME);
    }

    #[test]
    fn crosses_schwarzschild_radius_at_analytic_proper_time() {
        let mut collapse = RelativisticCollapse::default();
        let horizon_time = RelativisticCollapse::horizon_crossing_time();
        while collapse.elapsed + 0.002 < horizon_time {
            collapse.step(0.002).unwrap();
        }
        collapse.step(horizon_time - collapse.elapsed).unwrap();
        assert!(collapse.horizon_crossed());
        assert!((collapse.surface_radius_ratio() - INITIAL_COMPACTNESS).abs() < 1e-12);
        assert!((collapse.compactness() - 1.0).abs() < 1e-11);
    }

    #[test]
    fn display_stops_before_the_singularity_and_keeps_black_hole_state() {
        let mut collapse = RelativisticCollapse::default();
        while collapse.status.is_none() {
            collapse.step(0.002).unwrap();
        }
        assert!(collapse.horizon_crossed());
        assert_eq!(collapse.surface_radius_ratio(), MIN_DISPLAY_SCALE);
        assert!(collapse.paused);
    }

    #[test]
    fn rejects_invalid_timestep_without_advancing() {
        let mut collapse = RelativisticCollapse::default();
        assert!(collapse.step(f64::NAN).is_err());
        assert_eq!(collapse.elapsed, 0.0);
        assert_eq!(collapse.surface_radius_ratio(), 1.0);
    }
}
