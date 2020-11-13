use utils::{loop_vec_in_case, TypeCase};

fn main() {
    loop_vec_in_case(TypeCase::BETTER, |vec| {
        selection_sort(vec)
    });
}

fn selection_sort(arr: &mut [i32]) {
    let len_arr = arr.len();

    for i in 0..len_arr {
        for j in i + 1..len_arr {
            if arr[j] < arr[i] {
                arr.swap(i, j);
            }
        }
    }
}
