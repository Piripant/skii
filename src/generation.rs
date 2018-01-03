use game::*;
use ggez::graphics::Vector2;
use rand;
use rand::Rng;

impl World {
    pub fn generate_clear(&mut self, width: u32, height: u32) {
        self.tiles.clear();
        self.objects.clear();

        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(0);
            }
            self.tiles.push(row);
        }
    }

    pub fn generate_row(&mut self) {
        let mut rng = rand::thread_rng();

        let mut row = Vec::new();
        let y = self.height();
        for x in 0..self.width() {
            // If no tile has been choosen fall back on the most common one
            // (the first in the tiles vector)
            let mut choosen_tile = 0;

            // It tries tile_types.len() times to choose a random tile
            for _ in 0..self.tile_types.len() {
                let id = rng.gen_range(0, self.tile_types.len());
                let mut chance = 1.0 / self.tile_types[id].distribution;

                // The number of tiles of the same type
                let mut neighbors = self.get_close_tiles(x, y);
                neighbors.retain(|&tile_id| tile_id == id);
                let similar = neighbors.len();
                if similar >= 1 {
                    chance = (chance / 2.0).ceil();
                }
                if similar >= 4 {
                    chance *= 5.0;
                }

                if rng.gen_weighted_bool(chance as u32) {
                    choosen_tile = id;
                    break;
                }
            }

            row.push(choosen_tile);
        }

        self.tiles.push(row);
    }

    pub fn generate_objects(&mut self, height: usize) {
        let mut rng = rand::thread_rng();

        for x in 0..self.width() {
            for _ in 0..self.object_types.len() {
                let id = rng.gen_range(0, self.object_types.len());
                let mut chance = 1.0 / self.object_types[id].distribution;

                // x and y are the bottom left coordinates of the tile
                // Adding 0.5 places it in the center of the tile
                let position = Vector2::new(x as f32 + 0.5, height as f32 + 0.5);
                let objects_close = self.objects_in_radius(3.0, position).len();

                if objects_close >= 1 {
                    chance = (chance / 2.0).ceil();
                }
                if objects_close >= 2 {
                    chance *= 6.0;
                }

                if rng.gen_weighted_bool(chance as u32) {
                    let object_id = rng.gen_range(0, self.object_types.len());
                    self.objects.push((object_id, Object::new(position)));
                    break;
                }
            }
        }
    }
}
