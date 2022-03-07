#[cfg(test)]
mod tests_map {
    use star_line::functions::map;

    #[test]
    fn test_resolve_easy() {
        let map = Map::new(vec![1101], vec![0100]);
        map.resolve();
        assert_eq!(map.count, 2);
    }

    #[test]
    fn test_resolve_medium() {
        let map = Map::new(vec![101010], vec![010101]);
        map.resolve();
        assert_eq!(map.count, 26);
    }

    #[test]
    fn test_resolve_hard() {
        let map = Map::new(vec![11001001000], vec![10000110011]);
        map.resolve();
        assert_eq!(map.count, 877);
    }
}