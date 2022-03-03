use std::fmt;
use crate::functions::errors::CustomError;

#[derive(Clone)]
pub struct Map {
    pub lights: Vec<usize>,
    pub target: Vec<usize>,
    pub count: usize,
}

impl Map {
    pub fn new(start: Vec<usize>, target: Vec<usize>) -> Map {
        // let lights = Vec::new();
        // let target = Vec::new();
        // for light in start_str.chars() {
        //     lights.push(light.to_digit(RADIX).ok_or(CustomError::from("not a digit")));
        // }
        // start_str.chars().try_for_each(|light| -> Result<(), CustomError> {
            
        //     Ok(())
        // }) ;
        //target_str.chars().for_each(|light| target.push(light));
        Map {
            lights: start,
            target,
            count: 0,
        }
    }

    pub fn resolve(&mut self) -> &Self {

        return self;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for light in &self.lights {
            result.push_str(&light.to_string());
        }
        result.push('\n');
        write!(f, "{}", result)
    }
}