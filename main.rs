 fn main() {
   //let guess:i128 = "42".parse().expect("Not a number!");
     // println!("x={}", guess);

   let x = parse("xxx");
   println!("x={}", x);
 }

fn parse(value : &str) -> i64 {

  if value.len() == 0 {
   return 1
  }
   return 0
}