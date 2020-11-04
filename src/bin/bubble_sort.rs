use utils::loop_random_vec;

fn main() {
    loop_random_vec(|vec_shuffled| {
        bubble_sort(vec_shuffled);
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
