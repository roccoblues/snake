# Snake

A terminal based snake game with autopilot.

[![asciicast](https://asciinema.org/a/527684.svg)](https://asciinema.org/a/527684?t=5)

## Rationale

I was looking for a small project to start learning Rust when I stumbled upon [sssnake](https://github.com/AngelJumbo/sssnake) and decided to see if I could implement someting similar in Rust.

## Usage

```
Game of snake

Usage: snake [OPTIONS]

Options:
  -i, --interval <INTERVAL>    Snake advance interval in ms [default: 150]
  -g, --grid-size <GRID_SIZE>  Width and height of the grid [default: 20]
  -a, --autopilot              Autopilot mode
  -h, --help                   Print help information
~
```

### Keybindings

* _ESC_ or _q_ to quit
* _SPACE_ to pause
* Arrow keys to steer the snake when not in autopilot mode

## Credits

* I've used [AngelJumbos](https://github.com/AngelJumbo) [sssnake](https://github.com/AngelJumbo/sssnake) as inspiration. Definitely check out his version, it has much more features and looks way better. :smile:
* A\* Search Algorithm based on https://www.geeksforgeeks.org/a-search-algorithm/

## TODO

- [ ] option to restart without exiting
- [ ] separate grid width / height
- [ ] wander around longest path if no direct one can be found (https://github.com/chuyangliu/snake/blob/master/docs/algorithms.md#longest-path)
- [ ] don't redraw the complete screen on every step
- [ ] fix error handling mess
- [ ] release binaries
- [ ] document code
- [ ] add tests :D
- [ ] increase or decrease the speed with + and - in autopilot