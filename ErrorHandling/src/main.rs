use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
//Explain : Option Enum in Rust
// The `Option` enum in Rust is used to represent a value that can either be present (`Some`) or absent (`None`).
/*
    pub enum Option<T> {
        Some(T),
        None,
    }
*/
// This is useful for handling cases where a value might not exist, such as when searching for an item in a collection.

fn test_option_enum() {
    let mut scores : HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 50);

    let alice_score = scores.get("Alice");
    match alice_score {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice's score not found"),
    }
}

// Explain : Result Enum in Rust
// The `Result` enum in Rust is used for error handling and represents either a successful outcome (`Ok`) or an error (`Err`).
/*
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
*/
// This is useful for functions that can return a value or an error, allowing for more robust error handling.
// Example of using `Result` enum:

fn test_result_enum() -> Result<i32, String> {
 

    // let file_open_result = File::open("non_existent_file.txt");
    // if let Ok(file) = file_open_result {
    //     println!("File opened successfully: {:?}", file);
    // } else if let Err(e) = file_open_result {
    //     println!("Error opening file: {}", e);
    // }
    
    let x = 10;
    if x > 5 {
        Ok(x * 2)
    } else {
        Err("Value is too small".to_string())
    }
   
}

// Explain : Propagating Errors in Rust
// In Rust, you can propagate errors using the `?` operator. This operator can be used in functions that return a `Result` type.
// When the `?` operator is used, if the result is an `Err`, it will return that error from the function, allowing for easier error handling without needing to match on the result explicitly.
// Example of propagating errors:
fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}
fn main() {
    println!("Hello, world!");
}
