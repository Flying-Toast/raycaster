use std::collections::HashMap;
use std::str::{FromStr, Lines};
use crate::error::*;
use crate::protocol::payload::{Pieces, PayloadBuilder, Encodable, Decodable};
use proc::EnumFromStr;

#[derive(Debug, Clone, EnumFromStr)]
pub enum TileType {
    Air,
    Wall,
}

#[derive(Debug, Clone)]
pub struct Tile {
    tile_type: TileType,
}

impl Tile {
    fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
    string: String,
}

trait LinesExt {
    fn parse_next_line<T: FromStr>(&mut self) -> Option<T>;
}

impl LinesExt for Lines<'_> {
    fn parse_next_line<T: FromStr>(&mut self) -> Option<T> {
        match self.next() {
            Some(line) => line.parse().ok(),
            _ => None,
        }
    }
}

impl Map {
    /// An empty map
    pub fn dummy() -> Self {
        Self {
            string: String::new(),
            width: 0,
            height: 0,
            tiles: Vec::new(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }

    pub fn from_string(string: String) -> Result<Self, CME> {
        use CME::BadMapFormat as BMF;

        let mut lines = string.lines();
        let mut line_num = 0;

        line_num += 1;
        let width: usize = lines.parse_next_line().to(BMF{line_num})?;
        line_num += 1;
        let height: usize = lines.parse_next_line().to(BMF{line_num})?;
        line_num += 1;
        if width < 3 || height < 3 {
            return Err(CME::MapTooSmall);
        }
        let num_tiletypes: u32 = lines.parse_next_line().to(BMF{line_num})?;

        let mut tiletype_map = HashMap::new();
        for _ in 0..num_tiletypes {
            line_num += 1;
            let line = lines.next().to(BMF{line_num})?;
            let mut chars = line.chars();
            let key = chars.next().to(BMF{line_num})?;
            if chars.next() != Some('=') {
                return Err(BMF{line_num});
            }
            let type_string: String = chars.collect();
            let tiletype = TileType::from_str(&type_string).to(CME::BadTileType{type_string})?;
            tiletype_map.insert(key, tiletype);
        }

        let mut tiles = Vec::new();
        for line in lines {
            line_num += 1;
            let mut row = Vec::new();
            let chars = line.chars();
            for ch in chars {
                row.push(
                    Tile::new(
                        tiletype_map.get(&ch).to(BMF{line_num})?.clone()
                    )
                );
            }
            if row.len() != width {
                return Err(BMF{line_num});
            }
            if !matches!(row[0].tile_type, TileType::Wall) || !matches!(row.last().unwrap().tile_type, TileType::Wall) {
                return Err(CME::NonClosedMap);
            }
            tiles.push(row);
        }
        if tiles.len() != height {
            return Err(BMF{line_num});
        }

        for tile in tiles[0].iter()
            .chain(tiles[tiles.len() - 1].iter())
        {
            if !matches!(tile.tile_type, TileType::Wall) {
                return Err(CME::NonClosedMap);
            }
        }

        Ok(Map {
            width,
            height,
            tiles,
            string,
        })
    }
}

impl Encodable for &Map {
    fn encode_to(self, builder: &mut PayloadBuilder) {
        builder.add(self.as_str());
    }
}

impl Decodable for Map {
    fn decode_from(pieces: &mut Pieces) -> Result<Self, CME> {
        Map::from_string(pieces.get()?)
    }
}
