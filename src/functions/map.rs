use std::fmt;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Map {
    pub lights: VecDeque<usize>,
    pub target: VecDeque<usize>,
    pub count: usize,
}

impl Map {
    pub fn new(start: Vec<usize>, target: Vec<usize>) -> Map {
        Map {
            lights: VecDeque::from(start),
            target: VecDeque::from(target),
            count: 0,
        }
    }

    pub fn invert_light(lights: &mut VecDeque<usize>) -> usize {
        let count = 0;

        count
    }

    pub fn resolve(&mut self) {
        if self.lights.get(0) != self.target.get(0) {
            Map::invert_light(&mut self.lights);
        }
        else {
            self.lights.pop_front();
            self.target.pop_front();
        }
        if self.lights.len() > 0 {
            self.resolve();
        } 
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