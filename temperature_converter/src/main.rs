use std::io;                     

fn main() {
    loop {                       
        println!("Please select which way you'd like to convert");
        println!("_____________________________________________");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");

        let mut convert_type = String::new();   
        io::stdin().read_line(&mut convert_type)    
            .expect("Failed to read line");
        let convert_type = convert_type.trim();
        let convert_type = match convert_type {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("Please input 1 or 2!");
                continue;
            }
        };

        println!("Please input the temperature: ");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");
        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid temperature");
                continue;
            }
        };

        let converted_temperature = if convert_type == 1 {
            (temperature - 32.) / 1.8
        } else {
            temperature * 1.8 + 32.
        };
        println!("The converted temperature is {}", converted_temperature);
    }
}