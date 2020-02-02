mod utils;

use std::time::Instant;
use utils::gen_shuffle_vec;

fn main() {
    let mut n: i32 = 10;

    for _ in 0..5 {
        n *= 10;
        let mut v_base = gen_shuffle_vec(n);

        let now = Instant::now();
        insertion_sort(&mut v_base);
        let duration = now.elapsed().as_nanos();

        println!("{} nanoseconds for sorting {} integers.", duration, n);
    }
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
