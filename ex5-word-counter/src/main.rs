use std::env;
use std::fs::File;
use std::io::Read;

fn count_words(text: &str) -> usize {
  text.split_whitespace().count()
}

fn main() {

  // Collect command-line arguments
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    eprintln!("Usage: cargo run <file_path>");
    return;
  }

  let file_path = &args[1];
  println!("Reading fiule: {}", file_path);

  // Read the content
  let mut file = match File::open(file_path) {
    Ok(file) => file,
    Err(err) => {
      eprintln!("Error opening file: {}", err);
      return;
    }
  };
  
  // Get content
  let mut contents = String::new();
  
  if let Err(err) = file.read_to_string(&mut contents) {
    eprintln!("Error reading file: {}", err);
    return;
  }
  
  // Count words
  let word_count = count_words(&contents);
  println!("Word Count: {}", word_count);
}
