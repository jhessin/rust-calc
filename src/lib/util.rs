macro_rules! parse_input {
  ($x:expr, $t:ident) => {{
    #[allow(unused_imports)]
    use anyhow::Context;
    #[allow(unused_imports)]
    use std::io::{stdin, stdout, Write};
    let mut response = String::new();
    print!("{}? ->", $x);
    stdout().flush().unwrap();
    stdin().read_line(&mut response).expect("Failed to read line");
    response.trim().parse::<$t>().context("Couldn't parse input")
  }};
  ($x:expr) => {{
    use anyhow::Context;
    use std::io::{stdin, stdout, Write};
    let mut response = String::new();
    print!("{}? ->", $x);
    stdout().flush().unwrap();
    stdin().read_line(&mut response).expect("Failed to read line");
    response.trim().parse().context("Couldn't parse input")
  }};
}

macro_rules! prompt {
  ($p:expr, $t:ident) => {
    loop {
      if let Ok(result) = parse_input!($p, $t) {
        break result;
      }
    }
  };
  ($p:expr) => {
    loop {
      if let Ok(result) = parse_input!($p) {
        break result;
      }
    }
  };
}
