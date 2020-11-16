 fn main() {
   let m = Mask::I;

   if m == Mask::I {
    println!("I");
  }
   else {
    println!("?");
  }
 }


#[derive(PartialEq, Eq, Clone, Copy)]
enum Mask {
  M = 0,
  I = 1,
  V = 2
}



#[allow(unused_variables)]

fn main2()
{
  let mut sequence = 0..3;

  println!("Four consecutive `next` calls on 0..3");
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());

  let text = "word1 word2 word3";
  println!("{}", to_words(text).take(2).count());
}

fn to_words<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
  text.split(' ')
}

fn parse(value : &str) -> i64 {
  if value.len() == 0 {
   return 1
  }
   return 0
}
