use std::clone;
/*
    THREADS IN RUST

    Threads are a way to run multiple tasks concurrently in Rust.
    They allow you to perform multiple operations at the same time, which can improve performance and responsiveness in applications.
    In Rust, threads are created using the `std::thread` module.    
    Each thread has its own stack and can run independently of other threads.
    Rust provides a safe and efficient way to work with threads, ensuring that datais shared safely between threads using ownership and borrowing rules.
*/
/*
    CHANNELS IN RUST

    Channels are a way to communicate between threads in Rust.
    They allow you to send messages between threads, enabling them to work together and share data.
    Channels are created using the `std::sync::mpsc` module, which stands for "multiple producer, single consumer".
    Channels provide a way to send data from one thread to another, allowing for safe communication and synchronization.   
*/
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
fn main() {

    let v  = vec![1, 2, 3, 4, 5];

    let (sender, receiver) = mpsc::channel(); // Create a channel for communication

    let handle = thread::spawn(move || {
        for i in v {
            println!("Child Thread: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        sender.send("Hello from child thread!").unwrap(); // Send a message to the main thread
    });

    for i in 0..5 {
        println!("Main Thread: {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    let msg = receiver.recv().unwrap(); // Receive the message from the child thread -> this will block until a message is received
    println!("Main Thread: Received message from child thread {}", msg  );
    handle.join().unwrap(); // Wait for the child thread to finish

}

// Example of using Arc to share data between threads
// Arc (Atomic Reference Counted) is a thread-safe reference-counting pointer that allows multiple threads to share ownership of data.
// It is used to safely share data between threads without needing to use locks or other synchronization mechanisms.
// In this example, we create an Arc to share a vector of integers between multiple threads.
// The Arc allows us to clone the data and share it with multiple threads without needing to copy the data itself.
// This is useful when you want to share data between threads without incurring the overhead of copying the data.
fn main2() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];
    
    for _ in 0..3{
        let cloned_data = Arc::clone(&data); // Clone the Arc to share ownership of the data
        let handle = thread::spawn(move || {
            println!("Thread started {:?}", cloned_data);
        });
        handles.push(handle);
    }   

    for handle in handles {
        handle.join().unwrap(); // Wait for each thread to finish
    }
}

// Mutex (Mutual Exclusion) is a synchronization primitive that allows only one thread to access a resource at a time.
// It is used to protect shared data from concurrent access by multiple threads.
// In this example, we use a Mutex to safely share a counter between multiple threads.
fn main3() {
    let mut counter = Arc::new(Mutex::new(0)); // Create an Arc to share a Mutex-wrapped counter
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];

    for _ in 0..3 {
        let cloned_data = Arc::clone(&data); // Clone the Arc to share ownership of the data
        let cloned_counter = Arc::clone(&counter); // Clone the Arc to share ownership of the counter
        let handle = thread::spawn(move || {
            print!("Thread sees : {:?}",cloned_data);
            let mut num = cloned_counter.lock().unwrap(); // Lock the mutex to access the counter safely
            *num += 1; // Increment the counter
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for each thread to finish
    }

}