use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use crate::game::vector::Vector;
use crate::error::*;


#[derive(Debug, Clone)]
pub enum TileType {
    Air,
    Wall,
}

impl TileType {
    fn from_str(string: &str) -> Result<Self, RCE> {
        use TileType::*;
        match string {
            "air" => Ok(Air),
            "wall" => Ok(Wall),
            _ => Err(RCE::BadTileType)
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
    location: Vector,
}

impl Tile {
    fn new(tile_type: TileType, x: u32, y: u32) -> Self {
        Self {
            tile_type,
            location: Vector {
                x: x as f64,
                y: y as f64,
            }
        }
    }

    pub fn tile_type(&self) -> &TileType {
        &self.tile_type
    }

    pub fn location(&self) -> &Vector {
        &self.location
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

        let file = File::open(file_path).to(E)?;
        let mut lines = BufReader::new(file).lines();

        let width: u32 = lines.next().to(E)?.to(E)?.parse().to(E)?;
        let height: u32 = lines.next().to(E)?.to(E)?.parse().to(E)?;
        let num_tiletypes: u32 = lines.next().to(E)?.to(E)?.parse().to(E)?;

        let mut tiletype_map = HashMap::new();
        for _ in 0..num_tiletypes {
            let line = lines.next().to(E)?.to(E)?;
            let mut chars = line.chars();
            let key = chars.next().to(E)?;
            chars.next();
            let type_str: String = chars.collect();
            let tiletype = TileType::from_str(&type_str)?;
            tiletype_map.insert(key, tiletype);
        }

        let mut tiles = Vec::new();
        for (y, line) in lines.enumerate() {
            let line = line.to(E)?;
            let mut row = Vec::new();
            let chars = line.chars();
            for (x, ch) in chars.enumerate() {
                row.push(
                    Tile::new(
                        tiletype_map.get(&ch).to(E)?.clone(),
                        x as u32,
                        y as u32
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
