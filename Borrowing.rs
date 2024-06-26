// Immutable Borrowing

fn main() {
let s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
printIn! ("{} and {}", r1, r2); // r1 and r2 are no longer used after this point
}