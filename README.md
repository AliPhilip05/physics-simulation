# Physics Simulation

A Rust learning project working toward a simulation of a star's gravitational
collapse. The project combines computational physics, numerical methods, and an
interactive desktop view built with Macroquad.

**Current milestone: 1 — state and time, in progress.** Pause/resume, reset,
elapsed-time tracking, and the `Simulation::step()` method are implemented.
Fixed timesteps are the next exercise. The star is currently static; gravity
and collapse will be introduced in later milestones.

The [October 22 session log](docs/sessions/2026-10-22.md) records today's
completed checkpoint and where to resume.

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

The application starts paused, with an orange star, `R / R0 = 1.00`, and an
elapsed-time display of `0.00` seconds. Launching from a terminal opens a
separate desktop window.

## Controls

| Input | Behavior |
| --- | --- |
| Space | Toggle between paused and running. |
| R | Reset the star, elapsed time, and pause status to their defaults. |
| Escape | Close the application. |
| Resize the window | Recalculate the drawing scale while preserving the model radius. |

Reset returns the normalized radius to `1.0`, elapsed time to `0.0`, and
`paused` to `true`. Press Space to start again after resetting.

## How time and rendering work

`main.rs` reads keyboard input and, while running, obtains the frame duration
in seconds from `get_frame_time()`. It converts that `f32` value to `f64` and
passes it to `simulation.step(dt)`.

`Simulation::step()` adds the supplied duration to the stored elapsed time.
The pause check belongs to the caller in `main.rs`; calling `step()` directly
still advances time. Pausing preserves the accumulated total, while resetting
replaces the entire simulation with `Simulation::default()`.

The displayed time is rounded to two decimal places. The stored value retains
its `f64` precision. This is currently an elapsed-time clock driven by variable
frame durations; a fixed physics timestep has not yet been introduced.

The star's radius is normalized as `R / R0`, where `R0` is its initial radius.
`view.rs` converts that model value into pixels using the window dimensions.
Resizing changes the drawing scale, rather than the simulated radius.

For text labels, `format!` builds the text and `{:.2}` displays two decimal
places:

```rust
format!("TIME ELAPSED IN SECONDS: {:.2}", simulation.elapsed)
```

The remaining `draw_text` arguments set horizontal position, vertical text
baseline, font size, and color.

## Project structure

| File | Responsibility |
| --- | --- |
| [src/main.rs](src/main.rs) | Configure the window and coordinate input, timing, simulation updates, and drawing. |
| [src/simulation.rs](src/simulation.rs) | Define `Star` and `Simulation`, their default states, and the state update method. |
| [src/view.rs](src/view.rs) | Draw the current state in the desktop window and convert model values into screen coordinates. |
| [docs/first-session.md](docs/first-session.md) | First-session exercises, acceptance criteria, and comprehension checks. |
| [docs/first-session-answers.md](docs/first-session-answers.md) | Answer sheet for the comprehension checks and study questions. |
| [docs/sessions/2026-10-22.md](docs/sessions/2026-10-22.md) | Completed work, verification, and the next session's starting point. |
| [docs/roadmap.md](docs/roadmap.md) | The multiweek plan, scientific assumptions, and references. |

## Check a change

```sh
cargo fmt --check
cargo clippy --locked -- -D warnings
cargo build --locked
```

Run `cargo fmt` to apply standard Rust formatting when the formatting check
reports differences.

In the window, verify that time starts at zero, advances while running,
freezes while paused, and continues from its previous value after resuming.
Check reset both while running and while paused, then verify that Escape
closes the application.

## Learning approach

Work proceeds through small, runnable checkpoints. I write the implementation
and explain the results; my AI mentor reviews the code, checks behavior, and
asks comprehension questions before we introduce the next major concept.

The next steps are a fixed timestep and accumulator, followed by radial
gravity, spherical mass shells, numerical validation, and an idealized
relativistic collapse model. See the [roadmap](docs/roadmap.md) for the scope
and assumptions of each milestone.
