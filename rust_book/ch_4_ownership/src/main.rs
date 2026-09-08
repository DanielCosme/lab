fn main() {
  weird_borrow();
  strings();
  stringify();
  let mut dst = vec!["dan".to_string(), "cos".to_string(), "anastasi".to_string()];
  let src = [String::from("one_super_long_long_long")];
  add_big_strings(&mut dst, &src);
  for s in dst {
    println!("{s}");
  }

  let mut dst = vec!["dan".to_string(), "cos".to_string(), "anastasi".to_string()];
  let src = [String::from("one_super_long_long_long")];
  add_big_strings_2(&mut dst, &src);
  for s in dst {
    println!("{s}");
  }

  let mut dst = vec!["dan".to_string(), "cos".to_string(), "anastasi".to_string()];
  let src = [String::from("one_super_long_long_long")];
  add_big_strings_3(&mut dst, &src);
  for s in dst {
    println!("{s}");
  }

  rem();
  mut_dif();
  str_slice();
}

fn str_slice() {
  let s = String::from("hello world");
  let hello: &str = &s[..5];
  let world: &str = &s[6..];
  println!("{hello} {world}")
}

fn mut_dif() {
  let mut a = [0, 1, 2, 3];
  let x = &mut a[1];
  *x += 1;
  println!("{a:?}");
}

fn rem() {
  let mut v: Vec<String> = vec![String::from("Hello world")];
  let mut s: String = v.remove(0);
  s.push('!');
  println!("{s}");
  assert!(v.len() == 0);
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
  let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
  for s in src {
    if s.len() > largest.len() {
      dst.push(s.clone());
    }
  }
}

fn add_big_strings_2(dst: &mut Vec<String>, src: &[String]) {
  let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
  for s in src {
    if s.len() > largest_len {
      dst.push(s.clone());
    }
  }
}

fn add_big_strings_3(dst: &mut Vec<String>, src: &[String]) {
  let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
  let to_add: Vec<String> = src
    .iter()
    .filter(|s| s.len() > largest.len())
    .cloned()
    .collect();
  dst.extend(to_add);
}

fn weird_borrow() {
  let mut v: Vec<i32> = vec![1, 2, 3];
  let num: &mut i32 = &mut v[2];
  let num2: &i32 = &*num;
  println!("{} {}", *num, *num2);
}

fn stringify() {
  let name = vec![String::from("Ferris")];
  let first = &name[0];
  let name = stringify_name_with_title(&name);
  println!("{}", first);
  println!("{}", name);
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
  let mut full = name.join(" ");
  full.push_str(" Esq.");
  full
}

fn strings() {
  let string_1 = return_a_string_1();
  println!("{string_1}");

  let string_2 = return_a_string_2();
  println!("{string_2}");

  let string_3 = return_a_string_3();
  println!("{string_3}");

  let mut string_4 = String::new();
  return_a_string_4(&mut string_4);
  println!("{string_4}");
}

fn return_a_string_1() -> String {
  let s = String::from("Hello world");
  s
}

fn return_a_string_2() -> &'static str {
  "Hello World 2"
}

fn return_a_string_3() -> std::rc::Rc<String> {
  let s = std::rc::Rc::new(String::from("Hello World 3"));
  std::rc::Rc::clone(&s)
}

fn return_a_string_4(out: &mut String) {
  out.replace_range(.., "Hello world 4");
}
