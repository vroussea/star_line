use std::fmt;

#[derive(Clone)]
pub struct Map {
    pub lights: Vec<bool>,
    pub target: Vec<bool>,
    pub count: usize,
}

impl Map {
    pub fn new(start: Vec<bool>, target: Vec<bool>) -> Map {
        Map {
            lights: start,
            target,
            count: 0,
        }
    }

    fn is_rule_one(index: usize, size: usize) -> bool {
        if index == size - 1 {
            return true;
        }
        false
    }

    fn is_rule_two(slice: &[bool]) -> bool {
        if slice[0] != true {
            return false;
        }
        for i in 1..slice.len() {
            if slice[i] == true {
                return false
            }
        }
        true
    }

    pub fn invert_light(lights: &mut [bool]) -> usize {
        let mut count = 0;
        let result = !lights[0];

        if lights.len() == 1 {
            return 1;
        }
        while result != lights[0] { 
            for i in 0..lights.len() {
                if Map::is_rule_one(i, lights.len()) || Map::is_rule_two(&lights[1..]) {
                    lights[i] = !lights[i];
                    count += 1;
                } else {
                    Map::invert_light(&mut lights[1..]);
                }
            }
        }
        count
    }

    // 1001 0xxx
    // 1011
    // 1010
    // 1110
    // 1111
    // 1101
    // 1100
    // 0100

    // 101 0xx
    // 111
    // 110
    // 010 0x

    // 00 0

    // 0

    pub fn resolve(&mut self) {
        if self.lights[0] != self.target[0] {
            self.count += Map::invert_light(&mut self.lights);
        }
        self.lights.remove(0);
        self.target.remove(0);
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