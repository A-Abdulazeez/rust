use std::io;

fn main() {

    println!("Hello, world!");

    let x = 5;
    println!("The value of x is: {}", x);

    let name = "Azeez";
    println!("your name is {}", name);

    let celcius : f64 = 10.0;
    let fahrenheit : f64 = (celcius * 9.0/5.0) + 32.0;

    println!("The value of fahrenheit is {}", fahrenheit);

    println!("Enter your temperature in Celsius: ");
    let mut user_input = String::new();
    io::stdin().read_line( & mut user_input ).expect("Failed to read line");

    let celsius: f64 = user_input.trim().parse().expect("Not a number!");


    let fahrenheit : f64 = (celsius * 9.0/5.0) + 32.0;
    println!("The Fahrenheit is {}", fahrenheit);

    println!("Enter Your Birth Year: ");
    let mut user_input = String::new();
    io::stdin().read_line( & mut user_input).expect("Failed to read line");

    let year:i32 = user_input.trim().parse().expect("Not a number");

    let age = 2026 - year;
    println!("Your age: {}", age);

    println!("Enter item price: ");
    let mut price_input = String::new();
    io::stdin().read_line( & mut price_input).expect("Failed to read line");
    let price: i32 =  price_input.trim().parse().expect("Failed to parse into number");

    println!("Enter quantity: ");
    let mut quantity_input = String::new();
    io::stdin().read_line( & mut quantity_input).expect("Failed to read line");
    let quantity: i32 =  quantity_input.trim().parse().expect("Failed to parse into number");


    let total = price * quantity;
    println!("Your total is {:.2}", total);






}