// src/main.rs

mod bank;
use bank::*;

fn main() {
    let mut bank = Bank::new();

    bank.create_account("Manoj".to_string()).unwrap();
    bank.deposit("Manoj", 1000.0).unwrap();
    bank.withdraw("Manoj", 250.0).unwrap();

    match bank.get_balance("Manoj") {
        Ok(balance) => println!("Balance: {:.2}", balance),
        Err(e) => println!("Error: {:?}", e),
    }
}
    