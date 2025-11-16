// Palindrome words for testing: ignore white spaces, capitalization, punctualization

// racecar
// A man, a plan, a canal, Panama

use std::io::{self, Write};

/// Legge una riga da stdin e la restituisce come String, gestendo il caso di input vuoto.
fn get_input() -> String {
    let mut input = String::new();
    print!("Inserisci una frase da controllare: ");
    
    io::stdout().flush().unwrap(); // Assicura che il prompt venga stampato prima della lettura
    io::stdin().read_line(&mut input).expect("Errore nella lettura dell'input");
    
    let trimmed = input.trim();
    if trimmed.is_empty() {
        eprintln!("Per favore inserisci una stringa non vuota.");
        std::process::exit(1);
    }
    
    trimmed.to_string()
}

/// Rimuove spazi, punteggiatura e converte la stringa in minuscolo.
fn normalize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

/// Verifica se una stringa normalizzata è palindroma.
fn is_palindrome(input: &str) -> bool {
    let reversed: String = input.chars().rev().collect();
    input == reversed
}

fn main() {
    println!("Palindrome checker.\n");

    let original_input = get_input();
    let normalized = normalize_input(&original_input);

    if is_palindrome(&normalized) {
        println!("\"{}\" è una frase palindroma (normalizzata: \"{}\")", original_input, normalized);
    } else {
        println!("\"{}\" NON è una frase palindroma (normalizzata: \"{}\")", original_input, normalized);
    }
}