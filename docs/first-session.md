# First session: state before physics

Build a runnable desktop application with a static star, pause/resume, reset,
and an elapsed-time clock. This session establishes control over state and
time before introducing gravity.

## Establish the baseline

Run `cargo run --locked` from the project directory. Verify that the star is
centered, the radius reads `R / R0 = 1.00`, resizing preserves its proportions,
and Escape closes the window. The project includes `Cargo.lock`; the first
build needs internet access to download its dependencies.

## Implementation exercises

### State and defaults

Keep the existing `Star` type and introduce a `Simulation` containing a star,
elapsed time as `f64`, and a paused flag. Implement `Default` so a fresh
simulation contains the default star, zero elapsed time, and `paused = true`.

### Input and the elapsed-time clock

Use Space to toggle pause/resume. Use R to replace the simulation with its
default state. Handle reset after the Space handler and before the time
update, so reset wins if both keys are pressed in the same frame. Escape
closes the application.

While running, obtain the frame duration using `get_frame_time()`, convert it
to `f64`, and accumulate elapsed time. Pausing preserves the previous total.
Display the time in seconds with two decimal places and list the controls in
the window.

This is a clock exercise using variable frame durations. Fixed timesteps and
an accumulator belong to the next exercise.

### The simulation update method

Add an ordinary `impl Simulation` block, separate from its `Default` trait
implementation. Give it a method with this signature:

```rust
pub fn step(&mut self, dt: f64)
```

Implement its body so that it adds `dt` to `self.elapsed`. In `main.rs`, keep
the paused check and frame-time conversion, then call this method instead of
changing `elapsed` directly. Keep Macroquad imports out of `simulation.rs`.

## Acceptance criteria

- The initial display says paused and shows zero elapsed time.
- Space starts the clock; pressing it again freezes the clock.
- Pause/resume cycles preserve accumulated running time.
- R restores the initial star, zero time, and paused status from either state.
- Space starts the clock again after a reset.
- Resizing works and Escape closes the application.
- `view.rs` reads the state; `Simulation::step()` advances it.
- The simulation module does not import Macroquad.
- `cargo fmt --check` and `cargo clippy --locked -- -D warnings` pass.

## Comprehension checks

These collect the unique checks from our first session, including the original
baseline questions. Repeated checks are grouped together. Answer in your own
words before opening the [answer sheet](first-session-answers.md).

1. Where is the star's radius stored, and which source file defines that state?
2. Where is the model radius converted into pixels? What happens to the model
   radius when the window is resized?
3. Why does the application call `next_frame().await` inside its loop?
4. Why do we keep model state in `f64` and convert drawing values to `f32`?
5. Why would adding `1.0` every frame give the wrong elapsed time in seconds?
6. If `simulation.elapsed += clock` became `simulation.elapsed = clock`, what
   would the displayed value represent while running?
7. Elapsed time is already `4.0` seconds. You pause for 10 seconds, then resume
   for a frame lasting `0.02` seconds. With the original `+=` update, what is
   the new elapsed time?
8. In your own words, what does `+=` do with the value already stored in
   `elapsed`?
9. If the app renders at 30 FPS on one computer and 120 FPS on another, should
   this clock advance faster on the second computer? Explain using frame time.
10. If `get_frame_time()` already returns seconds, what does
    `f64::from(get_frame_time())` do?
11. Why does `Simulation::default()` restore the star's radius even though the
    R handler never mentions `radius`?
12. If `paused` is `true`, but some other code directly calls
    `simulation.step(0.1)`, does elapsed time change? Which part of the code
    controls whether the normal application loop calls `step()`?

## Study questions from our discussion

These additional questions capture the Rust and design topics we discussed.
Their answers continue the same numbering in the answer sheet.

13. Why do we call `Star::default()` rather than `star.default()`? What is the
    difference between `Self` and `self`?
14. How does `impl Default for Simulation` differ from `impl Simulation`?
    How do the calls to `default()` and `step()` differ?
15. Why does `pub struct Simulation` not automatically expose its fields?
    When is `pub(crate)` sufficient for this project?
16. Why did we start with a basic elapsed-time counter instead of a separate
    `Clock` struct? When might a separate type become useful?
17. Why are separate `if` statements appropriate for these keyboard controls?
    When would `match` be a useful choice?
18. What does `format!` control in a text label, what does `{:.2}` mean, and
    which arguments control the label's position and appearance?

## Your README update

Describe the current controls and checkpoint. Explain what the clock counts,
why the star is static, and what you observed while testing pause, resume,
and reset. Describe `main.rs` as the application loop, `simulation.rs` as
state and updates, and `view.rs` as drawing in the desktop window.

Ask for a code review and discuss your comprehension answers before moving to
fixed timesteps. Use the [answer sheet](first-session-answers.md) to check and
correct your reasoning after attempting each question.
