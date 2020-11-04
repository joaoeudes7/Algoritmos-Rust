use utils::loop_random_vec;

fn main() {
    loop_random_vec(|vec_shuffled| {
        selection_sort(vec_shuffled)
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
