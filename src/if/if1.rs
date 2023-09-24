// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub fn bigger(a: i32, b: i32) -> i32 {
  // Complete this function to return the bigger number!
  // Do not use:
  // - another function call
  // - additional variables
  if a > b {
    println!("{}", a);
    a
  } else {
    println!("{}", b);
    b
  }
}

fn main() {
  bigger(10, 8);
  bigger(32, 42);
  bigger(12, 12);
}
