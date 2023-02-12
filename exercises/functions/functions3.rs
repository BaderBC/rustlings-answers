// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me(420);
}

fn call_me(num: u32) {
    for i in 1..num + 1 {
        println!("Ring! Call number {}", i);
    }
}
