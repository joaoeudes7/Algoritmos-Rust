fn main() {
    let mut bins: Vec<String> = Vec::new();

    bins.push(String::from("merge_sort"));
    bins.push(String::from("quick_sort"));

    println!("Run `cargo run bin_name --release`");

    println!("Bin available:");
    println!("- merge_sort");
    println!("- quick_sort");
}
