use serde_json;
use serde_json::Value;

use std::io::prelude::*;
use ggez::graphics::{FilterMode, Image, Vector2};
use ggez::Context;
use game::{ObjectType, PlayerType, TileType};

const ASSETS_PATH: &str = "/config/";

pub fn load_resources(ctx: &mut Context) -> (PlayerType, Vec<ObjectType>, Vec<TileType>) {
    let mut player = None;
    let mut object_types: Vec<ObjectType> = Vec::new();
    let mut tile_types: Vec<TileType> = Vec::new();

    // Searchs for files in the assets folder
    let paths = ctx.filesystem.read_dir(ASSETS_PATH).unwrap();
    for path in paths {
        if let Some(ext) = path.extension() {
            // If the file is a json
            if ext.to_str().unwrap() == "json" {
                let mut json_src: String = "".to_string();
                ctx.filesystem
                    .open(path.clone())
                    .unwrap()
                    .read_to_string(&mut json_src)
                    .expect("Could not open asset file");
                let json: Value = serde_json::from_str(&json_src).unwrap();
                if let Value::String(ref type_name) = json["type"] {
                    match &type_name[..] {
                        "tile" => {
                            let tile = load_tile(&json["properties"], ctx);
                            tile_types.push(tile);
                        }
                        "object" => {
                            let obj = load_object(&json["properties"], ctx);
                            object_types.push(obj);
                        }
                        "player" => {
                            player = Some(load_player(&json["properties"], ctx));
                        }
                        _ => panic!("Unknown object type"),
                    }
                } else {
                    panic!("Type not specified");
                }
            }
        }
    }

    let player = player.expect("A player asset could not be found");
    (player, object_types, tile_types)
}

fn load_tile(json: &Value, ctx: &mut Context) -> TileType {
    let texture = load_texture(ctx, json["texture"].as_str().unwrap());
    let forward_friction = json["forward_friction"].as_f64().unwrap() as f32;
    let sideway_friction = json["sideway_friction"].as_f64().unwrap() as f32;
    let distribution = json["distribution"].as_f64().unwrap() as f32;

    TileType {
        texture,
        forward_friction,
        sideway_friction,
        distribution,
    }
}

fn load_object(json: &Value, ctx: &mut Context) -> ObjectType {
    let texture = load_texture(ctx, json["texture"].as_str().unwrap());
    let distribution = json["distribution"].as_f64().unwrap() as f32;
    let hitbox_json = json["hitbox"].as_object().unwrap();
    let hitbox = Vector2::new(
        hitbox_json["width"].as_f64().unwrap() as f32,
        hitbox_json["height"].as_f64().unwrap() as f32,
    );

    ObjectType {
        distribution,
        hitbox,
        texture,
    }
}

fn load_player(json: &Value, ctx: &mut Context) -> PlayerType {
    let texture = load_texture(ctx, json["texture"].as_str().unwrap());

    PlayerType { texture }
}

fn load_texture(ctx: &mut Context, tex_name: &str) -> Image {
    let path = "/textures/".to_owned() + tex_name;
    let mut image = Image::new(ctx, path).expect("Error loading texture file");
    image.set_filter(FilterMode::Nearest);
    image
}
