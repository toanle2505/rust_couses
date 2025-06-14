// Lifetime : explaining the concept of lifetimes in Rust
// This is a simple Rust program that demonstrates the concept of lifetimes.
// It defines a function that takes two string slices and returns the longer one.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/*
fn longest(s1: & str, s2: & str) -> & str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
    =? Error: the lifetime of the returned reference is not specified
    => This function takes two string slices as input and returns a reference to the longer one.
*/
// The lifetime 'a indicates that the returned string slice will live at least as long as both input slices.
// This ensures that the returned reference is valid and prevents dangling references.
// The main function is just a placeholder for this example.
// It can be used to test the longest function with different string slices.
// The main function is not necessary for the lifetime demonstration, but it is included for completeness.

/* Struct with lifetimes */
struct Book<'a> {
    title: &'a str,
}

fn main() {
    let str1 = String::from("Hello, world!");
    let str2 = String::from("Hello, Rust!");

    let result = longest(str1.as_str(), str2.as_str());
    println!("The longest string is: {}", result);

    let title = String::from("The Rust Programming Language");
    let book = Book { title: &title }; 
    print!("Book title: {}", book.title);   

}

/* Lifetime Elision Rule */
// The Rust compiler applies lifetime elision rules to simplify function signatures.
// For example, the following function signature is equivalent to the one with explicit lifetimes:
/*
fn longest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
*/
// The compiler infers that the returned reference has the same lifetime as the input references.
// This is because the function signature follows the lifetime elision rules, which allow the compiler to infer lifetimes in certain cases.
// The lifetime elision rules are:
// 1. If there is exactly one input lifetime, it is assigned to the output lifetime.
// 2. If there are multiple input lifetimes, the output lifetime is inferred to be the same as the first input lifetime.
// 3. If there are no input lifetimes, the output lifetime is inferred to be 'static (the entire duration of the program).
// These rules help reduce boilerplate code and make function signatures more concise while still ensuring safety with lifetimes.
// The Rust compiler is able to infer lifetimes in many cases, but sometimes you need to specify them explicitly.
// This is especially true when dealing with complex data structures or multiple references.
// In such cases, you can use explicit lifetimes to ensure that the references are valid and do not lead to dangling pointers.
// Lifetimes are a powerful feature of Rust that help ensure memory safety without needing a garbage collector.
// They allow you to express the relationships between references and their lifetimes, ensuring that references are valid for as long as they are used.
// This example demonstrates the basic concept of lifetimes in Rust and how they can be used to ensure memory safety.
// Lifetimes are a fundamental part of Rust's ownership system, and understanding them is crucial for writing safe and efficient code.
// The example also shows how to use lifetimes with structs, allowing you to create data structures that hold references with specific lifetimes.
// In conclusion, lifetimes in Rust are a powerful feature that helps ensure memory safety and prevent dangling references.

