extern crate rand;
use std::fmt::Debug;
use std::collections::BinaryHeap;
use std::thread;
use std::time::Duration;
use std::cmp::Ordering;
use sorting_algorithms::rand::prelude::*;

struct TreeNode<T: PartialOrd> {
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

struct RefNodeIterator<'a, T: PartialOrd + 'a> {
    stack: Vec<&'a TreeNode<T>>,
    next: Option<&'a T>,
}

impl<'a, T: PartialOrd + 'a> IntoIterator for &'a TreeNode<T> {
    type Item = &'a T;
    type IntoIter = RefNodeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = Vec::new();

        let smallest = pop_smallest_ref(self, &mut stack);

        RefNodeIterator { stack, next: Some(smallest) }
    }
}

impl<'a, T: PartialOrd + 'a> Iterator for RefNodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(next) = self.next.take() {
            return Some(next);
        }

        if let Some(node) = self.stack.pop() {
            if let Some(ref right) = node.right {
                self.stack.push(right);
            }
            return Some(&node.element);
        }

        None
    }
}

fn pop_smallest_ref<'a, T>(node: &'a TreeNode<T>, stack: &mut Vec<&'a TreeNode<T>>) -> &'a T
    where
        T: PartialOrd + 'a
{
    if let Some(ref left) = node.left {
        stack.push(node);
        return pop_smallest_ref(left, stack);
    }

    if let Some(ref right) = node.right {
        stack.push(right);
    }

    &node.element
}

impl<T: Ord + Debug> TreeNode<T> {
    fn add(&mut self, value: T) {
        if value <= self.element {
            match self.left {
                Some(ref mut node) => node.add(value),
                None => self.left = {
                    println!("Inserted left {:?}", value);
                    Some(Box::new(TreeNode { element: value, left: None, right: None }))
                },
            }
        } else {
            match self.right {
                Some(ref mut node) => node.add(value),
                None => self.right = {
                    println!("Inserted right {:?}", value);
                    Some(Box::new(TreeNode { element: value, left: None, right: None }))
                }
            }
        }
    }
}
fn swap(array: &mut [isize], i1: usize, i2: usize){
    let buf = array[i1];
    array[i1] = array[i2];
    array[i2] = buf;
}

pub fn gen_array(length: usize) -> Vec<isize> {
    let mut array = Vec::new();
    for _ in 0..length {
        array.push(rand::thread_rng().gen_range(-200,200));
    }
    array
}

pub fn bubble_sort(array: &mut [isize]) -> &mut[isize]{
    let mut b = true;
    let mut j = array.len() - 1;
    while b {
        b = false;
        for i in 0..j {
            if array[i] > array[i + 1] {
                swap(array, i, i+1);
                b = true;
            }
        }
        j-=1;
    }
    array
}

pub fn shaker_sort(array: &mut [isize])  -> &mut[isize] {
    let mut start = 0_usize;
    let mut end = array.len() - 1;
    let mut b = true;
    while b {
        b = false;
        for i in start..end {
            if array[i] > array[i + 1] {
                swap(array, i, i+1);
                b = true;
            }
        }
        if b == false {break;}
        for i in start..end {
            if array[end-i] < array[end-i-1] {
                swap(array, end - i, end - i - 1);
                b = true;
            }
        }
        end -=1;
        start+=1;
    }
    array
}

pub fn comb_sort(array: &mut [isize])  -> &mut[isize] {
    let k = 1.2473309;
    let len = array.len() - 1;
    let mut length = array.len() - 1;
    while length>1 {
        for i in 0..array.len() - length {
            if array[i] > array[length + i] {
                swap(array, i, length + i)
            }
        }
        length = ((length as f64) /  k) as usize;
    }
    println!("{:?}",array);
    bubble_sort(array)
}

pub fn insert_sort(array: &mut [isize]) -> &mut[isize] {
    for i in 1..array.len() {
        let key = array[i];
        let mut  j = (i-1) as isize;
        while j >= 0 && array[j as usize] > key {
            array[j as usize + 1] = array[j as usize];
            j = j - 1;
        }
        array[(j + 1) as usize] = key;
    }
    array
}

pub fn insertion_sort(array: &mut [isize], left: usize, right: usize) -> &mut[isize] {
    for i in (left + 1)..(right+1)  {
        let mut j = i;
        while j > left && array[j].cmp(&array[j - 1]) == Ordering::Less {
            //println!("Swapped");
            swap(array, j, j - 1);
            j = j - 1;
        }
    }
    array
}

pub fn tree_sort(array: &mut [isize]) -> &mut[isize]{
    let mut btree = TreeNode {element: array[0], left: None, right: None};
    for i in 1..array.len() {
        btree.add(array[i]);
        println!("Added {}", array[i])
    }

    for (i, elem) in btree.into_iter().enumerate() {
        array[i] = *elem;
        print!("{} ",elem);
    }
    array
}

pub fn selection_sort(array: &mut [isize]) -> &mut[isize]{
    for i in 0..array.len() - 1  {
        let mut min = array[i];
        for j in i..array.len()   {
            if array[j] < min {
                min = array[j];
                swap(array, i, j);
            }
        }
    }
    array
}

pub fn pyramidal_sort (array: &mut [isize]) -> &mut[isize] {
    let mut heap = BinaryHeap::new();
    for i in 0..array.len() {
        heap.push(array[i]);
    }
    for i in 0..array.len() {
        array[array.len()-i-1] = heap.pop().unwrap();
    }
    array
}

pub fn partition_hoare(array: &mut [isize], lo: isize, hi: isize) -> isize {
    let pivot = array[lo as usize];
    let mut i = lo - 1;
    let mut j = hi + 1;
    loop {
        'a: loop {
            i = i + 1;
            if array[i as usize] < pivot {break 'a;}
        }
        'b: loop {
            j = j - 1;
            if array[j as usize] > pivot {break 'b;}
        }
        if i >= j {
            break j
        }
        swap(array, i as usize, j as usize);
    }
}

pub fn partition_lomuto(array: &mut [isize], lo: isize, hi: isize) -> isize {
    let pivot = array[hi as usize];    // pivot
    let mut i = lo - 1;  // Index of smaller element

    for j in lo..hi
    {
        // If current element is smaller than or
        // equal to pivot
        if array[j as usize] <= pivot
        {
            i=i+1;    // increment index of smaller element
            swap(array, i as usize, j as usize);
        }
    }
    swap(array,(i + 1) as usize, hi as usize);
    return i + 1;
}

pub fn quick_sort_lomuto (array: &mut [isize], lo: isize, hi: isize) -> &mut[isize] {
    if lo < hi {
        if hi - lo < 27 {
            insertion_sort(array, lo as usize, hi as usize);
        } else {
            let p = partition_lomuto(array, lo, hi);
            quick_sort_lomuto(array, lo, p - 1);
            quick_sort_lomuto(array, p + 1, hi);
        }
    }
    array
}

pub fn quick_sort_hoare (array: &mut [isize], lo: isize, hi: isize) -> &mut[isize] {
    if lo < hi {
        if hi - lo < 32 {
            insertion_sort(array, lo as usize, hi as usize);
        } else {
            let p = partition_hoare(array, lo, hi);
            quick_sort_hoare(array, lo, p);
            quick_sort_hoare(array, p + 1, hi);
        }
    }
    array
}