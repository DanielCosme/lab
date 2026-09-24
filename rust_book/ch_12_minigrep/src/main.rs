use minigrep::{search, search_case_insensitive};
use std::error::Error;
use std::{env, fs, process};

fn main() {
  // Panics if any argument contains invalid Unicode.
  let args: Vec<String> = env::args().collect();
  let config = Config::parse(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  if let Err(e) = run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1)
  }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in results {
    println!("{line}");
  }
  Ok(())
}

struct Config {
  query: String,
  file_path: String,
  pub ignore_case: bool,
}

impl Config {
  fn parse(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok_and(|v| if v == "0" { false } else { true });

    Ok(Config {
      query,
      file_path,
      ignore_case,
    })
  }
}
