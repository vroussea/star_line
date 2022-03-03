use crate::functions::errors;
use std::io;
use crate::functions::map::Map;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.parse::<$t>()?)
}

pub fn into_vec(line: String) -> Result<Vec<usize>, errors::CustomError>{
    let mut vec = Vec::new();
    for i in line.trim().bytes() {
        vec.push(parse_input!(i.to_string(), usize));
    }
    Ok(vec)
}

pub fn parse() -> Result<Map, errors::CustomError> {
    let start = read_line()?;
    let target = read_line()?;

    match start.len() {
        len if len != target.len() => return Err(errors::CustomError::from("Both strings should be same size")),
        len if len < 1 || len > 25  => return Err(errors::CustomError::from("Strings N size should be 1 <= N <= 25")),
        _ => return Ok(Map::new(into_vec(start)?, into_vec(target)?)),
    }
}


fn read_line() -> Result<String, errors::CustomError>{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    Ok(input_line.trim_end().to_string())
}