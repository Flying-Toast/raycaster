use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TileType {
    Air,
    Wall,
}

impl TileType {
    pub fn from_str(string: &str) -> Result<Self, String> {
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
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
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
        for l in lines {
            let line = l?;
            let mut row = Vec::new();
            let chars = line.chars();
            for c in chars {
                row.push(
                    Tile::new(
                        tiletype_map.get(&c).ok_or("Invalid map")?.clone()
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
}
