use crate::ape::MyStruct;

mod ape {
  #[derive(Debug)]
  pub struct MyStruct {
    pub name: String,
  }
}

fn main() {
  let mut s = MyStruct {
    name: "World".to_string(),
  };
  println!("Hello {}", s.name);

  s.name = "other name".to_string();

  println!("Hello {}", s.name);
}
