#[cfg(test)]
mod tests_map {
    use star_line::functions::map::Map;
    use std::collections::VecDeque;

    #[test]
    fn test_resolve_easy() {
        let mut map = Map::new(vec![1101], vec![0100]);
        map.resolve();
        assert_eq!(map.count, 2);
    }

    #[test]
    fn test_resolve_medium() {
        let mut map = Map::new(vec![101010], vec![010101]);
        map.resolve();
        assert_eq!(map.count, 26);
    }

    #[test]
    fn test_resolve_hard() {
        let mut map = Map::new(vec![11001001000], vec![10000110011]);
        map.resolve();
        assert_eq!(map.count, 877);
    }

    #[test]
    fn test_invert_easy() {
        let mut vec = VecDeque::from(vec![0100]);

        assert_eq!(Map::invert_light(&mut vec), 1);
    }

    #[test]
    fn test_invert_medium() {
        let mut vec = VecDeque::from(vec![00111]);

        assert_eq!(Map::invert_light(&mut vec), 5);
    }

    #[test]
    fn test_invert_hard() {
        let mut vec = VecDeque::from(vec![01010101]);

        assert_eq!(Map::invert_light(&mut vec), 50);
    }
}