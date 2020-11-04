use utils::loop_random_vec;

fn main() {
    loop_random_vec(|vec_shuffled| {
        insertion_sort(vec_shuffled);
    });
}

fn insertion_sort(arr: &mut [i32]) {
    let len_arr = arr.len();

    for i in 1..len_arr {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}
