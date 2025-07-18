use std::io;

fn menu() {
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("3. Exit");
    println!("Enter your choice: ");
    println!("\n---------------\n");
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn read_line_from_stdin() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    loop {
        menu();

        let choice_input = read_line_from_stdin();
        let choice: u32 = match choice_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number from 1 to 3.\n");
                continue;
            }
        };

        match choice {
            1 => {
                println!("You selected: Celsius to Fahrenheit");
                println!("Enter temperature in Celsius: ");
                let temp_input = read_line_from_stdin();
                let celsius: f32 = match temp_input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid temperature. Try again.\n");
                        continue;
                    }
                };
                let fahrenheit = convert_celsius_to_fahrenheit(celsius);
                println!("{:.2}°C is {:.2}°F\n", celsius, fahrenheit);
            }
            2 => {
                println!("You selected: Fahrenheit to Celsius");
                println!("Enter temperature in Fahrenheit: ");
                let temp_input = read_line_from_stdin();
                let fahrenheit: f32 = match temp_input.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid temperature. Try again.\n");
                        continue;
                    }
                };
                let celsius = convert_fahrenheit_to_celsius(fahrenheit);
                println!("{:.2}°F is {:.2}°C\n", fahrenheit, celsius);
            }
            3 => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, or 3.\n");
                continue;
            }
        }

        println!("Do you want to convert another temperature? (y/n): ");
        let retry = read_line_from_stdin();

        if retry.to_lowercase() != "y" {
            println!("Goodbye!");
            break;
        }

        println!(); // riga vuota per separare i cicli
    }
}