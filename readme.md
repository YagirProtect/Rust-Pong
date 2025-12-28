# PONG (Rust + minifb)

A small Pong game written in Rust **without a full game engine**: pixel framebuffer rendering via **minifb**, a tiny scene/entity setup, simple AABB collisions, a basic bot, and bitmap text via **font8x8**.

This project is mainly a hands-on learning sandbox: understanding Rust ownership/borrowing in a real loop, building a render pipeline, update loop, collisions, and minimal UI.

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
  - AABB intersection check (`check_intersection`)
  - collision normal + penetration (`aabb_normal_and_penetration`)
  - collision solver (`solve_collisions`)
- Text:
  - `TextLabel` rendered with **font8x8** (8×8) with scaling (`scale`)
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

---

## Build & Run

### Requirements
- Rust (stable recommended)
- Cargo

### Run (debug)
```bash
cargo run
