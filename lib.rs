pub mod functions;

pub fn run() -> Result<(), functions::errors::CustomError> {
    let mut map: functions::map::Map = functions::parser::parse()?;
    map.resolve();
    println!("{}", map);

    return Ok(());
}