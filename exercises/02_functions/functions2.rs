// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {
    call_me(3);
}

fn call_me(num: i64) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
