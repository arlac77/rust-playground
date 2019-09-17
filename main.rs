 fn main() {
    let  x = 5;
    let y = &x;
    //let z = &mut x; // This was not allowed before 1.31.0.
    let z = &x; // This was not allowed before 1.31.0.
 
    println!("x={} y={} z={}", x,y, z);
 }

