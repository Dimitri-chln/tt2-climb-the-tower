use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;

use serde::Deserialize;

pub const START_FLOOR: u32 = 1;
pub const DOOR_COST: i32 = 1;

pub type FloorNumber = u32;

#[derive(Debug)]
pub enum Door {
    Left,
    Middle,
    Right,
}

#[derive(Deserialize, Debug)]
pub struct Floor {
    #[serde(alias = "Floor")]
    number: FloorNumber,
    #[serde(alias = "Left")]
    left: Option<FloorNumber>,
    #[serde(alias = "Middle")]
    middle: Option<FloorNumber>,
    #[serde(alias = "Right")]
    right: Option<FloorNumber>,
}

impl Floor {
    pub fn left(&self) -> Option<FloorNumber> {
        self.left
    }

    pub fn middle(&self) -> Option<FloorNumber> {
        self.middle
    }

    pub fn right(&self) -> Option<FloorNumber> {
        self.right
    }

    pub fn doors(&self) -> [Option<FloorNumber>; 3] {
        [self.left, self.middle, self.right]
    }
}

#[derive(Debug)]
pub struct Floors(HashMap<FloorNumber, Floor>);

impl Deref for Floors {
    type Target = HashMap<FloorNumber, Floor>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Floors {
    pub fn from_csv<P: AsRef<Path>>(path: P) -> csv::Result<Self> {
        let mut reader = csv::Reader::from_path(path)?;
        let mut floors = HashMap::new();

        for result in reader.deserialize() {
            let floor: Floor = result?;
            floors.insert(floor.number, floor);
        }

        Ok(Floors(floors))
    }
}
