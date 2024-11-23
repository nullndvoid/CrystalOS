use hashbrown::HashMap;

pub struct Map {
    tiles: HashMap<(i32, i32), Tile>
}

pub enum Tile {
    Empty,
    Wall,
}