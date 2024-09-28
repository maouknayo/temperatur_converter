use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    loop {
        println!("1: Convert Celsius to Fahrenheit");
        println!("2: Convert Celsius to Fahrenheit");

        print!("Selection: ");

        io::stdout().flush().expect("Failed to flush");

        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter an Integer please");
                continue;
            }
        };

        if selection == 1 {
            loop {
                clear_terminal_screen();

                print!("Enter Celsius to convert: ");
                io::stdout().flush().expect("Failed to flush");

                let mut celsius = String::new();

                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Failed to read line");

                let celsius: f32 = match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter an Integer please");
                        continue;
                    }
                };

                let result = convert_celsius_to_fahrenheit(celsius);

                println!("Fahrenheit: {result}");
                break;
            }
        } else if selection == 2 {
            loop {
                clear_terminal_screen();

                print!("Enter Fahremheit to convert: ");
                io::stdout().flush().expect("Failed to flush");
                let mut fahrenheit = String::new();

                io::stdin()
                    .read_line(&mut fahrenheit)
                    .expect("Failed to read line");

                let fahrenheit: f32 = match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter an Integer please");
                        continue;
                    }
                };

                let result = convert_fahrenheit_to_celsius(fahrenheit);

                println!("Celsius: {result}");
                break;
            }
        } else {
            println!("Enter 1 or 2 please")
        }
    }
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(["/c", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}
