use std::time::Duration;
use game::World;

use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::{Point2, Vector2};
use ggez::event::{Keycode, Mod};
use ggez::timer;

pub struct ViewState {
    pub world: World,
    pub dead: bool,

    pub window_size: (u32, u32),
    pub offset_y: f32,
    pub scale: f32,

    pub pressed_keys: Vec<Keycode>,

    pub font: graphics::Font,
}

impl ViewState {
    pub fn new(ctx: &mut Context) -> GameResult<ViewState> {
        let window_size = (ctx.conf.window_mode.width, ctx.conf.window_mode.height);

        graphics::set_background_color(ctx, graphics::WHITE);

        let mut world = World::new(ctx);
        world.reset(7, 16);

        let font = graphics::Font::new(ctx, "/fonts/DejaVuSerif.ttf", 16)?;

        Ok(ViewState {
            world,
            dead: false,
            window_size,
            offset_y: 2.0,
            scale: 5.0,
            pressed_keys: Vec::new(),
            font,
        })
    }

    /// Returns a point from world space to screen space
    pub fn get_screen_point(&self, point: &Vector2) -> Point2 {
        Point2::new(
            (point.x - self.world.width() as f32 / 2.0) * self.scale * 16.0
                + self.window_size.0 as f32 / 2.0,
            (-point.y + self.world.player.position.y - self.offset_y) * self.scale * 16.0
                + self.window_size.1 as f32,
        )
    }


    /// Draws a text with the currently loaded font
    fn draw_text(
        &mut self,
        ctx: &mut Context,
        content: &str,
        position: Point2,
        color: graphics::Color,
    ) -> GameResult<()> {
        let text = graphics::Text::new(ctx, content, &self.font)?;

        let position = graphics::Point2::new(
            position.x - text.width() as f32 / 2.0,
            position.y - text.height() as f32 / 2.0,
        );

        graphics::draw_ex(
            ctx,
            &text,
            graphics::DrawParam {
                dest: position,
                color: Some(color),
                ..Default::default()
            },
        )?;

        Ok(())
    }

    /// Draws the screen ui for when the player is alive
    fn draw_alive_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let content = format!(
            "{:.1} meters",
            self.world.real_y + self.world.player.position.y
        );
        let position = graphics::Point2::new(self.window_size.0 as f32 / 2.0, 15.0);
        self.draw_text(ctx, &content, position, graphics::BLACK)?;

        Ok(())
    }

    /// Draws the screen ui for when the player is dead
    fn draw_dead_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let content = format!(
            "You crashed after {:.2} meters! How unfortunate!",
            self.world.real_y + self.world.player.position.y
        );
        let position = graphics::Point2::new(
            self.window_size.0 as f32 / 2.0,
            self.window_size.1 as f32 / 2.0,
        );
        self.draw_text(ctx, &content, position, graphics::BLACK)?;

        let content = "ProTip: There is no need to hurry. Take it slowly.";
        let position = graphics::Point2::new(
            self.window_size.0 as f32 / 2.0,
            self.window_size.1 as f32 / 2.0 + 30.0,
        );
        self.draw_text(ctx, content, position, graphics::BLACK)?;

        let content = "Press Enter to restart";
        let position = graphics::Point2::new(
            self.window_size.0 as f32 / 2.0,
            self.window_size.1 as f32 / 2.0 + 60.0,
        );
        self.draw_text(ctx, content, position, graphics::BLACK)?;

        Ok(())
    }
}

impl event::EventHandler for ViewState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = to_seconds(&timer::get_delta(ctx));

            let mut steering_direction = 0.0;
            if self.pressed_keys.contains(&Keycode::Right) {
                steering_direction += 1.0;
            }
            if self.pressed_keys.contains(&Keycode::Left) {
                steering_direction -= 1.0;
            }

            // While turning the player slowly decreses the turning speed
            // When switching turning directions the player steers faster
            self.world.player.angular_velocity +=
                (steering_direction * 15.0 - self.world.player.angular_velocity * 0.2) * dt as f32;

            // Generate a new portion of map
            if self.world.player.position.y > 6.0 {
                self.world.scroll((6.0 - self.offset_y) as u32);
            }

            if !self.dead {
                self.dead = self.world.update(dt as f32);
            }
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if self.dead {
            // Restart the game
            if let Keycode::Return = keycode {
                self.dead = false;
                let width = self.world.width() as u32;
                let height = self.world.height() as u32;
                self.world.reset(width, height);
            }
        }

        if !repeat {
            // Add the key to the pressed keys
            self.pressed_keys.push(keycode);
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            // Remove the key from the pressed keys
            for i in 0..self.pressed_keys.len() {
                if self.pressed_keys[i] == keycode {
                    self.pressed_keys.remove(i);
                    break;
                }
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Draw the tiles
        for x in 0..self.world.width() {
            for y in 0..self.world.height() {
                let tile_id = self.world.tiles[y][x];
                let tile_type = &self.world.tile_types[tile_id];
                let position = self.get_screen_point(&Vector2::new(x as f32, y as f32));
                graphics::draw_ex(
                    ctx,
                    &tile_type.texture,
                    graphics::DrawParam {
                        dest: position,
                        scale: Point2::new(self.scale, self.scale),
                        ..Default::default()
                    },
                )?;
            }
        }

        // Draw the objects
        for &(ref object_id, ref object) in &self.world.objects {
            let object_type = &self.world.object_types[*object_id];
            let position = self.get_screen_point(&object.position);
            let width = object_type.texture.width();
            let height = object_type.texture.height();
            graphics::draw_ex(
                ctx,
                &object_type.texture,
                graphics::DrawParam {
                    dest: position,
                    scale: Point2::new(self.scale, self.scale),
                    rotation: object.rotation as f32,
                    offset: Point2::new(width as f32 / 32.0, height as f32 / 32.0),
                    ..Default::default()
                },
            )?;
        }

        // Draw the player
        let position = self.get_screen_point(&self.world.player.position);
        let width = self.world.player_type.texture.width();
        let height = self.world.player_type.texture.height();
        graphics::draw_ex(
            ctx,
            &self.world.player_type.texture,
            graphics::DrawParam {
                dest: position,
                scale: Point2::new(self.scale, self.scale),
                rotation: self.world.player.rotation as f32,
                offset: Point2::new(width as f32 / 32.0, height as f32 / 32.0),
                ..Default::default()
            },
        )?;

        if self.dead {
            self.draw_dead_ui(ctx)?;
        } else {
            self.draw_alive_ui(ctx)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

fn to_seconds(elapsed: &Duration) -> f64 {
    elapsed.as_secs() as f64 + f64::from(elapsed.subsec_nanos()) * 1e-9
}
