use std::cell::{Ref, RefCell, RefMut};
use std::slice::Iter;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util_structs::Color;

pub struct Map {
    tiles: Box<RefCell<dyn MapTilesVec>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: Map::create_map(),
        }
    }

    fn create_map() -> Box<RefCell<dyn MapTilesVec>> {
        let map_tiles = RefCell::new(vec![]);
        for y in 0..MAP_HEIGHT {
            for x in 00..MAP_WIDTH {
                map_tiles.borrow_mut().push(MapTile::new(x, y, true, Color::new(85, 125, 70, 255)));
            }
        }
        Box::new(map_tiles)
    }

    pub fn set_tile_occupied(&self, x: i32, y: i32, occupied: bool) {
        let mut tiles = self.borrow_tiles_mut();
        let mut tile = tiles.borrow_tile_mut(x, y);
        tile.occupied = occupied;
    }

    pub fn borrow_tiles(&self) -> Ref<dyn MapTilesVec> {
        self.tiles.borrow()
    }

    pub fn borrow_tiles_mut(&self) -> RefMut<dyn MapTilesVec> {
        self.tiles.borrow_mut()
    }
}

pub trait MapTilesVec {
    fn borrow_tile(&self, x: i32, y: i32) -> &MapTile;
    fn borrow_tile_mut(&mut self, x: i32, y: i32) -> &mut MapTile;
    fn iterator(&self) -> Iter<MapTile>;
}

impl MapTilesVec for Vec<MapTile> {
    fn borrow_tile(&self, x: i32, y: i32) -> &MapTile {
        if x < 0 || y < 0 || x >= MAP_WIDTH || y >= MAP_HEIGHT {
            panic!("Accessing map tiles outside of bounds!");
        }
        self.get((y * MAP_WIDTH + x) as usize).unwrap()
    }

    fn borrow_tile_mut(&mut self, x: i32, y: i32) -> &mut MapTile {
        self.get_mut((y * MAP_WIDTH + x) as usize).unwrap()
    }

    fn iterator(&self) -> Iter<MapTile> {
        return self.iter();
    }
}

#[derive(Debug)]
pub struct MapTile {
    pub x: i32,
    pub y: i32,
    pub walkable: bool,
    pub occupied: bool,
    pub color: Color,
}

impl MapTile {
    fn new(x: i32, y: i32, walkable: bool, color: Color) -> Self {
        Self {
            x,
            y,
            walkable,
            occupied: false,
            color,
        }
    }
}
