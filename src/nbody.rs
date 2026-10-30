//! Small, direct-summation 3D Newtonian N-body model.
//!
//! The particles are equal-mass collisionless tracers in a cold uniform sphere.
//! This is a spatial 3D model, not a fluid or relativistic star solver.

pub const PARTICLE_COUNT: usize = 64;
pub const PLUMMER_SOFTENING: f64 = 0.05;
const G_NORMALIZED: f64 = 1.0;

pub type Vec3d = [f64; 3];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Particle {
    position: Vec3d,
    velocity: Vec3d,
}

impl Particle {
    pub fn position(&self) -> Vec3d {
        self.position
    }
}

#[derive(Debug)]
pub struct ParticleCloud {
    particles: Vec<Particle>,
    pub elapsed: f64,
    pub paused: bool,
    pub status: Option<&'static str>,
    initial_energy: f64,
}

impl Default for ParticleCloud {
    fn default() -> Self {
        let half_count = PARTICLE_COUNT / 2;
        let mut positions = Vec::with_capacity(PARTICLE_COUNT);
        for index in 1..=half_count {
            // A deterministic low-discrepancy sample of a uniform unit ball.
            let radial_quantile = radical_inverse(index, 2);
            let radius = radial_quantile.cbrt();
            let cos_theta = 1.0 - 2.0 * radical_inverse(index, 3);
            let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
            let phi = std::f64::consts::TAU * radical_inverse(index, 5);
            let point = [
                radius * sin_theta * phi.cos(),
                radius * cos_theta,
                radius * sin_theta * phi.sin(),
            ];
            positions.push(point);
            positions.push(scale(point, -1.0));
        }

        let outer_radius = positions
            .iter()
            .map(|position| dot(*position, *position).sqrt())
            .fold(0.0, f64::max);
        for position in &mut positions {
            *position = scale(*position, 1.0 / outer_radius);
        }

        let particles = positions
            .into_iter()
            .map(|position| Particle {
                position,
                velocity: [0.0; 3],
            })
            .collect();
        let mut cloud = Self {
            particles,
            elapsed: 0.0,
            paused: true,
            status: None,
            initial_energy: 0.0,
        };
        cloud.initial_energy = cloud.total_energy();
        cloud
    }
}

impl ParticleCloud {
    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn mass_per_particle(&self) -> f64 {
        1.0 / self.particles.len() as f64
    }

    pub fn rms_radius(&self) -> f64 {
        (self
            .particles
            .iter()
            .map(|particle| dot(particle.position, particle.position))
            .sum::<f64>()
            / self.particles.len() as f64)
            .sqrt()
    }

    pub fn total_energy(&self) -> f64 {
        let mass = self.mass_per_particle();
        let kinetic = self
            .particles
            .iter()
            .map(|particle| 0.5 * mass * dot(particle.velocity, particle.velocity))
            .sum::<f64>();
        let mut potential = 0.0;
        for i in 0..self.particles.len() {
            for j in (i + 1)..self.particles.len() {
                let displacement = subtract(self.particles[j].position, self.particles[i].position);
                let softened_distance =
                    (dot(displacement, displacement) + PLUMMER_SOFTENING.powi(2)).sqrt();
                potential -= G_NORMALIZED * mass * mass / softened_distance;
            }
        }
        kinetic + potential
    }

    pub fn absolute_energy_error(&self) -> f64 {
        (self.total_energy() - self.initial_energy).abs()
    }

    /// Advance every particle with synchronized velocity-Verlet updates.
    /// A failed candidate leaves positions, velocities, and time unchanged.
    pub fn step(&mut self, dt: f64) -> Result<(), &'static str> {
        if let Some(reason) = self.status {
            self.paused = true;
            return Err(reason);
        }
        if !dt.is_finite() || dt <= 0.0 {
            return self.stop("Timestep must be finite and positive");
        }
        if !self.elapsed.is_finite()
            || self.elapsed < 0.0
            || self.particles.len() != PARTICLE_COUNT
            || !self.particles.iter().all(|particle| {
                particle.position.iter().all(|v| v.is_finite())
                    && particle.velocity.iter().all(|v| v.is_finite())
            })
        {
            return self.stop("Current particle state is invalid");
        }

        let new_elapsed = self.elapsed + dt;
        if !new_elapsed.is_finite() {
            return self.stop("Proposed simulation time is not finite");
        }

        let old_accelerations = accelerations(&self.particles);
        let new_positions: Vec<Vec3d> = self
            .particles
            .iter()
            .zip(&old_accelerations)
            .map(|(particle, acceleration)| {
                add(
                    add(particle.position, scale(particle.velocity, dt)),
                    scale(*acceleration, 0.5 * dt * dt),
                )
            })
            .collect();
        if !new_positions
            .iter()
            .all(|position| position.iter().all(|value| value.is_finite()))
        {
            return self.stop("Proposed particle position is not finite");
        }

