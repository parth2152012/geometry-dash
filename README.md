# Geometry Dash Clone in Rust 🦀

A high-performance, side-scrolling rhythmic platformer clone inspired by *Geometry Dash*. This project is written entirely in **Rust** using the lightweight and fast **Macroquad** game engine.

---

## 🎮 Game Features

- **Endless Procedural Level Generation:** Levels are built dynamically on the fly using a randomized, chunk-based layout algorithm.
- **Optimised Memory Footprint:** Out-of-sight obstacles are automatically purged from system memory as they slide off-screen left.
- **Forgiving Arcade Hitboxes:** Leverages custom-centered inner rectangle boxes for spikes rather than full triangles, mirroring the forgiving design of the official game.
- **Parallax Scrolling:** Dynamic multilayered background elements slide at separate speeds to give an illusion of depth.
- **Zero Coordinate Drift:** Uses a relative pivot matrix offset to ensure the player cube rotates flawlessly without drifting under the floor.
- **Smooth Physics:** Implements realistic gravity, collision detection, and platform mechanics for responsive gameplay.
- **UI Framework:** Clean menu system with start screen and game over states.

---

## 📂 Project Structure

The codebase is split into modular files to keep engine logic cleanly separated:

```text
geometry-dash/
├── assets/                # Contains converted native PNG game assets
│   ├── bg.png             # Parallax background texture
│   ├── logo.png           # Title/logo graphic for start screen
│   ├── play_btn.png       # Play button image asset
│   ├── player.png         # Rotated cube skin texture
│   ├── spike_small.png    # Small short hazard texture
│   └── spike_tall.png     # Tall narrow hazard texture
├── src/
│   ├── main.rs            # Game initialization, input, and main frame render loops
│   ├── assets.rs          # Asset loading and texture management
│   ├── obstacles.rs       # Struct definitions and constructors for spikes
│   ├── player.rs          # Player physics, collision, and rendering
│   ├── lvl_gen.rs         # Randomized chunk-generation algorithms
│   ├── state.rs           # Game state enumeration
│   └── ui.rs              # UI rendering and menu system
└── Cargo.toml             # Project manifest and workspace dependencies
```

---

## ⚙️ Requirements & Dependencies

Make sure your `Cargo.toml` contains the following dependencies for physics math, asset handling, and window rendering:

```toml
[package]
name = "geometry-dash"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4.15"
rand = "0.8"
```

*Note: Macroquad relies on a standard system graphical backend. If you are compilation debugging on Linux (e.g., Kali Linux), ensure you have `libx11-dev` and `libxi-dev` installed.*

---

## 🚀 How to Run

1. Clone or navigate to your local repository directory:
   ```bash
   cd ~/rust/geometry-dash
   ```

2. Make sure your asset image files are in the `assets/` folder and are fully encoded as native standard PNG containers (not WebP chunks).
   
3. Compile and execute the build profile using Cargo:
   ```bash
   cargo run
   ```

---

## 🕹️ Controls

- **[SPACEBAR] or [LEFT MOUSE CLICK]:** Instantly triggers a jump block action when grounded.
- **[SPACEBAR] (On Crash Menu):** Resets character placement coordinates, wipes active obstacles, and restarts the procedural runner loop.
- **[ESC]:** Returns to the start screen from the game.

---

## 🐛 Bug Fixes & Updates

### Fixed Issues:
- ✅ **Cargo.toml Edition:** Corrected from invalid `"2024"` to `"2021"` 
- ✅ **Rand Crate Imports:** Fixed incorrect `RngExt` import to correct `Rng` trait
- ✅ **Random Number Generation:** Changed `rand::rng()` to `rand::thread_rng()` for proper initialization
- ✅ **RNG Methods:** Changed `random_range()` to `gen_range()` and `random_bool()` to `gen_bool()`
- ✅ **Rand Version:** Downgraded from `0.10.1` to `0.8` for API compatibility
- ✅ **Code Cleanup:** Removed outdated comments and improved code clarity

---

## 📦 Building & Distribution

To build an optimized release binary:

```bash
cargo build --release
```

The compiled executable will be in `target/release/geometry-dash`.

---

## 🎨 Customization

- **Game Speed:** Adjust the obstacle movement speed in `main.rs` (line 77): `spike.x -= 6.0;`
- **Spawn Rate:** Modify the gap range in `lvl_gen.rs` to change difficulty: `rng.gen_range(200.0..400.0)`
- **Physics:** Tweak gravity and jump force values in `player.rs` for different feel
- **Visual Settings:** Adjust sprite sizes and filter modes in `assets.rs`

---

## 🤝 Contributing

Feel free to fork this project, submit issues, and create pull requests for improvements!

---

## 📝 License

This project is open source and available for educational purposes.

---

## ✨ Acknowledgments

- Inspired by the original **Geometry Dash** by RobTop Games
- Built with **Macroquad** - a minimalist game engine for Rust
- Powered by **Rust** - a systems programming language focused on safety and performance
