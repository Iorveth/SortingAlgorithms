extern crate time;
mod sorting_algorithms;
mod benchmark;
use std::isize;
use sorting_algorithms::*;

fn main() {
    const ARRAY_LENGTH: usize = 4000;
    let mut array = sorting_algorithms::gen_array(ARRAY_LENGTH, - 200, 200).clone();

    let mut array1 = array.clone();
    let bubble_time = time_sort(&bubble_sort, &mut array1);

    /*let mut array2 = array.clone();
    let shaker_time = time_sort(&shaker_sort, &mut array2);*/

    let mut array3 = array.clone();
    let comb_time = time_sort(&comb_sort, &mut array3);

    let mut array4 = array.clone();
    let insert_time = time_sort(&insert_sort, &mut array4);

    let mut array5 = array.clone();
    let selection_time = time_sort(&selection_sort, &mut array5);

    let mut array6 = array.clone();
    let pyramidal_time = time_sort(&pyramidal_sort, &mut array6);

    let mut array7 = array.clone();
    let insertion_time = time_sort_2(&insertion_sort, &mut array7, 0, (ARRAY_LENGTH - 1) as isize, None);

    let mut array8 = array.clone();
    let quick_sort_lomuto_time = time_sort_2(&quick_sort_lomuto, &mut array8, 0, (ARRAY_LENGTH - 1) as isize, None);

    let mut array9 = array.clone();
    let quick_sort_hoare_static_pivot_time = time_sort_2(&quick_sort_hoare, &mut array9, 0, (ARRAY_LENGTH - 1) as isize, Some(Pivot::Static));

    let mut array10 = array.clone();
    let quick_sort_hoare_random_pivot_time = time_sort_2(&quick_sort_hoare, &mut array10, 0, (ARRAY_LENGTH - 1) as isize, Some(Pivot::Random));

    println!("sorting {} elements\n", ARRAY_LENGTH);
    println!("bubble: {} ms", bubble_time);
    //println!("shaker: {} ms", shaker_time);
    println!("comb: {} ms", comb_time);
    println!("insert: {} ms", insert_time);
    println!("selection: {} ms", selection_time);
    println!("pyramidal: {} ms", pyramidal_time);
    println!("insertion: {} ms", insertion_time);
    println!("quick sort lomuto: {} ms", quick_sort_lomuto_time);
    println!("quick sort hoare static pivot time: {} ms", quick_sort_hoare_static_pivot_time);
    println!("quick sort hoare random pivot time: {} ms", quick_sort_hoare_random_pivot_time);
}

fn time_sort(sort: &Fn(&mut[isize]) -> &mut[isize], array: &mut[isize]) -> usize {
    let mut timer = benchmark::Timer::new();

    timer.start();
    let sorted_array = sort(array);
    timer.stop();
    verify_sorted(sorted_array);
    timer.get_elapsed_in_mcs() as usize
}

fn time_sort_2(sort: &Fn(&mut[isize], isize, isize, Option<Pivot>) -> &mut[isize], array: &mut[isize], lo: isize, hi: isize, pivot: Option<Pivot>) -> usize {
    let mut timer = benchmark::Timer::new();

    timer.start();
    let sorted_array = sort(array, lo, hi, pivot);
    timer.stop();
    verify_sorted(sorted_array);
    timer.get_elapsed_in_mcs() as usize
}

fn verify_sorted(array: &mut[isize]) {
    let mut i_min = isize::MIN;
    for i in 0..array.len() {
        if array[i] >= i_min {
            i_min = array[i];
        } else {
            panic!("Sort method failed to sort!");
        }
    }
}

