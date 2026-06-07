# Geometry Dash Clone in Rust 🦀

A high-performance, side-scrolling rhythmic platformer clone inspired by *Geometry Dash*. This project is written entirely in **Rust** using the lightweight and fast **Macroquad** game engine. It features arcade-style physics, custom box hitboxes, a modular architecture, and procedurally generated endless levels.

---

## 🎮 Game Features

- **Endless Procedural Level Generation:** Levels are built dynamically on the fly using a randomized, chunk-based layout algorithm.
- **Optimised Memory Footprint:** Out-of-sight obstacles are automatically purged from system memory as they slide off-screen left.
- **Forgiving Arcade Hitboxes:** Leverages custom-centered inner rectangle boxes for spikes rather than full triangles, mirroring the forgiving design of the official game.
- **Parallax Scrolling:** Dynamic multilayered background elements slide at separate speeds to give an illusion of depth.
- **Zero Coordinate Drift:** Uses a relative pivot matrix offset to ensure the player cube rotates flawlessly without drifting under the floor.

---

## 📂 Project Structure

The codebase is split into modular files to keep engine logic cleanly separated:

```text
geometry-dash/
├── assets/                # Contains converted native PNG game assets
│   ├── bg.png             # Parallax background texture
│   ├── player.png         # Rotated cube skin texture
│   ├── spike_small.png    # Small short hazard texture
│   └── spike_tall.png     # Tall narrow hazard texture
├── src/
│   ├── main.rs            # Game initialization, input, and main frame render loops
│   ├── obstacles.rs       # Struct definitions and constructors for spikes
│   └── lvl_gen.rs         # Randomized chunk-generation algorithms
└── Cargo.toml             # Project manifest and workspace dependencies
```

---

## ⚙️ Requirements & Dependencies

Make sure your `Cargo.toml` contains the following dependencies for physics math, asset handling, and window rendering:

```toml
[dependencies]
macroquad = "0.4"
rand = "0.8"
```

*Note: Macroquad relies on a standard system graphical backend. If you are compilation debugging on Linux (e.g., Kali Linux), ensure you have `libx11-dev` and `libxi-dev` installed.*

---

## 🚀 How to Run

1. Clone or navigate to your local repository directory:
   ```bash
   cd ~/rust/geometry-dash
   ```

2. Make sure your asset image files are in the right root folder location and fully encoded as native standard PNG containers (not WebP chunks).
   
3. Compile and execute the build profile using Cargo:
   ```bash
   cargo run
   ```

---

## 🕹️ Controls

- **[SPACEBAR] or [LEFT MOUSE CLICK]:** Instantly triggers a jump block action when grounded.
- **[SPACEBAR] (On Crash Menu):** Resets character placement coordinates, wipes active obstacles, and restarts the procedural runner loop.
