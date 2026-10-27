/// Numerical stopping radius for the Newtonian exercise; it is not a horizon.
pub const RADIUS_CUTOFF: f64 = 0.25;
const INITIAL_SPECIFIC_ENERGY: f64 = -1.0;

/// Normalized state of one spherical outer material boundary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Star {
    /// Radius divided by the initial radius, R / R0.
    pub radius: f64,
    /// Radial velocity divided by v0 = R0 / t0; negative points inward.
    pub radial_velocity: f64,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            radius: 1.0,
            radial_velocity: 0.0,
        }
    }
}

impl Star {
    /// Newtonian radial acceleration in normalized units where G M = 1.
    pub fn radial_acceleration(&self) -> f64 {
        -1.0 / (self.radius * self.radius)
    }

    /// Return a validated velocity-Verlet candidate without mutating this state.
    pub fn velocity_verlet(&self, dt: f64) -> Result<Star, &'static str> {
        if !dt.is_finite() || dt <= 0.0 {
            return Err("Timestep must be finite and positive");
        }
        if !self.radius.is_finite()
            || self.radius <= RADIUS_CUTOFF
            || !self.radial_velocity.is_finite()
        {
            return Err("Current radial state is invalid");
        }

        let acceleration = self.radial_acceleration();
        let new_radius = self.radius + self.radial_velocity * dt + 0.5 * acceleration * dt * dt;
        if !new_radius.is_finite() {
            return Err("Proposed radius is not finite");
        }
        if new_radius <= RADIUS_CUTOFF {
            return Err("Numerical radius cutoff reached");
        }

        let new_acceleration = -1.0 / (new_radius * new_radius);
        let new_velocity = self.radial_velocity + 0.5 * (acceleration + new_acceleration) * dt;
        if !new_velocity.is_finite() {
            return Err("Proposed radial velocity is not finite");
        }

        Ok(Star {
            radius: new_radius,
            radial_velocity: new_velocity,
        })
    }

    /// Newtonian specific energy per unit mass, epsilon = v^2/2 - 1/r.
    pub fn specific_energy(&self) -> f64 {
        0.5 * self.radial_velocity * self.radial_velocity - 1.0 / self.radius
    }
}

#[derive(Debug)]
pub struct Simulation {
    pub star: Star,
    /// Simulation time divided by t0 = sqrt(R0^3 / (G M)).
    pub elapsed: f64,
    pub paused: bool,
    /// A terminal numerical stop; reset clears it.
    pub status: Option<&'static str>,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            star: Star::default(),
            elapsed: 0.0,
            paused: true,
            status: None,
        }
    }
}

impl Simulation {
    pub fn specific_energy(&self) -> f64 {
        self.star.specific_energy()
    }

    pub fn absolute_energy_error(&self) -> f64 {
        (self.specific_energy() - INITIAL_SPECIFIC_ENERGY).abs()
    }

    /// Accept the candidate star and time together, or preserve both on failure.
    pub fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        if let Some(reason) = self.status {
            self.paused = true;
            return Err(reason);
        }
        if !dt.is_finite() || dt <= 0.0 {
            return self.stop("Timestep must be finite and positive");
        }
        if !self.elapsed.is_finite() || self.elapsed < 0.0 {
            return self.stop("Simulation time is invalid");
        }
        let new_elapsed = self.elapsed + dt;
        if !new_elapsed.is_finite() {
            return self.stop("Proposed simulation time is not finite");
        }
        let new_star = match self.star.velocity_verlet(dt) {
            Ok(candidate) => candidate,
            Err(reason) => return self.stop(reason),
        };
        self.star = new_star;
        self.elapsed = new_elapsed;
        Ok(())
    }

    fn stop(&mut self, reason: &'static str) -> Result<(), &'static str> {
        self.status = Some(reason);
        self.paused = true;
        Err(reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual} != {expected}"
        );
    }

    #[test]
    fn acceleration_scales_with_inverse_square_radius() {
        for (radius, expected) in [(1.0, -1.0), (0.5, -4.0), (2.0, -0.25), (0.25, -16.0)] {
            close(
                Star {
                    radius,
                    radial_velocity: 0.0,
                }
                .radial_acceleration(),
                expected,
                1e-12,
            );
        }
    }

    #[test]
    fn first_velocity_verlet_step_matches_reference() {
        let mut simulation = Simulation::default();
        simulation.step(0.01).unwrap();
        close(simulation.star.radius, 0.99995, 1e-12);
        close(simulation.star.radial_velocity, -0.0100005000375025, 1e-12);
        close(simulation.elapsed, 0.01, 1e-12);
    }

    #[test]
    fn rejected_steps_preserve_state_and_report_terminal_status() {
        for dt in [-1.0, f64::NAN, f64::INFINITY, 1.5] {
            let mut simulation = Simulation::default();
            let star_before = simulation.star;
            let elapsed_before = simulation.elapsed;
            assert!(simulation.step(dt).is_err());
            assert_eq!(simulation.star, star_before);
            assert_eq!(simulation.elapsed, elapsed_before);
            assert!(simulation.paused);
            assert!(simulation.status.is_some());
        }
    }

    #[test]
    fn invalid_current_state_is_rejected_before_gravity() {
        let mut simulation = Simulation::default();
        simulation.star.radius = f64::NAN;
        assert!(simulation.step(0.01).is_err());
        assert!(simulation.status.is_some());
        assert_eq!(simulation.elapsed, 0.0);
    }

    #[test]
    fn reset_restores_the_complete_initial_state() {
        let mut simulation = Simulation::default();
        simulation.step(-1.0).unwrap_err();
        simulation = Simulation::default();
        assert_eq!(simulation.star, Star::default());
        assert_eq!(simulation.elapsed, 0.0);
        assert!(simulation.paused);
        assert_eq!(simulation.status, None);
        assert_eq!(simulation.specific_energy(), -1.0);
        assert_eq!(simulation.absolute_energy_error(), 0.0);
    }

    #[test]
    fn energy_error_improves_with_smaller_timestep_at_common_time() {
        let errors = [0.01, 0.005, 0.0025].map(|dt| {
            let mut simulation = Simulation::default();
            for _ in 0..(0.5 / dt) as usize {
                simulation.step(dt).unwrap();
            }
            simulation.absolute_energy_error()
        });
        assert!(errors[1] < errors[0] && errors[2] < errors[1], "{errors:?}");
    }
}
