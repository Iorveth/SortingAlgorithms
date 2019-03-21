pub fn swap(array: &mut[i8], i1: usize, i2: usize){
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