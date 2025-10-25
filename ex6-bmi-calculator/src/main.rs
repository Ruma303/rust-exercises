use std::io;
use std::fmt::{Display, Formatter, Result};

fn main() {
    let weight = loop {
        println!("Insert your weight in kilograms (Kg): ");
        match get_input_as_f64() {
            Some(value) if validate_input(value, "Weight", 0.0) => break value,
            _ => println!("Invalid input. Please enter a positive number."),
        }
    };

    let height_meters = loop {
        println!("Insert your height in centimeters (cm): ");
        match get_input_as_f64() {
            Some(value) if validate_input(value, "Height", 0.0) => break value / 100.0,
            _ => println!("Invalid input. Please enter a positive number."),
        }
    };

    let user_bmi = calculate_bmi(weight, height_meters);
    let bmi_class = classify_bmi(user_bmi);

    println!("Your BMI is {:.2}. You're classified as {}.", user_bmi, bmi_class);
}

enum BMI {
    UnderWeight,
    Normal,
    OverWeight,
    Obese,
}

impl Display for BMI {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BMI::UnderWeight => write!(f, "Underweight"),
            BMI::Normal => write!(f, "Normal weight"),
            BMI::OverWeight => write!(f, "Overweight"),
            BMI::Obese => write!(f, "Obese"),
        }
    }
}

fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    input.trim().parse::<f64>().ok()
}

fn validate_input<T>(value: T, field_name: &str, min: T) -> bool
where
    T: PartialOrd + Display,
{
    if value <= min {
        println!("{} cannot be less than or equal to {}.", field_name, min);
        false
    } else {
        true
    }
}

fn calculate_bmi(weight: f64, height_m: f64) -> f64 {
    weight / (height_m * height_m)
}

fn classify_bmi(bmi: f64) -> BMI {
    if bmi < 18.5 {
        BMI::UnderWeight
    } else if bmi < 25.0 {
        BMI::Normal
    } else if bmi < 30.0 {
        BMI::OverWeight
    } else {
        BMI::Obese
    }
}