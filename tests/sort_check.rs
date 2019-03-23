extern crate SortingAlgorithms;

#[cfg(test)]
mod integration_tests {
    use SortingAlgorithms::sorting_algorithms;
    #[test]
    pub fn bubble_sort() {
        let mut array = [1, 7, 0, -5, 6, 4, 2];
        let sorted = [-5, 0, 1, 2, 4, 6, 7];
        let not_sorted = [1, 7, 0, -5, 6, 4, 2];
        assert_eq!(sorting_algorithms::bubble_sort(&mut array), sorted);
        assert!(sorting_algorithms::bubble_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::bubble_sort(&mut array), not_sorted);
    }
    #[test]
    pub fn shaker_sort() {
        let mut array = [1, 7, 0, -5, 6, 4, 2];
        let sorted = [-5, 0, 1, 2, 4, 6, 7];
        let not_sorted = [1, 7, 0, -5, 6, 4, 2];
        assert_eq!(sorting_algorithms::shaker_sort(&mut array), sorted);
        assert!(sorting_algorithms::shaker_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::shaker_sort(&mut array), not_sorted);
    }
    #[test]
    pub fn comb_sort() {
        let mut array = [1, 7, 0, -5, 6, 4, 2];
        let sorted = [-5, 0, 1, 2, 4, 6, 7];
        let not_sorted = [1, 7, 0, -5, 6, 4, 2];
        assert_eq!(sorting_algorithms::comb_sort(&mut array), sorted);
        assert!(sorting_algorithms::comb_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::comb_sort(&mut array), not_sorted);
    }
    #[test]
    pub fn insert_sort() {
        let mut array = [1, 7, 0, -5, 6, 4, 2];
        let sorted = [-5, 0, 1, 2, 4, 6, 7];
        let not_sorted = [1, 7, 0, -5, 6, 4, 2];
        assert_eq!(sorting_algorithms::insert_sort(&mut array), sorted);
        assert!(sorting_algorithms::insert_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::insert_sort(&mut array), not_sorted);
    }
    #[test]
    pub fn tree_sort() {
        let mut array = [1, 7, 0, 14, -5, -9, 2, 17];
        let sorted = [-9, -5, 0, 1, 2, 7, 14, 17];
        let not_sorted = [1, 7, 0, 14, 5, -9, 2, 17];
        assert_eq!(sorting_algorithms::tree_sort(&mut array), sorted);
        assert!(sorting_algorithms::tree_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::tree_sort(&mut array), not_sorted);
    }
}