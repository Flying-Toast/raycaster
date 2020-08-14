use std::io::{BufRead, BufReader, Lines};
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::EnumString;
use rand::{seq::IteratorRandom, thread_rng};
use crate::error::*;
use common::vector::Vector;


#[derive(Debug, Clone, EnumString)]
pub enum TileType {
    Air,
    Wall,
    SpawnPoint,
}

#[derive(Debug)]
pub struct Tile {
    tile_type: TileType,
    location: Vector,
}

impl Tile {
    fn new(tile_type: TileType, location: Vector) -> Self {
        Self {
            tile_type,
            location,
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
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

trait LinesExt {
    fn parse_next_line<T: FromStr>(&mut self) -> Option<T>;
}
impl LinesExt for Lines<BufReader<File>> {
    fn parse_next_line<T: FromStr>(&mut self) -> Option<T> {
        match self.next() {
            Some(Ok(line)) => line.parse().ok(),
            _ => None,
        }
    }
}

impl Map {
    pub fn from_file(file_path: &str) -> Result<Self, BKE> {
        use BKE::BadMapFormat as BMF;

        let file = File::open(file_path).to(BKE::MapFileRead)?;
        let mut lines = BufReader::new(file).lines();
        let mut line_num = 0;

        line_num += 1;
        let width: usize = lines.parse_next_line().to(BMF{line_num})?;
        line_num += 1;
        let height: usize = lines.parse_next_line().to(BMF{line_num})?;
        line_num += 1;
        let num_tiletypes: u32 = lines.parse_next_line().to(BMF{line_num})?;

        let mut tiletype_map = HashMap::new();
        for _ in 0..num_tiletypes {
            line_num += 1;
            let line = lines.next().to(BMF{line_num})?.to(BMF{line_num})?;
            let mut chars = line.chars();
            let key = chars.next().to(BMF{line_num})?;
            if chars.next() != Some('=') {
                return Err(BMF{line_num});
            }
            let type_string: String = chars.collect();
            let tiletype = TileType::from_str(&type_string).to(BKE::BadTileType{type_string})?;
            tiletype_map.insert(key, tiletype);
        }

        let mut tiles = Vec::new();
        for (y, line) in lines.enumerate() {
            line_num += 1;
            let line = line.to(BMF{line_num})?;
            let mut row = Vec::new();
            let chars = line.chars();
            for (x, ch) in chars.enumerate() {
                row.push(
                    Tile::new(
                        tiletype_map.get(&ch).to(BMF{line_num})?.clone(),
                        Vector::new(x as f32, y as f32)
                    )
                );
            }
            if row.len() != width {
                return Err(BMF{line_num});
            }
            tiles.push(row);
        }
        if tiles.len() != height {
            return Err(BMF{line_num});
        }

        Ok(Map {
            width,
            height,
            tiles,
        })
    }

    pub fn find_spawnpoint(&self) -> Vector {
        let (x, y) =
            self.tiles
                .iter()
                .flatten()
                .filter(|tile| matches!(tile.tile_type(), TileType::SpawnPoint))
                .map(|tile| (tile.location().x, tile.location().y))
                .choose(&mut thread_rng())
                .unwrap_or((0.0, 0.0));

        Vector::new(x + 0.5, y + 0.5)
    }

    /// Returns the tile that the given vector is in.
    pub fn get_tile(&self, v: &Vector) -> Option<&Tile> {
        self.tiles.get(v.y as usize)?.get(v.x as usize)
    }
}
