use std::fmt::Debug;
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
fn swap(array: &mut[i8], i1: usize, i2: usize){
    let buf = array[i1];
    array[i1] = array[i2];
    array[i2] = buf;
}

pub fn bubble_sort(array: &mut [i8]) -> &mut[i8]{
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

pub fn shaker_sort(array: &mut [i8])  -> &mut[i8] {
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

pub fn comb_sort(array: &mut [i8])  -> &mut[i8] {
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

pub fn insert_sort(array: &mut [i8])  -> &mut[i8] {
    for i in 1..array.len() {
        let key = array[i];
        let mut  j = (i-1) as i8;
        while j >= 0 && array[j as usize] > key {
            array[j as usize + 1] = array[j as usize];
            j = j - 1;
        }
        array[(j + 1) as usize] = key;
    }
    array
}

pub fn tree_sort(array: &mut [i8]) -> &mut[i8]{
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