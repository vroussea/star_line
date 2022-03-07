use std::fmt;

#[derive(Clone)]
pub struct Map {
    pub lights: Vec<usize>,
    pub target: Vec<usize>,
    pub count: usize,
}

impl Map {
    pub fn new(start: Vec<usize>, target: Vec<usize>) -> Map {
        Map {
            lights: start,
            target,
            count: 0,
        }
    }

    pub fn resolve(&mut self) {
        
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