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
    #[should_panic]
    #[test]
    pub fn tree_sort_1() {
        let mut array = [1, 7, 0, 14, -5, -9, 2, 17];
        let sorted = [-9, -5, 0, 1, 2, 7, 14, 17];
        let not_sorted = [1, 7, 0, 14, 5, -9, 2, 17];
        assert_ne!(sorting_algorithms::tree_sort(&mut array), sorted);
        assert!(sorting_algorithms::tree_sort(&mut array) == sorted);
        assert_eq!(sorting_algorithms::tree_sort(&mut array), not_sorted);
    }
    /*
    #[test]
    pub fn tree_sort_2() {
        let mut array = [1, 14, 0, 7];
        let sorted = [0, 1, 7, 14];
        let not_sorted = [1, 14, 0, 7];
        assert_eq!(sorting_algorithms::tree_sort(&mut array), sorted);
        assert!(sorting_algorithms::tree_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::tree_sort(&mut array), not_sorted);
    }
*/
    #[test]
    pub fn selection_sort() {
        let mut array = [1, 7, 0, 14, -5, -9, 2, 17];
        let sorted = [-9, -5, 0, 1, 2, 7, 14, 17];
        let not_sorted = [1, 7, 0, 14, 5, -9, 2, 17];
        assert_eq!(sorting_algorithms::selection_sort(&mut array), sorted);
        assert!(sorting_algorithms::selection_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::selection_sort(&mut array), not_sorted);
    }

    #[test]
    pub fn pyramidal_sort() {
        let mut array = [-8, 6, 1, 0, 0, -6, 4, 9, 8, -4, 7, 0, 5, -9, 5, 8, 0, 1, 3, -8];
        let sorted = [-9, -8, -8, -6, -4, 0, 0, 0, 0, 1, 1, 3, 4, 5, 5, 6, 7, 8, 8, 9];
        let not_sorted = [-8, 6, 1, 0, 0, -6, 4, 9, 8, -4, 7, 0, 5, -9, 5, 8, 0, 1, 3, -8];
        assert_eq!(sorting_algorithms::pyramidal_sort(&mut array), sorted);
        assert!(sorting_algorithms::pyramidal_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::pyramidal_sort(&mut array), not_sorted);
    }
}