        let new_accelerations = accelerations_at(&new_positions);
        let candidates: Vec<Particle> = self
            .particles
            .iter()
            .zip(new_positions)
            .zip(old_accelerations.iter().zip(new_accelerations))
            .map(
                |((particle, position), (old_acceleration, new_acceleration))| Particle {
                    position,
                    velocity: add(
                        particle.velocity,
                        scale(add(*old_acceleration, new_acceleration), 0.5 * dt),
                    ),
                },
            )
            .collect();
        if !candidates.iter().all(|particle| {
            particle.position.iter().all(|value| value.is_finite())
                && particle.velocity.iter().all(|value| value.is_finite())
        }) {
            return self.stop("Proposed particle state is not finite");
        }

        self.particles = candidates;
        self.elapsed = new_elapsed;
        Ok(())
    }

    fn stop(&mut self, reason: &'static str) -> Result<(), &'static str> {
        self.status = Some(reason);
        self.paused = true;
        Err(reason)
    }
}

fn accelerations(particles: &[Particle]) -> Vec<Vec3d> {
    accelerations_at(
        &particles
            .iter()
            .map(|particle| particle.position)
            .collect::<Vec<_>>(),
    )
}

fn accelerations_at(positions: &[Vec3d]) -> Vec<Vec3d> {
    let mass = 1.0 / positions.len() as f64;
    let softening_squared = PLUMMER_SOFTENING.powi(2);
    let mut result = vec![[0.0; 3]; positions.len()];
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let displacement = subtract(positions[j], positions[i]);
            let distance_squared = dot(displacement, displacement) + softening_squared;
            let factor = G_NORMALIZED * mass / (distance_squared * distance_squared.sqrt());
            let pair_acceleration = scale(displacement, factor);
            result[i] = add(result[i], pair_acceleration);
            result[j] = subtract(result[j], pair_acceleration);
        }
    }
    result
}

fn radical_inverse(mut index: usize, base: usize) -> f64 {
    let mut factor = 1.0 / base as f64;
    let mut value = 0.0;
    while index > 0 {
        value += factor * (index % base) as f64;
        index /= base;
        factor /= base as f64;
    }
    value
}

fn add(left: Vec3d, right: Vec3d) -> Vec3d {
    [left[0] + right[0], left[1] + right[1], left[2] + right[2]]
}

fn subtract(left: Vec3d, right: Vec3d) -> Vec3d {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn scale(vector: Vec3d, factor: f64) -> Vec3d {
    [vector[0] * factor, vector[1] * factor, vector[2] * factor]
}

fn dot(left: Vec3d, right: Vec3d) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_cloud_is_three_dimensional_equal_mass_and_centered() {
        let cloud = ParticleCloud::default();
        assert_eq!(cloud.particles.len(), PARTICLE_COUNT);
        assert!((cloud.mass_per_particle() * PARTICLE_COUNT as f64 - 1.0).abs() < 1e-14);
        assert!((cloud.rms_radius() - (3.0_f64 / 5.0).sqrt()).abs() < 0.08);
        let center_of_mass = cloud
            .particles
            .iter()
            .fold([0.0; 3], |sum, particle| add(sum, particle.position));
        assert!(
            center_of_mass
                .iter()
                .all(|component| component.abs() < 1e-14)
        );
        assert!(
            cloud
                .particles
                .iter()
                .all(|particle| particle.velocity == [0.0; 3])
        );
    }

    #[test]
    fn softened_pair_accelerations_conserve_total_momentum() {
        let cloud = ParticleCloud::default();
        let accelerations = accelerations(&cloud.particles);
        let net_acceleration = accelerations
            .iter()
            .fold([0.0; 3], |sum, acceleration| add(sum, *acceleration));
        assert!(
            net_acceleration
                .iter()
                .all(|component| component.abs() < 1e-12)
        );
    }

    #[test]
    fn accepted_step_moves_particles_inward_and_preserves_center_of_mass() {
        let mut cloud = ParticleCloud::default();
        let initial_radius = cloud.rms_radius();
        cloud.step(0.001).unwrap();
        assert!(cloud.rms_radius() < initial_radius);
        let center_of_mass = cloud
            .particles
            .iter()
            .fold([0.0; 3], |sum, particle| add(sum, particle.position));
        assert!(
            center_of_mass
                .iter()
                .all(|component| component.abs() < 1e-12)
        );
        assert_eq!(cloud.elapsed, 0.001);
    }

    #[test]
    fn invalid_step_is_rejected_atomically() {
        let mut cloud = ParticleCloud::default();
        let particles_before = cloud.particles.clone();
        assert!(cloud.step(f64::NAN).is_err());
        assert_eq!(cloud.particles, particles_before);
        assert_eq!(cloud.elapsed, 0.0);
        assert!(cloud.status.is_some());
    }
}
