use std::fmt;

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

    pub fn is_rule_one(slice: &[bool]) -> bool {
        if !slice[1] {
            return false;
        }
        if slice.len() > 2 {
            for i in 2..slice.len() {
                if slice[i] {
                    return false;
                }
            }
        }
        true
    }

    pub fn new_target(size: usize) -> Vec<bool> {
        let mut vec = vec![false; size];
        if size > 0 {
            vec[0] = true;
        }
        vec
    }

    fn invert_light(lights: &mut[bool], target: &[bool]) -> usize {
        if lights.len() == 1 || Map::is_rule_one(&lights) {
            lights[0] = target[0];
            1 + Map::check_lights(lights, target)
        }
        else {
            let new_target = Map::new_target(lights.len() - 1);
            Map::check_lights(&mut lights[1..], &new_target) + Map::check_lights(lights, target)
        }
    }

    pub fn check_lights(lights: &mut[bool], target: &[bool]) -> usize {
        if lights[0] != target[0] {
            Map::invert_light(lights, target)
        }
        else if lights != target {
            Map::check_lights(&mut lights[1..], &target[1..])
        }
        else {
            0
        }
    }

    pub fn resolve(&mut self) {
        self.count = Map::check_lights(&mut self.lights, &self.target);
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = format!("lights: {:?}\ntarget: {:?}\ncount: {}", self.lights, self.target, self.count);
        write!(f, "{}", result)
    }
}