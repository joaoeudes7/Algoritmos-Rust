use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{collections::HashMap, time::Instant};

pub enum TypeCase {
    BETTER,
    MEDIUM,
    WORSE,
}

pub struct ResultSampleBench {
    pub total_items: Vec<i32>,
    pub total_time: Vec<f32>,
}

impl ResultSampleBench {
    fn new(total_items: Vec<i32>, total_time: Vec<f32>) -> Self {
        ResultSampleBench {
            total_items,
            total_time,
        }
    }
}

pub fn loop_vec_in_case(type_case: TypeCase, callback: fn(&mut Vec<i32>)) -> ResultSampleBench {
    let mut n = 100;
    let mut data: HashMap<i32, f32> = HashMap::new();

    let loop_media: f32 = 3.0;

    for _ in 0..250 {
        for _ in 1..=loop_media as i32 {
            let mut vec: Vec<i32> = match type_case {
                TypeCase::BETTER => { (0..n).collect() }
                TypeCase::MEDIUM => { gen_shuffled_vec(n) }
                TypeCase::WORSE => { (0..n).rev().collect() }
            };

            let now = Instant::now();

            callback(&mut vec);

            let duration = now.elapsed().as_secs_f32();

            data.entry(n)
                .and_modify(|old| { *old += duration / loop_media })
                .or_insert(duration / loop_media);
        }

        println!("{}, {}", n, *data.get(&n).unwrap());

        n += 100;
    }

    let mut times = vec![];
    let mut items = vec![];

    for (item, time) in data.drain() {
        items.push(item);
        times.push(time);
    }

    return ResultSampleBench::new(items, times);
}

pub fn gen_shuffled_vec(lim: i32) -> Vec<i32> {
    let mut _vec: Vec<i32> = (0..lim).collect();

    _vec.shuffle(&mut thread_rng());

    return _vec;
}
