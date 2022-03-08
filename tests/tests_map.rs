#[cfg(test)]
mod tests_map {
    use star_line::functions::map::Map;

    // 1101 to 0100
    #[test]
    fn test_resolve_easy() {
        let mut map = Map::new(vec![true, true, false, true], vec![false, true, false, false]);
        map.resolve();
        assert_eq!(map.count, 2);
    }

    // 101010 to 010101
    #[test]
    fn test_resolve_medium() {
        let mut map = Map::new(vec![true, false, true, false, true, false], vec![false, true, false, true, false, true]);
        map.resolve();
        assert_eq!(map.count, 26);
    }

    // 11001001000 to 10000110011
    #[test]
    fn test_resolve_hard() {
        let mut map = Map::new(vec![true, true, false, false, true, false, false, true, false, false, false], vec![true, false, false, false, false, true, true, false, false, true, true]);
        map.resolve();
        assert_eq!(map.count, 877);
    }

    #[test]
    fn new_target() {
        assert_eq!(Map::new_target(3), vec![true, false, false]);
    }

    #[test]
    fn new_target_empty() {
        assert_eq!(Map::new_target(0), vec![]);
    }

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