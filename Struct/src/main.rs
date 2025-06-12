
// Struct in Rust : Structs are used to create custom data types.
// A struct is a custom data type that lets you package together related data.

#[derive(Debug,Clone)]
// The `derive` attribute automatically implements the `Debug` trait for the struct,
// allowing it to be printed using the `{:?}` format specifier.
// The `Debug` trait is useful for debugging purposes, enabling formatted output of the struct's fields.
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,   
}
// Tuple structs are a special kind of struct that allows you to define a struct with named fields.
// They are similar to regular structs, but they do not have named fields.
// Tuple structs are useful when you want to create a simple data structure without the need for named fields.
// Tuple structs are defined using parentheses instead of curly braces.
#[derive(Debug)]
struct Color(u8, u8, u8);
fn main() {
    //example_1();
    //example_2();
}
// The `example_1` function demonstrates how to create and manipulate a `User` struct.
// It initializes a `User` instance, modifies one of its fields, and prints the struct's contents.
// Shows how to handle immutability in Rust.
fn example_1() {
    let mut user1 = User {
        username: String::from("john_doe"),
        email: String::from("toanle2550@gmail.com"),  
        age: 30,
        active: true, 
    };

    user1.active = false; // This line will cause a compile-time error because `user1` is immutable.
    // To fix this, we need to make `user1` mutable by adding the `mut` keyword when declaring it.


    print!("User 1: {:#?}\n", user1);// The `#?` format specifier is used to print the struct in a more readable, multi-line format.

}
fn example_2() {
    let mut user1 = User {
        username: String::from("john_doe"),
        email: String::from("toanle2550@gmail.com"),  
        age: 30,
        active: true, 
    };
    
    //let username = user1.username; cmment this line to avoid moving the `username` field out of `user1`.

    print!("User 1: {:#?}\n", user1);// This will cause a compile-time error because `username` is moved out of `user1`.
    // To fix this, we can either clone the `username` field or use a reference to it.
    // Here, we will clone the `username` field to avoid moving it out of `user1`.
    let username = user1.username.clone();

    print!("Username: {}\n", username); // Now we can use `username` without affecting `user1`.
    
}
// The `example_3` function demonstrates how to use the `..` syntax to copy fields from one struct instance to another.
// This syntax allows you to create a new struct instance while reusing fields from an existing instance.
// It is useful when you want to create a new instance with some fields initialized to the same values as another instance.
// The `..` syntax is a shorthand for copying all remaining fields from the specified instance.
// In this case, `user2` will have the same `email`, `age`, and `active` fields as `user1`, but a different `username`.
// The `clone` method is used to create a copy of `user1` so that we can use its fields without moving them.
// This is useful when you want to create a new instance with some fields initialized to the same values as another instance.
// The `clone` method is necessary because the `..` syntax requires the fields to be copied, not moved.
// The `clone` method is used to create a copy of `user1` so that we can use its fields without moving them.
fn example_3(){
       let mut user1 = User {
        username: String::from("john_doe"),
        email: String::from("toanle2550@gmail.com"),  
        age: 30,
        active: true, 
    };

    let user2 = User {
        username: String::from("jane_doe"),
        ..user1.clone() // This line uses the `..` syntax to copy the remaining fields from `user1` into `user2`.
    };
}

fn example_4(){
    let color = Color(255, 0, 0); // Create a new instance of the `Color` tuple struct with RGB values for red.
    println!("Color: ({}, {}, {})", color.0, color.1, color.2); // Access the fields of the tuple struct using dot notation.
}



//Method & Associated Functions
// In Rust, methods are functions that are associated with a struct or an enum.
// They are defined within the `impl` block of the struct or enum.
struct Circle {
    radius: f64,
}

impl Circle {
    // The `new` function is a constructor that creates a new instance of the `Circle` struct.
    // It takes a `radius` parameter and returns a new `Circle` instance with the specified radius.
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // The `area` function calculates the area of the circle using the formula πr².
    // It returns the area as a `f64` value.
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    
    fn set_radius(&mut self, radius: f64) {
        self.radius = radius; // This method allows you to change the radius of the circle.
    }
}

// Trait : Traits in Rust are a way to define shared behavior across types.
trait Greet {
    fn say_hello(&self); // The `greet` method is defined in the `Greet` trait.
}
struct Person {
    name: String,
    age: u32,
}

impl  Greet for Person {
    // The `say_hello` method is implemented for the `User` struct.
    // It prints a greeting message that includes the user's username.
    fn say_hello(&self) {
        println!("Hello, {}!", self.name);
    }
    
}
