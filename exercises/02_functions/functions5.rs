// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
    // 3 options:
    // 1. remove the semicolon
    // 2. add `return num`
    // 3. add `return num;`
    // which is best?
}
