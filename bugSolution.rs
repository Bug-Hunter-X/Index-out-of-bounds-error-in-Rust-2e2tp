fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let number = numbers.get(10);
    match number {
        Some(num) => println!("The number is {}", num),
        None => println!("Index out of bounds"),
    }
} 

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    if let Some(num) = numbers.get(1) {
        println!("The number is: {}", num);
    } else {
        println!("Index out of bounds");
    }
} 