use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn gen_shuffle_vec(lim: i32) -> Vec<i32> {
    let mut _vec: Vec<i32> = (0..lim).collect();

    _vec.shuffle(&mut thread_rng());

    _vec
}
