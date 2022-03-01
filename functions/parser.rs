use crate::functions::errors;
use std::io;
use crate::functions::map::{Map, Cell};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>()?)
}

pub fn parse() -> Result<Map, errors::CustomError> {
    let (width, height) = read_size()?;
    let mut map = Map::new(height, width);
    for _ in 0..height {
        map.cells.push(read_line()?);
    }
    
    return Ok(map);
}


fn read_input() -> Result<String, errors::CustomError>{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    return Ok(input_line);
}

fn read_line() -> Result<Vec<Cell>, errors::CustomError>{
    let line = read_input()?.trim_end().to_string();
    return Ok(convert_to_cells(&line));
}

fn read_size() -> Result<(u8, u8), errors::CustomError>{
    let input_line = read_input()?;
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let width = parse_input!(inputs[0], u8);
    let height = parse_input!(inputs[1], u8);
    if width < 3 {
        return Err(errors::CustomError::from("Witdh must be >= 3"));
    }
    if height > 100 {
        return Err(errors::CustomError::from("Height must be <= 100"));    
    }
    return Ok((width, height));
}

pub fn convert_to_cells(line: &str) -> Vec<Cell>{
    let mut converted_line: Vec<Cell> = Vec::new();

    let line = line.replace(" ", "");
    let line: Vec<char> = line.chars().collect();
    for i in 0..line.len() {
        if line[i] != '-' {
            let mut left = false;
            let mut right = false;
            if i > 0 && line[i - 1] == '-' {
                left = true;
            }
            if i + 1 < line.len() && line[i + 1] == '-' {
                right = true;
            }
            converted_line.push(Cell::new((left, right), line[i]));
        } 
    }

    return converted_line;
}
