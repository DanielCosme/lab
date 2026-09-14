use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::result::Result::Ok;

fn main() {
  unsafe {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("SHOULD_PANIC", "0");
  }

  may_panic(); // Can panic.
  open(); // Can panic.
  println!("Username: {}", read_username_from_file().unwrap());
  println!("Username: {}", read_username_from_file_mini().unwrap());
  println!("Username: {}", read_username_from_file_micro().unwrap());
  println!("Username: {}", read_username_from_file_pico().unwrap());
}

fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("hello.txt");
  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut username = String::new();
  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}

fn read_username_from_file_mini() -> Result<String, io::Error> {
  let mut username_file = File::open("user.txt")?;
  let mut username = String::new();
  username_file.read_to_string(&mut username)?;
  Ok(username)
}

fn read_username_from_file_micro() -> Result<String, io::Error> {
  let mut username = String::new();
  File::open("user.txt")?.read_to_string(&mut username)?;
  Ok(username)
}

fn read_username_from_file_pico() -> Result<String, io::Error> {
  fs::read_to_string("user.txt")
}

fn open() {
  let file = File::open("hello.txt");
  match file {
    Ok(f) => println!("is file?: {}", f.metadata().unwrap().is_file()),
    Err(e) => println!("err: {}", e),
  }

  let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {error:?}");
      })
    } else {
      panic!("Problem opening the file: {error:?}");
    }
  });

  let mut greeting_file = match File::open("hello.txt") {
    Ok(f) => f,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => {
          println!("File created");
          fc
        }
        Err(e) => {
          panic!("Problem creating the file: {:?}", e)
        }
      },
      _ => panic!("Problem opening the file: {:?}", err),
    },
  };

  let mut buf = String::new();
  greeting_file
    .read_to_string(&mut buf)
    .expect("failed to parse file input");
  println!("buf: {}", buf)
}

fn may_panic() {
  let should_panic = env::var_os("SHOULD_PANIC").unwrap();
  if should_panic == "1" {
    panic!("PANIC: crash and burn");
  }
  println!("YES! I didin't panic");
}
