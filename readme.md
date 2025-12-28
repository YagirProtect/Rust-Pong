# Mega Pong (Rust + minifb)

A small Pong game written in Rust **without a full game engine**: pixel framebuffer rendering via **minifb**, a tiny scene/entity setup, simple AABB collisions, a basic bot, and bitmap UI text via **font8x8**.

This project is primarily a practical playground to learn Rust ownership/borrowing in a real-time game loop while keeping the codebase small and hackable.

---

## Screenshots

<p float="left">
  <img src="/screenshots/sc_1.png" width="49%" />
  <img src="/screenshots/sc_2.png" width="49%" />
</p>

---

## Features

- Window + pixel framebuffer rendering (**minifb**)
- `Canvas` (clear/background)
- Primitives:
  - `Rectangle` drawing from center coordinates
  - AABB bounds + clipping
- Game entities:
  - `Player`
  - `Ball`
  - `PlayerBot` (AI)
- Collisions:
  - AABB intersection check
  - collision normal + penetration
  - collision solver
- Text/UI:
  - `TextLabel` rendered with **font8x8** (8×8) with scaling (`scale`)
  - `ButtonLabel` rendered with **font8x8** (8×8) with scaling (`scale`)
- Scenes:
  - `GameScene` holding `Vec<Box<dyn Entity>>`
  - `WorldContext` for shared cross-entity data (score/UI signals/etc.)

---

## Tech Stack / Dependencies

- `minifb` — window + framebuffer
- `vek` — vectors (`Vec2`)
- `rand` — randomness (e.g., initial ball direction)
- `serde`, `serde_json` — config
- `font8x8` — 8×8 bitmap font
- `lerp` — interpolation helpers (where needed)
- `rodio` — for beeps

---

## Build & Run

### Requirements
- Rust (stable recommended)
- Cargo

### Run (debug)
```bash
cargo run
