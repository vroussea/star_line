pub mod functions;

pub fn run() -> Result<(), functions::errors::CustomError> {
    let map = functions::map::Map::new();
    return Ok(());
}