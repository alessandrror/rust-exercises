// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
  let answer = square(3);
  println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
  num * num
}

// Rust returns implicity a value when we let an statement without ;
// and if we want to return using the ; we can just add a "return"
// before the answer

// It is important because Rust read the ; and expect another operation
// or something else, so...