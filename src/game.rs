use loader::load_resources;
use ggez::Context;
use ggez::graphics::{Image, Vector2};

/// Holds the general information about a tile type
/// Eg: snow, ice
pub struct TileType {
    pub forward_friction: f32,
    pub sideway_friction: f32,
    pub distribution: f32,
    pub texture: Image,
}

/// The actual object present in the scene
pub struct Object {
    pub position: Vector2,
    pub rotation: f32,
}

impl Object {
    pub fn new(position: Vector2) -> Object {
        Object {
            position,
            rotation: 0.0,
        }
    }
}

/// Holds all the information about the type of object, indetical for each instance
/// Eg: a Rock places is general information about size and texture here
pub struct ObjectType {
    pub texture: Image,
    pub distribution: f32,
    pub hitbox: Vector2,
}

/// The actual player object present in the scene
pub struct Player {
    pub position: Vector2,
    pub rotation: f32,
    pub velocity: Vector2,
    pub angular_velocity: f32,
}

impl Player {
    pub fn update(&mut self, under_tile: &TileType, dt: f32) {
        // Get the velocity along the sideways vector
        let norm_vector = Vector2::new(-self.rotation.cos(), self.rotation.sin());
        let sideways_velocity = norm_vector * self.velocity.dot(&norm_vector);

        // Apply forward friction
        self.velocity -= self.velocity * under_tile.forward_friction * dt;
        // Apply sideways firction
        self.velocity -= sideways_velocity * under_tile.sideway_friction * dt;
        // Apply sideways friction to angular velocity
        self.angular_velocity -= self.angular_velocity * under_tile.sideway_friction / 2.0 * dt;

        self.position += self.velocity * dt;
        self.rotation += self.angular_velocity * dt;
    }
}

/// Holds all the information about the type of player, indetical for each instance
/// In the future would hold other data about the skies
pub struct PlayerType {
    pub texture: Image,
}

pub struct World {
    // The player is just a normal Object
    pub player: Player,
    pub player_type: PlayerType,
    pub real_y: f32,
    // The usize rappresents the tile id
    pub tiles: Vec<Vec<usize>>,
    pub tile_types: Vec<TileType>,
    // The usize rappresents the object id
    pub objects: Vec<(usize, Object)>,
    pub object_types: Vec<ObjectType>,
}

impl World {
    pub fn new(window: &mut Context) -> World {
        let player = Player {
            position: Vector2::new(0.0, 0.0),
            rotation: 0.0,
            velocity: Vector2::new(0.0, 1.0),
            angular_velocity: 0.0,
        };

        let (player_type, mut object_types, mut tile_types) = load_resources(window);
        // Sort from most common to most common
        tile_types.sort_unstable_by_key(|tile_type| (1.0 / tile_type.distribution) as i32);
        // Sort from most common to most uncommon
        object_types.sort_unstable_by_key(|object_type| (1.0 / object_type.distribution) as i32);

        World {
            player,
            player_type,
            real_y: 0.0,
            tiles: Vec::new(),
            tile_types,
            objects: Vec::new(),
            object_types,
        }
    }

    pub fn reset(&mut self, width: u32, height: u32) {
        self.player.position.x = width as f32 / 2.0;
        self.player.position.y = 0.0;
        self.player.velocity = Vector2::new(0.0, 0.0);
        self.real_y = 0.0;

        self.generate_clear(width, height);
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn objects_in_radius(&self, radius: f32, point: Vector2) -> Vec<usize> {
        let mut obj_in_radius = Vec::new();
        for i in 0..self.objects.len() {
            let object = &self.objects[i].1;
            if (object.position - point).norm() <= radius {
                obj_in_radius.push(i);
            }
        }

        obj_in_radius
    }

    /// Gets the neighbors tiles with a Moore neighborhood
    pub fn get_close_tiles(&self, x: usize, y: usize) -> Vec<usize> {
        let mut tiles = Vec::new();

        let directions = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];

        for &(ref dx, ref dy) in &directions {
            let tile_x = x as i32 + dx;
            let tile_y = y as i32 + dy;
            if tile_x >= 0 && tile_y >= 0 && (tile_x as usize) < self.width()
                && (tile_y as usize) < self.height()
            {
                let tile = self.tiles[tile_y as usize][tile_x as usize];
                tiles.push(tile);
            }
        }

        tiles
    }


    pub fn scroll(&mut self, scrolling: u32) {
        // To zero out the effect of the map scrolling
        // All objects must be reset tiles back
        self.player.position.y -= scrolling as f32;
        self.real_y += scrolling as f32;

        let mut i = 0;
        while i < self.objects.len() {
            self.objects[i].1.position.y -= scrolling as f32;
            if self.objects[i].1.position.y < 0.0 {
                self.objects.remove(i);
            } else {
                i += 1;
            }
        }

        let height = self.height();
        for i in 0..scrolling {
            self.tiles.remove(0);
            self.generate_row();
            self.generate_objects(height - i as usize);
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        let rounded_pos_x = self.player.position.x as usize;
        let rounded_pos_y = self.player.position.y as usize + 1;
        let tile_under = self.tiles[rounded_pos_y][rounded_pos_x];

        self.player.velocity.y += 1.5 * dt;
        self.player.update(&self.tile_types[tile_under], dt);

        self.collided()
    }

    fn collided(&mut self) -> bool {
        if self.player.position.x >= self.width() as f32 || self.player.position.x <= 0.0 {
            return true;
        }

        for &(ref object_id, ref object) in &self.objects {
            let object_type = &self.object_types[*object_id];

            let min = object.position - object_type.hitbox / 2.0;
            let max = object.position + object_type.hitbox / 2.0;

            if self.player.position >= min && self.player.position <= max {
                return true;
            }
        }

        false
    }
}
