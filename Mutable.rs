fn main() {
  let mut s = String::from("hello"); // s is the mutable
let r1 = &mut s; // r1 is a mutable reference
printIn! ("{}", r1);
}
