module BubbleSort = {
    // bubble sort
    let bubble_sort = (array: array<'a>) => {
        if Belt.Array.size(array) == 0 {
            true
        } else {
            let sorted = ref(false);
            let len = Belt.Array.length(array);
            while !sorted.contents {
                sorted.contents = true;
                // because rescript for loop so there end is len - 2
                for i in 0 to (len - 2) {
                    if array[i + 1] < array[i] {
                        let temp = array[i];
                        array[i] = array[i + 1];
                        array[i + 1] = temp;
                        sorted.contents = false;
                    }
                }
            }
            sorted.contents
        }
    }


    // swap
    let swap = (~x, ~y) => {
        let temp = x.contents
        x := y.contents
        y := temp
    }
}