use std::time::Instant;

fn main() {
    for n in (10..200).step_by(10) {
        let mut media_duration = 0;
        let mut pow = 0;

        for _ in 0..4 {
            let now = Instant::now();

            pow = pow_r(n);

            let duration = now.elapsed().as_nanos();

            media_duration += duration / 4;
        }

        let _ = unsafe { core::ptr::read_volatile(&pow as *const _) };
        println!("{} {}", n, media_duration);
    }
}

fn pow_r(n: i32) -> i128 {
    if n == 0 {
        return 1;
    }

    let x = 2 * pow_r(n - 1);

    return x;
}
