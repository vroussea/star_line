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

    pub fn invert_light(lights: &mut[bool], target: &[bool]) -> usize {
        if lights.len() == 0 {
            return 0;
        }
        if lights[0] != target[0] {
            if lights.len() == 1 || Map::is_rule_one(&lights) {
                lights[0] = target[0];
                return 1 + Map::invert_light(lights, target);
            }
            else {
                let new_target = Map::new_target(lights.len() - 1);
                return Map::invert_light(&mut lights[1..], &new_target) + Map::invert_light(lights, target);
            }
        }
        else if lights != target {
            return Map::invert_light(&mut lights[1..], &target[1..]);
        }
        else {
            return 0;
        }
            // + Map::invert_light(lights, target);
    }

    pub fn resolve(&mut self) {
        self.count = Map::invert_light(&mut self.lights, &self.target);
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