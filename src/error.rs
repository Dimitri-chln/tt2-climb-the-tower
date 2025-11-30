#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read the CSV file: {0}")]
    Csv(#[from] csv::Error),
    #[error("The chosen floor is unreachable at the moment")]
    UnreachableFloor,
}
