 fn main() {
  let mut sequence = 0..3;

  println!("Four consecutive `next` calls on 0..3");
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
 }

fn parse(value : &str) -> i64 {
  if value.len() == 0 {
   return 1
  }
   return 0
}