use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::EnumString;
use crate::error::*;
use crate::game::vector::Vector;


#[derive(Debug, Clone, EnumString)]
pub enum TileType {
    Air,
    Wall,
}

#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
}

impl Tile {
    fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
        }
    }

    pub fn tile_type(&self) -> &TileType {
        &self.tile_type
    }
}

#[derive(Debug)]
pub struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn from_file(file_path: &str) -> Result<Self, RCE> {
        const E: RCE = RCE::BadMapFormat;

        let file = File::open(file_path).to(RCE::MapFileRead)?;
        let mut lines = BufReader::new(file).lines();

        let width: u32 = lines.next().to(E)?.to(E)?.parse().to(E)?;
        let height: u32 = lines.next().to(E)?.to(E)?.parse().to(E)?;
        let num_tiletypes: u32 = lines.next().to(E)?.to(E)?.parse().to(E)?;

        let mut tiletype_map = HashMap::new();
        for _ in 0..num_tiletypes {
            let line = lines.next().to(E)?.to(E)?;
            let mut chars = line.chars();
            let key = chars.next().to(E)?;
            if chars.next() != Some('=') {
                return Err(E);
            }
            let type_str: String = chars.collect();
            let tiletype = TileType::from_str(&type_str).to(RCE::BadTileType)?;
            tiletype_map.insert(key, tiletype);
        }

        let mut tiles = Vec::new();
        for line in lines {
            let line = line.to(E)?;
            let mut row = Vec::new();
            let chars = line.chars();
            for ch in chars {
                row.push(
                    Tile::new(
                        tiletype_map.get(&ch).to(E)?.clone()
                    )
                );
            }
            tiles.push(row);
        }

        Ok(Map {
            width,
            height,
            tiles,
        })
    }

    /// Returns the tile that the given vector is in.
    pub fn get_tile(&self, v: Vector) -> Option<&Tile> {
        self.tiles.get(v.y as usize)?.get(v.x as usize)
    }
}
