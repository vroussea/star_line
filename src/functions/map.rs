use std::fmt;

#[derive(Clone)]
pub struct Map {
    pub lights: Vec<usize>,
    pub target: Vec<usize>,
    pub count: usize,
}

impl Map {
    pub fn new(start: String, target: String) -> Map {
        let lights = Vec::new();
        start.chars().for_each(|light| lights.push(light));
        return Map {
            lights: Vec::new(),
            target: Vec::new(),
            count: 0,
        };
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
        for light in self.lights {
            result.push_str(&light.to_string());
        }
        result.push('\n');
        write!(f, "{}", result)
    }
}