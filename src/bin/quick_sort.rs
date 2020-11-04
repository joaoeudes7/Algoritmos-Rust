use utils::loop_random_vec;

fn main() {
    loop_random_vec(|vec_shuffled| {
        quick_sort(vec_shuffled)
    });
}

fn partition(arr: &mut [i32]) -> usize {
    let p = arr[0];
    let mut i = 1;
    let mut j = arr.len() - 1;

    loop {
        while i < arr.len() && arr[i] <= p {
            i += 1;
        }

        while j > 0 && arr[j] >= p {
            j -= 1;
        }

        if i >= j {
            break;
        }

        arr.swap(i, j);
    }

    arr.swap(0, j);

    j
}

fn quick_sort(arr: &mut [i32]) {
    let mid = partition(arr);

    if arr[..mid].len() > 1 {
        quick_sort(&mut arr[..mid]);
    }

    if arr[mid + 1..].len() > 1 {
        quick_sort(&mut arr[mid + 1..]);
    }
}
