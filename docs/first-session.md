# First session: state before physics

## First, establish the baseline

Run `cargo run` from this folder. The first build requires crates.io access and
creates `Cargo.lock`; retain it and use `--locked` for subsequent verification.
Verify the star is centered, the
display reads `R / R0 = 1.00`, resizing preserves its proportions, and Escape
closes the window. There is intentionally no collapse yet.

Read the three Rust source files. Be ready to explain:

1. Which file owns the star's radius?
2. Where is that radius converted into pixels?
3. Why does the application call `next_frame().await` in a loop?
4. Why do we keep model values in `f64` and drawing values in `f32`?

## Your first implementation

Add pause/resume, reset, and an elapsed-time display. Keep the star static while
you establish control over the application's state. Do not implement gravity in
this exercise.

Suggested interface, for you to implement:

```rust
pub struct Simulation {
    pub star: Star,
    pub elapsed: f64,
    pub paused: bool,
}
```

Decide how to create a fresh `Simulation` using `Default`. Start paused. Space
toggles pause; R restores the whole initial state, including the initial radius,
zero elapsed time, and paused status. The main loop handles keyboard input;
the view displays the state and lists the new controls.

For this first exercise, use `get_frame_time()` as the elapsed wall-clock
increment when running, converting it to `f64`. This is a clock exercise, not
yet a physics integrator. The next exercise replaces variable physics updates
with a fixed timestep and an accumulator. When reset is pressed, skip the time
increment for that frame so that the displayed initial state is exact.

Hints to investigate: `KeyCode::Space`, `KeyCode::R`, `is_key_pressed`, and mutable
bindings. The starter already demonstrates Escape handling.

## Acceptance criteria

- The initial display says paused and shows zero elapsed time.
- Space starts the clock; pressing it again freezes the clock.
- Repeated pause/resume cycles preserve the accumulated elapsed time.
- R resets everything, including after a running session.
- The window still resizes correctly and Escape still closes it.
- The simulation module does not import Macroquad.
- `cargo fmt --check` and `cargo clippy --locked -- -D warnings` pass.

## Your README update

Write a short project description and document the current controls. Explain
what the clock measures and why the star does not move yet. Record what you
observed when you checked pause and reset.

When finished, ask me to review these changes. I will review your implementation
and your explanation before we introduce fixed timesteps and gravity.
