use std::path::PathBuf;

use clap::Parser;

use crate::floor::FloorNumber;

#[derive(Parser)]
pub struct Command {
    pub csv: PathBuf,
    #[clap(long = "start", short = 's')]
    pub start_floor: Option<FloorNumber>,
    #[clap(long = "goal", short = 'g')]
    pub goal_floor: Option<FloorNumber>,
}
