# Physics Simulation

A Rust learning project working toward a simulation of a star's gravitational
collapse. The project combines computational physics, numerical methods, and an
interactive desktop view built with Macroquad.

**Milestone 1 — state and time: complete. Milestone 2 — radial motion: in progress.**
The application has pause/resume, reset, fixed timesteps, an accumulator, and
normalized radius, velocity, and acceleration readouts. The standalone
velocity-Verlet candidate calculation is implemented and checked against
one-step reference values. Normal and deliberately slower rendering have also
been compared, with results recorded below.

The visible star is still static: `Simulation::step()` currently advances only
time. Validating and accepting candidate states, reporting stop reasons, and
wiring motion into the application are the next exercise. The numerical cutoff
is specified but not yet enforced. Energy diagnostics and convergence studies
remain pending.

The [October 22 session log](docs/sessions/2026-10-22.md) records the initial
state and clock exercises. The [October 23 session log](docs/sessions/2026-10-23.md)
records fixed updates, the rendering-rate comparison, gravity, velocity Verlet,
and the next session's starting point.

## Run

With Rust and Cargo installed, run from the project directory:

```sh
cargo run --locked
```

For the local checkout used during development:

```sh
cd "$HOME/Projects/Physics Simulation"
cargo run --locked
```

The first build needs internet access to download dependencies. Keep
`Cargo.lock` with the project so subsequent builds use the recorded dependency
versions. A desktop session is required to open the window.

The application starts paused, with an orange star, radius `1.00`, radial
velocity `0.00`, radial acceleration `-1.00`, normalized simulation time `0.00`,
and `UPDATES THIS FRAME: 0`. Launching from a terminal opens a separate desktop
window.

## Controls

| Input | Behavior |
| --- | --- |
| Space | Toggle between paused and running. |
| R | Restore the initial star and clock, pause the application, and clear the accumulator. |
| Escape | Close the application. |
| Resize the window | Recalculate the drawing scale while preserving the model radius. |

Reset returns the normalized radius to `1.0`, radial velocity to `0.0`, elapsed
time to `0.0`, and `paused` to `true`. It also clears pending time in the
accumulator; the frame counter shows zero. Acceleration is calculated from the
radius, so it returns to `-1.0`. Press Space to start again after resetting.

## How time and rendering work

`main.rs` defines a fixed timestep of `FIXED_DT = 0.01` normalized model time
units. While running, it obtains the rendering frame's duration in real seconds
from `get_frame_time()`, converts that `f32` value to `f64`, and adds it to an
accumulator. At normal playback, one real running second contributes one model
time unit.

The accumulator stores time that has passed but has not yet been simulated:

1. Add the measured frame duration.
2. While at least `FIXED_DT` remains, call `simulation.step(FIXED_DT)` and
   subtract `FIXED_DT` from the accumulator.
3. Preserve the leftover time for the next frame.
4. Draw the updated state once, then await the next rendering frame.

The accumulator is initialized outside the application loop so it persists
between frames. A rendering frame can contain zero, one, or several simulation
updates, but every update advances simulated time by `0.01` model units. For
example, about `0.035` accumulated model units allow three updates and leave
about `0.005` units for later.

`Simulation::step()` adds the supplied duration to the stored elapsed time.
The pause check belongs to the caller in `main.rs`; calling `step()` directly
still advances time. While paused, the loop neither adds frame time nor consumes
the accumulator. Pausing preserves elapsed time and any leftover pending time.
Reset replaces the simulation with `Simulation::default()` and separately sets
the accumulator to zero.

The displayed clock represents `t / t0`, accumulated in fixed steps. Its numeric
value should approximately follow real running seconds at normal playback;
time spent paused is excluded. A physical duration requires choosing the scales
defined below. The stored value uses `f64`, and the display rounds it to two
decimal places. A fixed timestep establishes the update schedule; the accuracy
of gravitational trajectories still requires separate validation.

`steps_this_frame` is initialized to zero inside each iteration of the outer
application loop and incremented once per simulation update. It is passed to
`view::draw()` for display. It counts updates in the current rendering frame,
rather than total updates or frames per second. It shows zero while paused;
while running, zero is also valid when not enough time has accumulated for a step.

The star's radius is normalized as `R / R0`, where `R0` is its initial radius.
`view.rs` converts that model value into pixels using the window dimensions.
Resizing changes the drawing scale, rather than the simulated radius.

For text labels, `format!` builds the text and `{:.2}` displays two decimal
places:

```rust
format!("SIMULATION TIME (T / T0) {:.2}", simulation.elapsed)
```

The remaining `draw_text` arguments set horizontal position, vertical text
baseline, font size, and color.

## Project structure

| File | Responsibility |
| --- | --- |
| [src/main.rs](src/main.rs) | Configure the window; handle input, the accumulator, fixed updates, and the frame counter; request drawing. |
| [src/simulation.rs](src/simulation.rs) | Define state and defaults, radial acceleration, the unchecked Verlet candidate, and the current clock-only simulation update. |
| [src/view.rs](src/view.rs) | Draw radius, velocity, acceleration, time, and the update counter; convert model values into screen coordinates. |
| [docs/first-session.md](docs/first-session.md) | First-session exercises, acceptance criteria, and comprehension checks. |
| [docs/first-session-answers.md](docs/first-session-answers.md) | Answer sheet for the comprehension checks and study questions. |
| [docs/sessions/2026-10-22.md](docs/sessions/2026-10-22.md) | Historical record of the initial state and clock checkpoint. |
| [docs/sessions/2026-10-23.md](docs/sessions/2026-10-23.md) | Timing validation, the first physics calculations, and the end-of-session handoff. |
| [docs/assignments/02-radial-collapse.md](docs/assignments/02-radial-collapse.md) | Requirements and resources for motion, stopping behavior, diagnostics, and numerical validation. |
| [AGENTS.md](AGENTS.md) | Mentoring agreement and the progression toward independent implementation. |
| [docs/roadmap.md](docs/roadmap.md) | The multiweek plan, scientific assumptions, and references. |

