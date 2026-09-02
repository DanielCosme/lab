fn main() {
  for i in 1..=10 {
    println!("fibo of {}: {}", i, fibo(i));
  }
  for i in -40i32..=0 {
    println!("C: {}: F: {:.2}", i, c_to_f(i));
  }

  examples()
}

fn c_to_f(celsius: i32) -> f32 {
  celsius as f32 * 1.8 + 32.0
}

fn fibo(n: u128) -> u128 {
  if n == 0 {
    0
  } else if n == 1 {
    1
  } else {
    let mut prev2 = 0;
    let mut prev1 = 1;

    for _ in 2..=n {
      let current = prev1 + prev2;
      prev2 = prev1;
      prev1 = current;
    }
    prev1
  }
}

fn examples() {
  // Tuple example.
  let tup = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of y is: {y} x: {x} z: {z}");

  // Loops are expressions. And the can have labels to make break target the specified labed.
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");

  for number in (1..4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!!!");
}
