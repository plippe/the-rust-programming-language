fn mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
}

fn floating_point() {
    let x = 2.0;
    let y: f32 = 3.0;
}

fn numeric_operations() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
}

fn boolean_type() {
    let t = true;
    let f: bool = false;
}

fn character_type() {
    let c = 'z';
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

fn main() {
    mutable();
    shadowing();
    floating_point();
    numeric_operations();
    boolean_type();
    character_type();
    tuple_type();
    array_type();
}
