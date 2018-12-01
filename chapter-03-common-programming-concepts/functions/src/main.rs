fn main() {
    another_function(5, 6);
    expresssions();
    return_value();
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn expresssions() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn return_value() {
    fn five() -> i32 {
        5
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(x);
    println!("The value of x is: {}", x);
}
