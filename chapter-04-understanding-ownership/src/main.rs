fn string_type() {
    let mut s = String::from("Hello");
    s.push_str(", world!");

    println!("{}", s);
}

fn clone() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn ownership() {
    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn make_copy(some_integer: i32) {
        println!("{}", some_integer);
    }

    fn calculate_lenght(s: &String) -> usize {
        s.len()
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world!");
    }

    let mut s = String::from("Hello");
    // takes_ownership(s);

    let x = 5;
    make_copy(x);

    println!("The lenght of '{}' is {}.", s, calculate_lenght(&s));

    change(&mut s);
    println!("The lenght of '{}' is {}.", s, calculate_lenght(&s));
}

fn slice() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    fn second_word(s: &str) -> &str {
        let first = first_word(s);
        let s_without_first = s[first.len()..].trim();

        let bytes = s_without_first.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s_without_first[..i];
            }
        }

        &s_without_first[..]
    }

    let s = String::from("Hello, world!");
    println!("The first word is: {}", first_word(&s[..]));
    println!("The second word is: {}", second_word(&s[..]));
}

fn main() {
    string_type();
    clone();
    copy();
    ownership();
    slice();
}
