# Snake

A terminal based snake game with autopilot.

[![asciicast](https://asciinema.org/a/529541.svg)](https://asciinema.org/a/529541?t=1)

## Rationale

I was looking for a small project to start learning Rust when I stumbled upon [sssnake](https://github.com/AngelJumbo/sssnake) and decided to see if I could implement something similar in Rust.

## Usage

```
Game of snake

Usage: snake [OPTIONS]

Options:
  -i, --interval <INTERVAL>        Snake advance interval in ms [default: 175]
  -w, --grid-width <GRID_WIDTH>    Width of the grid [default: 20]
  -h, --grid-height <GRID_HEIGHT>  Height of the grid [default: 15]
  -f, --fit-grid                   Fit the grid to the screen
  -n, --no-obstacles               Don't draw obstacles on the grid
      --autopilot                  The computer controls the snake
      --arcade                     The snake gets faster with every food eaten
      --help                       Print help information
```

### Keybindings

* _ESC_ or _q_ to quit
* _SPACE_ to pause/resume and restart
* Arrow keys to steer the snake when not in autopilot mode
* _+_/_-_ to increase / decrease speed when not in arcade mode

## Credits

* I've used [AngelJumbos](https://github.com/AngelJumbo) [sssnake](https://github.com/AngelJumbo/sssnake) as inspiration. Definitely check out his version, it has much more features and looks way better. :smile:
* A\* Search Algorithm based on https://www.geeksforgeeks.org/a-search-algorithm/

## TODO

- [ ] countdown before start
- [ ] release binaries
- [ ] multiple snakes
