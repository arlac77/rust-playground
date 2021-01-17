
#[allow(unused_macros)]

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

#[allow(unused_macros)]

macro_rules! count {
    ($($tts:tt)*) => {0usize $(+ replace_expr!($tts 1usize))*};
}

#[allow(unused_macros)]

macro_rules!m1 {
  ($x:expr ) => { println!("{:?}",$x); };
}

#[allow(unused_macros)]

macro_rules!m2 {
  ( $( $x:expr ),* ) => { println!("{:?}", $( $x ),* ); };
}

macro_rules! write_html {
  ( ) => (());

  ( $e:tt) => (println!( "{}", $e));

  ($tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
    println!( "<{}>", stringify!($tag));
      write_html!( $($inner)*);
      println!( "</{}>", stringify!($tag));
      write_html!( $($rest)*);
  }};
}


fn main() {
  write_html!(
    html[
        head[title["Macros guide"]]
        body[h1["Macros are the best!"]]
    ]);
}
	
