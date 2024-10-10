# rust-sdl2-fractals

## Usage

``` bash
nix run github:johannesloetzsch/rust-sdl2-fractals
## or
cargo run --release
```

### [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set)
![mandelbrot](./examples/mandelbrot.gif?raw=true)
<!-- convert -delay 20 -loop 0 examples/m{1..6}.png examples/mandelbrot.gif -->
![mandelbrot](./examples/mandelbrot.png?raw=true)
![mandelbrot](./examples/mandelbrot_state.png?raw=true)
``` bash
cargo run --bin mandelbrot
```

### [Koch Snowflake](https://en.wikipedia.org/wiki/Koch_snowflake)
![snowflake](./examples/snowflake.png?raw=true)
``` bash
cargo run --bin snowflake
```
