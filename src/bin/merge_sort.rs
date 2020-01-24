extern crate rand;
extern crate time;

use rand::prelude::*;
use std::time::Instant;

fn main() {
    let mut n = 100;

    for i in 1..5 {
        n = i32::pow(n, i);

        let mut v_base: Vec<i32> = (0..n).collect();

        let mut rng = rand::thread_rng();
        v_base.shuffle(&mut rng);

        let len = v_base.len();

        let now = Instant::now();
        merge_sort_recursive(&mut v_base, 0, len - 1);
        let duration = now.elapsed().as_millis();

        println!("{} milliseconds for sorting {} integers.", duration, n);
    }
}

pub fn merge_sort_recursive(a: &mut Vec<i32>, b: usize, e: usize) {
    if b < e {
        let m = (b + e) / 2;
        merge_sort_recursive(a, b, m);
        merge_sort_recursive(a, m + 1, e);
        merge(a, b, m, e);
    }
}

fn merge(a: &mut Vec<i32>, b: usize, m: usize, e: usize) {
    let mut left = a[b..m + 1].to_vec();
    let mut right = a[m + 1..e + 1].to_vec();

    left.reverse();
    right.reverse();

    for k in b..e + 1 {
        if left.is_empty() {
            a[k] = right.pop().unwrap();
            continue;
        }
        if right.is_empty() {
            a[k] = left.pop().unwrap();
            continue;
        }
        if right.last() < left.last() {
            a[k] = right.pop().unwrap();
        } else {
            a[k] = left.pop().unwrap();
        }
    }
}
