extern crate rand;
extern crate time;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    let mut n: i32 = 10;

    for _ in 0..5 {
        n *= 10;
        let mut v_base: Vec<_> = (0..n).collect();

        v_base.shuffle(&mut thread_rng());

        let now = Instant::now();
        selection_sort(&mut v_base);
        let duration = now.elapsed().as_nanos();

        println!("{} nanoseconds for sorting {} integers.", duration, n);
    }
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
