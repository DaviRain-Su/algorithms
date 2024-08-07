/// 这段代码是使用 Rust 编写的快速排序算法的实现。具体来说：
///
/// 1. `quickify` 函数是排序的主函数。如果给定数组（`arr`）的长度小于或等于 1，函数就直接返回，
///   因为长度小于或等于 1 的数组已经被认为是有序的。然后，它会选择一个 "pivot" （即基准点或枢轴），
///   然后将数组分为两部分。一部分包含所有小于或等于 pivot 的元素，另一部分包含所有大于 pivot 的元素。然后，
///   它会递归地调用自己来对这两部分数组进行排序。
///
/// 2. `partition` 函数是为了划分数组。它的目标是调整数组的元素，使得所有小于或等于 pivot 的元素都在 pivot 的左边，
///   而所有大于 pivot 的元素都在其右边。这个函数首先将最后一个元素设为 pivot。然后，它会遍历数组的其他元素。
///   如果当前元素小于或等于 pivot，那么它就会将该元素与数组的第一个未处理的元素交换位置，然后增加这个未处理元素的计数器。
///   最后，它会将 pivot 与第一个大于 pivot 的元素交换位置，从而确保 pivot 左边的所有元素都小于或等于它，
///   而它右边的所有元素都大于它。函数最后返回 pivot 的位置。
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

/// 当然可以，我来给你解释一下。
///
/// 我们使用一个整数数组作为例子： `[9, 7, 5, 11, 12, 2, 14, 3, 10, 6]`
///
/// 1. 在我们的 `partition` 函数中，`pivot` 是最后一个元素，所以 `pivot` = 6。
///
/// 2. 初始化 `i = 0`，这是一个“指针”，它指向我们要交换的下一个元素。
///
/// 3. 我们遍历数组中除 `pivot` 的其他元素。对于每个元素 `j`，如果 `arr[j] <= pivot`，
///   那么我们就交换 `arr[i]` 和 `arr[j]`，然后 `i` 自增 1。这样做的目的是将所有小于或等于 `pivot` 的元素移动到数组的左侧。
///
/// 4. 当我们完成遍历时，`arr` 可能看起来像这样：`[5, 2, 3, 11, 12, 9, 14, 7, 10, 6]`。
///   现在，`i = 3`，也就是 `arr[i] = 11`。注意到所有小于等于 `pivot`（6）的元素都在左侧，而所有大于 `pivot` 的元素都在右侧。但是，`pivot` 本身还在数组的最右侧。
///
/// 5. 我们将 `pivot` 与 `arr[i]` 交换，得到：`[5, 2, 3, 6, 12, 9, 14, 7, 10, 11]`。
///   这样就确保了 `pivot` 的左侧都是小于或等于它的元素，右侧都是大于它的元素。
///
/// 6. 最后，`partition` 函数返回 `pivot` 的位置，也就是 `i` 的值，它是 3。
///
/// 在 `quickify` 函数中，我们会对 `pivot` 左侧的元素（索引 `0` 到 `pivot - 1`，
/// 即 `[5, 2, 3]`）和右侧的元素（索引 `pivot + 1` 到 `n - 1`，即 `[12, 9, 14, 7, 10, 11]`）进行递归排序。
/// 这样，在几轮递归后，整个数组就会被完全排序。
///
/// 上述代码实现了快速排序算法的基本形式，它对于大多数情况已经非常高效了。然而，如果你对此有特别的需求，以下是一些可能的优化方向：
///
/// 1. **Pivot 的选择**：在上述代码中，Pivot 是固定选择的每一轮数组的最后一个元素，这可能会导致在处理已经接近有序的数据时效率降低。一个常见的优化是使用"三数取中"法来选取 Pivot，即从数组的首元素、中间元素和尾元素中选取中位数作为 Pivot。
///
/// 2. **插入排序的应用**：对于小数组，快速排序并不一定是最高效的。在这种情况下，一种常见的优化是当数组的大小降到某个阈值（比如 10）时，切换到插入排序。
///
/// 3. **去除尾递归**：上述代码使用了两次递归，但其实你可以去除一个递归。方法是将对右侧数组的递归操作改为迭代，在每次迭代中，仅对左侧数组进行递归。
///
/// 4. **并行化**：如果你处理的是非常大的数据，或者你的硬件具有多个处理核心，你可以考虑并行化快速排序算法。简单来说，就是让每个核心处理数组的一部分。这需要更复杂的编程技术，并且必须小心地处理并发操作，以避免数据的不一致。
///
/// 这些优化对于基本的快速排序算法来说可能会有所帮助，但是在应用之前，你应该先确定你的代码的性能瓶颈在哪里，以及这些优化是否值得你的时间和精力去实现。
#[test]
fn test_quickify() {
    let mut arr = [1, 3, 2, 5, 4, 7, 9, 8, 6];
    quickify(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
