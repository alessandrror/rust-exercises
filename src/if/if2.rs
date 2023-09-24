// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

pub fn foo_if_fizz(fizzish: &str) -> &str {
  if fizzish == "fizz" {
    println!("foo");
    "foo"
  } else if fizzish == "fuzz" {
    println!("bar");
    "bar"
  } else {
    println!("baz");
    "baz"
  }
}

fn main() {
  foo_if_fizz("fuzz");
  foo_if_fizz("fizz");
  foo_if_fizz("fizzish");
}