#[macro_use]
mod macros;

mod foo;

let mut a:u16 = 33;


pub use foo::Bar;

pub fn foo() {
  let _ = fmt!("...");
  println!("Hi");
}
