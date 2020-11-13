use utils::{TypeCase, loop_vec_in_case};

fn main() {
    loop_vec_in_case(TypeCase::MEDIUM, |vec| {
        insertion_sort(vec);
    });
}

pub fn insertion_sort(arr: &mut [i32]) {
    for j in 1..(arr.len() as i32) {
        let k = arr[j as usize];
        let mut i = j - 1;

        while i > -1 && arr[i as usize] > k {
            arr[(i+1) as usize] = arr[i as usize];
            i -= 1;
        }

        arr[(i+1) as usize] = k;

    }
}
