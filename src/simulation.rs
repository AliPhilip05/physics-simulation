/// Normalized state of one spherical outer material boundary.
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
    pub fn radial_acceleration(&self) -> f64 {
        let acceleration: f64 = -1.0 / (self.radius * self.radius);
        acceleration
    }

    /// Produces an unchecked candidate in normalized model units.
    /// Input validation, cutoff rejection, and simulation acceptance are the next exercise.
    #[allow(dead_code)] // Intentionally unwired until the acceptance/stopping exercise.
    pub fn velocity_verlet(&self, dt: f64) -> Star {
        let new_radius: f64 = self.radius
            + (self.radial_velocity * dt)
            + ((0.5) * self.radial_acceleration() * dt * dt);
        let new_acceleration: f64 = -1.0 / (new_radius * new_radius);

        let new_radial_velocity: f64 =
            self.radial_velocity + (0.5) * (self.radial_acceleration() + new_acceleration) * dt;

        let new_star: Star = create_star(new_radius, new_radial_velocity);
        new_star
    }
}

#[allow(dead_code)] // Used by the intentionally unwired candidate integrator.
fn create_star(new_radius: f64, new_radial_velocity: f64) -> Star {
    let new_star: Star = Star {
        radius: new_radius,
        radial_velocity: new_radial_velocity,
    };
    new_star
}
pub struct Simulation {
    pub star: Star,
    /// Simulation time divided by t0, where t0 = sqrt(R0^3 / (G M)).
    pub elapsed: f64,
    pub paused: bool,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            star: Star::default(),
            elapsed: 0.0,
            paused: true,
        }
    }
}

impl Simulation {
    pub fn step(&mut self, dt: f64) {
        self.elapsed += dt;
    }
}

#[cfg(test)]
mod tests {
    // Add accepted-step, rejected-step, reset, and cutoff tests in the next exercise.
}
