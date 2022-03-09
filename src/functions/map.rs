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

    fn is_rule_one(slice: &[bool]) -> bool {
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

    fn new_target(size: usize) -> Vec<bool> {
        let mut vec = vec![false; size];
        if size > 0 {
            vec[0] = true;
        }
        vec
    }

    fn invert_light(lights: &mut [bool], target: &[bool]) -> usize {
        if lights.len() == 1 || Map::is_rule_one(&lights) {
            lights[0] = target[0];
            1 + Map::check_lights(lights, target)
        } else {
            let new_target = Map::new_target(lights.len() - 1);
            Map::check_lights(&mut lights[1..], &new_target) + Map::check_lights(lights, target)
        }
    }

    pub fn check_lights(lights: &mut [bool], target: &[bool]) -> usize {
        if lights[0] != target[0] {
            Map::invert_light(lights, target)
        } else if lights != target {
            Map::check_lights(&mut lights[1..], &target[1..])
        } else {
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
        let result = format!(
            "lights: {:?}\ntarget: {:?}\ncount: {}",
            self.lights, self.target, self.count
        );
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod map {
    use super::*;

    mod resolve {
        use super::*;

        // 1101 to 0100
        #[test]
        fn test_resolve_easy() {
            let mut map = Map::new(
                vec![true, true, false, true],
                vec![false, true, false, false],
            );
            map.resolve();
            assert_eq!(map.count, 2);
        }

        // 101010 to 010101
        #[test]
        fn test_resolve_medium() {
            let mut map = Map::new(
                vec![true, false, true, false, true, false],
                vec![false, true, false, true, false, true],
            );
            map.resolve();
            assert_eq!(map.count, 26);
        }

        // 11001001000 to 10000110011
        #[test]
        fn test_resolve_hard() {
            let mut map = Map::new(
                vec![
                    true, true, false, false, true, false, false, true, false, false, false,
                ],
                vec![
                    true, false, false, false, false, true, true, false, false, true, true,
                ],
            );
            map.resolve();
            assert_eq!(map.count, 877);
        }

        #[test]
        fn test_resolve_same() {
            let mut map = Map::new(
                vec![
                    true, true, false, false, true, false, false, true, false, false, false,
                ],
                vec![
                    true, true, false, false, true, false, false, true, false, false, false,
                ],
            );
            map.resolve();
            assert_eq!(map.count, 0);
        }
    }

    mod new_target {
        use super::*;

        #[test]
        fn new_target_small() {
            assert_eq!(Map::new_target(3), vec![true, false, false]);
        }

        #[test]
        fn new_target_big() {
            assert_eq!(
                Map::new_target(20),
                vec![
                    true, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false
                ]
            );
        }

        #[test]
        fn new_target_empty() {
            assert_eq!(Map::new_target(0), vec![]);
        }
    }

    mod rule_one {
        use super::*;

        #[test]
        fn rule_one_small_wrong_end() {
            assert!(!Map::is_rule_one(&[false, false]));
        }

        #[test]
        fn rule_one_small_right() {
            assert!(Map::is_rule_one(&[false, true]));
        }

        #[test]
        fn rule_one_long_wrong_start() {
            assert!(!Map::is_rule_one(&[true, false, false, false, false]));
        }

        #[test]
        fn rule_one_long_wrong_middle() {
            assert!(!Map::is_rule_one(&[false, true, true, false, false]));
        }

        #[test]
        fn rule_one_long_wrong_end() {
            assert!(!Map::is_rule_one(&[false, true, false, false, true]));
        }

        #[test]
        fn rule_one_long_right() {
            assert!(Map::is_rule_one(&[false, true, false, false, false]));
        }
    }
}
