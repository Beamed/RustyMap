pub mod coordinate;
pub mod tile;
pub mod terrain;

use map::coordinate::Coordinate;

//THIS WILL BE MOVED
pub fn get_coordinate(x: i32, y: i32) -> Result<coordinate::Coordinate, String> {
    Ok(Coordinate {x: x, y: y})
}