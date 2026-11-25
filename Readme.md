### A simple game of life made with minifb

It's my first real project in Rust, I used  https://blog.adamchalmers.com/grids-1/#defining-the-grid-trait for everything that is related to the grid logic
and https://medium.com/@sunside/the-game-of-life-vs-convolutions-bc495c962de8 to help for the rules and the convolution.

### WebAssembly

The same simulation can run in the browser. The browser build keeps the
desktop's **1000 × 600** grid and colour values, so the output matches the
native `minifb` window. `minifb` creates the canvas inside the
`#gol-container` element, and the loop uses `requestAnimationFrame` instead of
`thread::sleep`. No Wayland/X11 development packages are needed for this
target.

```bash
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --release --out-dir web/pkg
python3 -m http.server 8080 -d web
```

Then open <http://localhost:8080>.

