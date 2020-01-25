extern crate rand;
extern crate time;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    let mut n = 10;

    for _ in 0..5 {
        n *= 10;
        let mut v_base: Vec<_> = (0..n).collect();

        v_base.shuffle(&mut thread_rng());

        let now = Instant::now();
        quick_sort(&mut v_base);
        let duration = now.elapsed().as_nanos();

        println!("{} nanoseconds for sorting {} integers.", duration, n);
    }
}

fn partition(arr: &mut [usize]) -> usize {
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

        swap(arr, i, j);
    }

    swap(arr, 0, j);

    j
}

fn swap(arr: &mut [usize], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn quick_sort(arr: &mut [usize]) {
    let mid = partition(arr);

    if arr[..mid].len() > 1 {
        quick_sort(&mut arr[..mid]);
    }
    if arr[mid + 1..].len() > 1 {
        quick_sort(&mut arr[mid + 1..]);
    }
}
