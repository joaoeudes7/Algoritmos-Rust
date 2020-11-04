use gnuplot::{AxesCommon, Figure, Graph};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

pub fn loop_random_vec(callback: fn(&mut Vec<i32>)) {
    let mut n = 1024;
    let mut x = Vec::<i32>::new();
    let mut y = Vec::<usize>::new();

    for _ in 0..20 {
        n = n + n / 2;
        let mut shuffled_vec = gen_shuffled_vec(n);

        let now = Instant::now();

        callback(&mut shuffled_vec);

        let duration = now.elapsed().as_micros();

        x.push(n);
        y.push(duration as usize);

        println!("{}, {}", duration, n);
    }

    plot(x, y);
}

pub fn gen_shuffled_vec(lim: i32) -> Vec<i32> {
    let mut _vec: Vec<i32> = (0..lim).collect();

    _vec.shuffle(&mut thread_rng());

    return _vec;
}

pub fn plot(x: Vec<i32>, y: Vec<usize>) {
    let mut fg = Figure::new();

    fg.axes2d()
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_title("Result", &[])
        .set_x_label("Qtd Items", &[])
        .set_y_label("Duration (less is better)", &[])
        .lines(&x, &y, &[]);

    fg.show().unwrap();
}