## Check a change

```sh
cargo fmt --check
cargo clippy --locked -- -D warnings
cargo test --locked
cargo build --locked
```

Run `cargo fmt` to apply standard Rust formatting when the formatting check
reports differences.

The repository's test module is currently a placeholder with no executable
tests. A successful `cargo test` run with zero tests is not numerical coverage.
Session review checked the actual model methods with temporary reference tests;
permanent acceptance, rejection, reset, and convergence tests are part of the
next assignment. The candidate integrator and its factory have narrowly scoped
`dead_code` allowances while intentionally unwired; remove them once simulation
acceptance uses those functions.

In the window, verify that time starts at zero, advances while running,
freezes while paused, and continues from its previous value after resuming.
Check that the update counter shows zero while paused and counts individual
simulation steps while running. Check reset both while running and while paused:
the clock and counter should return to zero, with the application paused. Verify
that resizing preserves the model radius and Escape closes the application.

The October 23 review of the accumulator and counter implementation passed
formatting, Clippy, and build checks. Desktop-window review confirmed one and
two updates per frame while running, zero updates while paused, and zero time
and updates after reset.

## Rendering-rate comparison

To repeat the rendering-rate comparison manually:

1. Run normally and compare the displayed clock with roughly 10 seconds of
   running time on a stopwatch.
2. Temporarily enable this line immediately before `view::draw(...)` in
   `main.rs`, then repeat the comparison:

   ```rust
   std::thread::sleep(std::time::Duration::from_millis(50));
   ```

3. Observe the update counter and clock, then disable the delay again.

This is a **50-millisecond delay (0.05 seconds)** per rendering frame. It slows
the application loop without changing `FIXED_DT`. The expected result is more
simulation updates per rendering frame while the clock still approximately
tracks running time. Exact counts depend on the actual frame durations.

The October 23 automated comparison used a temporary measurement build of the
current accumulator loop. It started running automatically and sampled elapsed
simulation time after at least 10 seconds, using `std::time::Instant` as an
independent wall clock. Keyboard pause/reset handling was omitted in that
measurement build so the run remained continuous. The project source files
were unchanged by the experiment.

| Rendering mode | Added delay per frame | Wall time (s) | Model time (t / t0) | Frames | Typical updates per frame |
| --- | --- | --- | --- | --- | --- |
| Normal | None | 10.004 | 10.000 | 601 | Mostly 1–2 |
| Delayed | 50 ms | 10.026 | 10.040 | 183 | Mostly 5–6 |

Every simulation update still used `FIXED_DT = 0.01`. With one model unit per
real running second, both runs passed the comparison's 0.1 tolerance between
the model-time count and real seconds. These
are one local run per mode; exact frame counts and timing vary between runs.
The results verify basic scheduling under these rendering conditions. Numerical
accuracy of the future gravitational model will require its own validation.
Keep the artificial delay disabled for normal use.

## Current physics checkpoint

The Newtonian model uses normalized radius `R / R0` and time
`t / t0`, with `t0 = sqrt(R0^3 / (G M))`. Here `R0` is the initial radius and
`M` is the fixed enclosed mass. The velocity scale is `v0 = R0 / t0`, so the
velocity field represents radial velocity divided by `v0`: negative is inward,
positive is outward. This choice makes the normalized gravitational parameter
equal to one.

The window labels time and radial velocity with these normalized scales. The
playback convention maps one real running second to one model time unit; no
physical duration in seconds can be assigned
until physical values for `R0` and `M` are chosen.

`Star::radial_acceleration()` implements `a = -1 / r^2` for the normalized outer
material boundary enclosing fixed mass. The model assumes spherical symmetry,
pressureless matter, and radial motion. `Star::velocity_verlet(dt)` calculates
a new candidate `Star` without changing the original. Its position update uses
the starting acceleration, and its velocity update averages the starting and
ending accelerations.

Reference checks cover acceleration at radii `1`, `0.5`, `2`, and `0.25`.
From radius `1` and velocity `0`, one candidate step of `0.01` should produce
radius `0.99995` and velocity approximately `-0.0100005000375`.

The candidate method is currently unchecked: it does not reject invalid input
or a cutoff-crossing radius before evaluating ending gravity. It is deliberately
not called from the running simulation yet.

## Resume next session

Continue [Assignment 2](docs/assignments/02-radial-collapse.md) with guarded step
acceptance. A successful step must update the star and advance time together;
a rejected step must preserve both and report a stop reason. Validate positive,
finite timesteps and valid starting state, and check the proposed radius before
evaluating gravity there.

Stop before accepting a radius at or below the numerical cutoff `0.25`. End
the frame's remaining accumulator updates on a terminal stop, count accepted
updates only, prevent Space from restarting the stopped simulation, and make
R restore all state. Then add permanent tests, specific-energy diagnostics,
and the common-time timestep comparison required by the assignment.

## Learning approach

Work proceeds through runnable checkpoints that grow in scope as I gain
familiarity. I research the references, choose interfaces, implement, and explain
the evidence. My AI mentor supplies requirements and resources, reviews the
results, and provides targeted help when requested. Implementation guidance
decreases toward requirements and references only in the final stage.

The next stages are a validated radial trajectory, spherical mass shells,
further numerical validation, and an idealized relativistic collapse model. See the
[roadmap](docs/roadmap.md) for the scope and assumptions of each milestone.
