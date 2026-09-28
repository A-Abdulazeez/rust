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

    println!("Welcome to Score Grader");
    println!("Please enter your score: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let score: u32 = user_input.trim().parse().expect("Please enter a number");

    match score {
        80 ..= 100 => println!("A"),
        70 ..= 79 => println!("B"),
        60 ..=  69 => println!("C"),
        50 ..= 59 => println!("D"),
        0 ..=  49 => println!("F"),
        _ => println!("Not a valid score"),

    }

    println!("Enter a number: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let number: i32 = user_input.trim().parse().expect("Not a number");

    if number > 0  {println!("{} is positive", number);}
    else if number < 0 {println!("{} is negative", number);}
    else {println!("{} is zero", number);}

    if number % 2 == 0{println!("{} is an even number", number); }
    else{println!("{} is an odd number", number);}

    or
    match number {
        n if n > 0 => println!("{} is positive", n),
        n if n < 0 => println!("{} is negative", n),
        _ => println!("{} is Zero", number),
    }

    match number % 2 {
        0 => println!("{} is Even", number),
        _ => println!("{} is odd", number),
    }

    println!("Mini calculator");
    println!("enter first number: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let first_number:i32 = user_input.trim().parse().expect("Not a number");

    println!("enter operator: ");
    let mut input_operator = String::new();
    io::stdin().read_line(&mut input_operator).expect("Failed to read line");
    let operator = input_operator.trim();

    println!("enter second number: ");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Failed to read line");
    let second_number:i32 = input_number.trim().parse().expect("Not a number");

    match operator {
        "+" => println!("The answer is {}", first_number + second_number),
        "-" => println!("The answer is {}", first_number - second_number),
        "*" => println!("The answer is {}", first_number * second_number),
        "/" => println!("The answer is {}", first_number / second_number),
        _ => println!("Not an operator"),
    }

    println!("Number Guessing Game");
    let correct_number: u32 = 9;
    println!("Please input your guess: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let guess: u32 = user_input.trim().parse().expect("Please type a number!");

    if guess == correct_number {
        println!("Correct guess!");
    }
    else {
        println!("Wrong guess!");
    }






}