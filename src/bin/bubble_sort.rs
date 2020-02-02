mod utils;

use std::time::Instant;
use utils::gen_shuffle_vec;

fn main() {
    let mut n: i32 = 10;

    for _ in 0..5 {
        n *= 10;
        let mut v_base = gen_shuffle_vec(n);

        let now = Instant::now();
        bubble_sort(&mut v_base);
        let duration = now.elapsed().as_nanos();

        println!("{} nanoseconds for sorting {} integers.", duration, n);
    }
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
