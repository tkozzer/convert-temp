use std::io;
use std::{thread, time};

fn main() {
    println!("Welcome to the ConvertTemp app!");
    println!("Please enter a temperature followed by either a C or F denoting celsius or farenheit respectively: ");
    loop {
        let mut input = String::new();

        
        io:: stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
        if input.trim().eq_ignore_ascii_case("quit") || input.trim().eq_ignore_ascii_case("q"){
            break;
        }
    
        let temp = get_temp(&input);
        let temp_type = get_temp_type(&input); 
    
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nConverting temperature...");
        thread::sleep(time::Duration::from_millis(1000));
    
        if temp_type.eq_ignore_ascii_case("f") {
            let temp_in_celsius = convert_from_farenheit_to_celsius(temp);
            println!("{temp}°F converted to celsius is: {temp_in_celsius:.2}°C \n");
        } else if temp_type.eq_ignore_ascii_case("c") {
            let temp_in_farenheit = convert_from_celsius_to_farenheit(temp);
            println!("{temp}°C converted to farenheit is: {temp_in_farenheit:.2}°F \n");
        } else {
            println!("Sorry this is an invalid temp type :<>()");
        }
        
        println!("Enter another temp (ex: 32f or 108C) or enter 'quit' or 'q' to exit:")
    }
}

fn get_temp_type(value: &str) -> &str {
    let last_char: &str = &value[value.len() - 2..value.len() - 1];
    last_char
}

fn get_temp(value: &str) -> &str {
    let temp: &str = &value[0..value.len()-2];
    temp
}

fn convert_from_farenheit_to_celsius(temp: f64) -> f64 {
    let celsius: f64 = 5.0 / 9.0 * (temp as f64 - 32f64);
    celsius
}

fn convert_from_celsius_to_farenheit(temp: f64) -> f64 {
    let farenheit: f64 = (temp as f64 * 9.0 / 5.0) + 32f64;
    farenheit
}
