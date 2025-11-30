#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub struct FloorRequest {
    floor: u8,
    direction: Direction,
}

impl FloorRequest {

    pub fn new(floor: u8, direction: Direction) -> Self {
        assert!(floor < 4, "Floor is out of range");
        Self { floor, direction }
    }

    // Getter für id
    pub(crate) fn floor(&self) -> u8 { self.floor }

}