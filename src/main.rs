fn main() {
    // let mut s = String::from("hello");
    //
    // s.push_str(", world!");
    //
    // // let x = s; // s moved in x
    //
    // println!("{}", s)
    //
    // // s is no longer available here

    let m = Message::Write(String::from("Hello World"));
    m.call()
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
}

impl Message {
    fn call(&self) {
        dbg!(&self);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions
    // rect.area()
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated function without self has first params
    // are mostly used as constructors
    // Rectangle::square(3)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn return_value_and_scope() {
    let s1 = gives_ownership(); // return value moves into s1

    let s2 = String::from("Hello");

    let s3 = takes_and_give_back(s2); // s2 moves into the scope of takes_and_give_back
                                      // it returns back and moves into s3
                                      // s3 and s1 goes out of scope and are dropped, s2 no longer exist so nothing happens
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn string_slices() {
    let s = String::from("Hello world");

    let hello = &s[0..5]; // [..2]
    let world = &s[6..11]; // [6..0]
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn create_user() {
    let mut user1 = User {
        active: true,
        username: String::from("ogaboss"),
        email: String::from("oga@boss.com"),
        sign_in_count: 2,
    };

    user1.email = String::from("another@boss.com");
}
