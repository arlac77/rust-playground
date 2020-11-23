use std::cmp::Ordering;


fn compare( a: &usize,b: &usize)
-> Ordering
{
    println!("compare {:?}<>{:?}",a,b);

    return b.cmp(a); //Ordering::Equal;
}


fn main() {
  let mut x = vec![1,2,3];

  print!("{:?}",x);
  x.sort_by(compare);
  print!("{:?}",x);
 }

