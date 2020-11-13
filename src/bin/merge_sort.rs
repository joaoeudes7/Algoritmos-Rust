use utils::{loop_vec_in_case, TypeCase};

fn main() {
    loop_vec_in_case(TypeCase::MEDIUM, |vec| {
        let len = vec.len() - 1;

        merge_sort(vec, 0, len);
    });
}

pub fn merge_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let mid = (start + end) / 2;

        merge_sort(arr, start, mid);
        merge_sort(arr, mid + 1, end);
        merge(arr, start, mid, end);
    }
}

fn merge(arr: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    let mut i = start;
    let mut j = mid + 1;
    let diff = end - start + 1;

    let mut b: Vec<i32> = vec![0; diff];

    for k in 0..diff {
        if j > end || arr[i] < arr[j] && i <= mid {
            b[k] = arr[i];
            i += 1;
        } else {
            b[k] = arr[j];
            j += 1;
        }
    }

    for k in 0..diff {
        arr[start + k] = b[k];
    }
}

