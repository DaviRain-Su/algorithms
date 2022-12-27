pub fn binary_search<T: std::cmp::PartialOrd>(a: &[T], key: T) -> Option<usize> {
    let mut lo = 0usize;
    let mut hi = a.len() - 1; // NOTICE: hi is a.len() - 1
    while lo <= hi {
        //  NOTICE: condition is lo <= hi
        let mid = lo + (hi - lo) / 2; //  NOTICE: mid is lo + (hi - lo) / 2
        if key < a[mid] {
            hi = mid - 1;
        } else if key > a[mid] {
            lo = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

#[test]
fn test_binary_search() {
    let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = binary_search(&array, 4);
    assert_eq!(result, Some(3));
}
