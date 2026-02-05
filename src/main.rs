use clap::Parser;
use tt2_climb_the_tower::cli::Command;
use tt2_climb_the_tower::error::Error;
use tt2_climb_the_tower::floor::Floors;
use tt2_climb_the_tower::optimizer::{Optimizer, OptimizerResult};

fn main() {
    let command = Command::parse();

    if let Err(error) = optimize(command) {
        eprintln!("An error occurred: {error}");
    }
}

fn optimize(command: Command) -> Result<(), Error> {
    let floors = Floors::from_csv(command.csv)?;
    let optimizer = Optimizer::new(floors);

    let OptimizerResult {
        start_floor,
        reached_floor,
        path,
        cost,
    } = match command.goal_floor {
        Some(goal_floor) => optimizer.reach(goal_floor, command.start_floor)?,
        None => optimizer.highest_floor(command.start_floor),
    };

    let max_digits = reached_floor.checked_ilog10().unwrap_or(0) as usize + 1;

    println!("Reached floor {reached_floor} with a cost of {cost}.",);
    println!("Optimal path from floor {start_floor} to floor {reached_floor}:",);

    for (floor_number, door) in path {
        println!(
            "Floor {floor_number:0>width$} > {door:?} door",
            width = max_digits
        );
    }

    Ok(())
}
