```
  _____                     ___                             _ _        
 |_   _|__ _ __ _ __ __ _  |_ _|_ __   ___ ___   __ _ _ __ (_) |_ __ _ 
   | |/ _ \ '__| '__/ _` |  | || '_ \ / __/ _ \ / _` | '_ \| | __/ _` |
   | |  __/ |  | | | (_| |  | || | | | (_| (_) | (_| | | | | | || (_| |
   |_|\___|_|  |_|  \__,_| |___|_| |_|\___\___/ \__, |_| |_|_|\__\__,_|
                                                |___/                  
```
##### Embark on a treacherous roguelike journey as you conquer monsters, unearth treasures, and pursue the legendary mystical weapons for ultimate power.

## Table of Contents
- [Setup](#setup)
- [First Steps](#first-steps)
- [RoadMap](#roadmap)
- [Configuration](#configuration)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Setup
This is an early alpha build so building from source is necessary.
```
git clone https://github.com/benlloyd50/terra_incognita.git
cd terra_incognita
cargo run --release
```

## First Steps
Simply running the commands above will put you on to the start screen of the game. From there, arrow keys or vi keys can be used to navigate. Comma and period for stair traversal and ESC to save the game. The last game may be loaded from the main menu.

## RoadMap
[Milestones](./planning/milestones.md)

## Configuration
There is a config.toml in the resources folder containing a few options to play with

- `fullscreen` is recommended and self explanatory
- `dev_mode` gives a few bonuses like being able to see everything and skipping the main menu
- `font_file` can be changed given you put the font in the resources folder
- `font_size` should be updated alongside font_file, it is in pixels
- `world_seed` is enterable here but will be enterable in game in the future

WARNING:
screensize and map sizes are not currently stable to be changed. Be advised when changing these numbers

## License
This project is licensed under the [MIT License](./LICENSE.md).

## Acknowledgments
- Thanks to anyone starring or supporting however
- Thanks to my gf ;3
