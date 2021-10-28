use std::io::{stdin, stdout, Write};

pub fn prompt(prompt: &str) -> anyhow::Result<String> {
  let mut response = String::new();

  print!("{}? ->", prompt);
  stdout().flush()?;
  stdin().read_line(&mut response).expect("Failed to read line");
  Ok(response)
}
