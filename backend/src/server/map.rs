use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use crate::server::vector::Vector;


#[derive(Debug, Clone)]
pub enum TileType {
    Air,
    Wall,
}

impl TileType {
    fn from_str(string: &str) -> Result<Self, String> {
        use TileType::*;
        match string {
            "air" => Ok(Air),
            "wall" => Ok(Wall),
            _ => Err(format!("{} is not a valid tile type", string))
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
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut lines = BufReader::new(file).lines();

        let width: u32 = lines.next().ok_or("Invalid map")??.parse()?;
        let height: u32 = lines.next().ok_or("Invalid map")??.parse()?;
        let num_tiletypes: u32 = lines.next().ok_or("Invalid map")??.parse()?;

        let mut tiletype_map = HashMap::new();
        for _ in 0..num_tiletypes {
            let line = lines.next().ok_or("Invalid map")??;
            let mut chars = line.chars();
            let key = chars.next().ok_or("Invalid map")?;
            chars.next();
            let type_str: String = chars.collect();
            let tiletype = TileType::from_str(&type_str)?;
            tiletype_map.insert(key, tiletype);
        }

        let mut tiles = Vec::new();
        for (y, line) in lines.enumerate() {
            let line = line?;
            let mut row = Vec::new();
            let chars = line.chars();
            for (x, ch) in chars.enumerate() {
                row.push(
                    Tile::new(
                        tiletype_map.get(&ch).ok_or("Invalid map")?.clone(),
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
