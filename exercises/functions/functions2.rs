// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

fn main() {
    call_me(vec![3]);
}

fn call_me(num: Vec<i32>) {
    for i in 0..num.len() {
        println!("Ring! Call number {}", i + 1);
    }
}
