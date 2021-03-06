use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::result::Result;

struct Token {}

struct AddToken {}
impl AddToken {
  pub fn new() -> Token {
    Token {}
  }
}

struct SubToken {}
impl SubToken {
  pub fn new() -> Token {
    Token {}
  }
}

struct Cell {
  data: u64,
}

struct Instruction {}

fn read_file(file_name: String) -> Result<String, Error> {
  let mut file = File::open(file_name)?;

  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;

  Ok(String::from_utf8(buffer).unwrap())
}

fn tokenize(contents: String) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();
  for c in contents.chars() {
    match c {
      '+' => {
        tokens.push(AddToken::new());
        println!("ADD");
      }
      '-' => {
        tokens.push(SubToken::new());
        println!("SUBTRACT");
      }
      _ => {
        continue;
      }
    }
  }

  tokens
}

fn optimize(tokens: Vec<Token>) -> Vec<Instruction> {
  Vec::new()
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_name: String = args[1].parse().unwrap();

  match read_file(file_name) {
    Err(_e) => {
      println!("File not found");
    }
    Ok(contents) => {
      //      let mut cells: Vec<Cell> = Vec::new();
      let tokens = tokenize(contents);
      let optimized_instructions = optimize(tokens);
    }
  }

  println!("Hello, world!");
}
