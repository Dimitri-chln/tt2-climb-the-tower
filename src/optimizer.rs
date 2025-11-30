use pathfinding::prelude::*;

use crate::error::Error;
use crate::floor::{DOOR_COST, Door, FloorNumber, Floors, START_FLOOR};

#[derive(Debug)]
pub struct Optimizer {
    floors: Floors,
}

impl Optimizer {
    pub fn new(floors: Floors) -> Self {
        Self { floors }
    }

    fn improve_path(&self, path: Vec<FloorNumber>) -> Vec<(FloorNumber, Door)> {
        path.windows(2)
            .map(|window| {
                let floor_number = window[0];
                let next_floor_number = window[1];

                if self.floors[&floor_number].left() == Some(next_floor_number) {
                    return (floor_number, Door::Left);
                }
                if self.floors[&floor_number].middle() == Some(next_floor_number) {
                    return (floor_number, Door::Middle);
                }
                if self.floors[&floor_number].right() == Some(next_floor_number) {
                    return (floor_number, Door::Right);
                }

                unreachable!()
            })
            .collect()
    }

    pub fn reach(
        &self,
        goal: FloorNumber,
        from: Option<FloorNumber>,
    ) -> Result<OptimizerResult, Error> {
        let start_floor = from.unwrap_or(START_FLOOR);

        let (path, cost) = dijkstra(
            &start_floor,
            |floor_number| {
                self.floors[&floor_number]
                    .doors()
                    .into_iter()
                    .filter_map(|door| door.map(|d| (d, DOOR_COST)))
            },
            |floor_number| floor_number == &goal,
        )
        .ok_or(Error::UnreachableFloor)?;

        Ok(OptimizerResult {
            start_floor,
            reached_floor: goal,
            path: self.improve_path(path),
            cost,
        })
    }

    pub fn highest_floor(&self, from: Option<FloorNumber>) -> OptimizerResult {
        let start_floor = from.unwrap_or(START_FLOOR);

        let parents = dijkstra_all(&start_floor, |floor_number| {
            self.floors[&floor_number]
                .doors()
                .into_iter()
                .filter_map(|door| door.map(|d| (d, DOOR_COST)))
        });

        let (highest_floor, (_, cost)) = parents
            .iter()
            .max_by_key(|(floor_number, _)| *floor_number)
            .unwrap_or((&START_FLOOR, &(START_FLOOR, 0)));

        let path = build_path(highest_floor, &parents);

        OptimizerResult {
            start_floor,
            reached_floor: *highest_floor,
            path: self.improve_path(path),
            cost: *cost,
        }
    }
}

#[derive(Debug)]
pub struct OptimizerResult {
    pub start_floor: FloorNumber,
    pub reached_floor: FloorNumber,
    pub path: Vec<(FloorNumber, Door)>,
    pub cost: i32,
}
