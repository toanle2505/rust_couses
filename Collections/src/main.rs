//String
//&str : Immutable string slice,Fixed size, Cannot be changed
//String : Growable, mutable, heap-allocated string type 


// HashMap
use std::collections::HashMap;

// Iterators
// Iterators are used to process elements in a collection sequentially.
// They provide a way to access elements without exposing the underlying structure.
fn test_iterators(){
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Using an iterator to sum the elements
    let sum: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", sum);

    // Using a filter to get even numbers
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("Even numbers: {:?}", evens);
}

fn test_string(){
    let string_slice: &str = "Hello, world!";
    let mut string_object: String = String::from("Hello, world!");

    string_object.push_str(" How are you?"); // Append to String
    string_object.push('!'); // Append a character

    string_object = string_object.replace("world", "Rust"); // Replace substring
    string_object = string_object.to_uppercase(); // Convert to uppercase
}

fn test_hashmap() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 50);
    scores.insert(String::from("Bob"), 40);
    
    print!("{:#?}", scores); // Print the HashMap

    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    scores.entry("Alice".to_string()).or_insert(60);
}
fn main() {
}
