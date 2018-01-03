extern crate ggez;
extern crate skii;

use std::{env, path};

use ggez::ContextBuilder;
use ggez::conf;
use ggez::event;

fn main() {
    let mut cb = ContextBuilder::new("Skii", "Piripant")
        .window_setup(conf::WindowSetup::default().title("Skii"))
        .window_mode(conf::WindowMode::default().dimensions(720, 720));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources/");
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();
    let state = &mut skii::renderer::ViewState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
