use utils::{loop_vec_in_case, TypeCase};

fn main() {
    loop_vec_in_case(TypeCase::BETTER,|vec| {
        bubble_sort(vec);
    });
}

fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in ((i + 1)..arr.len()).rev() {
            if arr[j - 1] > arr[i] {
                arr.swap(j - 1, i);
            }
        }
    }
}
