extern crate time;
mod benchmark;
mod sorting_algorithms;
use sorting_algorithms::*;
use std::isize;

fn main() {
    const ARRAY_LENGTH: usize = 2000;
    let array = sorting_algorithms::gen_array(ARRAY_LENGTH, -200, 200).clone();
    let mut array1 = array.clone();
    let bubble_time = time_sort(&bubble_sort, &mut array1);

    let mut array2 = array.clone();
    let shaker_time = time_sort(&shaker_sort, &mut array2);

    let mut array3 = array.clone();
    let comb_time = time_sort(&comb_sort, &mut array3);

    let mut array4 = array.clone();
    let insert_time = time_sort(&insert_sort, &mut array4);

    let mut array5 = array.clone();
    let selection_time = time_sort(&selection_sort, &mut array5);

    let mut array6 = array.clone();
    let pyramidal_time = time_sort(&pyramidal_sort, &mut array6);

    let mut array7 = array.clone();
    let insertion_time = time_sort_2(
        &insertion_sort,
        &mut array7,
        0,
        (ARRAY_LENGTH - 1) as isize,
        None,
    );

    let mut array8 = array.clone();
    let quick_sort_lomuto_time = time_sort_2(
        &quick_sort_lomuto,
        &mut array8,
        0,
        (ARRAY_LENGTH - 1) as isize,
        None,
    );

    let mut array9 = array.clone();
    let quick_sort_hoare_static_pivot_time = time_sort_2(
        &quick_sort_hoare,
        &mut array9,
        0,
        (ARRAY_LENGTH - 1) as isize,
        Some(Pivot::Static),
    );

    let mut array10 = array.clone();
    let quick_sort_hoare_random_pivot_time = time_sort_2(
        &quick_sort_hoare,
        &mut array10,
        0,
        (ARRAY_LENGTH - 1) as isize,
        Some(Pivot::Random),
    );

    let mut array11 = array.clone();
    let merge_sort_time = time_sort_2(
        &merge_sort,
        &mut array11,
        0,
        (ARRAY_LENGTH - 1) as isize,
        None,
    );

    let mut array12 = array.clone();
    let counting_sort_time = time_sort(&counting_sort, &mut array12);

    let mut array13 = array.clone();
    let radix_sort_time = time_sort(&radix_sort, &mut array13);
    println!("sorting {} elements\n", ARRAY_LENGTH);
    println!("bubble: {} ms", bubble_time);
    println!("shaker: {} ms", shaker_time);
    println!("comb: {} ms", comb_time);
    println!("insert: {} ms", insert_time);
    println!("selection: {} ms", selection_time);
    println!("pyramidal: {} ms", pyramidal_time);
    println!("insertion: {} ms", insertion_time);
    println!("quick sort lomuto: {} ms", quick_sort_lomuto_time);
    println!(
        "quick sort hoare static pivot time: {} ms",
        quick_sort_hoare_static_pivot_time
    );
    println!(
        "quick sort hoare random pivot time: {} ms",
        quick_sort_hoare_random_pivot_time
    );
    println!("merge: {} ms", merge_sort_time);
    println!("counting: {} ms", counting_sort_time);
    println!("radix: {} ms", radix_sort_time);
}

fn time_sort<T: PartialOrd + Copy>(sort: &Fn(&mut [T]) -> &mut [T], array: &mut [T]) -> usize {
    let mut timer = benchmark::Timer::new();

    timer.start();
    let sorted_array = sort(array);
    timer.stop();
    verify_sorted(sorted_array);
    timer.get_elapsed_in_mcs() as usize
}

fn time_sort_2<T: PartialOrd + Copy>(
    sort: &Fn(&mut [T], isize, isize, Option<Pivot>) -> &mut [T],
    array: &mut [T],
    lo: isize,
    hi: isize,
    pivot: Option<Pivot>,
) -> usize {
    let mut timer = benchmark::Timer::new();

    timer.start();
    let sorted_array = sort(array, lo, hi, pivot);
    timer.stop();
    verify_sorted(sorted_array);
    timer.get_elapsed_in_mcs() as usize
}

fn verify_sorted<T: PartialOrd + Copy>(array: &mut [T]) {
    let mut i_min = array[0];
    for i in 1..array.len() {
        if array[i] >= i_min {
            i_min = array[i];
        } else {
            panic!("Sort method failed to sort!");
        }
    }
}
