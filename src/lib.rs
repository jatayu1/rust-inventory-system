// In a Rust-based inventory system, items can have various states, such as InStock, OutOfStock, Discontinued, etc. Suppose you initially implement these states using enums but later decide to switch to a trait and struct-based approach.

// Discuss the refactoring process you would undertake to make this change. What are the potential challenges and benefits of transitioning from an enum to a struct/trait-based state representation in this context?

// These questions are designed to test your understanding of Object-Oriented Design Patterns in Rust, particularly focusing on state management using structs, traits, and enums. They also encourage you to think about the design implications and trade-offs of different implementation strategies.

// count field - convert it to out of stock if count is 0
// add few restrictions

pub struct Inventory {
    good: String,
    current_state: Option<Items>,
    is_available: String,
    quantity: u32,
}

impl Inventory {
    pub fn new(good: String, quantity: u32) -> Self {
        let mut status = good.clone();
        status.push_str(" is available");
        
        Inventory {
            good : good,
            current_state : Some(Items::InStock),
            is_available : status,
            quantity: quantity
        }
    }
    fn convert_to_outofstock(&mut self) {
        if let Some(s) = self.current_state.take() {
            self.current_state = Some(s.convert_to_outofstock());
            self.is_available = self.available();
        }
    }
    fn convert_to_instock(&mut self) {
        if let Some(s) = self.current_state.take() {
            self.current_state = Some(s.convert_to_instock());
            self.is_available = self.available();
        }
    }
    pub fn convert_to_discontinued(&mut self) {
        if let Some(s) = self.current_state.take() {
            self.current_state = Some(s.convert_to_discontinued());
            self.is_available = self.available();
        }
    }
    fn available(& self) -> String {
        let mut status = self.good.clone();
        match self.current_state.as_ref().unwrap() {
            Items::InStock => {
                status.push_str(" is available");
                status
            },
            Items::OutOfStock => {
                status.push_str(" is Out Of Stock");
                status
            },
            Items::Discontinued => {
                status.push_str(" is discontinued");
                status
            }
        }
    }
    pub fn availability_status(&self) -> &str {
        &self.is_available
    }
    pub fn sell(&mut self, quantity:u32) {
        if let Some(ref mut s) = self.current_state {
            match &s {
                Items::InStock => {
                    if self.quantity >= quantity {
                        self.quantity = self.quantity - quantity;
                        println!("{} {} sold, {} remaining", quantity, self.good, self.quantity);
                        if self.quantity == 0 {
                            self.convert_to_outofstock();
                        }
                    }else{
                        println!("not enough {}, only {} available", self.good, self.quantity);
                    }
                },
                Items::OutOfStock => {
                    println!("{} is out of stock", self.good);
                },
                Items::Discontinued => {
                    println!("{} is discontinued", self.good);
                },
            }
        }else{
            println!("none")
        }
    }
    pub fn purchase(&mut self, quantity:u32) {
        if quantity > 0{
            if let Some(ref mut s) = self.current_state {
                match &s {
                    Items::InStock | Items::OutOfStock => {
                        self.quantity = self.quantity + quantity;
                        println!("{} {} baught, {} Available", quantity, self.good, self.quantity);
                        self.convert_to_instock();
                    },
                    Items::Discontinued => {
                        println!("{} is discontinued", self.good);
                    },
                }
            }else{
                println!("none");
            }
        }
    }
    // pub fn stop_dealing(&self) -> &str {
    //     &self.is_available
    // }
}

trait ItemFunctions {
    fn convert_to_outofstock(self) -> Self;
    fn convert_to_instock(self) -> Self;
    fn convert_to_discontinued(self) -> Self;
}
enum Items {
    InStock,
    OutOfStock,
    Discontinued
}

impl ItemFunctions for Items {
    fn convert_to_outofstock(self) -> Self {
        Self::OutOfStock
    }
    fn convert_to_instock(self) -> Self {
        match self {
            Items::InStock | Items::OutOfStock => Items::InStock,
            Items::Discontinued => {
                println!("can't convert to itstock, if it's already Discontinued");
                Items::Discontinued
            },
        }
    }
    fn convert_to_discontinued(self) -> Self {
        match self {
            Items::Discontinued | Items::OutOfStock => Items::Discontinued,
            Items::InStock => {
                println!("can't convert to Discontinued, if it's itstock");
                Items::InStock
            },
        }
    }
}