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
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
}