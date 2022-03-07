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

    // 0100
    #[test]
    fn test_invert_easy() {
        let mut slice = [false, true, false, false];

        assert_eq!(Map::invert_light(&mut slice), 1);
    }

    // 00111
    #[test]
    fn test_invert_medium() {
        let mut slice = [false, false, true, true, true];

        assert_eq!(Map::invert_light(&mut slice), 5);
    }

    // 01010101
    #[test]
    fn test_invert_hard() {
        let mut slice = [false, true, false, true, false, true, false, true];

        assert_eq!(Map::invert_light(&mut slice), 50);
    }
}