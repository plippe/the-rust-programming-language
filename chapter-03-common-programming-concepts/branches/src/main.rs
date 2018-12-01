fn if_expressions() {
    let number = 5;

    if number < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }

    if number != 0 {
        println!("The number was something other than zero");
    }

    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by ");
    } else {
        println!("The number is not divisible by 4, 3, or 2");
    }

    let number = if true { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn repetition_with_loops() {
    let mut number = 3;
    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn main() {
    if_expressions();
    repetition_with_loops();
}
