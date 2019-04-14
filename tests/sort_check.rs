extern crate SortingAlgorithms;

#[cfg(test)]
mod integration_tests {
    use SortingAlgorithms::sorting_algorithms;
    use SortingAlgorithms::sorting_algorithms::Pivot;
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
    pub fn insertion_sort() {
        let mut array = [1, 7, 0, -5, 6, 4, 2];
        let sorted = [-5, 0, 1, 2, 4, 6, 7];
        let not_sorted = [1, 7, 0, -5, 6, 4, 2];
        assert_eq!(
            sorting_algorithms::insertion_sort(&mut array, 0, (sorted.len() - 1) as isize, None),
            sorted
        );
        assert!(
            sorting_algorithms::insertion_sort(&mut array, 0, (sorted.len() - 1) as isize, None)
                == sorted
        );
        assert_ne!(
            sorting_algorithms::insertion_sort(&mut array, 0, (sorted.len() - 1) as isize, None),
            not_sorted
        );
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
        let mut array = [
            -8, 6, 1, 0, 0, -6, 4, 9, 8, -4, 7, 0, 5, -9, 5, 8, 0, 1, 3, -8,
        ];
        let sorted = [
            -9, -8, -8, -6, -4, 0, 0, 0, 0, 1, 1, 3, 4, 5, 5, 6, 7, 8, 8, 9,
        ];
        let not_sorted = [
            -8, 6, 1, 0, 0, -6, 4, 9, 8, -4, 7, 0, 5, -9, 5, 8, 0, 1, 3, -8,
        ];
        assert_eq!(sorting_algorithms::pyramidal_sort(&mut array), sorted);
        assert!(sorting_algorithms::pyramidal_sort(&mut array) == sorted);
        assert_ne!(sorting_algorithms::pyramidal_sort(&mut array), not_sorted);
    }

    #[test]
    pub fn quick_sort_lomuto() {
        let mut array = sorting_algorithms::gen_array(4000, -200, 250).clone();
        let mut array_cloned = array.clone();
        assert_eq!(
            sorting_algorithms::quick_sort_lomuto(&mut array, 0, 3999, None),
            sorting_algorithms::pyramidal_sort(&mut array_cloned)
        );
    }

    #[test]
    pub fn quick_sort_hoare_static_pivot() {
        let mut array = sorting_algorithms::gen_array(2000, -250, 200).clone();
        let mut array_cloned = array.clone();
        assert_eq!(
            sorting_algorithms::quick_sort_hoare(&mut array, 0, 1999, Some(Pivot::Static)),
            sorting_algorithms::pyramidal_sort(&mut array_cloned)
        );
    }

    #[test]
    pub fn quick_sort_hoare_random_pivot() {
        let mut array = sorting_algorithms::gen_array(1000, -200, 200).clone();
        let mut array_cloned = array.clone();
        assert_eq!(
            sorting_algorithms::quick_sort_hoare(&mut array, 0, 999, Some(Pivot::Random)),
            sorting_algorithms::pyramidal_sort(&mut array_cloned)
        );
    }

    #[test]
    pub fn merge_sort() {
        let mut array = sorting_algorithms::gen_array(500, -100, 100).clone();
        let mut array_cloned = array.clone();
        assert_eq!(
            sorting_algorithms::merge_sort(&mut array, 0, 499, None),
            sorting_algorithms::pyramidal_sort(&mut array_cloned)
        );
    }

    #[test]
    pub fn counting_sort() {
        let mut array = sorting_algorithms::gen_array(3000, -150, 400).clone();
        let mut array_cloned = array.clone();
        assert_eq!(
            sorting_algorithms::counting_sort(&mut array),
            sorting_algorithms::merge_sort(&mut array_cloned, 0, 2999, None)
        );
    }

    #[test]
    pub fn radix_sort() {
        let mut array = sorting_algorithms::gen_array(2000, -250, 400).clone();
        let mut array_cloned = array.clone();
        let mut array_cloned_2 = array.clone();
        assert_eq!(
            sorting_algorithms::radix_sort(&mut array),
            sorting_algorithms::merge_sort(&mut array_cloned, 0, 1999, None)
        );

        assert!(
            sorting_algorithms::radix_sort(&mut array) ==
            sorting_algorithms::pyramidal_sort(&mut array_cloned_2)
        );
    }
}
