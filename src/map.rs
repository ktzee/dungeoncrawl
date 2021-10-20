use crate::prelude::*;

// NOTE: constants can only include other constants (including constant functions)
// NOTE: usize is a special type. It uses the preferred bit size for the CPU (64bits on x64)
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// NOTE: since tiles have a pre-defined set of tile-types, enum is great here
// NOTE: we derive the Copy, Clone and PartialEq functions. 
#[derive(Copy,Clone,PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>
}
impl Map {
    pub fn new() -> Self {
        Self {
            // NOTE: vec! is a macro that lets you initialize a vector like you would an array
            // NOTE: we create NUM_TILES entries of TileType::Floor
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        // NOTE: iterating through y first is faster because of memory cache, since we're using row-first
        for y in camera.top_y .. camera.bottom_y {
            for x in camera.left_x .. camera.right_x {
                if self.in_bounds(Point::new(x,y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.')
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#')
                            );
                        }
                    }
                }
            }
        }
    }

    // NOTE: returns TRUE if all && conditions are met
    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
            && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point : Point) -> bool {
        self.in_bounds(point)
            && self.tiles[map_idx(point.x, point.y)]==TileType::Floor
    }

    pub fn try_idx(&self, point : Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

