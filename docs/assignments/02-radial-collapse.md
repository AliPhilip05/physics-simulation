# Assignment 2: first Newtonian radial collapse

## Objective

Turn the static star into a contracting Newtonian surface released from rest.
Deliver one runnable checkpoint containing motion, stopping behavior, diagnostics,
and numerical evidence. Choose the interfaces and implementation structure.

## Session checkpoint — October 23, 2026

Normalized state and readouts, gravity, and an unchecked one-step Verlet
candidate are implemented. The position coefficient and ending-radius gravity
calculation have been corrected. `Simulation::step()` still advances only time.

Resume with input/candidate validation, accepted-step state and time updates,
rejection reasons, and terminal-stop handling in the accumulator loop. A valid
request needs a positive finite timestep, finite radius above the cutoff,
finite radial velocity, and finite nonnegative elapsed time. An inward
(negative) velocity is valid. Permanent tests, energy diagnostics, and the
timestep study below remain part of this assignment.

## Model and units

- Track one spherical outer material boundary with fixed enclosed mass. The
  model is pressureless, nonrotating, and Newtonian.
- Use normalized radius `r = R / R0`, time `tau = t / t0`, and radial velocity
  `v = V / v0`, with `t0 = sqrt(R0^3 / (G M))` and `v0 = R0 / t0`.
- Start at `r = 1`, `v = 0`, and `tau = 0`. The acceleration is `a(r) = -1 / r^2`.
- The surface's normalized specific energy is `epsilon = v^2 / 2 - 1 / r`.
  Its initial value is `-1`. This is energy per unit mass for the boundary,
  rather than the total energy of the star.
- Use `r = 0.25` as this assignment's numerical cutoff. Label it as a numerical
  cutoff; its value is not a physical horizon calculation.

## Required behavior

1. Evolve radius and radial velocity using **velocity Verlet** in the existing
   fixed-timestep system. Advance simulation time only for accepted steps.
   Keep the model independent of Macroquad.
2. Stop before accepting a step that reaches or crosses the numerical cutoff.
   Preserve the last accepted state and display a clear stop reason. A stop must
   also end any remaining catch-up updates in that rendering frame.
3. Reject invalid timesteps and invalid model states. Prevent newly accepted
   states with non-finite values or nonpositive radius. Check a proposed radius
   before evaluating gravity there.
4. Keep pause/resume working during valid evolution. Space must not restart a
   terminally stopped simulation. R must restore the complete initial state,
   accumulator, diagnostics, and stop status.
5. Show the contracting star, radius, radial velocity, acceleration, simulation
   time, updates completed in the current frame, specific energy, absolute
   energy error relative to the initial energy, and stop status in the window.

## Evidence required for review

- Initial state: radius `1`, velocity `0`, acceleration `-1`, time `0`, and
  specific energy `-1`.
- From the initial state, one velocity-Verlet update of `0.01` model time units
  produces radius approximately `0.99995` and a negative radial velocity.
- During ordinary evolution, radius decreases and velocity points inward.
  Pause holds state fixed; reset restores the initial values.
- Automated tests cover acceleration scaling, the first integration step,
  reset, cutoff behavior, and rejection of invalid input. Use tolerances for
  approximate floating-point comparisons.
- Compare the absolute energy error at the common model time `tau = 0.5` for
  timesteps `0.01`, `0.005`, and `0.0025`. Record actual results and explain
  whether reducing the timestep improves conservation. Compare at the same
  model time, before the cutoff, rather than after the same number of steps.
- Record the model assumptions, unit conventions, stop behavior, and measured
  convergence results in your project notes or README. Include the unfinished
  unit comments and removal of the old disabled delay block in this assignment.

Keep each intermediate saved version runnable. The final submission should pass
`cargo fmt --check`, `cargo clippy --locked -- -D warnings`,
`cargo test --locked`, and `cargo build --locked`. Demonstrate the result in the
desktop window with `cargo run --locked`.

## Resources

- [GROMACS reference manual: the velocity Verlet integrator](https://manual.gromacs.org/current/reference-manual/algorithms/molecular-dynamics.html#the-velocity-verlet-integrator)
  — focus on the integrator section and its update equations.
- [NASA Basics of Spaceflight glossary](https://science.nasa.gov/learn/basics-of-space-flight/glossary/)
  — the entries for gravity and gravitation give the inverse-square relation.
- [The Rust Book: How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
  — test structure, assertions, and checking expected behavior.

Bring back the running implementation, test results, and the energy comparison
for review. Ask for a targeted hint if your own investigation reaches a blocker.
