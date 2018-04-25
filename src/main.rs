use std::mem;

fn datatypes() {
  // Unsigned 8-bit integer binding set to 12, binding defaults to immutable
  let a: u8 = 12;
  // Signed 8-bit integer binding set to -1, binding defaults to immutable
  let b: i8 = -1;
  // Signed 8-bit integer binding set to -1, binding is mutable (mut)
  let mut c: i8 = 23;

  println!("Hello, world!");

  println!("a = {}", a);
  println!("b = {}", b);
  println!("c = {}", c);

  c = 42;

  println!("c(2nd time) = {}", c);

  // Type inferred to i32
  let d = 123456789;
  println!("d = {}, size is {} bytes", d, mem::size_of_val(&d));

  //isize is size of pointer (signed) which will differ between 32 bit vs 64 bit machines
  // could also use usize for unsigned pointer value
  let e: isize = 123;
  println!("e = {}, size is {} bytes", e, mem::size_of_val(&e));

  let f = 'a';
  // characters in rust are biggest size for unicode - i.e. 4 bytes. Not 1 byte ala ascii or 2-bytes utf
  println!("f = {}, size is {} bytes", f, mem::size_of_val(&f));

  // Inferred to double precision or f64
  let g = 2.5;
  println!("Inferred double: g = {}, size is {} bytes", g, mem::size_of_val(&g));

  // Inferred to double precision or f64
  let h = true;
  println!("Boolean: h = {}, size is {} bytes", g, mem::size_of_val(&g));

  /*
  Types:
  i8/u8
  i16/u16
  i32/u32
  i64/u64
  f32
  f64
  bool (1 byte)
  char (4 bytes)
  */
}

fn main() {
  datatypes();
}
