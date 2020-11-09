use utils::{loop_medium_case_vec};

fn main() {
    loop_medium_case_vec(|vec_shuffled| {
        let end = vec_shuffled.len() - 1;

        quick_sort(vec_shuffled, 1, end);
    });
}

fn quick_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let p = partition(arr, start, end);

        quick_sort(arr, start, p - 1);
        quick_sort(arr, p + 1, end);
    }
}

fn partition(arr: &mut Vec<i32>, start: usize, end: usize) -> usize {
    let mut i = start;

    for j in start..(end - 1) {
        if arr[j] < arr[end] {
            arr.swap(i, j);

            i += 1;
        }
    }

    arr.swap(i, end);

    return i;
}
