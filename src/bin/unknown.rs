fn main() {
    let mut vec: Vec<i32> = (0..4).collect();
    let len = vec.len() as i32;

    unknown(&mut vec, 0, len - 1);

    println!("{:?}", vec);
}

fn unknown(vec: &mut Vec<i32>, s: i32, e: i32) {
    if s + 1 < e - 1 {
        unknown(vec, s + 1, e - 1);
    }

    vec.swap(s as usize, e as usize);
}

fn has_item(vec: Vec<i32>, item: i32) -> bool {
    for i in 0..vec.len() {
        for j in 0..i {
            if item == vec[j as usize] {
                return true;
            }
        }
    }

    return false;
}
