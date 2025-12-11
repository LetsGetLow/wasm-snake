# WASM Snake Game
A simple Snake game implemented in WebAssembly (WASM) with a bit of TypeScript for painting the canvas, calculation of 
delta time and handling of keyboard events. The game logic is written in Rust and compiled to WASM. This project serves as a
demonstration of using Rust and WebAssembly for web-based games. It is meant for testing performance of WASM to see what is 
possible without WegGL or wgpu.

### Performance issues on Firefox

For some reason, Firefox has performance issues when resizing the canvas to 2000x2000 px I could notice a significant
drop in the frame rate while Chrome was stable at 150fps and more. 

You can try the following settings in the `about:config` page to improve performance on Firefox.
But it may not work for everyone:
```ini
layers.acceleration.force-enabled = true
gfx.webrender.all = true
```