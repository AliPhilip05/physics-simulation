# Physics Simulation

A guided Rust project exploring a star's gravitational collapse.

## Run

```sh
cd "$HOME/Projects/Physics Simulation"
cargo run
```

The first build needs access to crates.io to download dependencies and generate
`Cargo.lock`. Keep that lockfile with the project; later you can use
`cargo run --locked` for a build using the recorded dependency versions.

A desktop window shows an orange circle
representing the initial star, with `R / R0 = 1.00`. Resize the window to check
that the star stays centered; press Escape or close the window to quit.

Current milestone: **0 — static initial state; no physics evolution yet.**

## Controls

# SPACE BAR

The state of the program _running/paused_ is controlled by the space bar key. The space
bar key will pause the simulation or resume the simulation.

# R

The state of the program is reset by pressing the R key on the keyboard.

main.rs

```
 if is_key_pressed(KeyCode::Space) {
            simulation.paused = !simulation.paused;
        }

```

view.rs

```
 draw_text(
        format!("TIME ELAPSED: {L.2}", simulation.elapsed),
        24.0,
        124.0,
        22.0,
        BLUE,
    );

```

## Project notes — your writing

You own this README. Use [docs/roadmap.md](docs/roadmap.md) for the project plan
and [docs/first-session.md](docs/first-session.md) for your first exercise.
After each milestone, explain the model, controls, expected behavior, and what
you checked. Start by replacing this paragraph with your own project description.

# Main.rs

The main file is responsibile for running and exuciting the files found in the
directory.

# Simulation.rs

The simulation.rs file is responsible for holding the objects needed to run a
simulation.

# View.rs

The view.rs file is responsible for building a visual view of the project through a
terminal prompt.

```
// NOTE: format! builds the text; {:.2} displays the radius to two decimal places.
// The remaining draw_text arguments control its position and appearance:
format!("R / R0 = {:.2}", simulation.star.radius),
24.0,  // Horizontal position in pixels from the left.
100.0, // Text baseline in pixels from the top.
22.0,  // Font size.
GOLD,  // Text color.
```
# physics-simulation
