 fn main() {
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
