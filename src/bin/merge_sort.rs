use utils::{loop_random_vec};

fn main() {
    loop_random_vec(|vec_shuffled| {
        let len = vec_shuffled.len();
        merge_sort_recursive(vec_shuffled, 0, len - 1)
    });
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
