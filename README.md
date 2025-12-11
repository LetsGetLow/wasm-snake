# WASM Snake Game

A simple Snake game implemented in WebAssembly (WASM) with a bit of TypeScript for painting the canvas, calculation of
delta time and handling of keyboard events. The game logic is written in Rust and compiled to WASM. This project serves
as a
demonstration of using Rust and WebAssembly for web-based games. It is meant for testing performance of WASM to see what
is
possible without WegGL or wgpu.

I used the project to test performance of WASM in the browser without WebGL or wgpu in order to see if it is possible to
create
real-time games with complex logic running in WASM in order to create a Doom clone. It turns out that it is indeed
possible to
achieve high frame rates with WASM even without hardware acceleration for rendering. Especially Chrome seems to be very
well
optimized for WASM performance.

The game runs at a solid 150+ FPS in Chrome on a 2000x2000 px canvas.

## Features

- Classic Snake game mechanics with a twist you can go to the edge of the screen and appear
  on the other side if there is no wall.
- Smooth rendering using HTML5 Canvas
- Responsive controls with keyboard input
- Delta time calculation for consistent movement speed
- High performance with WebAssembly
- Shared memory between Rust and TypeScript for efficient canvas rendering

## How to run the project

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install)
   and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed.
2. Make sure you have bun installed. You can get it from [here](https://bun.sh/).
3. Make sure wasm-pack is installed. You can install it via cargo:
   ```bash
   cargo install wasm-pack
    ```
4. Clone this repository.
5. Navigate to the project directory and into the frontend folder.
4. Install dependencies using bun:
   ```bash
   bun install
   ```
5. Build the project:
   You have two options to build the project:

   run the dev environment with hot reloading
   ```bash
   bun run dev
   ```
   or just build to run a static server (need to run on a server to load WASM properly):
   ```bash
   bun run build
   ```

## Performance issues on Firefox

Firefox has performance issues when resizing the canvas to 2000x2000px I could notice a significant
drop in the frame rate while Chrome was stable at 150fps and more. What I could find out is that Firefox
2D canvas buffer uploading seems not as optimized as in Chrome.

You can try the following settings in the `about:config` page to improve performance on Firefox. But don't expect
the same performance as in Chrome.
```ini
layers.acceleration.force-enabled = true
gfx.webrender.all = true
```