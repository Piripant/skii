# Skii
Skii is a 2D top-down skiing game with a procedurally generated landscape and fully json constumizable assets.

## Building
Skii uses cargo, so compiling only consists in writing:

`cargo build`

If you want to directly play the game, you can also use:

`cargo run --release`

## Dependecies
Skii has only one non-cargo handled dependecy, SDL, derived from ggez. To find instruction on how to install it, you may want to read [this.](https://github.com/Rust-SDL2/rust-sdl2#user-content-requirements)

## Modding
The game reads json file in `resources/config` to find info about the tiles, objects and player.

There are three kinds of descriptions files, distiguisced by the `type` property: tiles, objects, and players.

### Tile
* `type`: the file type
* `properties`: all the tile properties
    * `texture`: the tile texture, found in `resources/textures`
    * `forward_friction`: the forward friction with the skies
    * `sideway_friction`: the sideways friction with the skies
    * `distribution`: the base chance of generating the tile

```json
{
    "type": "tile",
    "properties": {
        "texture": "deep_snow.png",
        "forward_friction": 0.2,
        "sideway_friction": 30.0,
        "distribution": 0.05
    }
}
```

### Object
* `type`: the file type
* `properties`: all the object properties
    * `texture`: the object texture, found in `resources/textures`
    * `distribution`: the base chance of generating the object
    * `hitbox`: the object hitbox
        * `width`: the hitbox width
        * `height`: the hitbox height
```json
{
    "type": "object",
    "properties": {
        "texture": "tree1.png",
        "distribution": 0.06,
        "hitbox": {
            "width": 1.0,
            "height": 1.0
        }
    }
}
```

### Player
* `type`: the file type
* `properties`: all the object properties
    * `texture`: the player texture
```json
{
    "type": "player",
    "properties": {
        "texture": "player.png"
    }
}
```

## Generation
The generation algorithms, (found in `src/generation.rs`) are cellular automata inspired, and modify the generation chance starting from the distrubution value in the json files.

### Tiles
```
if identical neighbors is between 1 and 3 => generating chance *= 2
if identical neighbors is more than 3 => generating change /= 5
```
### Objects
```
if objects in 3.0 radius are between 1 and 2 => generating chance *= 2
if objects in 3.0 radius are more than 2 => generating change /= 6
```
Note that the objects in range do not need to be the same as the object we are trying to generate