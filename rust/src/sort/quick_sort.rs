/// 这段代码是使用 Rust 编写的快速排序算法的实现。具体来说：
///
/// 1. `quickify` 函数是排序的主函数。如果给定数组（`arr`）的长度小于或等于 1，函数就直接返回，
/// 因为长度小于或等于 1 的数组已经被认为是有序的。然后，它会选择一个 "pivot" （即基准点或枢轴），
/// 然后将数组分为两部分。一部分包含所有小于或等于 pivot 的元素，另一部分包含所有大于 pivot 的元素。然后，
/// 它会递归地调用自己来对这两部分数组进行排序。
///
/// 2. `partition` 函数是为了划分数组。它的目标是调整数组的元素，使得所有小于或等于 pivot 的元素都在 pivot 的左边，
/// 而所有大于 pivot 的元素都在其右边。这个函数首先将最后一个元素设为 pivot。然后，它会遍历数组的其他元素。
/// 如果当前元素小于或等于 pivot，那么它就会将该元素与数组的第一个未处理的元素交换位置，然后增加这个未处理元素的计数器。
/// 最后，它会将 pivot 与第一个大于 pivot 的元素交换位置，从而确保 pivot 左边的所有元素都小于或等于它，
/// 而它右边的所有元素都大于它。函数最后返回 pivot 的位置。
///
/// 在整个过程中，`quickify`函数通过在每次调用`partition`函数后递归地对 pivot 左边和右边的部分进行排序，
/// 以对整个数组进行排序。快速排序的平均时间复杂度为 O(n log n)，其中 n 是数组的大小。
///
pub fn quickify<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quickify(&mut arr[..pivot]);
    quickify(&mut arr[pivot + 1..]);
}

pub fn partition<T: Ord + std::fmt::Debug>(arr: &mut [T]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    println!("arr: {:?}", arr);
    i
}

#[test]
fn test_quickify() {
    let mut arr = [1, 3, 2, 5, 4, 7, 9, 8, 6];
    quickify(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
