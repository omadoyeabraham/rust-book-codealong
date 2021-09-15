//! Temperature converter between celsius and fahrenheit
//!

use std::io;

fn main() {
    println!("---- CONVERT BETWEEN CELSIUS AND FAHRENHEIT ----");

    let mut temp_unit = String::new();
    let mut temp_value = String::new();

    // Get the temperature unit
    let temp_unit = loop {
        println!("Enter your temperature unit (c/f)");

        io::stdin()
        .read_line(&mut temp_unit)
        .expect("Could not read the temperature unit provided");
        let unit = temp_unit.trim().chars().last().unwrap().to_lowercase().to_string();

        if unit != "c" && unit != "f" {
            println!("Please provide either c (celsuis) or f (fahrenheit) as the temp unit");
            continue;
        } else {
            break unit;
        }
    };

    // Get the temperature value
    let temp_value = loop {
        println!("Enter your temperature value");

        io::stdin()
        .read_line(&mut temp_value)
        .expect("Could not correctly read your temp value");
        let value: f64 = match temp_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid number as your temperature value");
                continue;
            }
        };

        break value;
    };

    // Display the conversion result
    match temp_unit.as_str() {
        "c" => {
            println!("{}C is {}F", temp_value, (temp_value * 1.8) + 32.0)
        },
        "f" => {
            println!("{}F is {}C", temp_value, (temp_value - 30.0) / 2.0)
        },
        _ => {
            println!("Wrong unit provided. Please compute again !!!")
        }
    }
}
