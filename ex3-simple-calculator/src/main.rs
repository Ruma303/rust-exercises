use std::io;

fn main() {
  println!("Simple Calculator");
  println!("Available operations: +, -, *, /");
  println!("Enter your expression (e.g., 5 + 3):");

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  let tokens: Vec<&str> = input.trim().split_whitespace().collect();

  if tokens.len() != 3 {
    println!("Invalid input format. Please use the format 'number operator number'.");
    return;
  }

  let num1 = match tokens[0].parse::<f64>() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid number format.");
      return;
    }
  };

  let operator = tokens[1];

  let num2 = match tokens[2].parse::<f64>() {
    Ok(num) => num,
    Err(_) => {
      println!("Invalid number format.");
      return;
    }
  };

  let result = match operator {
    "+" => num1 + num2,
    "-" => num1 - num2,
    "*" => num1 * num2,
    "/" => {
      if num2 == 0.0 {
        println!("Division by zero is not allowed.");
        return;
      }
      num1 / num2
    }
    _ => {
      println!("Invalid operator. Supported operators are +, -, *, and /.");
      return;
    }
  };

  println!("Result: {}", result);
}