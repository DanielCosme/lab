use std::collections::HashMap;

fn main() {
  play();
  play_vec();
}

fn play_vec() {
  let mut list: Vec<u32> = vec![
    1, 1, 2, 3, 4, 5, 6, 7, 10, 2, 5, 8, 9, 9, 11, 9, 3, 5, 9, 9, 1,
  ];
  list.sort();

  println!("{:?}", list);
  let half = list.len() / 2;
  let median = list.get(half).unwrap().clone();

  let mut mode: u32 = 0;
  let mut count: HashMap<u32, u32> = HashMap::new();
  for n in list {
    let num = count.entry(n).or_insert(0);
    *num += 1;
  }
  println!("count {:?}", count);
  for (k, v) in count {
    if v > mode {
      mode = k
    }
  }

  println!("median: {}", median);
  println!("mode: {}", mode);
}

fn play() {
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2);
  v.push(4);

  for i in 0..v.len() {
    println!("index: {} - value: {}", i, v[i]);
  }

  let mut s = String::from("Hello ");
  s.push_str("World");
  s = s + " from daniel";
  println!("{}", s);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{s1}-{s2}-{s3}");
  println!("{}", s);

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  scores.entry(String::from("Green")).or_insert(60);

  let score = scores.get("Blue").copied().unwrap_or(0);
  println!("Score of Blue: {}", score);

  for (key, value) in &scores {
    println!("{key}: {value}");
  }

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("{:?}", map)
}
