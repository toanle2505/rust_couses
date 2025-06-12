fn main() { 
    repitition_loop()
}

// 1
fn variables_and_mutability() {
    // Variables are immutable by default
    // To make a variable mutable, use the `mut` keyword
    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; // This will cause an error because x is immutable
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // This is allowed because y is mutable
    println!("The value of y is: {}", y);
}

// 2
fn constants() {
    // Constants are immutable and must have a type annotation
    // They can be declared in any scope, including inside functions
    // Constants can be declared with the `const` keyword
    // Constants are always immutable
    // Constants can be declared with any type, including user-defined types
    // Rust’s naming convention for constants is to use all uppercase with underscores between words
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

// 3
fn shadowing() {
    // Shadowing allows you to reuse a variable name
    // The new variable shadows the old one
    // This is different from mutability because the old variable is still there, but it is not accessible
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1; // This shadows the old x
    println!("The value of x is: {}", x);
    {
        let x = x * 2; // This shadows the old x again
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x); // This will print the outer x
}

// 4
fn data_types() 
{
    // Two data type subsets: scalar and compound
    // Scalar types represent a single value
    // Scalar types: integers, floating-point numbers, booleans, and characters
    // Integers: i8, i16, i32, i64, i128, isize (signed), u8, u16, u32, u64, u128, usize (unsigned)
    // Floating-point numbers: f32, f64
    // Booleans: true or false
    // Characters: a single Unicode character, represented as a char type
    
    
    
    
    // Compound types can group multiple values into one type
    // Compound types: tuples and arrays
    // Tuples: a fixed-size collection of values of different types
    let tup: (i32, f64, char) = (500, 6.4, 'y');

    // To access the values in a tuple, use pattern matching or indexing
    // Pattern matching
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // Indexing
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("The value of x is: {x}");

    // Arrays: a fixed-size collection of values of the same type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // To access the values in an array, use indexing
    let first = arr[0];
    let second = arr[1];
    println!("The first value is: {first}");
    println!("The second value is: {second}");
    // Arrays are stack-allocated, while vectors are heap-allocated
    // Vectors: a growable array type, represented as Vec<T>
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    // To access the values in a vector, use indexing
    let first = vec[0]; 
    let second = vec[1];
    println!("The first value is: {first}");
    println!("The second value is: {second}");
    // Vectors are more flexible than arrays, but they are also less efficient
    // because they are heap-allocated and require more memory management
    // Vectors can grow and shrink in size, while arrays have a fixed size
    // Vectors are created using the vec! macro
    // Vectors can be created with a specific size and initial value
    let vec2: Vec<i32> = vec![0; 5]; // Creates a vector of size 5 with all values set to 0
}

// 5 
fn functions() {
    // Functions are declared with the `fn` keyword
    // Functions can take parameters and return values
    // Functions can be declared with or without a return type
    // Functions can be declared with or without a body
    // Functions can be declared with or without a name
    // Functions can be declared with or without a type annotation
    // Functions can be declared with or without a lifetime annotation
    // Functions can be declared with or without a visibility modifier
    // Functions can be declared with or without a mutability modifier
    // Functions can be declared with or without a const modifier
}

fn another_function(x: i32) -> i32 {
    // This function takes an i32 parameter and returns an i32
    // The return type is specified after the arrow (->)
    // The return type is optional, but it is good practice to include it
    // The return type can be inferred by the compiler, but it is good practice to include it
    x + 1 // This is the return value
}

fn statement_and_expression() {
    // Statements are instructions that perform some action
    // Expressions evaluate to a value
    // Expressions can be used as statements, but statements cannot be used as expressions
    let x = 5; // This is a statement
    let y = {
        let x = 3; // This is an expression
        x + 1 // This is also an expression, and it is the return value of the block
    };
    println!("The value of y is: {}", y);
}

// 6 

fn control_flow_if() {
    // Control flow statements allow you to control the flow of execution in your program
    // The most common control flow statements are if, else if, and else
    // The if statement is used to execute a block of code if a condition is true
    // The else if statement is used to execute a block of code if the previous condition is false and the current condition is true
    // The else statement is used to execute a block of code if all previous conditions are false
    let x: i32 = 5;
    if x < 5 {
        println!("x is less than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is greater than 5");
    }

    let condition = true; // This is a boolean expression
    let number = if condition { 5 } else { 6 }; 
}  

fn repitition_loop() {
    // The loop statement is used to execute a block of code repeatedly
    // The loop statement can be used with or without a condition
    // The loop statement can be used with or without a label
    // The loop statement can be used with or without a break statement
    // The loop statement can be used with or without a continue statement
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break; // This will exit the loop when count is equal to 5
        }
        println!("Count: {}", count);
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // labeling loops
    let mut count = 0;
    'outer: loop {
        println!("Outer loop: {}", count);
        let mut inner_count = 0;
        loop {
            if inner_count == 5 {
                break 'outer; // This will exit the outer loop
            }
            println!("Inner loop: {}", inner_count);
            inner_count += 1;
        }
        count += 1;
    }
    println!("Outer loop exited");
    println!("Count: {}", count);
}

fn repitition_while() {
    // The while statement is used to execute a block of code repeatedly while a condition is true
    // The while statement can be used with or without a label
    // The while statement can be used with or without a break statement
    // The while statement can be used with or without a continue statement
    let mut count = 0;
    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
}

fn repitition_for() {
    // The for statement is used to execute a block of code for each item in a collection
    // The for statement can be used with or without a label
    // The for statement can be used with or without a break statement
    // The for statement can be used with or without a continue statement
    let arr = [1, 2, 3, 4, 5];
    for element in arr.iter() {
        println!("Element: {}", element);
    }

    // Using a range
    for number in 1..6 {
        println!("Number: {}", number);
    }
}