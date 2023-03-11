use core::cmp::PartialEq;
use core::cmp::PartialOrd;

/// binary search function
pub fn search<T>(list: &[T], item: Option<&T>) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    let mut low = 0usize;
    let mut high = list.len(); // or way2: let mut high = list.len() - 1

    // or way2: while low <= high {
    while low < high {
        let mid = (low + high) / 2;
        let guess = list.get(mid);
        if guess == item {
            return Some(mid);
        } else if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

#[test]
fn test_binary_search() {
    let list = vec![1];
    let idx = search(&list, Some(&1));
    println!("idx = {idx:?}");
}
