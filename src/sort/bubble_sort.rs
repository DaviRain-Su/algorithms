// Sort the array using bubble sort. The idea behind
// bubble sort is to look for adjacent indexes which
// are out of place and interchange their elements
// until the entire array is sorted.

/// bubble sort can customize the comparison logic based on the comparison function passed in
pub fn bubble_sort_by<T: core::cmp::PartialOrd, F>(array: &mut [T], cmp_fun: F) -> bool 
    where F: FnOnce(&T, &T) -> bool + core::marker::Copy,
{
    if array.is_empty() {
        return true;
    }

    let mut sorted = false;
    let len = array.len();

    while !sorted {
        sorted = true;
        (0..(len - 1)).for_each(|idx| {
            if cmp_fun(&array[idx + 1], &array[idx]) {
                array.swap(idx, idx + 1);
                sorted = false;
            }
        })
    }

    sorted
}

/// The ascending sort algorithm for bubble sort
pub fn bubble_sort<T: core::cmp::PartialOrd>(array: &mut [T]) -> bool 
{
    bubble_sort_by(array, |v1, v2| v1 < v2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![10, 4, 6, 8, 13, 2, 3];
        let ret = bubble_sort(&mut arr);
        println!("{:?}, ret = {}", arr, ret);
    }
    
    #[test]
    fn test_bubble_sort_a_empty_arr() {
        let mut arr = Vec::<i32>::new();
        let ret = bubble_sort(&mut arr);
        println!("{:?}, ret = {}", arr, ret);
    }
}

