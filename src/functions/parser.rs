use crate::functions::errors;
use std::io;
use crate::functions::map::Map;

pub fn into_vec(line: String) -> Result<Vec<bool>, errors::CustomError>{
    let mut vec = Vec::new();
    for c in line.trim().chars() {
        match c {
            '1' => vec.push(true),
            '0' => vec.push(false),
            _ => return Err(errors::CustomError::from("Only digits accepted: 1 or 0")),
        }
    }
    if line.len() < 1 || line.len() > 25 {
        return Err(errors::CustomError::from("Strings size (N) should be 1 <= N <= 25"));
    }
    Ok(vec)
}

pub fn parse() -> Result<Map, errors::CustomError> {
    let start = read_line()?;
    let target = read_line()?;
    if start.len() != target.len() {
        return Err(errors::CustomError::from("Both strings should be same size"));
    }
    Ok(Map::new(into_vec(start)?, into_vec(target)?))
}


fn read_line() -> Result<String, errors::CustomError>{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    Ok(input_line.trim_end().to_string())
}