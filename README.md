# algorithms-rs

![build status](https://app.travis-ci.com/DaviRain-Su/algorithms-rs.svg?branch=main)

Introduction to algorithms thrid edition implement by rust programming

## Present implement

- Heap
  - rec build heap
    - max_heapify
      - `max_heapify` function
    - min-heapify
      - `min_heapify` function
  - no rec build heap
    - min_sift
      - `min_sift_up` function
      - `min_sift_down` function
    - max_sift
      - `max_sift_up` function
      - `max_sift_down` function
  - max heap
    - max_shift_up
      - `build_max_heap_by_shift_up` function
    - max_heapify
      - `build_max_heap_by_max_heapify` function
  - min heap
    - min_shift_up
      - `build_min_heap_by_siftup` function
    - min_heapify
      - `build_min_heap_by_min_heapify` function
  - heap sort algorithms
    - asc sort by Max-Heap by Max heapify
      - `heap_sort_by_max_heap` function
    - dec sort by Min-Heap by Min Heapify
      - `heap_sort_by_min_heap` function
    - asc sort by Max-Heap by Max shift_up and shift_down
      - `asc_sort_with_max_sift` function
    - dec sort by Min-Heap by Min Shift_up and shift_down
      - `dec_sort_with_min_sift` function
- Stack
  - push element
  - pop element
- Queue
  - pop head element
  - push tail element

## support no-std feature

config setting:

```
algorithms-rs = { version = "0.1", default-features = false }
```
