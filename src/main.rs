// In a Rust-based inventory system, items can have various states, such as InStock, OutOfStock, Discontinued, etc. Suppose you initially implement these states using enums but later decide to switch to a trait and struct-based approach.

// Discuss the refactoring process you would undertake to make this change. What are the potential challenges and benefits of transitioning from an enum to a struct/trait-based state representation in this context?

// These questions are designed to test your understanding of Object-Oriented Design Patterns in Rust, particularly focusing on state management using structs, traits, and enums. They also encourage you to think about the design implications and trade-offs of different implementation strategies.
use variables::Inventory;

fn main() {
    let mut stock = Inventory::new(String::from("soap"), 100);
    println!("{}", stock.availability_status());
    stock.sell(50);
    println!("after selling. {}", stock.availability_status());
    stock.purchase(150);
    println!("after purchase. {}", stock.availability_status());
    stock.sell(200);
    println!("after selling. {}", stock.availability_status());
    stock.convert_to_discontinued();
    println!("{}", stock.availability_status());
}