use utils::{loop_better_case_vec};

fn main() {
    loop_better_case_vec(|vec_shuffled| {
        insertion_sort(vec_shuffled);
    });
}

pub fn insertion_sort(arr: &mut [i32]) {
    let len_arr = arr.len();

    for i in 0..len_arr {
        while i > 0 && arr[i] < arr[i - 1] {
            arr.swap(i, i - 1);
        }
    }
}
