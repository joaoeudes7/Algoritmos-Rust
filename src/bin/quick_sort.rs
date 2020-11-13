use utils::{loop_vec_in_case, TypeCase};

fn main() {
    loop_vec_in_case(TypeCase::BETTER, |vec| {
        let end = vec.len() - 1;
        quick_sort( vec, 0, end as i32);
    });
}

pub fn quick_sort(arr: &mut [i32], start: i32, end: i32) {
    if start < end {
        let p = partition(arr, start as usize, end as usize);

        quick_sort(arr, start, p - 1);
        quick_sort(arr, p + 1, end);
    }
}

fn partition(arr: &mut [i32], start: usize, end: usize) -> i32 {
    let mut i = start;

    for j in start..end {
        if arr[j] < arr[end] {
            arr.swap(i, j);

            i += 1;
        }
    }

    arr.swap(i, end);

    return i as i32;
}
