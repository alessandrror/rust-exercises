// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
  let mut x = 3;
  println!("Number {}", x);
  x = 5; // don't change this line
  println!("Number {}", x);
}

// In Rust, variables are immutable by default. When a variable is immutable,
// once a value is bound to a name, you can’t change that value.

// If we use "mut" right before the variable, we can indicate to Rust that this variable
// can be mut.