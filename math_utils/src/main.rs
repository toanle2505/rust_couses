use math_utils::operations::add::add;
use math_utils::operations::multiply::multiply;
use rand::Rng;
fn main() {
    let a = 5;
    let b = 10;
    let result = add(a, b);
    println!("The sum of {} and {} is {}", a, b, result);

    print!("The product of {} and {} is ", a, b);
    let product = multiply(a, b);
    println!("{}", product);

    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=100);
    println!("Random number between 1 and 100: {}", random_number);
}
